//! Compute@Edge starter kit for beacon termination
use fastly::error::anyhow;
use fastly::geo::{geo_lookup, Continent};
use fastly::{uap_parse, Error};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

/// `ClientData` models information about a client.
///
/// Models information about a client which sent the NEL report request, such as
/// geo IP data and User Agent.
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientData {
    client_ip: String,
    client_user_agent: String,
    client_asn: u32,
    client_asname: String,
    client_city: String,
    client_country_code: String,
    client_continent_code: Continent,
    client_latitude: f64,
    client_longitude: f64,
}

impl ClientData {
    /// Returns a `ClientData` using information from the downstream request.
    pub fn new(client_ip: IpAddr, client_user_agent: &str) -> Result<ClientData, Error> {
        // First, truncate the IP to a privacy safe prefix.
        let truncated_ip = truncate_ip_to_prefix(client_ip)?;
        // Lookup the geo IP data from the client IP. If no match return an
        // error.
        match geo_lookup(client_ip) {
            Some(geo) => Ok(ClientData {
                client_ip: truncated_ip,
                client_user_agent: UserAgent::from_str(client_user_agent)?.to_string(), // Parse the User-Agent string to family, major, minor, patch.
                client_asn: geo.as_number(),
                client_asname: geo.as_name().to_string(),
                client_city: geo.city().to_string(),
                client_country_code: geo.country_code().to_string(),
                client_latitude: geo.latitude(),
                client_longitude: geo.longitude(),
                client_continent_code: geo.continent(),
            }),
            None => Err(anyhow!("Unable to lookup geo IP data")),
        }
    }
}

/// Parses an IP string input and truncates it a prefix mask.
///
/// Truncates an IP to a privacy safe prefix mask and returns the network as a
/// CIDR string, such as `167.98.105.176/28`.
///
/// For IPv4 addresses we truncate to a /28 prefix and for IPv6 addresses we
/// truncate to /56.
pub fn truncate_ip_to_prefix(ip: IpAddr) -> Result<String, Error> {
    match ip {
        IpAddr::V4(ip) => ipnet::Ipv4Net::new(ip, 28)
            .map(|i| i.trunc().to_string())
            .map_err(Error::new),
        IpAddr::V6(ip) => ipnet::Ipv6Net::new(ip, 56)
            .map(|i| i.trunc().to_string())
            .map_err(Error::new),
    }
}

/// UserAgent is a structured representation of a User Agent string.
///
/// Uses `uap_parse` the User Agent string. A User Agent string lets servers and
/// network peers identify the application, operating system, vendor, and/or
/// version of the requesting user agent.
///
/// Implements the [`FromStr`][from-str] trait to facilitate parsing from a
/// User-Agent header value.
///
/// [from-str]: https://doc.rust-lang.org/std/str/trait.FromStr.html
#[derive(Clone)]
pub struct UserAgent {
    family: String,
    major: String,
    minor: String,
    patch: String,
}

impl FromStr for UserAgent {
    type Err = Error;

    fn from_str(s: &str) -> Result<UserAgent, Error> {
        let (family, major, minor, patch) = uap_parse(s)?;
        Ok(UserAgent {
            family,
            major: major.unwrap_or_default(),
            minor: minor.unwrap_or_default(),
            patch: patch.unwrap_or_default(),
        })
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}.{}.{}",
            self.family, self.major, self.minor, self.patch
        )
    }
}
