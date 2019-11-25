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
struct NetworkResponse {
    objects: Vec<Network>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct Network {
    _ref: String,
    network: String,
    network_view: String,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} network={} network_view={}", self._ref, self.network, self.network_view)
    }
}

const ENDPOINT: &'static str = "network";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/\?name\=foo.domain.com\&view\=View

pub fn execute(network_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut network_search = "";
    let mut search_string = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = network_matches.subcommand_matches("search") {
        match _get_matches.value_of("network"){
            Some(value) => { search_string = value },
            None => println!("network Required")
        }
        let search=format!("{}?{}", ENDPOINT, search_string);
        get_network(&search, config.clone());
    }

    if let Some(_get_matches) = network_matches.subcommand_matches("get") {
        match _get_matches.value_of("network"){
            Some(value) => { network_search = value },
            None => println!("Network Required")
        }
        let search=format!("{}?network={}", ENDPOINT, network_search);
        get_network(&search, config.clone());
    }

    if let Some(_get_matches) = network_matches.subcommand_matches("create") {
        let mut network_search = "";
        match _get_matches.value_of("network"){
            Some(value) => { network_search = value },
            None => println!("network Required")
        }
        create_network(network_search.to_string(), config.clone());
    }

    if let Some(_get_matches) = network_matches.subcommand_matches("delete") {
        match _get_matches.value_of("network"){
            Some(value) => { network_search = value },
            None => println!("network Required")
        }
        delete_network(network_search.to_string(), config.clone());
    }
}

fn delete_network(network_search: String, config: bloxconfig::Config) {
    let search=format!("{}?network={}", ENDPOINT, &network_search);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &network_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let network: Network = serde_json::from_value(entry).unwrap();
            match r.delete(network._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &network_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_network(network: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "network": network,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<Network> {
    let entries: Vec<Value> = entries;
    let mut return_networks = vec![];
    for entry in entries {
        let network: Network = serde_json::from_value(entry).unwrap();
        return_networks.push(network);
    }
    return return_networks;

}

fn get_network(network_string: &str, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(network_string.to_string()));
    if api_out.count == 0 {
        println!("Error: {} not found.", network_string);
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
    use mockito::{Matcher, mock, reset};
    use mockito::SERVER_URL;
    use network_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_cname_empty () {
        let out = r#"[]"#;
        let url = SERVER_URL.to_string();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:cname?name=foo&view=Public");
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let r = restapi::RESTApi {
            config: config
        };
        // There is a bug on windows that always sets the verb to <unknown>
        // https://github.com/lipanski/mockito/issues/41
        let mut verb = "get";
        if cfg!(windows) {
            verb = "<UNKNOWN>";
        }
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
    fn test_get_cname_single_response () {
        let out = r#"[{
            "_ref": "network/10.0.0.0/24",
            "network": "10.0.0.0/24",
            "network_view": "Private"
          }]"#;
        let url = SERVER_URL.to_string();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let search=format!("record:cname?name=foo&view=Public");
        let r = restapi::RESTApi {
            config: config
        };
        // There is a bug on windows that always sets the verb to <unknown>
        // https://github.com/lipanski/mockito/issues/41
        let mut verb = "get";
        if cfg!(windows) {
            verb = "<UNKNOWN>";
        }
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