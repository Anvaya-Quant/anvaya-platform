export function computeBlochVector(state: Float64Array): [number, number, number] {
  if (state.length < 4) throw new Error('State vector must have at least 4 elements');
  const re0 = state[0];
  const im0 = state[1];
  const re1 = state[2];
  const im1 = state[3];

  const alphaBetaConjReal = re0 * re1 + im0 * im1;
  const alphaBetaConjImag = im0 * re1 - re0 * im1;

  const x = 2 * alphaBetaConjReal;
  const y = 2 * alphaBetaConjImag;
  const z = (re0 * re0 + im0 * im0) - (re1 * re1 + im1 * im1);

  return [x, y, z];
}
