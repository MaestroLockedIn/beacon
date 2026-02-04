use std::error;

use reqwest::Response;
use serde::Deserialize;

use crate::config::EndpointConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct HttpPing {
    url: String,
    rules: HttpRule,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpRule {
    is_successful: bool,
}

impl HttpPing {
    pub fn new(config: &EndpointConfig) -> Self {
        Self {
            url: config.url.clone(),
            rules: HttpRule {
                is_successful: true,
            },
        }
    }

    pub async fn ping(&self) -> Result<(), Box<dyn error::Error>> {
        let resp = reqwest::get(self.url.as_str()).await?;
        let request_status = request_match(resp, &self.rules);
        match request_status {
            Ok(_) => {
                println!("Ping Request is successful");
                return Ok(());
            }
            Err(err) => {
                println!("{}", err);
                return Err(format!("{}", err).into());
            }
        }
    }
}

fn request_match(resp: Response, rules: &HttpRule) -> Result<bool, &str> {
    if rules.is_successful && !resp.status().is_success() {
        return Err("Ping Request was not successful");
    }
    //TODO: Once the rules for the config is set validate the rules here
    Ok(true)
}
