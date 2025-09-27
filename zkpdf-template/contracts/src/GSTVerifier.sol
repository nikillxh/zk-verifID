// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

struct PublicValuesStruct {
    string gst_number;
    string legal_name;
    bool signature_valid;
    bytes32 document_commitment;
    bytes32 public_key_hash;
}

/// @title GSTVerifier.
/// @author Succinct Labs
/// @notice This contract implements GST certificate verification using SP1 zkVM proofs.
contract GSTVerifier {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public verifier;

    /// @notice The verification key for the GST verification program.
    bytes32 public gstProgramVKey;

    /// @notice Mapping to store verified GST certificates
    mapping(bytes32 => bool) public verifiedCertificates;

    /// @notice Mapping to store verified public key hashes
    mapping(bytes32 => bool) public verifiedPublicKeys;

    /// @notice Event emitted when a GST certificate is verified
    event GSTCertificateVerified(
        string indexed gst_number,
        string legal_name,
        bytes32 document_commitment,
        bytes32 public_key_hash
    );

    constructor(address _verifier, bytes32 _gstProgramVKey) {
        verifier = _verifier;
        gstProgramVKey = _gstProgramVKey;
    }

    /// @notice The entrypoint for verifying the proof of a GST certificate.
    /// @param _publicValues The encoded public values.
    /// @param _proofBytes The encoded proof.
    function verifyGSTProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (string memory, string memory, bool, bytes32, bytes32)
    {
        ISP1Verifier(verifier).verifyProof(gstProgramVKey, _publicValues, _proofBytes);
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));
        return (
            publicValues.gst_number,
            publicValues.legal_name,
            publicValues.signature_valid,
            publicValues.document_commitment,
            publicValues.public_key_hash
        );
    }

    /// @notice Verify GST certificate and store the verification result
    /// @param _publicValues The encoded public values.
    /// @param _proofBytes The encoded proof.
    function verifyAndStoreGST(bytes calldata _publicValues, bytes calldata _proofBytes)
        external
        returns (string memory, string memory, bool, bytes32, bytes32)
    {
        ISP1Verifier(verifier).verifyProof(gstProgramVKey, _publicValues, _proofBytes);
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));
        
        // Store verification results
        verifiedCertificates[publicValues.document_commitment] = true;
        verifiedPublicKeys[publicValues.public_key_hash] = true;

        // Emit event
        emit GSTCertificateVerified(
            publicValues.gst_number,
            publicValues.legal_name,
            publicValues.document_commitment,
            publicValues.public_key_hash
        );

        return (
            publicValues.gst_number,
            publicValues.legal_name,
            publicValues.signature_valid,
            publicValues.document_commitment,
            publicValues.public_key_hash
        );
    }

    /// @notice Check if a document commitment has been verified
    /// @param _documentCommitment The document commitment to check
    function isDocumentVerified(bytes32 _documentCommitment) external view returns (bool) {
        return verifiedCertificates[_documentCommitment];
    }

    /// @notice Check if a public key hash has been verified
    /// @param _publicKeyHash The public key hash to check
    function isPublicKeyVerified(bytes32 _publicKeyHash) external view returns (bool) {
        return verifiedPublicKeys[_publicKeyHash];
    }
}
