use serde::{Deserialize, Serialize};
use serde_json::value::Value;

/// `ReportBody` models the body a Core Web Vital metric.
///
/// [specification]: https://github.com/GoogleChrome/web-vitals#metric
#[derive(Serialize, Deserialize, Clone)]
pub struct ReportBody {
    pub name: String,
    pub value: f32,
    pub delta: f32,
    pub id: String,
    #[serde(rename = "isFinal")]
    pub is_final: bool,
    pub entries: Value,
}
