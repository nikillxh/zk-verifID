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
use zkpdf_template_lib::{utils::generate_commitment, verify_gst_certificate, PublicValuesStruct};

pub fn main() {
    // Read PDF bytes as input to the program.
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    // Verify the GST certificate and extract information.
    let gst_cert =
        verify_gst_certificate(pdf_bytes.clone()).expect("Failed to verify GST certificate");

    // Generate commitment hash using the new function
    let document_commitment = generate_commitment(&gst_cert);
    let public_key_hash = keccak256(&gst_cert.signature.public_key);

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
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

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
