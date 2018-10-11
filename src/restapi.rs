use reqwest;
use bloxconfig;
use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string;
use reqwest::StatusCode;



#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct RESTResponse {
    pub error: String,
    pub code: String,
    pub text: String,
}

#[allow(dead_code)]
pub struct InfobloxResponse {
    pub count: usize,
    pub is_error: bool,
    pub is_empty: bool,
    pub response: Vec<Value>
}

impl Default for InfobloxResponse {
    fn default() -> InfobloxResponse {
        InfobloxResponse { 
            count: 0,
            is_error: false,
            is_empty: true,
            response: Vec::new()
        }
    }

}
fn resp_text_to_vec(mut i_resp: reqwest::Response) -> Result<Vec<Value>, reqwest::Error> {
    match i_resp.text() {
        Ok(_str) => { 
            let retval = from_str(&_str).unwrap();
            Ok(vec![retval])
        },
        Err(_err) => { Err(_err) }
    }
}

#[allow(dead_code)]
impl InfobloxResponse {

    fn calculate_count(&mut self, response: &Option<Vec<Value>>) {
        match response {
            Some(resp) => {
                self.count = resp.len();
            },
            None => { self.count = 0 }
        }
    }

    fn calculate_empty(&mut self, response: &Option<Vec<Value>>) {
        match response {
            Some(resp) => {
                if resp.len() > 0 {
                    self.is_empty = false;
                } else {
                    self.is_empty = true;
                }
            },
            None => { self.is_empty = true; }
        }
    }

    pub fn process(&mut self, response: Option<Vec<Value>>) {
        self.calculate_empty(&response);
        self.calculate_count(&response);
        match response {
            Some(_resp) => { self.response = _resp },
            None => { 
                self.is_error = true;
            }
        }
    }

}
fn format_error(mut _resp: reqwest::Response) -> String {
    let error_text = _resp.text().unwrap();
    let error_response: Value = from_str(&error_text).unwrap();
    let error_response_text = to_string(&error_response["text"]).unwrap();
    let error_response_text_trimmed = error_response_text.trim_matches('"').to_string();
    return format!("{}", error_response_text_trimmed);

}

pub struct RESTApi {
    pub config: bloxconfig::Config
}

impl RESTApi {

    pub fn get_client(&self) -> reqwest::Client {
        return reqwest::Client::new();
    }

    fn get_url(&self, hostpath: &String, ipath: &String) -> String {

        return format!("{}/{}",hostpath, ipath);

    }

    fn trim_quotes(&self, url: String) -> String {
        return url.trim_matches('"').to_string();
    }

    pub fn get(&self, iref: String) -> Option<Vec<Value>> {
        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = self.get_url(&host_path, &iref);
        let resp = client.get(full_path.as_str()).basic_auth(config.username, Some(config.password)).send();
        match resp {
            Ok(mut _resp) => {
                match _resp.status(){
                    StatusCode::OK => { 
                        return _resp.json().unwrap();
                    },
                    StatusCode::UNAUTHORIZED => {
                        println!("Invalid Username/Password.")
                    },
                    s => { println!("Unknown: {}", s) }
                }
            },
            Err(_err) => { 
                println!("{}", _err);
            }
        }
        None
    }

    pub fn delete(&self, iref: String) -> Result<Value, String> {

        let client = self.get_client();
        let config = self.config.clone();
        let _ref = self.trim_quotes(iref);
        let host_path = config.full_path();
        let full_path = self.get_url(&host_path, &_ref);
        let resp = client.delete(full_path.as_str()).basic_auth(config.username, Some(config.password)).send();
        match resp {
            Ok(mut _resp) => {
                Ok(_resp.json().unwrap())
            }
            Err(_error) => {
                return Result::Err("Uknown Error".to_string());
            }
        }
    }



    pub fn create(&self, iref: String, post_data: Value) -> Option<Vec<Value>> {
        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = self.get_url(&host_path, &iref);
        let resp = client.post(full_path.as_str()).basic_auth(config.username, Some(config.password)).json(&post_data).send();
        match resp {
            Ok(mut _resp) => {
                match _resp.status(){
                    StatusCode::CREATED => { 
                        match resp_text_to_vec(_resp) {
                           Ok(_val) => { return Some(_val) } ,
                           Err(_) => { () }
                        }
                    },
                    StatusCode::OK => { 
                        match resp_text_to_vec(_resp) {
                           Ok(_val) => { return Some(_val) } ,
                           Err(_) => { () }
                        }
                    },
                    StatusCode::UNAUTHORIZED => {
                        println!("Invalid Username/Password.");
                    },
                    StatusCode::BAD_REQUEST => { 
                        let out_string = format_error(_resp);
                        println!("{}", out_string);
                    }
                    s => { println!("Unknown Response: {}", s)}
                }
            },
            Err(_err) => { 
                println!("{}", _err);
            }
        }
        None
    }
}

#[cfg(test)]
use serde_json::from_str;
#[test]
fn test_trim_quotes() {
    let config = bloxconfig::Config{
        host: "https://localhost".to_string(),
        username: "admin".to_string(),
        password: "admin".to_string()

    };
    let tmp = RESTApi {
        config: config
    };
    let untrimmed = "\"foo.bar.baz/cname\"".to_string();
    let trimmed = tmp.trim_quotes(untrimmed);
    assert_eq!(trimmed, "foo.bar.baz/cname");
}
#[test]
fn test_infoblox_response_default_is_error_false (){
    let api_out = InfobloxResponse{ ..Default::default() };
    assert_eq!(api_out.is_error, false);
}

#[test]
fn test_infoblox_response_default_is_empty_true (){
    let api_out = InfobloxResponse{ ..Default::default() };
    assert_eq!(api_out.is_empty, true);
}

#[test]
fn test_infoblox_response_process_empty_array(){
    let mut api_out = InfobloxResponse{ ..Default::default() };
    let response = r#"[]"#;
    let vec = from_str(response).unwrap();
    api_out.process(Some(vec));
    assert_eq!(api_out.is_empty, true);
}

#[test]
fn test_infoblox_response_sets_empty_count(){
    let mut api_out = InfobloxResponse{ ..Default::default() };
    let response = r#"[]"#;
    let vec = from_str(response).unwrap();
    api_out.process(Some(vec));
    assert_eq!(api_out.count, 0);
}

#[test]
fn test_infoblox_response_process_populated_array(){
    let mut api_out = InfobloxResponse{ ..Default::default() };
    let response = r#"["foo"]"#;
    let vec = from_str(response).unwrap();
    api_out.process(Some(vec));
    assert_eq!(api_out.is_empty, false);
}

#[test]
fn test_infoblox_response_process_sets_proper_count(){
    let mut api_out = InfobloxResponse{ ..Default::default() };
    let response = r#"["foo"]"#;
    let vec = from_str(response).unwrap();
    api_out.process(Some(vec));
    assert_eq!(api_out.count, 1);
}