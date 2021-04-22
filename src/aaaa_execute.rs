use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use serde_json::Value;
use std;
use std::process::exit;

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct AAAAResponse {
    objects: Vec<AAAA>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct AAAA {
    _ref: String,
    ipv6addr: String,
    name: String,
    view: String,
}

impl std::fmt::Display for AAAA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} ipv6addr={} view={}", self._ref, self.name, self.ipv6addr, self.view)
    }
}

const ENDPOINT: &'static str = "record:aaaa";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:aaaa\?name\=foo.domain.com\&view\=View

pub fn execute(aaaa_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut aaaa_search = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = aaaa_matches.subcommand_matches("get") {
        match _get_matches.value_of("aaaa"){
            Some(value) => { aaaa_search = value },
            None => println!("AAAA Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_aaaa(aaaa_search.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = aaaa_matches.subcommand_matches("create") {
        let mut aaaa_search = "";
        let mut ipv6address = "";
        let mut view = "";
        match _get_matches.value_of("aaaa_record"){
            Some(value) => { aaaa_search = value },
            None => println!("AAAA Required")
        }
        match _get_matches.value_of("ipv6address"){
            Some(value) => { ipv6address = value },
            None => println!("ipv6address Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        create_aaaa(aaaa_search.to_string(), ipv6address.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = aaaa_matches.subcommand_matches("delete") {
        match _get_matches.value_of("aaaa_record"){
            Some(value) => { aaaa_search = value },
            None => println!("aaaa Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_aaaa(aaaa_search.to_string(), view.to_string(), config.clone());
    }
}

fn delete_aaaa(aaaa_search: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, &aaaa_search, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &aaaa_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let aaaa: AAAA = serde_json::from_value(entry).unwrap();
            match r.delete(aaaa._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &aaaa_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_aaaa(aaaa: String, ipv6addr: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "ipv6addr": ipv6addr,
        "name": aaaa,
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

#[allow(dead_code)]
fn serialize_entries(entries: Vec<Value>) -> Vec<AAAA> {
    let entries: Vec<Value> = entries;
    let mut return_aaaas = vec![];
    for entry in entries {
        let aaaa: AAAA = serde_json::from_value(entry).unwrap();
        return_aaaas.push(aaaa);
    }
    return_aaaas

}

fn get_aaaa(aaaa: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, aaaa, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", aaaa);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
}
#[cfg(test)]
mod test_aaaa {
    use bloxconfig;
    use mockito::{Matcher, mock};
    use aaaa_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_aaaa_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:aaaa?name=foo&ipv6addr=fe80::1&view=Public");
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
    }
    #[test]
    fn test_get_aaaa_single_response () {
        let out = r#"[{
            "name": "foo.mozilla.com",
            "_ref": "asfdsadf/Private",
            "view": "Private",
            "name": "mozilla.com",
            "ipv6addr": "fe80::1"
          }]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let search=format!("record:aaaa?name=foo&ipv6addr=fe80::1&view=Public");
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
    }
}