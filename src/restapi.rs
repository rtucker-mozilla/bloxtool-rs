use reqwest;
use bloxconfig;
use serde_json::Value;
use reqwest::StatusCode;

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
    pub config: bloxconfig::Config
}

impl RESTApi {

    pub fn get_client(&self) -> reqwest::Client {
        return reqwest::Client::new();
    }

    fn get_url(&self, hostpath: &String, ipath: &String) -> String {

        return format!("{}/{}",hostpath, ipath);

    }
    /*

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
    */

    /*
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
    */

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
                        //println!("{:#?}", _resp.json());
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

    pub fn create(&self, iref: String, post_data: Value) -> Result<Value, String> {
        let client = self.get_client();
        let config = self.config.clone();
        let host_path = config.full_path();
        let full_path = self.get_url(&host_path, &iref);
        let resp = client.post(full_path.as_str()).basic_auth(config.username, Some(config.password)).json(&post_data).send();
        match resp {
            Ok(mut _resp) => {
                Ok(_resp.json().unwrap())
            },
            Err(_error) => {
                return Result::Err("Uknown Error".to_string());
            }
        }
    }
}

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