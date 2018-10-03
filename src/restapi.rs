use reqwest;
use bloxconfig;
use serde_json::Value;

#[allow(dead_code)]
pub struct InfobloxResponse {
    pub json: Vec<Value>
}


#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct RESTResponse {
    pub error: String,
    pub code: String,
    pub text: String,
}

#[allow(dead_code)]
impl InfobloxResponse {
    pub fn output_json(&self) -> String {
        return format!("{:#?}", self.json);
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

    pub fn get_object(&self) -> Option<InfobloxResponse>{
        match self.get() { 
            // MAke sure we are able to talk to the server
            Ok(mut data) => { 
                // Make sure the json response is valid
                match data.json() {
                    Ok(inside) => { 
                        let objects: Vec<Value> = inside;
                        if objects.len() == 0 {
                            return None;
                        } else {
                            let resp = InfobloxResponse{
                                json: objects
                            };
                            return Some(resp);
                        }
                        // convert json list to array of Cname structs
                    },
                    Err(_error) => { return None }
                }
            },
            Err(_error) => { return None }
        }

    }

    pub fn delete_object(&self) -> Option<bool> {
        match self.get_object() {
            Some(obj) => {
                let _ref = obj.json[0]["_ref"].to_string();
                match self.delete(_ref) {
                    Ok(_obj) => { return Some(true) },
                    Err(_err) => { return None }
                }
            },
            None => { return None }
        }
    }

    pub fn create_object(&self, post_data: Value) {
        match self.post(post_data) {
            Ok(mut resp) => { 
                let resp_obj: Value = resp.json().unwrap();
                if resp.status() == 400 {
                    println!("Error={}.", resp_obj["text"]);
                }
                if resp.status() == 201 {
                    println!("Success={}.", resp_obj);
                }
            },
            Err(error) => { println!("Error={}.", error)}
        }

    }

    fn trim_quotes(&self, url: String) -> String {
        return url.trim_matches('"').to_string();
    }

    pub fn get(&self) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        let resp = client.get(self.url.as_str()).basic_auth(config.username, Some(config.password)).send();
        return resp;
    }

    pub fn delete(&self, asdf: String) -> Result<reqwest::Response, reqwest::Error>{
        let mut _ref = asdf;

        let client = self.get_client();
        let config = self.config.clone();
        println!("will delete {}", _ref);
        _ref = self.trim_quotes(_ref);
        let url = format!("{}/{}", &config.full_path(), _ref);
        println!("{}", url);
        return client.delete(url.as_str()).basic_auth(config.username, Some(config.password)).send()
    }

    pub fn post(&self, post_data: Value) -> Result<reqwest::Response, reqwest::Error>{
        let client = self.get_client();
        let config = self.config.clone();
        return client.post(self.url.as_str()).basic_auth(config.username, Some(config.password)).json(&post_data).send();
    }

}