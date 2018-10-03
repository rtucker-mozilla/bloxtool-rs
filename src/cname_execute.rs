use clap;
use bloxconfig;
use restapi;
use serde_json;
use std;

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

fn delete_cname(cname: String, view: String, config: bloxconfig::Config) {
    let cname_search=format!("{}?name={}&view={}", ENDPOINT, cname, view);
    let url = format!("{}/{}", &config.full_path(), cname_search);
    let r = restapi::RESTApi {
        url: url,
        config: config.clone()
    };

    match r.delete_object() {
        // need to decide how to reformat the url here for delete
        Some(_status) => { println!("deleted")},
        None => { println!("Error=Not Found." )}
    }
}
fn create_cname(cname: String, name: String, view: String, config: bloxconfig::Config) {
    let url = format!("{}/{}", config.full_path(), ENDPOINT);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };

    let post_data = json!({
        "canonical": cname,
        "name": name,
        "view": view,
    });

    r.create_object(post_data);
}

fn get_cname(cname_search: String, view: String, config: bloxconfig::Config) {
    let cname_search=format!("{}?name~={}&view={}", ENDPOINT, cname_search, view);
    let url = format!("{}/{}", config.full_path(), cname_search);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };
    match r.get_object() {
        Some(resp) => {
            for obj in resp.json {
                let c: Cname = serde_json::from_value(obj).unwrap();
                println!("{}", c);
            }
        },
        None => { println!("Not Found.") }
    }
}