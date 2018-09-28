use reqwest;
use bloxconfig;

#[allow(dead_code)]
pub struct RESTOutput {
    pub line: String
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

    fn get_client(&self) -> reqwest::Client {
        return reqwest::Client::new();
    }

    pub fn get(&self) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        return client.get(self.url.as_str()).basic_auth(config.username, Some(config.password)).send()
    }

}