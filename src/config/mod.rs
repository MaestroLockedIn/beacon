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
    //TODO: Figure out the rules that needs to be applied  here
    // I need to come up with other rules for http ping
}
