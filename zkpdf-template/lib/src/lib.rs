//! GST Certificate Verification Library
//!
//! This library provides functions for verifying GST certificates and extracting
//! key information from PDF documents. It handles PDF parsing, signature verification,
//! and data extraction with proper error handling.

use alloy_sol_types::sol;

use zkpdf_lib::{verify_and_extract, PdfSignatureResult};

use crate::utils::GSTVerificationError;

pub mod utils;

pub struct GSTCertificate {
    pub gst_number: String,
    pub legal_name: String,
    pub signature: PdfSignatureResult,
}

pub struct GSTCertificate {
    pub pan_number: String,
    pub legal_name: String,
    pub signature: PdfSignatureResult,
}

gstsol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        string gst_number;
        string legal_name;
        bool signature_valid;
        bytes32 document_commitment;
        bytes32 public_key_hash;
    }
}

pansol! {
    struct PublicValuesStruct {
        string gst_number;
        string legal_name;
        bool signature_valid;
        bytes32 document_commitment;
        bytes32 public_key_hash;
    }
}

/// GST Certificate verification function that extracts legal name and GST number
pub fn verify_gst_certificate(pdf_bytes: Vec<u8>) -> Result<GSTCertificate, GSTVerificationError> {
    let verified_content = verify_and_extract(pdf_bytes)
        .map_err(|e| GSTVerificationError::PdfVerificationFailed(e.to_string()))?;

    let full_text = verified_content.pages.join(" ");

    let gst_pattern =
        regex::Regex::new(r"([0-9]{2}[A-Z]{5}[0-9]{4}[A-Z]{1}[1-9A-Z]{1}[Z]{1}[0-9A-Z]{1})")
            .map_err(|e| GSTVerificationError::RegexCompilationFailed(e.to_string()))?;

    let gst_number = gst_pattern
        .captures(&full_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or(GSTVerificationError::GSTNumberNotFound)?;

    let legal_name_pattern =
        regex::Regex::new(r"Legal Name\s*([A-Za-z\s&.,]+?)(?:\n|Trade Name|Additional|$)")
            .map_err(|e| GSTVerificationError::RegexCompilationFailed(e.to_string()))?;

    let legal_name = legal_name_pattern
        .captures(&full_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or(GSTVerificationError::LegalNameNotFound)?;

    Ok(GSTCertificate {
        gst_number,
        legal_name,
        signature: verified_content.signature,
    })
}
