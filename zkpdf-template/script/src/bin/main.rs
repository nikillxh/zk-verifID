//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```
//! You can also specify a custom PDF path:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute --pdf-path path/to/your/certificate.pdf
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use zkpdf_template_lib::{GSTValuesStruct, PANValuesStruct};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZKPDF_TEMPLATE_ELF: &[u8] = include_elf!("zkpdf-template-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=5))]
    kind: u8,

    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "../samples/PAN-card.pdf")]
    pdf_path: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Read PDF bytes from file
    let pdf_bytes = std::fs::read(&args.pdf_path)
        .unwrap_or_else(|_| panic!("Failed to read PDF file from: {}", args.pdf_path));

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&pdf_bytes);

    println!("PDF Path: {}", args.pdf_path);
    println!("PDF Size: {} bytes", pdf_bytes.len());

    // GST Certificate
    if args.execute && args.kind == 0 {
        // Execute the program
        let (output, report) = client.execute(ZKPDF_TEMPLATE_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = GSTValuesStruct::abi_decode(output.as_slice()).unwrap();
        let GSTValuesStruct {
            gst_number,
            legal_name,
            signature_valid,
            document_commitment,
            public_key_hash,
        } = decoded;
        println!("GST Number: {}", gst_number);
        println!("Legal Name: {}", legal_name);
        println!("Signature Valid: {}", signature_valid);
        println!(
            "Document Commitment: 0x{}",
            hex::encode(document_commitment.as_ref() as &[u8])
        );
        println!(
            "Public Key Hash: 0x{}",
            hex::encode(public_key_hash.as_ref() as &[u8])
        );

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ZKPDF_TEMPLATE_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }

    // PAN Card
    if args.execute && args.kind == 1 {
        // Execute the program
        let (output, report) = client.execute(ZKPDF_TEMPLATE_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PANValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PANValuesStruct {
            signature_valid,
            document_commitment,
            public_key_hash,
            pan_number,
            legal_name,
            dob,
        } = decoded;
        println!("PAN Number: {}", pan_number);
        println!("Signature Valid: {}", signature_valid);
        println!(
            "Document Commitment: 0x{}",
            hex::encode(document_commitment.as_ref() as &[u8])
        );
        println!(
            "Public Key Hash: 0x{}",
            hex::encode(public_key_hash.as_ref() as &[u8])
        );

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ZKPDF_TEMPLATE_ELF);

        // Generate the proof
        let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
