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
struct CAAResponse {
    objects: Vec<CAA>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct CAA {
    _ref: String,
    ca_flag: u32,
    ca_tag: String,
    ca_value: String,
    name: String,
    view: String,
}

impl std::fmt::Display for CAA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} ca_value={} name={} view={}", self._ref, self.ca_value, self.name, self.view)
    }
}

const ENDPOINT: &'static str = "record:caa";

pub fn execute(caa_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut domain = "";
    let mut view = "";
    let mut tag = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = caa_matches.subcommand_matches("get") {
        match _get_matches.value_of("domain"){
            Some(value) => { domain = value },
            None => println!("domain Required")
        }
        match _get_matches.value_of("tag"){
            Some(value) => { tag = value },
            None => println!("tag Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_caa(domain.to_string(), tag.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = caa_matches.subcommand_matches("create") {
        let mut domain = "";
        let mut tag = "";
        let mut value = "";
        let mut view = "";
        match _get_matches.value_of("domain"){
            Some(value) => { domain = value },
            None => println!("domain Required")
        }
        match _get_matches.value_of("tag"){
            Some(value) => { tag = value },
            None => println!("tag Required")
        }
        match _get_matches.value_of("value"){
            Some(ivalue) => { value = ivalue },
            None => println!("value Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("view Required")
        }
        create_caa(domain.to_string(), tag.to_string(), value.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = caa_matches.subcommand_matches("delete") {
        let mut domain = "";
        let mut tag = "";
        let mut value = "";
        let mut view = "";
        match _get_matches.value_of("domain"){
            Some(value) => { domain = value },
            None => println!("domain Required")
        }
        match _get_matches.value_of("tag"){
            Some(value) => { tag = value },
            None => println!("tag Required")
        }
        match _get_matches.value_of("value"){
            Some(ivalue) => { value = ivalue },
            None => println!("value Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("view Required")
        }
        delete_caa(domain.to_string(), tag.to_string(), value.to_string(), view.to_string(), config.clone());
    }
}

fn delete_caa(domain: String, tag: String, value: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&ca_tag={}&view={}&_return_fields=name,ca_tag,ca_flag,ca_value,view", ENDPOINT, domain, tag, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &domain);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let caa: CAA = serde_json::from_value(entry).unwrap();
            match r.delete(caa._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {} for domain {}", &value, &domain);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_caa(domain: String, tag: String, value: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "name": domain,
        "ca_tag": tag,
        "ca_flag": 0,
        "ca_value": value,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<CAA> {
    let entries: Vec<Value> = entries;
    let mut return_caas = vec![];
    for entry in entries {
        let caa: CAA = serde_json::from_value(entry).unwrap();
        return_caas.push(caa);
    }
    return_caas

}

fn get_caa(domain: String, tag: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&ca_tag={}&view={}&_return_fields=name,ca_tag,ca_flag,ca_value,view", ENDPOINT, domain, tag, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", domain);
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
    use mockito::SERVER_URL;
    use caa_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_caa_empty () {
        let out = r#"[]"#;
        let url = SERVER_URL.to_string();
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
    fn test_get_caa_single_response () {
        let out = r#"[{
            "name": "foo.mozilla.com",
            "ca_tag": "issue",
            "ca_flag": 0,
            "ca_value": "digicert.com",
            "_ref": "asfdsadf/Private",
            "view": "Private"
          }]"#;
        let url = SERVER_URL.to_string();
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