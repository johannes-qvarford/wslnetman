//! Packet module
//!
//! This module provides functionality to send packets (ping and HTTP over TCP).

pub mod http;
pub mod ping;

// Re-export the result types from submodules
pub use http::HttpRequestResult;
pub use ping::PingResult;

/// Send a ping request to the specified destination
///
/// This function uses the system's ping command to send ICMP echo requests.
///
/// # Arguments
///
/// * `destination` - The IP address or hostname to ping
/// * `count` - The number of ping requests to send (default: 4)
/// * `timeout` - The timeout for each ping request in seconds (default: 5)
///
/// # Returns
///
/// A `Result` containing the ping result or an error
pub fn send_ping(
    destination: &str,
    count: Option<u32>,
    timeout: Option<u32>,
) -> Result<PingResult, Box<dyn std::error::Error>> {
    ping::send_ping(destination, count, timeout)
}

/// Send an HTTP request to the specified URL
///
/// This function uses the `reqwest` crate to send HTTP requests.
///
/// # Arguments
///
/// * `url` - The URL to send the HTTP request to
/// * `method` - The HTTP method to use (default: "GET")
/// * `timeout` - The timeout for the request in seconds (default: 30)
///
/// # Returns
///
/// A `Result` containing the HTTP request result or an error
pub async fn send_http_request(
    url: &str,
    method: Option<&str>,
    timeout: Option<u64>,
) -> Result<HttpRequestResult, Box<dyn std::error::Error>> {
    http::send_http_request(url, method, timeout).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_ping() {
        // Since we're simulating the ping, we can't test the actual network functionality
        // But we can test that our function returns a result
        let result = send_ping("127.0.0.1", Some(4), Some(5)).unwrap();

        assert_eq!(result.destination, "127.0.0.1");
        assert_eq!(result.transmitted, 4);
    }
}
