//! Common ICAP Protocol Types

use std::collections::HashMap;

/// ICAP Request
#[derive(Debug, Clone)]
pub struct IcapRequest {
    pub method: super::icap::IcapMethod,
    pub uri: String,
    pub version: super::icap::IcapVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// ICAP Response
#[derive(Debug, Clone)]
pub struct IcapResponse {
    pub version: super::icap::IcapVersion,
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// Encapsulated Data
#[derive(Debug, Clone)]
pub struct EncapsulatedData {
    pub req_hdr: Option<Vec<u8>>,
    pub res_hdr: Option<Vec<u8>>,
    pub req_body: Option<Vec<u8>>,
    pub res_body: Option<Vec<u8>>,
    pub opt_body: Option<Vec<u8>>,
}
