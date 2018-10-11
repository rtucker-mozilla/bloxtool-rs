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
struct Cname {
    _ref: String,
    canonical: String,
    name: String,
    view: String,
}

impl std::fmt::Display for Cname {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} view={}", self._ref, self.name, self.view)
    }
}

const ENDPOINT: &'static str = "record:cname";

// Main entry point for someone running bloxtool host <subcommand>
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
                    println!("Deleted: {}", &cname_search);
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
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let cname: Cname = serde_json::from_value(entry).unwrap();
            println!("{}", cname);
        }
    }
}