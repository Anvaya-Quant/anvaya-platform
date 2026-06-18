use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Channel {
    Drive(usize),
    Measure(usize),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PulseShape {
    Gaussian { sigma: f64 },
    Square,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pulse {
    pub shape: PulseShape,
    pub amplitude: f64,
    pub duration: f64,
    pub frequency: f64,
    pub phase: f64,
    pub channel: Channel,
}

impl Pulse {
    pub fn new(shape: PulseShape, amplitude: f64, duration: f64, channel: Channel) -> Self {
        Pulse {
            shape,
            amplitude,
            duration,
            frequency: 0.0,
            phase: 0.0,
            channel,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledPulse {
    pub pulse: Pulse,
    pub start_time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PulseSequence {
    pub pulses: Vec<ScheduledPulse>,
    pub total_duration: f64,
}
