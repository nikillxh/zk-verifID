# SP1 zkPDF Template

This is a template for creating [zkPDF](https://github.com/privacy-ethereum/zkpdf) projects using [SP1](https://github.com/succinctlabs/sp1) that verifies PDF document content and extracts information from PDF files to generate zero-knowledge proofs.

## What This Template Does

This is an example repository that demonstrates how to verify digitally signed PDFs in ZKVM:

- **Digital Signature Verification**: Shows how to validate PDF digital signatures
- **Content Extraction**: Demonstrates extracting information from PDF content (GST numbers, legal names, etc.)
- **Zero-Knowledge Proofs**: Example of generating proofs that verify document validity without revealing sensitive data
- **On-Chain Verification**: Complete example with smart contracts for verifying proofs on Ethereum

You can check out the digitally signed PDF in `samples/` and see how the verification process works.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## Running the Project

There are 3 main ways to run this project: execute a program, generate a core proof, and generate an EVM-compatible proof.

### Build the Program

The program is automatically built through `script/build.rs` when the script is built.

### Execute the Program

To run the program without generating a proof:

```sh
cd script
RUST_LOG=info cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
RUST_LOG=info cargo run --release -- --prove
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 16GB RAM to generate a Groth16 or PLONK proof. View the [SP1 docs](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) for more information.

Generating a proof that is cheap to verify on the EVM (e.g. Groth16 or PLONK) is more intensive than generating a core proof.

To generate a Groth16 proof:

```sh
cd script
RUST_LOG=info cargo run --release --bin evm -- --system groth16
```

To generate a PLONK proof:

```sh
cd script
RUST_LOG=info cargo run --release --bin evm -- --system plonk
```

These commands will also generate fixtures that can be used to test the verification of SP1 proofs inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command in `script`:

```sh
cargo run --release --bin vkey
```

## Using the Prover Network

We highly recommend using the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction) for any non-trivial programs or benchmarking purposes. For more information, see the [key setup guide](https://docs.succinct.xyz/docs/network/developers/key-setup) to get started.

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `NETWORK_PRIVATE_KEY` environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following command:

```sh
cd script
RUST_LOG=info SP1_PROVER=network NETWORK_PRIVATE_KEY=... cargo run --release --bin evm
```

## Web API

The template includes a web API for easy integration:

```sh
cd script
RUST_LOG=info cargo run --package zkpdf-template-script --bin api
```

Visit `http://localhost:3000` to use the web interface.

## Custom PDF Path

You can specify a custom PDF path:

```sh
RUST_LOG=info cargo run --package zkpdf-template-script -- --execute --pdf-path /path/to/your/certificate.pdf
```

## Project Structure

- `program/` - Core ZK program for PDF verification
- `lib/` - Shared library with verification logic
- `script/` - CLI tools and web API
- `contracts/` - Solidity contracts for on-chain verification
- `samples/` - Example PDF documents
