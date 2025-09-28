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

    constructor() {
        sp1Verifier = ISP1Verifier(0x3B6041173B80E77f038f3F2C0f9744f04837185e);
    }

    function verifySuccinctProof(
        bytes calldata vk,
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external view returns (bool) {
        return sp1Verifier.verifyProof(vk, proof, publicInputs);
    }
}
