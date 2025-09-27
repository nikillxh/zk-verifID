//! GST Certificate Verification Library
//!
//! This library provides functions for verifying GST certificates and extracting
//! key information from PDF documents. It handles PDF parsing, signature verification,
//! and data extraction with proper error handling.

use alloy_sol_types::sol;

use zkpdf_lib::{verify_and_extract, PdfSignatureResult};

use crate::utils::{GSTVerificationError, PANVerificationError};

pub mod utils;

pub struct GSTCertificate {
    pub gst_number: String,
    pub legal_name: String,
    pub signature: PdfSignatureResult,
}

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct GSTValuesStruct {
        string gst_number;
        string legal_name;
        bool signature_valid;
        bytes32 document_commitment;
        bytes32 public_key_hash;
    }

    struct PANValuesStruct {
        string pan_number;
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

pub struct PANCertificate {
    pub pan_number: String,
    pub legal_name: String,
    pub signature: PdfSignatureResult,
}

/// PAN Certificate verification function that extracts legal name and PAN number
pub fn verify_pan_certificate(pdf_bytes: Vec<u8>) -> Result<PANCertificate, PANVerificationError> {
    let verified_content = verify_and_extract(pdf_bytes)
        .map_err(|e| PANVerificationError::PdfVerificationFailed(e.to_string()))?;

    let full_text = verified_content.pages.join(" ");

    // Regex pattern for PAN: 5 letters + 4 digits + 1 letter
    let pan_pattern =
        regex::Regex::new(r"([A-Z]{5}[0-9]{4}[A-Z]{1})")
            .map_err(|e| PANVerificationError::RegexCompilationFailed(e.to_string()))?;

    let pan_number = pan_pattern
        .captures(&full_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or(PANVerificationError::PANNumberNotFound)?;

    // Legal name pattern (similar approach to GST, adjust keywords if needed)
    let legal_name_pattern =
        regex::Regex::new(r"Name\s*([A-Za-z\s&.,]+?)(?:\n|Father|DOB|$)")
            .map_err(|e| PANVerificationError::RegexCompilationFailed(e.to_string()))?;

    let legal_name = legal_name_pattern
        .captures(&full_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
        .ok_or(PANVerificationError::LegalNameNotFound)?;

    Ok(PANCertificate {
        pan_number,
        legal_name,
        signature: verified_content.signature,
    })
}
