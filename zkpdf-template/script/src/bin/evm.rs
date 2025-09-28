//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can have an
//! EVM-Compatible proof generated which can be verified on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system plonk
//! ```
//! You can also specify a custom PDF path:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16 --pdf-path path/to/your/certificate.pdf
//! ```

use alloy_sol_types::SolType;
use alloy_primitives::keccak256;
use chrono::{NaiveDate, Utc};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;
use zkpdf_template_lib::{GSTValuesStruct, PANValuesStruct};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZKPDF_TEMPLATE_ELF: &[u8] = include_elf!("zkpdf-template-program");

/// The arguments for the EVM command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct EVMArgs {
    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,
    #[arg(long, default_value = "../samples/PAN-card.pdf")]
    pdf_path: String,
}

/// Enum representing the available proof systems
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1GSTProofFixture {
    gst_number: String,
    legal_name: String,
    signature_valid: bool,
    document_commitment: String,
    public_key_hash: String,
    vkey: String,
    public_values: String,
    proof: String,
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs for PAN card validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1PANProofFixture {
    pan_number_commitment: String,   // Commitment to PAN (not the raw number)
    holder_name: String,             // Legal name on PAN
    dob_commitment: String,          // Commitment to Date of Birth
    age_proof_over18: bool,          // ZK check: is age >= 18
    signature_valid: bool,           // Whether the digital signature on the PAN PDF is valid
    document_commitment: String,     // Commitment to the full PAN PDF
    public_key_hash: String,         // Hash of the issuer's signing key
    vkey: String,                    // Verification key
    public_values: String,           // Public values from zkVM
    proof: String,                   // The actual proof bytes
}

/// A fixture that can be used to test the verification of SP1 zkVM proofs for Driving License validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1DLProofFixture {
    dl_number_commitment: String,     // Commitment to Driving License number
    holder_name: String,              // Name of the license holder
    issuing_authority_commitment: String, // RTO authority commitment (so it’s verifiable but hidden)
    license_type: String,             // e.g., LMV, MCWG, Commercial (could be revealed)
    expiry_valid: bool,               // License not expired (ZK-checked inside circuit)
    age_proof_over18: bool,           // ZK check: holder is >= 18
    signature_valid: bool,            // Issuer’s digital signature on DL verified
    document_commitment: String,      // Commitment to the entire DL PDF/image
    public_key_hash: String,          // Hash of issuer's signing key
    vkey: String,                     // Verification key
    public_values: String,            // Public values exposed by zkVM
    proof: String,                    // Proof bytes
}


fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = EVMArgs::parse();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the program.
    let (pk, vk) = client.setup(ZKPDF_TEMPLATE_ELF);

    // Read PDF bytes from file
    let pdf_bytes = std::fs::read(&args.pdf_path)
        .unwrap_or_else(|_| panic!("Failed to read PDF file from: {}", args.pdf_path));

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&pdf_bytes);

    println!("PDF Path: {}", args.pdf_path);
    println!("PDF Size: {} bytes", pdf_bytes.len());
    println!("Proof System: {:?}", args.system);

    // Generate the proof based on the selected proof system.
    let proof = match args.system {
        ProofSystem::Plonk => client.prove(&pk, &stdin).plonk().run(),
        ProofSystem::Groth16 => client.prove(&pk, &stdin).groth16().run(),
    }
    .expect("failed to generate proof");

    create_proof_fixture(&proof, &vk, args.system);
}

/// Create a fixture for the given proof (PAN or GST).
fn create_proof_fixture(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
    system: ProofSystem,
) {
    let bytes = proof.public_values.as_slice();

    // Try decoding as GST first
    if let Ok(GSTValuesStruct {
        gst_number,
        legal_name,
        signature_valid,
        document_commitment,
        public_key_hash,
    }) = GSTValuesStruct::abi_decode(bytes)
    {
        let fixture = SP1GSTProofFixture {
            gst_number,
            legal_name,
            signature_valid,
            document_commitment: format!("0x{}", hex::encode(document_commitment.as_ref() as &[u8])),
            public_key_hash: format!("0x{}", hex::encode(public_key_hash.as_ref() as &[u8])),
            vkey: vk.bytes32().to_string(),
            public_values: format!("0x{}", hex::encode(bytes)),
            proof: format!("0x{}", hex::encode(proof.bytes())),
        };

        save_fixture(&fixture, system);
        return;
    }

    // If not GST, try decoding as PAN
    if let Ok(PANValuesStruct {
        pan_number,
        legal_name,
        dob,
        signature_valid,
        document_commitment,
        public_key_hash,
    }) = PANValuesStruct::abi_decode(bytes)
    {
        // commitments
        let pan_number_commitment =
            format!("0x{}", hex::encode(keccak256(pan_number.as_bytes())));
        let holder_name =
            format!("0x{}", hex::encode(keccak256(legal_name.as_bytes())));
        let dob_commitment =
            format!("0x{}", hex::encode(keccak256(dob.as_bytes())));

        // age calculation
        let dob_parsed = NaiveDate::parse_from_str(&dob, "%Y-%m-%d")
            .expect("DOB must be in YYYY-MM-DD format");
        let today = Utc::now().naive_utc().date();
        let age = today.years_since(dob_parsed).unwrap_or(0);
        let age_proof_over18 = age >= 18;

        let fixture = SP1PANProofFixture {
            signature_valid,
            document_commitment: format!("0x{}", hex::encode(document_commitment.as_ref() as &[u8])),
            public_key_hash: format!("0x{}", hex::encode(public_key_hash.as_ref() as &[u8])),
            vkey: vk.bytes32().to_string(),
            public_values: format!("0x{}", hex::encode(bytes)),
            proof: format!("0x{}", hex::encode(proof.bytes())),
            pan_number_commitment,
            holder_name,
            dob_commitment,
            age_proof_over18,
        };

        save_fixture(&fixture, system);
        return;
    }

    panic!("Public values could not be decoded as GST or PAN struct!");
}

/// Helper to save fixture JSON
fn save_fixture<T: serde::Serialize>(fixture: &T, system: ProofSystem) {
    println!("Verification Key: {}", serde_json::to_string_pretty(&fixture).unwrap());

    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", system).to_lowercase()),
        serde_json::to_string_pretty(fixture).unwrap(),
    )
    .expect("failed to write fixture");
}
