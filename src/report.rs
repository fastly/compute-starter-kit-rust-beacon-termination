//! Compute@Edge starter kit for beacon termination
use serde::{Deserialize, Serialize};

/// `Report` models a beacon payload.
///
/// This is the report which user agent is expected to deliver to the
/// report endpoint. Follows the schema defined by the
/// [W3C reporting API][reporting-api].
///
/// [reporting-api]: https://www.w3.org/TR/reporting-1/
#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    /// The report User Agent.
    ///
    /// The value of the User-Agent header string of the request from which
    /// the report was generated.
    pub user_agent: String,
    /// The report URL.
    ///
    /// Typically the address of the Document or Worker from which the report
    /// was generated.
    pub url: String,
    #[serde(rename = "type")]
    pub report_type: String,
    /// The report body.
    ///
    /// The fields contained in a report's body are determined by the report's
    /// type.
    pub body: ReportBody,
    pub age: i64,
}

/// `ReportBody` models the body of a report.
///
/// It details the network error that occurred in a given phase.
/// Note: view the Network Error Logging [specification][specification]
/// for detailed information on the report structure.
///
/// [specification]: https://www.w3.org/TR/network-error-logging
#[derive(Serialize, Deserialize, Clone)]
pub struct ReportBody {
    #[serde(rename = "type")]
    pub error_type: String,
    pub elapsed_time: i32,
    pub method: String,
    pub phase: String,
    pub protocol: String,
    pub referrer: String,
    pub sampling_fraction: f32,
    pub server_ip: String,
    pub status_code: i32,
}
