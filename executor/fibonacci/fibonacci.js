const { FGL } = require("pil-stark");

class FibonacciJS {
  async buildConstants(pols_) {
    const pols = pols_.Fibonacci;
    const N = pols.L1.length;
    for (let i = 0; i < N; i++) {
      pols.L1[i] = (i == 0) ? 1n : 0n;
      pols.LLAST[i] = (i == N-1) ? 1n : 0n;
    }
  }

  async execute(pols_, input) {
    const pols = pols_.Fibonacci;
    const N = pols.l1.length;
    pols.l2[0] = BigInt(input[0]);
    pols.l1[0] = BigInt(input[1]);

    for (let i = 1; i < N; i ++) {
      pols.l2[i] =pols.l1[i-1];
      pols.l1[i] =FGL.add(FGL.square(pols.l2[i-1]), FGL.square(pols.l1[i-1]));
    }
    return pols.l1[N - 1];
  }
}

module.exports = FibonacciJS;