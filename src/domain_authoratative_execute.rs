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
struct AuthoratativeDomainResponse {
    objects: Vec<AuthoratativeDomain>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct AuthoratativeDomain {
    _ref: String,
    fqdn: String,
    view: String,
}

impl std::fmt::Display for AuthoratativeDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} view={}", self._ref, self.fqdn, self.view)
    }
}

const ENDPOINT: &'static str = "zone_auth";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:cname\?name\=foo.domain.com\&view\=View

pub fn execute(domain_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut name = "";
    let mut view = "";
    let mut nameserver_group = "";
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
        match _get_matches.value_of("nameserver_group"){
            Some(value) => { nameserver_group = value },
            None => { }
        }
        create_authoratative_domain(name.to_string(), view.to_string(), nameserver_group.to_string(), config.clone());
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
            let domain: AuthoratativeDomain = serde_json::from_value(entry).unwrap();
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

fn create_authoratative_domain(name: String, view: String, nameserver_group: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let mut post_data = json!({
        "fqdn": name,
        "view": view,
    });
    if nameserver_group != "" {
        post_data["ns_group"] = serde_json::Value::String(nameserver_group);
    }
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
fn serialize_entries(entries: Vec<Value>) -> Vec<AuthoratativeDomain> {
    let entries: Vec<Value> = entries;
    let mut return_domains = vec![];
    for entry in entries {
        let domain: AuthoratativeDomain = serde_json::from_value(entry).unwrap();
        return_domains.push(domain);
    }
    return_domains

}

#[cfg(test)]
mod test_cname {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use domain_authoratative_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_cname_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
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
}