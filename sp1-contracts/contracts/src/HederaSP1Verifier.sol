// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface ISP1Verifier {
    function verifyProof(
        bytes calldata vk,
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external view returns (bool);
}

contract HederaSP1Verifier {
    ISP1Verifier public sp1Verifier;

    constructor(address _sp1Verifier) {
        sp1Verifier = ISP1Verifier(_sp1Verifier);
    }

    function verifySuccinctProof(
        bytes calldata vk,
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external view returns (bool) {
        return sp1Verifier.verifyProof(vk, proof, publicInputs);
    }
}
