//! Compute@Edge starter kit for beacon termination
//!
//! CSP violation report example.
use serde::{Deserialize, Serialize};

/// `ReportBody` models the body of a CSP report object.
///
/// [specification]: https://www.w3.org/TR/CSP2/#generate-a-violation-report-object
#[derive(Serialize, Deserialize, Clone)]
pub struct ReportBody {
  #[serde(rename = "csp-report")]
  pub csp_report: CSPViolation,
}

/// `CSPViolation` models the body a CSP violation.
///
/// [specification]: https://www.w3.org/TR/CSP2/#violation-reports
#[derive(Serialize, Deserialize, Clone)]
pub struct CSPViolation {
  #[serde(rename = "blocked-uri")]
  pub blocked_uri: String,
  #[serde(rename = "document-uri")]
  pub document_uri: String,
  #[serde(rename = "effective-directive")]
  pub effective_directive: String, 
  #[serde(rename = "original-policy")]
  pub original_policy: String,  
  pub referrer: String,
  #[serde(rename = "status-code")]
  pub status_code: i32,
  #[serde(rename = "violated-directive")]
  pub violated_directive: String
}
