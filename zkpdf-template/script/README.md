# GST PDF Verification API

Web API for generating and verifying GST certificate proofs.

## Setup

1. Copy environment file:

```bash
cp .env.example .env
```

2. Set required environment variables in `.env`:

```
SP1_PROVER=network
NETWORK_PRIVATE_KEY=your_private_key_here
```

## Running the API

Start the server:

```bash
cargo run --package zkpdf-template-script --bin api
```

The API will be available at `http://localhost:3002`

## Endpoints

### `GET /`

Serves the web interface for uploading PDFs and generating proofs.

### `POST /prove`

Generates a zero-knowledge proof for a GST certificate PDF.

**Request:**

```json
{
  "pdf_bytes": [1, 2, 3, ...]
}
```

**Response:**
Returns the generated proof with public values.

### `POST /verify`

Verifies a previously generated proof.

**Request:**

```json
{
  "proof": "...",
  "public_values": "..."
}
```

**Response:**

```json
{
  "valid": true,
  "error": null
}
```

## Web Interface

The web interface allows you to:

- Upload GST certificate PDFs
- Generate zero-knowledge proofs
- Verify proofs
- View verification results

## CLI Tools

### Execute Program

```bash
RUST_LOG=info cargo run --package zkpdf-template-script -- --execute
```

### Generate Proof

```bash
RUST_LOG=info cargo run --package zkpdf-template-script -- --prove
```

### Generate EVM Proof

```bash
RUST_LOG=info cargo run --package zkpdf-template-script --bin evm -- --system groth16
```

### Get Verification Key

```bash
RUST_LOG=info cargo run --package zkpdf-template-script --bin vkey
```

## Custom PDF Path

Use your own PDF file:

```bash
RUST_LOG=info cargo run --package zkpdf-template-script -- --execute --pdf-path /path/to/certificate.pdf
```
