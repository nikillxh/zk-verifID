// SP1VerifierABI.js
export const SP1VerifierABI = [
  {
    "type":"constructor",
    "inputs":[],
    "stateMutability":"nonpayable"
  },
  {
    "type":"function",
    "name":"sp1Verifier",
    "inputs":[],
    "outputs":[{"name":"","type":"address","internalType":"contract ISP1Verifier"}],
    "stateMutability":"view"
  },
  {
    "type":"function",
    "name":"verifySuccinctProof",
    "inputs":[
      {"name":"vk","type":"bytes","internalType":"bytes"},
      {"name":"proof","type":"bytes","internalType":"bytes"},
      {"name":"publicInputs","type":"uint256[]","internalType":"uint256[]"}
    ],
    "outputs":[{"name":"","type":"bool","internalType":"bool"}],
    "stateMutability":"view"
  }
];
