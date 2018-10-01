use reqwest;
use bloxconfig;
use serde_json::Value;

#[allow(dead_code)]
pub struct RESTOutput {
    pub line: String
}

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct RESTResponse {
    pub error: String,
    pub code: String,
    pub text: String,
}

impl RESTOutput {
    pub fn output(&self) -> String {
        return format!("{}", self.line.to_string());
    }
}

pub struct RESTApi {
    pub url: String,
    pub config: bloxconfig::Config
}

impl RESTApi {

    pub fn get_client(&self) -> reqwest::Client {
        return reqwest::Client::new();
    }

    pub fn get(&self) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        return client.get(self.url.as_str()).basic_auth(config.username, Some(config.password)).send()
    }

    pub fn delete(&self) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        return client.delete(self.url.as_str()).basic_auth(config.username, Some(config.password)).send()
    }

    pub fn post(&self, post_data: Value) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        return client.post(self.url.as_str()).basic_auth(config.username, Some(config.password)).json(&post_data).send();
    }

}