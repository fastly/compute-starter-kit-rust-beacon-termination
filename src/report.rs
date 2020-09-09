//! Generic JSON report example.
use serde::{Deserialize, Serialize};

/// `Report` models a beacon payload.
///
/// This is the report which user agent is expected to deliver to the
/// report endpoint. Follows the schema defined by the
/// [W3C reporting API][reporting-api].
///
/// [reporting-api]: https://www.w3.org/TR/reporting-1/
#[derive(Serialize, Deserialize, Clone)]
pub struct Report<T> {
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
    /// type. This generic structure uses an untyped JSON value.
    /// https://github.com/serde-rs/json#operating-on-untyped-json-values
    pub body: T,
    pub age: i64,
}
