//! Utility functions for GST certificate processing
//!
//! This module contains helper functions for generating cryptographic commitments
//! and error handling for GST certificate verification.

use alloy_primitives::keccak256;
use std::error::Error;
use std::fmt;

use crate::GSTCertificate;
use crate::PANCertificate;

/// Generate a commitment hash from the GST certificate data
pub fn gst_generate_commitment(gst: &GSTCertificate) -> [u8; 32] {
    let mut combined_input = Vec::new();
    combined_input.extend_from_slice(&gst.signature.message_digest);
    combined_input.extend_from_slice(gst.gst_number.as_bytes());
    combined_input.extend_from_slice(gst.legal_name.as_bytes());
    combined_input.extend_from_slice(&gst.signature.public_key);

    keccak256(&combined_input).as_slice().try_into().unwrap()
}

/// GST
#[derive(Debug)]
pub enum GSTVerificationError {
    PdfVerificationFailed(String),
    RegexCompilationFailed(String),
    GSTNumberNotFound,
    LegalNameNotFound,
}

impl fmt::Display for GSTVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GSTVerificationError::PdfVerificationFailed(msg) => {
                write!(f, "PDF verification failed: {}", msg)
            }
            GSTVerificationError::RegexCompilationFailed(msg) => {
                write!(f, "Regex compilation failed: {}", msg)
            }
            GSTVerificationError::GSTNumberNotFound => {
                write!(f, "GST number not found in PDF")
            }
            GSTVerificationError::LegalNameNotFound => {
                write!(f, "Legal name not found in PDF")
            }
        }
    }
}

impl Error for GSTVerificationError {}

/// Generate a commitment hash from the PAN certificate data
pub fn pan_generate_commitment(pan: &PANCertificate) -> [u8; 32] {
    let mut combined_input = Vec::new();
    combined_input.extend_from_slice(&pan.signature.message_digest);
    combined_input.extend_from_slice(pan.pan_number.as_bytes());
    combined_input.extend_from_slice(pan.legal_name.as_bytes());
    combined_input.extend_from_slice(&pan.signature.public_key);

    keccak256(&combined_input).as_slice().try_into().unwrap()
}

/// PAN
#[derive(Debug)]
pub enum PANVerificationError {
    PdfVerificationFailed(String),
    RegexCompilationFailed(String),
    PANNumberNotFound,
    LegalNameNotFound,
}


impl fmt::Display for PANVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PANVerificationError::PdfVerificationFailed(msg) => {
                write!(f, "PDF verification failed: {}", msg)
            }
            PANVerificationError::RegexCompilationFailed(msg) => {
                write!(f, "Regex compilation failed: {}", msg)
            }
            PANVerificationError::PANNumberNotFound => {
                write!(f, "PAN number not found in PDF")
            }
            PANVerificationError::LegalNameNotFound => {
                write!(f, "Legal name not found in PDF")
            }
        }
    }
}

impl Error for PANVerificationError {}
