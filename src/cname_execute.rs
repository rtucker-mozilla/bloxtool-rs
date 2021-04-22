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
struct CnameResponse {
    objects: Vec<Cname>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct Cname {
    _ref: String,
    canonical: String,
    name: String,
    view: String,
}

impl std::fmt::Display for Cname {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} target={} view={}", self._ref, self.name, self.canonical, self.view)
    }
}

const ENDPOINT: &'static str = "record:cname";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:cname\?name\=foo.domain.com\&view\=View

pub fn execute(cname_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut cname_search = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = cname_matches.subcommand_matches("get") {
        match _get_matches.value_of("cname"){
            Some(value) => { cname_search = value },
            None => println!("cname Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_cname(cname_search.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = cname_matches.subcommand_matches("create") {
        let mut cname_search = "";
        let mut alias = "";
        let mut view = "";
        match _get_matches.value_of("cname"){
            Some(value) => { cname_search = value },
            None => println!("cname Required")
        }
        match _get_matches.value_of("alias"){
            Some(value) => { alias = value },
            None => println!("alias Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        create_cname(cname_search.to_string(), alias.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = cname_matches.subcommand_matches("delete") {
        match _get_matches.value_of("cname"){
            Some(value) => { cname_search = value },
            None => println!("cname Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_cname(cname_search.to_string(), view.to_string(), config.clone());
    }
}

fn delete_cname(cname_search: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, &cname_search, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &cname_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let cname: Cname = serde_json::from_value(entry).unwrap();
            match r.delete(cname._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &cname_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_cname(cname: String, name: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "canonical": cname,
        "name": name,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<Cname> {
    let entries: Vec<Value> = entries;
    let mut return_cnames = vec![];
    for entry in entries {
        let cname: Cname = serde_json::from_value(entry).unwrap();
        return_cnames.push(cname);
    }
    return_cnames

}

fn get_cname(cname: String, view: String, config: bloxconfig::Config) {
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
    use mockito;
    use mockito::{Matcher, mock, reset};
    use cname_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_cname_empty () {
        let out = r#"[]"#;
        let url = &mockito::server_url().to_string();
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
            "name": "foo.mozilla.com",
            "_ref": "asfdsadf/Private",
            "view": "Private",
            "canonical": "mozilla.com"
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
