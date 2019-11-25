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
struct TxtRecordResponse {
    objects: Vec<TxtRecord>
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct TxtRecord {
    _ref: String,
    name: String,
    text: String,
    view: String,
}

impl std::fmt::Display for TxtRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} text={} view={}", self._ref, self.name, self.text, self.view)
    }
}

const ENDPOINT: &'static str = "record:txt";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v1.4.1/record:a\?name\=foo.domain.com\&view\=View

pub fn execute(txt_record_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut txt_record_search = "";
    let mut view = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = txt_record_matches.subcommand_matches("get") {
        match _get_matches.value_of("txt_record"){
            Some(value) => { txt_record_search = value },
            None => println!("TXT Record Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_txt_record(txt_record_search.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = txt_record_matches.subcommand_matches("create") {
        let mut txt_record_search = "";
        let mut text = "";
        let mut view = "";
        match _get_matches.value_of("txt_record"){
            Some(value) => { txt_record_search = value },
            None => println!("txt_record Required")
        }
        match _get_matches.value_of("text"){
            Some(value) => { text = value },
            None => println!("Text Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        create_txt_record(txt_record_search.to_string(), text.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = txt_record_matches.subcommand_matches("delete") {
        match _get_matches.value_of("txt_record"){
            Some(value) => { txt_record_search = value },
            None => println!("TXT Record Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_txt_record(txt_record_search.to_string(), view.to_string(), config.clone());
    }
}

fn delete_txt_record(txt_record_search: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, &txt_record_search, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &txt_record_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let txt_record: TxtRecord = serde_json::from_value(entry).unwrap();
            match r.delete(txt_record._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &txt_record_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}

fn create_txt_record(txt_record: String, text: String, view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };

    let post_data = json!({
        "name": txt_record,
        "text": text,
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

fn serialize_entries(entries: Vec<Value>) -> Vec<TxtRecord> {
    let entries: Vec<Value> = entries;
    let mut return_txt_records = vec![];
    for entry in entries {
        let txt_record: TxtRecord = serde_json::from_value(entry).unwrap();
        return_txt_records.push(txt_record);
    }
    return_txt_records

}

fn get_txt_record(txt_record: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, txt_record, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", txt_record);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
}
#[cfg(test)]
mod test_wtxt {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use mockito::SERVER_URL;
    use txt_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_txt_record_empty () {
        let out = r#"[]"#;
        let url = SERVER_URL.to_string();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:txt?name=foo&view=Public");
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
    fn test_get_txt_record_single_response () {
        let out = r#"[{
            "name": "foo.mozilla.com",
            "_ref": "asfdsadf/Private",
            "view": "Private",
            "text": "thetext"
          }]"#;
        let url = SERVER_URL.to_string();
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