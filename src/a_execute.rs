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
struct AddressRecordResponse {
    objects: Vec<AddressRecord>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct AddressRecord {
    _ref: String,
    ipv4addr: String,
    name: String,
    view: String,
}

impl std::fmt::Display for AddressRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} ip={} view={}", self._ref, self.name, self.ipv4addr, self.view)
    }
}

const ENDPOINT: &'static str = "record:a";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:a\?name\=foo.domain.com\&view\=View

pub fn execute(address_record_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut address_record_search = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = address_record_matches.subcommand_matches("get") {
        match _get_matches.value_of("address_record"){
            Some(value) => { address_record_search = value },
            None => println!("Address Record Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_address_record(address_record_search.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = address_record_matches.subcommand_matches("create") {
        let mut address_record_search = "";
        let mut ip_address = "";
        let mut view = "";
        match _get_matches.value_of("address_record"){
            Some(value) => { address_record_search = value },
            None => println!("address_record Required")
        }
        match _get_matches.value_of("ip_address"){
            Some(value) => { ip_address = value },
            None => println!("IP Address Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        create_address_record(address_record_search.to_string(), ip_address.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = address_record_matches.subcommand_matches("delete") {
        match _get_matches.value_of("address_record"){
            Some(value) => { address_record_search = value },
            None => println!("Address Record Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_address_record(address_record_search.to_string(), view.to_string(), config.clone());
    }
}

fn delete_address_record(address_record_search: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, &address_record_search, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &address_record_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let address_record: AddressRecord = serde_json::from_value(entry).unwrap();
            match r.delete(address_record._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &address_record_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_address_record(address_record: String, ip_address: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "name": address_record,
        "ipv4addr": ip_address,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<AddressRecord> {
    let entries: Vec<Value> = entries;
    let mut return_address_records = vec![];
    for entry in entries {
        let address_record: AddressRecord = serde_json::from_value(entry).unwrap();
        return_address_records.push(address_record);
    }
    return_address_records

}

fn get_address_record(cname: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, cname, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", cname);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
}
#[cfg(test)]
mod test_cname {
    use bloxconfig;
    use mockito::{Matcher, mock};
    use a_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_address_record_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:a?name=foo&view=Public");
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
    fn test_get_address_record_single_response () {
        let out = r#"[{
            "name": "foo.mozilla.com",
            "_ref": "asfdsadf/Private",
            "view": "Private",
            "ipv4addr": "10.0.0.1"
          }]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let search=format!("record:a?name=foo&view=Public");
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
