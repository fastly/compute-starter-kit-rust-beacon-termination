//! Compute@Edge starter kit for beacon termination
//!
//! Core Web Vital report example.
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use crate::report::Report;

/// `Report` models a Core Web Vital payload.
///
/// This is the report which user agent is expected to deliver to the
/// report endpoint. Follows the schema defined by the
/// [W3C reporting API][reporting-api].
///
/// [reporting-api]: https://www.w3.org/TR/reporting-1/
#[derive(Serialize, Deserialize, Clone)]
impl CoreWebVital for Report {
    pub body: ReportBody,
}

/// `ReportBody` models the body a Core Web Vital metric.
///
/// [specification]: https://github.com/GoogleChrome/web-vitals#metric
#[derive(Serialize, Deserialize, Clone)]
pub struct ReportBody {
    // The name of the metric (in acronym form).
  pub name: 'CLS' | 'FCP' | 'FID' | 'LCP' | 'TTFB',
  // The current value of the metric.
  pub value: f32,
  // The delta between the current value and the last-reported value.
  // On the first report, `delta` and `value` will always be the same.
  pub delta: f32,
  // A unique ID representing this particular metric that's specific to the
  // current page. This ID can be used by an analytics tool to dedupe
  // multiple values sent for the same metric, or to group multiple deltas
  // together and calculate a total.
  pub id: String,
  // `false` if the value of the metric may change in the future,
  // for the current page.
  pub isFinal: bool,
  // Any performance entries used in the metric value calculation.
  // Note, entries will be added to the array as the value changes.
  pub entries: Value[]
}
