use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use serde_json::Value;
use std;
use std::process::exit;

// https://en.wikipedia.org/wiki/DNS_Certification_Authority_Authorization
// https://docs.infoblox.com/download/attachments/8945695/Infoblox_RESTful_API_Documentation_2.9.pdf?version=1&modificationDate=1531345898303&api=v2
// ca_flag which as of now is always 0
// ca_tag issue|issuewild|iodef
// ca_value 
// name

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct SRVResponse {
    objects: Vec<SRV>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct SRV {
    _ref: String,
    name: String,
    priority: u32,
    weight: u32,
    port: u32,
    target: String,
    view: String,
}

impl std::fmt::Display for SRV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} view={}", self._ref, self.name, self.view)
    }
}

const ENDPOINT: &'static str = "record:srv";

pub fn execute(caa_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut name = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = caa_matches.subcommand_matches("get") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("view Required")
        }
        get_srv(name.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = caa_matches.subcommand_matches("create") {
        let mut name = "";
        let mut priority = 0;
        let mut weight = 0;
        let mut port = 0;
        let mut target = "";
        let mut view = "";
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("priority"){
            Some(value) => { priority = value.parse::<u32>().unwrap() },
            None => println!("priority Required")
        }
        match _get_matches.value_of("weight"){
            Some(value) => { weight = value.parse::<u32>().unwrap() },
            None => println!("weight Required")
        }
        match _get_matches.value_of("port"){
            Some(value) => { port = value.parse::<u32>().unwrap() },
            None => println!("port Required")
        }
        match _get_matches.value_of("target"){
            Some(ivalue) => { target = ivalue },
            None => println!("target Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("view Required")
        }
        create_srv(name.to_string(), priority, weight, port, target.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = caa_matches.subcommand_matches("delete") {
        match _get_matches.value_of("name"){
            Some(ivalue) => { name = ivalue },
            None => println!("name Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("view Required")
        }
        delete_srv(name.to_string(), view.to_string(), config.clone());
    }
}

fn delete_srv(name: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, name, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &name);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let caa: SRV = serde_json::from_value(entry).unwrap();
            match r.delete(caa._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &name);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_srv(name: String, priority: u32, weight: u32, port: u32, target: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "name": name,
        "priority": priority,
        "weight": weight,
        "port": port,
        "target": target,
        "view": view,
    });
    let url = ENDPOINT.to_string();

    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.create(url, post_data));
    if api_out.is_error == true {
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            println!("Success: {}", entry)
        }
    }
}

fn serialize_entries(entries: Vec<Value>) -> Vec<SRV> {
    let entries: Vec<Value> = entries;
    let mut returns = vec![];
    for entry in entries {
        let srv: SRV = serde_json::from_value(entry).unwrap();
        returns.push(srv);
    }
    returns

}

fn get_srv(name: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}&_return_fields=name,port,priority,target,view,weight", ENDPOINT, name, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", name);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
}
#[cfg(test)]
mod test_caa {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use srv_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_caa_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:caa?name=foo&ca_flag=issue&ca_tag=0&view=Public");
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let r = restapi::RESTApi {
            config: config
        };
        let verb = "get";
        let _mock = mock(verb, Matcher::Any)
          .with_header("content-type", "application/json")
          .with_body(out)
          .with_status(200)
          .create();
        api_out.process(r.get(search));
        let entries = serialize_entries(api_out.response);
        assert_eq!(entries.len(), 0);
        reset();
    }
    #[test]
    fn test_get_caa_single_response () {
        let out = r#"[{
            "name": "_http._tcp.foo.mozilla.com",
            "priority": 10,
            "weight": 20,
            "port": 0,
            "target": "mozilla.com",
            "_ref": "asfdsadf/Private",
            "view": "Private"
          }]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let search=format!("record:caa?name=foo&view=Public");
        let r = restapi::RESTApi {
            config: config
        };
        let verb = "get";
        let _mock = mock(verb, Matcher::Any)
          .with_header("content-type", "application/json")
          .with_body(out)
          .with_status(200)
          .create();
        api_out.process(r.get(search));
        let entries = serialize_entries(api_out.response);
        assert_eq!(entries.len(), 1);
        reset();
    }
}