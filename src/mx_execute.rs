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
struct MXResponse {
    objects: Vec<MX>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct MX {
    _ref: String,
    mail_exchanger: String,
    name: String,
    preference: u16,
    view: String,
}

impl std::fmt::Display for MX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} mail_exchanger={} preference={} view={}", self._ref, self.name, self.mail_exchanger, self.preference, self.view)
    }
}

const ENDPOINT: &'static str = "record:mx";

// Main entry point for someone running bloxtool record:mx <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v2.9/record:mx\?name\=foo.domain.com\&view\=View

pub fn execute(mx_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut name = "";
    let mut mail_exchanger = "";
    let mut preference = 0;
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = mx_matches.subcommand_matches("get") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }

        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_mx(name.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = mx_matches.subcommand_matches("create") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("mail_exchanger"){
            Some(value) => { mail_exchanger = value },
            None => println!("mail_exchanger Required")
        }

        match _get_matches.value_of("preference"){
            Some(value) => { preference = value.parse::<u16>().unwrap() },
            None => println!("preference Required")
        }

        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        create_mx(name.to_string(), mail_exchanger.to_string(), preference, view.to_string(), config.clone());
    }

    if let Some(_get_matches) = mx_matches.subcommand_matches("delete") {
        match _get_matches.value_of("name"){
            Some(value) => { name = value },
            None => println!("name Required")
        }
        match _get_matches.value_of("mail_exchanger"){
            Some(value) => { mail_exchanger = value },
            None => println!("mail_exchanger Required")
        }

        match _get_matches.value_of("preference"){
            Some(value) => { preference = value.parse::<u16>().unwrap() },
            None => println!("preference Required")
        }

        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_mx(name.to_string(), mail_exchanger.to_string(), preference, view.to_string(), config.clone());
    }
}

fn delete_mx(name: String, mail_exchanger: String, preference: u16, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&mail_exchanger={}&preference={}&view={}", ENDPOINT, &name, &mail_exchanger, &preference, view);
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
            let mx: MX = serde_json::from_value(entry).unwrap();
            match r.delete(mx._ref) {
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

fn create_mx(name: String, mail_exchanger: String, preference: u16, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "name": name,
        "mail_exchanger": mail_exchanger,
        "preference": preference,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<MX> {
    let entries: Vec<Value> = entries;
    let mut return_mx = vec![];
    for entry in entries {
        let mx: MX = serde_json::from_value(entry).unwrap();
        return_mx.push(mx);
    }
    return_mx

}

fn get_mx(name: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, name, view);
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
mod test_mx {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use mx_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_mx_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:mx?name=foo&view=Public");
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
    fn test_get_mx_single_response () {
        let out = r#"[{
            "name": "foo.mozilla.com",
            "mail_exchanger": "smtp.mozilla.com",
            "preference": 11,
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
        let search=format!("record:cname?name=foo&view=Public");
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
        let entry = &entries[0];
        assert_eq!(entry.name, "foo.mozilla.com");
        assert_eq!(entry.preference, 11);
        reset();
    }
}