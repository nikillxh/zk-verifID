//! GST Certificate Verification Program
//!
//! This program verifies GST certificate PDFs and extracts key information:
//! - GST number
//! - Legal name  
//! - Digital signature validity
//! - Document commitment hash
//! - Public key hash
//!
//! The program runs inside the SP1 zkVM to generate zero-knowledge proofs
//! that prove the document is valid without revealing sensitive data.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_primitives::keccak256;
use alloy_sol_types::SolType;
use zkpdf_template_lib::{utils::{gst_generate_commitment, pan_generate_commitment}, verify_gst_certificate, verify_pan_certificate, GSTValuesStruct, PANValuesStruct};

pub fn main() {
    // Read PDF bytes from the prover
    let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    // Try verifying GST first
    if let Ok(gst_cert) = verify_gst_certificate(pdf_bytes.clone()) {
        let document_commitment = gst_generate_commitment(&gst_cert);
        let public_key_hash = keccak256(&gst_cert.signature.public_key);

        let gst_bytes = GSTValuesStruct::abi_encode(&GSTValuesStruct {
            gst_number: gst_cert.gst_number,
            legal_name: gst_cert.legal_name,
            signature_valid: gst_cert.signature.is_valid,
            document_commitment: document_commitment
                .as_slice()
                .try_into()
                .expect("Failed to convert document commitment to FixedBytes"),
            public_key_hash: public_key_hash
                .as_slice()
                .try_into()
                .expect("Failed to convert public key hash to FixedBytes"),
        });

        sp1_zkvm::io::commit_slice(&gst_bytes);
        return; // Stop here since GST certificate was found
    }

    // If GST verification fails, try PAN
    if let Ok(pan_cert) = verify_pan_certificate(pdf_bytes) {
        let document_commitment = pan_generate_commitment(&pan_cert);
        let public_key_hash = keccak256(&pan_cert.signature.public_key);

        let pan_bytes = PANValuesStruct::abi_encode(&PANValuesStruct {
            pan_number: pan_cert.pan_number,
            legal_name: pan_cert.legal_name,
            signature_valid: pan_cert.signature.is_valid,
            document_commitment: document_commitment
                .as_slice()
                .try_into()
                .expect("Failed to convert document commitment to FixedBytes"),
            public_key_hash: public_key_hash
                .as_slice()
                .try_into()
                .expect("Failed to convert public key hash to FixedBytes"),
        });

        sp1_zkvm::io::commit_slice(&pan_bytes);
        return; // Stop here since PAN certificate was found
    }

    // If neither GST nor PAN was found, fail the program
    panic!("No valid GST or PAN certificate found in PDF");
}
