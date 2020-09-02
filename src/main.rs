//! Compute@Edge starter kit for beacon termination
//!
//! A Compute@Edge service which exposes a HTTP reporting endpoint for beacon termination.
use chrono::Utc;
use fastly::http::{header, Method, StatusCode};
use fastly::log::Endpoint;
use fastly::{downstream_client_ip_addr, Body, Error, Request, Response, ResponseExt};
use serde::{Deserialize, Serialize};
use serde_json::value::Value as ReportBody;
// The line above allows any valid JSON value in the report body.
// Try type-checking a specific beacon payload by importing the `ReportBody`
// data structure from one of the examples provided instead, e.g.:
// mod example_network_error_log;
// use crate::example_network_error_log::ReportBody;
use std::io::Write;

// Import the `Report` and `ClientData` data structures.
mod client_data;
mod report;

use crate::client_data::ClientData;
use crate::report::Report;

/// Main application entrypoint.
///
/// This controls the routing logic for the application, it accepts a `Request`
/// and passes it to any matching request handlers before returning a `Response`
/// back downstream.
#[fastly::main]
fn main(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        // If a CORS preflight OPTIONS request return a 204 no content.
        (&Method::OPTIONS, "/report") => generate_no_content_response(),
        // If a POST request pass to the `handler_reports` request handler.
        (&Method::POST, "/report") => handle_reports(req),
        // For all other requests return a 404 not found.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))?),
    }
}

/// Handle reports.
///
/// It attempts to extract the beacon reports from the POST request body and maps
/// over each report adding additional information before emitting a log line
/// to the `reports` logging endpoint if valid. It always returns a synthetic
/// `204 No content` response, regardless of whether the log reporting was
/// successful.
fn handle_reports(req: Request<Body>) -> Result<Response<Body>, Error> {
    let (parts, body) = req.into_parts();

    // Parse the beacon reports from the request JSON body using serde_json.
    // If successful, bind the reports to the `reports` variable, 
    // optionally transform and typecheck the payload, and log.
    if let Ok(reports) = serde_json::from_reader::<Body, Vec<Report<ReportBody>>>(body) {
        // Extract information about the client from the downstream request,
        // such as the User-Agent and IP address.
        let client_user_agent = parts
            .headers
            .get(header::USER_AGENT)
            .and_then(|header| header.to_str().ok())
            .unwrap_or("");
        let client_ip = downstream_client_ip_addr().expect("should have client IP");

        // Construct a new `ClientData` structure from the IP and User Agent.
        let client_data = ClientData::new(client_ip, client_user_agent)?;

        // Generate a list of reports to be logged by mapping over each raw beacon
        // payload, merging it with the `ClientData` from above and transform it
        // to a `LogLine`.
        // We assume that the input is an array, to allow the client to sent multiple
        // reports at once. This is always the case for reports sent out-of-band
        // through the Reporting API, e.g., network errors, CSP violations, browser
        // interventions, and feature policy violations.
        let logs: Vec<LogLine<ReportBody>> = reports
            .into_iter()
            .map(|report| LogLine::new(report, client_data.clone()))
            .filter_map(Result::ok)
            .collect();

        // Create a handle to the upstream logging endpoint that we want to emit
        // the reports to.
        let mut endpoint = Endpoint::from_name("reports");

        // Loop over each log line serializing it back to JSON and write it to
        // the logging endpoint.
        for log in logs.iter() {
            if let Ok(json) = serde_json::to_string(&log) {
                writeln!(endpoint, "{}", json)?;
            }
        }
    };

    // Return and empty 204 no content response to the downstream client,
    // regardless of successful logging.
    generate_no_content_response()
}

/// `LogLine` models the structure of a log line.
///
/// This is the data  structure that we serialize and emit to the logging
/// endpoint.
#[derive(Serialize, Deserialize)]
pub struct LogLine<T=ReportBody> {
    /// The log timestamp.
    ///
    /// A unix timestamp generated when we receive the report.
    timestamp: i64,
    // The GeoIP client data.
    client: ClientData,
    /// The sanitized report.
    report: Report<T>,
}

impl LogLine {
    // Construct a new LogLine from a `Report` and `ClientData` and decorate
    // with a Unix timestamp.
    pub fn new<T>(report: Report<T>, client: ClientData) -> Result<LogLine<T>, Error> {
        // 
        Ok(LogLine {
            timestamp: Utc::now().timestamp(),
            client,
            report,
        })
    }
}

/// Utility to generate a synthetic `204 No Content` response.
///
/// Generates a response with a 204 status code, ensures the response is
/// non-cacheable via cache-control header directives and adds appropriate CORS
/// headers required for the beacon preflight request.
pub fn generate_no_content_response() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::CACHE_CONTROL,
            "no-cache, no-store, max-age=0, must-revalidate",
        )
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, header::CONTENT_TYPE)
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS")
        .header(header::CONNECTION, "keep-alive")
        .body(Body::new())?)
}
