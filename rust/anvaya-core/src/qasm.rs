use crate::circuit::{Circuit, GateOperation};
use crate::gate::Gate;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

// ── Tokenizer ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Number(f64),
    Semicolon,
    Comma,
    LSquare,
    RSquare,
    LParen,
    RParen,
    Arrow,     // ->
    Include,   // 'include' keyword
    OpenQasm,  // 'OPENQASM' keyword
    Qubit,     // 'qubit' keyword
    Bit,       // 'bit' keyword
    BarrierKw, // 'barrier'
    MeasureKw, // 'measure'
    StringLit(String),
    Eof,
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();
        match self.chars.next() {
            None => Ok(Token::Eof),
            Some(';') => Ok(Token::Semicolon),
            Some(',') => Ok(Token::Comma),
            Some('[') => Ok(Token::LSquare),
            Some(']') => Ok(Token::RSquare),
            Some('(') => Ok(Token::LParen),
            Some(')') => Ok(Token::RParen),
            Some('-') => {
                if self.chars.peek() == Some(&'>') {
                    self.chars.next();
                    Ok(Token::Arrow)
                } else {
                    Err("unexpected '-' not part of '->'".to_string())
                }
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut ident = String::from(c);
                while let Some(&nc) = self.chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        ident.push(self.chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "OPENQASM" => Ok(Token::OpenQasm),
                    "include" => Ok(Token::Include),
                    "qubit" => Ok(Token::Qubit),
                    "bit" => Ok(Token::Bit),
                    "barrier" => Ok(Token::BarrierKw),
                    "measure" => Ok(Token::MeasureKw),
                    _ => Ok(Token::Ident(ident)),
                }
            }
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let mut num_str = String::from(c);
                while let Some(&nc) = self.chars.peek() {
                    if nc.is_ascii_digit()
                        || nc == '.'
                        || nc == 'e'
                        || nc == 'E'
                        || nc == '+'
                        || nc == '-'
                    {
                        if nc == '+' || nc == '-' {
                            let prev = num_str.chars().last().unwrap();
                            if prev != 'e' && prev != 'E' {
                                break;
                            }
                        }
                        num_str.push(self.chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let value: f64 = num_str
                    .parse()
                    .map_err(|e| format!("bad number '{}': {}", num_str, e))?;
                Ok(Token::Number(value))
            }
            Some('"') => {
                let mut s = String::new();
                while let Some(&nc) = self.chars.peek() {
                    if nc == '"' {
                        self.chars.next();
                        break;
                    }
                    s.push(self.chars.next().unwrap());
                }
                Ok(Token::StringLit(s))
            }
            Some(other) => Err(format!("unexpected character: '{}'", other)),
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() {
                    self.chars.next();
                } else {
                    break;
                }
            }
            if self.chars.peek() == Some(&'/') {
                let mut iter = self.chars.clone();
                iter.next();
                if iter.peek() == Some(&'/') {
                    self.chars.next();
                    self.chars.next();
                    while let Some(&c) = self.chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.chars.next();
                    }
                    continue;
                }
            }
            break;
        }
    }
}

// ── Parser ─────────────────────────────────────────────────────────────────

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current: Option<Token>,
    qubit_map: HashMap<String, usize>,
    qubit_count: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(input),
            current: None,
            qubit_map: HashMap::new(),
            qubit_count: 0,
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current = Some(self.tokenizer.next_token()?);
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current.as_ref() == Some(&expected) {
            self.advance()
        } else {
            Err(format!("expected {:?}, got {:?}", expected, self.current))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.current.clone() {
            Some(Token::Ident(s)) => {
                self.advance()?;
                Ok(s)
            }
            _ => Err(format!("expected identifier, got {:?}", self.current)),
        }
    }

    fn expect_number(&mut self) -> Result<f64, String> {
        match self.current {
            Some(Token::Number(n)) => {
                self.advance()?;
                Ok(n)
            }
            _ => Err(format!("expected number, got {:?}", self.current)),
        }
    }

    fn parse(&mut self) -> Result<Circuit, String> {
        self.advance()?;
        if self.current == Some(Token::OpenQasm) {
            self.advance()?;
            let _ver = self.expect_number()?;
            self.expect(Token::Semicolon)?;
        }
        while self.current == Some(Token::Include) {
            self.advance()?;
            while self.current != Some(Token::Semicolon) && self.current != Some(Token::Eof) {
                self.advance()?;
            }
            if self.current == Some(Token::Semicolon) {
                self.advance()?;
            }
        }
        while self.current == Some(Token::Qubit) {
            self.advance()?;
            let size = if self.current == Some(Token::LSquare) {
                self.advance()?;
                let n = self.expect_number()? as usize;
                self.expect(Token::RSquare)?;
                n
            } else {
                1
            };
            let name = self.expect_ident()?;
            self.expect(Token::Semicolon)?;
            self.qubit_map.insert(name, self.qubit_count);
            self.qubit_count += size;
        }
        while self.current == Some(Token::Bit) {
            self.advance()?;
            if self.current == Some(Token::LSquare) {
                self.advance()?;
                let _n = self.expect_number()?;
                self.expect(Token::RSquare)?;
            }
            let _name = self.expect_ident()?;
            self.expect(Token::Semicolon)?;
        }
        let mut circuit = Circuit::new(self.qubit_count);
        loop {
            if self.current.is_none() || self.current == Some(Token::Eof) {
                break;
            }
            self.parse_gate_operation(&mut circuit)?;
        }
        Ok(circuit)
    }

    fn resolve_qubit(&self, name: &str, index: Option<usize>) -> Result<usize, String> {
        let base = self
            .qubit_map
            .get(name)
            .ok_or_else(|| format!("unknown qubit variable '{}'", name))?;
        if let Some(off) = index {
            Ok(base + off)
        } else {
            Ok(*base)
        }
    }

    fn parse_qubit_ref(&mut self) -> Result<(String, Option<usize>), String> {
        let name = self.expect_ident()?;
        if self.current == Some(Token::LSquare) {
            self.advance()?;
            let idx = self.expect_number()? as usize;
            self.expect(Token::RSquare)?;
            Ok((name, Some(idx)))
        } else {
            Ok((name, None))
        }
    }

    fn parse_gate_operation(&mut self, circuit: &mut Circuit) -> Result<(), String> {
        match self.current.clone() {
            Some(Token::MeasureKw) => {
                self.advance()?;
                let (qname, qidx) = self.parse_qubit_ref()?;
                self.expect(Token::Arrow)?;
                let _ = self.parse_qubit_ref()?;
                self.expect(Token::Semicolon)?;
                let qubit = self.resolve_qubit(&qname, qidx)?;
                circuit
                    .add_gate(Gate::Measure, vec![qubit])
                    .map_err(|e| format!("circuit error: {:?}", e))?;
                return Ok(());
            }
            Some(Token::BarrierKw) => {
                self.advance()?;
                let mut targets = vec![];
                loop {
                    let (qname, qidx) = self.parse_qubit_ref()?;
                    targets.push(self.resolve_qubit(&qname, qidx)?);
                    if self.current == Some(Token::Comma) {
                        self.advance()?;
                    } else {
                        break;
                    }
                }
                self.expect(Token::Semicolon)?;
                circuit
                    .add_gate(Gate::Barrier, targets)
                    .map_err(|e| format!("circuit error: {:?}", e))?;
                return Ok(());
            }
            _ => {}
        }
        let gate_name = self.expect_ident()?.to_lowercase();
        let angle = if matches!(gate_name.as_str(), "rx" | "ry" | "rz") {
            self.expect(Token::LParen)?;
            let a = self.expect_number()?;
            self.expect(Token::RParen)?;
            Some(a)
        } else {
            None
        };
        let mut targets = vec![];
        loop {
            let (qname, qidx) = self.parse_qubit_ref()?;
            targets.push(self.resolve_qubit(&qname, qidx)?);
            if self.current == Some(Token::Comma) {
                self.advance()?;
            } else {
                break;
            }
        }
        self.expect(Token::Semicolon)?;
        let gate = match gate_name.as_str() {
            "x" => Gate::X,
            "y" => Gate::Y,
            "z" => Gate::Z,
            "h" => Gate::H,
            "s" => Gate::S,
            "t" => Gate::T,
            "rx" => Gate::Rx(angle.ok_or("missing angle for rx")?),
            "ry" => Gate::Ry(angle.ok_or("missing angle for ry")?),
            "rz" => Gate::Rz(angle.ok_or("missing angle for rz")?),
            "cx" | "cnot" => Gate::CNOT,
            "cz" => Gate::CZ,
            "swap" => Gate::SWAP,
            _ => return Err(format!("unsupported gate: {}", gate_name)),
        };
        circuit
            .add_gate(gate, targets)
            .map_err(|e| format!("circuit error: {:?}", e))?;
        Ok(())
    }
}

/// Parse an OpenQASM 3.0 string into a `Circuit`.
pub fn parse_qasm(input: &str) -> Result<Circuit, String> {
    let mut parser = Parser::new(input);
    parser.parse()
}

// ── Exporter ───────────────────────────────────────────────────────────────

/// Export a `Circuit` to an OpenQASM 3.0 string.
pub fn to_qasm(circuit: &Circuit) -> String {
    let mut s = String::new();
    s.push_str("OPENQASM 3.0;\n");
    s.push_str("include \"stdgates.inc\";\n");
    s.push_str(&format!("qubit[{}] q;\n", circuit.num_qubits));
    s.push_str(&format!("bit[{}] c;\n", circuit.num_qubits));
    s.push('\n');

    for GateOperation { gate, targets } in &circuit.operations {
        match gate {
            Gate::Measure => {
                let q = targets[0];
                s.push_str(&format!("measure q[{}] -> c[{}];\n", q, q));
            }
            Gate::Barrier => {
                let targs: Vec<String> = targets.iter().map(|t| format!("q[{}]", t)).collect();
                s.push_str(&format!("barrier {};\n", targs.join(", ")));
            }
            _ => {
                let gate_str = gate_to_qasm(gate);
                let targs: Vec<String> = targets.iter().map(|t| format!("q[{}]", t)).collect();
                s.push_str(&format!("{} {};\n", gate_str, targs.join(", ")));
            }
        }
    }
    s
}

fn gate_to_qasm(gate: &Gate) -> String {
    match gate {
        Gate::X => "x".into(),
        Gate::Y => "y".into(),
        Gate::Z => "z".into(),
        Gate::H => "h".into(),
        Gate::S => "s".into(),
        Gate::T => "t".into(),
        Gate::Rx(theta) => format!("rx({})", theta),
        Gate::Ry(theta) => format!("ry({})", theta),
        Gate::Rz(theta) => format!("rz({})", theta),
        Gate::CNOT => "cx".into(),
        Gate::CZ => "cz".into(),
        Gate::SWAP => "swap".into(),
        Gate::Measure | Gate::Barrier => unreachable!(),
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulator::simulate;
    use approx::assert_abs_diff_eq;

    #[test]
    fn parse_simple_circuit() {
        let qasm = r#"
            OPENQASM 3.0;
            include "stdgates.inc";
            qubit[2] q;
            bit[2] c;
            h q[0];
            cx q[0], q[1];
        "#;
        let circuit = parse_qasm(qasm).expect("parse ok");
        assert_eq!(circuit.num_qubits, 2);
        assert_eq!(circuit.operations.len(), 2);
        let state = simulate(&circuit).unwrap();
        let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
        assert_abs_diff_eq!(state[0].re, inv_sqrt2, epsilon = 1e-10);
        assert_abs_diff_eq!(state[3].re, inv_sqrt2, epsilon = 1e-10);
    }

    #[test]
    fn roundtrip_circuit() {
        let mut original = Circuit::new(2);
        original.add_gate(Gate::Rx(0.5), vec![0]).unwrap();
        original.add_gate(Gate::H, vec![1]).unwrap();
        original.add_gate(Gate::CNOT, vec![0, 1]).unwrap();
        original.add_gate(Gate::Measure, vec![0]).unwrap();
        original.add_gate(Gate::Barrier, vec![0, 1]).unwrap();

        let qasm = to_qasm(&original);
        let parsed = parse_qasm(&qasm).expect("roundtrip parse");
        let orig_state = simulate(&original).unwrap();
        let parsed_state = simulate(&parsed).unwrap();
        for (a, b) in orig_state.iter().zip(parsed_state.iter()) {
            assert_abs_diff_eq!(a.re, b.re, epsilon = 1e-10);
            assert_abs_diff_eq!(a.im, b.im, epsilon = 1e-10);
        }
    }

    #[test]
    fn parse_rotation_with_pi() {
        let qasm =
            "OPENQASM 3.0;\ninclude \"stdgates.inc\";\nqubit[1] q;\nrx(3.141592653589793) q[0];\n";
        let circuit = parse_qasm(qasm).unwrap();
        if let Gate::Rx(angle) = &circuit.operations[0].gate {
            assert_abs_diff_eq!(*angle, std::f64::consts::PI, epsilon = 1e-10);
        } else {
            panic!("expected Rx");
        }
    }

    #[test]
    fn parse_barrier_and_measure() {
        let qasm = "OPENQASM 3.0;\ninclude \"stdgates.inc\";\nqubit[2] q;\nbit[2] c;\nbarrier q[0], q[1];\nmeasure q[0] -> c[0];\n";
        let circuit = parse_qasm(qasm).unwrap();
        assert_eq!(circuit.operations.len(), 2);
        assert_eq!(circuit.operations[0].gate, Gate::Barrier);
        assert_eq!(circuit.operations[0].targets, vec![0, 1]);
        assert_eq!(circuit.operations[1].gate, Gate::Measure);
        assert_eq!(circuit.operations[1].targets, vec![0]);
    }
}
