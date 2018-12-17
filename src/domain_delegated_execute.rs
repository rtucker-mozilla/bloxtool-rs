use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use serde_json::Value;
use std;
use std::net::ToSocketAddrs;
use std::process::exit;

#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct DelegatedDomainResponse {
    objects: Vec<DelegatedDomain>
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct DelegateTo {
    name: String,
    address: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct DelegatedDomain {
    _ref: String,
    fqdn: String,
    view: String,
}

impl std::fmt::Display for DelegatedDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} view={}", self._ref, self.fqdn, self.view)
    }
}

const ENDPOINT: &'static str = "zone_delegated";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:cname\?name\=foo.domain.com\&view\=View

pub fn execute(domain_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut name = "";
    let mut nameservers = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = domain_matches.subcommand_matches("create") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        match _get_matches.value_of("nameservers"){
            Some(value) => { 
                nameservers = value
            },
            None => println!("nameservers Required")
        }
        create_delegated_domain(name, nameservers, view, config.clone());
    }
    if let Some(_get_matches) = domain_matches.subcommand_matches("delete") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_authoratative_domain(name.to_string(), view.to_string(), config.clone());
    }

}

fn delete_authoratative_domain(name: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?fqdn={}&view={}", ENDPOINT, &name, view);
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
            let domain: DelegatedDomain = serde_json::from_value(entry).unwrap();
            match r.delete(domain._ref) {
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

fn create_delegated_domain(name: &str, nameservers: &str, view: &str,  config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let nameservers_vec = extract_nameservers(nameservers).unwrap();
    let mut delegated_vec: Vec<DelegateTo> = vec![];
    for i in nameservers_vec {
        let tmp = DelegateTo {
            name: i.to_string(),
            address: hostname_to_ip(&i).unwrap()
        };
        delegated_vec.push(tmp);

    }
    let nameserver_json = json!(delegated_vec);

    let post_data = json!({
        "fqdn": name,
        "delegate_to": nameserver_json,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<DelegatedDomain> {
    let entries: Vec<Value> = entries;
    let mut return_domains = vec![];
    for entry in entries {
        let domain: DelegatedDomain = serde_json::from_value(entry).unwrap();
        return_domains.push(domain);
    }
    return_domains

}

fn hostname_to_ip(hostname: &str) -> Result<String, String> {
    let lookup_address = format!("{}:53", hostname);
    let error_string = "Unable to lookup DNS Server.".to_string();
    match lookup_address.to_socket_addrs() {
        Ok(lookup_results) => {
            let mut return_address = String::from("");
            for i in lookup_results {
                if i.is_ipv4() {
                    if return_address == "" {
                        return_address = i.ip().to_string();
                    }
                }
            }
            if return_address != "" {
                return Ok(return_address)
            } else {
                return Err(error_string)
            }
        },
        Err(_err) => { return Err(error_string)}
    }
}

fn extract_nameservers(nameservers: &str) -> Result<Vec<String>, String> {
    let split_string: Vec<&str> = nameservers.split(",").collect();
    let mut return_split = vec![];
    for i in split_string {
        if i.len() > 0 {
            return_split.push(String::from(i));
        }
    }
    if return_split.len() > 0 {
        return Ok(return_split);
    } else {
        return Err(String::from("Unable to Split String"));
    }
}
#[cfg(test)]
mod test_cname {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use mockito::SERVER_URL;
    use domain_delegated_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;
    use domain_delegated_execute;

    #[test]
    fn test_extract_nameservers_single_ns_no_comma() {
        let nameservers = "ns.domain.com";
        let proper = vec!["ns.domain.com"];
        let response = domain_delegated_execute::extract_nameservers(nameservers).unwrap();
        assert_eq!(response, proper);
    }
    #[test]
    fn test_extract_nameservers_single_ns_with_comma() {
        let nameservers = "ns.domain.com,";
        let proper = vec!["ns.domain.com"];
        let response = domain_delegated_execute::extract_nameservers(nameservers).unwrap();
        assert_eq!(response, proper);
    }
    #[test]
    fn test_extract_multiple_nameservers() {
        let nameservers = "ns.domain.com,ns2.domain.com";
        let proper = vec!["ns.domain.com","ns2.domain.com"];
        let response = domain_delegated_execute::extract_nameservers(nameservers).unwrap();
        assert_eq!(response, proper);
    }
    #[test]
    fn test_get_cname_empty () {
        let out = r#"[]"#;
        let url = SERVER_URL.to_string();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
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
}