# Compute@Edge starter kit for beacon termination

A Rust based Compute@Edge starter kit for a beacon reporting endpoint.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**.

## What are beacons?

Beacons are HTTP requests, usually `POST`s, sent from a web browser to record some analytics data. Browsers offer native support for beacons via the `navigator.sendBeacon` method, and via the [Reporting API](https://developers.google.com/web/updates/2018/09/reportingapi) for out-of-band reports on browser-generated warnings like CSP and feature policy violations, deprecations, browser interventions, and network errors. Native apps will also often send beacon data back to base.

**For an in-depth guide to beacon termination using VCL, see the [Beacon Termination pattern](https://developer.fastly.com/solutions/patterns/beacon-termination) on Fastly's developer hub**.

## Features

* Exposes a `POST /reports` endpoint to receive beacon reports (in batches);
* Deserializes individual reports from JSON to Rust data structures, with optional type-checking (see [Payload examples](#payload-examples));
* Enriches the data with information available at the edge, e.g. by adding geo data;
* Sends reports to a logging endpoint as individual JSON lines;
    N.B.: Depending on which [logging endpoint type](https://developer.fastly.com/reference/api/logging/) is chosen, these lines may be batched.
* Responds with a synthetic 204.

### Payload examples

This starter kit allows an individual report to be any valid JSON value.

For optional type-checking, it also includes the data structures for some common report payloads. These structures can be imported from modules following the `example_...` naming convention:


* CSP Violations
    ```
    mod example_csp_violation;
    use crate::example_csp_violation::ReportBody;
    ```
* Network Errors
    ```
    mod example_network_error_log;
    use crate::example_network_error_log::ReportBody;
    ```
* Core Web Vitals
    ```
    mod example_core_web_vital;
    use crate::example_core_web_vital::ReportBody;
    ```
    Tip: Use the [web-vitals JavaScript library](https://web.dev/vitals/) to measure all the Core Web Vitals.

## Requirements

The following resources need to exist on your active Fastly service version for this starter kit to work:

- A [logging endpoint](https://docs.fastly.com/en/guides/about-fastlys-realtime-log-streaming-features) called `reports`.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
