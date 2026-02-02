use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub endpoints: Vec<EndpointConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EndpointConfig {
    pub name: String,
    pub url: String,
    pub interval_seconds: u64,
    pub expected_status_range: [u64; 2],
    pub latency: u64,
    pub max_response_time_ms: u64,
    pub timeout_second: u64,
}
