use serde::{Deserialize, Serialize};

/// `ReportBody` models the body of a Network Error Logging (NEL) report.
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
