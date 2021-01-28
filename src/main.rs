//! Compute@Edge starter kit for beacon termination.
mod client_data;
mod example_core_web_vital;
mod example_csp_violation;
mod example_network_error_log;
mod report;

use crate::client_data::ClientData;
use crate::report::Report;
use chrono::Utc;
use fastly::http::{header, Method, StatusCode};
use fastly::log::Endpoint;
use fastly::{Error, Request, Response};
use serde::{Deserialize, Serialize};

// This line allows any valid JSON value in the report body.
// Try type-checking a specific beacon payload by importing the `ReportBody`
// data structure from one of the examples provided instead, e.g.:
// mod example_network_error_log;
// use crate::example_network_error_log::ReportBody;
use serde_json::value::Value as ReportBody;

/// Main application entrypoint.
///
/// This controls the routing logic for the application, it accepts a `Request`
/// and passes it to any matching request handlers before returning a `Response`
/// back downstream.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Pattern match on the request method and path.
    match (req.get_method(), req.get_path()) {
        // For a CORS preflight OPTIONS request, return an empty 204 response.
        (&Method::OPTIONS, "/report") => Ok(generate_empty_204_response()),
        // Pass a POST request to the `handler_reports` request handler.
        (&Method::POST, "/report") => {
            let _ = handle_reports(req)?;
            // Return an empty 204 response to the downstream client.
            Ok(generate_empty_204_response())
        }
        // For all other requests return a 404.
        _ => Ok(Response::from_body("Not found").with_status(StatusCode::NOT_FOUND)),
    }
}

/// Handle reports.
///
/// It attempts to extract the beacon reports from the POST request body and maps
/// over each report adding additional information before emitting a log line
/// to the `reports` logging endpoint if valid.
fn handle_reports(mut req: Request) -> Result<(), Error> {
    // Parse the beacon reports from the request JSON body using serde_json.
    // If successful, bind the reports to the `reports` variable,
    // optionally transform and typecheck the payload, and log.
    let reports = req.take_body_json::<Vec<Report<ReportBody>>>()?;

    // Extract information about the client from the downstream request,
    // such as the User-Agent and IP address.
    let client_user_agent = req.get_header_str(header::USER_AGENT).unwrap_or("");
    let client_ip = req.get_client_ip_addr().expect("should have client IP");

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
        serde_json::to_writer(&mut endpoint, &log)?;
    }

    Ok(())
}

/// `LogLine` models the structure of a log line.
///
/// This is the data structure that we serialize and emit to the logging
/// endpoint.
#[derive(Serialize, Deserialize)]
pub struct LogLine<T = ReportBody> {
    /// The log timestamp.
    ///
    /// A Unix timestamp generated when we receive the report.
    timestamp: i64,
    /// The GeoIP client data.
    client: ClientData,
    /// The sanitized report.
    report: Report<T>,
}

impl LogLine {
    // Construct a new LogLine from a `Report` and `ClientData` and decorate
    // with a Unix timestamp.
    pub fn new<T>(report: Report<T>, client: ClientData) -> Result<LogLine<T>, Error> {
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
pub fn generate_empty_204_response() -> Response {
    Response::from_status(StatusCode::NO_CONTENT)
        .with_header(header::CONTENT_TYPE, "application/json")
        .with_header(
            header::CACHE_CONTROL,
            "no-cache, no-store, max-age=0, must-revalidate",
        )
        .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .with_header(header::ACCESS_CONTROL_ALLOW_HEADERS, header::CONTENT_TYPE)
        .with_header(header::ACCESS_CONTROL_ALLOW_METHODS, "POST, OPTIONS")
        .with_header(header::CONNECTION, "keep-alive")
}
