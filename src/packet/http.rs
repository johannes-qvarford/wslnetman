//! HTTP over TCP implementation
//! 
//! This module provides functionality to send HTTP requests and receive responses.
//! It uses the `reqwest` crate to perform HTTP operations.

use reqwest::Client;
use std::time::Duration;

/// Represents the result of an HTTP request
#[derive(Debug, Clone)]
pub struct HttpRequestResult {
    pub url: String,
    pub status_code: u16,
    pub response_time: f64, // in milliseconds
    pub response_body: String,
    pub response_headers: String,
    pub raw_output: String,
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
    let method = method.unwrap_or("GET");
    let timeout = timeout.unwrap_or(30);
    
    // Create a client with the specified timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()?;
    
    // Record the start time
    let start_time = std::time::Instant::now();
    
    // Send the request based on the method
    let response = match method {
        "GET" => client.get(url).send().await?,
        "POST" => client.post(url).send().await?,
        "PUT" => client.put(url).send().await?,
        "DELETE" => client.delete(url).send().await?,
        _ => return Err(format!("Unsupported HTTP method: {method}").into()),
    };
    
    // Calculate the response time
    let response_time = start_time.elapsed().as_millis() as f64;
    
    // Get the status code
    let status_code = response.status().as_u16();
    
    // Get the response headers before consuming the response
    let response_headers = response
        .headers()
        .iter()
        .map(|(name, value)| format!("{name}: {value:?}"))
        .collect::<Vec<_>>()
        .join("\n");
    
    // Get the response body
    let response_body = response.text().await?;
    
    let result = HttpRequestResult {
        url: url.to_string(),
        status_code,
        response_time,
        response_body: response_body.clone(),
        response_headers: response_headers.clone(),
        raw_output: format!("Status: {status_code}\nHeaders:\n{response_headers}\nBody:\n{response_body}"),
    };
    
    Ok(result)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_send_http_request() {
        // We can't test with an actual URL in tests, but we can verify the function signature compiles
        // In a real test, we might use a mock server or test against a known reliable endpoint
    }
}