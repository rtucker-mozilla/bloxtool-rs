use clap;
use bloxconfig;
use restapi;
use serde_json;
use std;


#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
struct Ipv4addr {
    _ref: String,
    #[serde(default)]
    mac: String,
    #[serde(default)]
    ipv4addr: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
struct Host {
    _ref: String,
    name: String,
    ipv4addrs: Vec<Ipv4addr>,
    view: String,
}
impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_ref={} name={} view={}", self._ref, self.name, self.view)
    }
}

const ENDPOINT: &'static str = "record:host";


// Main entry point for someone running bloxtool host <subcommand>
pub fn execute(host_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut host_search = "";
    let mut view = "";
    let mut ipv4addr = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("hostname"){
            Some(value) => { host_search = value },
            None => println!("Host Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        get_host(host_search.to_string(), view.to_string(), config.clone());
    }

    if let Some(_get_matches) = host_matches.subcommand_matches("create") {
        let mut mac = "";
        match _get_matches.value_of("hostname"){
            Some(value) => { host_search = value },
            None => println!("Host Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        match _get_matches.value_of("ipv4addr"){
            Some(value) => { ipv4addr = value },
            None => println!("ipv4addr Required")
        }
        match _get_matches.value_of("mac"){
            Some(value) => { mac = value },
            None => { }
        }
        create_host(host_search.to_string(), ipv4addr.to_string(), view.to_string(), mac.to_string(), config.clone());
    }

    if let Some(_get_matches) = host_matches.subcommand_matches("delete") {
        match _get_matches.value_of("hostname"){
            Some(value) => { host_search = value },
            None => println!("Host Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        delete_host(host_search.to_string(), view.to_string(), config.clone());
    }
}
fn delete_host(host: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, host, view);
    let url = format!("{}/{}", &config.full_path(), search);
    let r = restapi::RESTApi {
        url: url,
        config: config.clone()
    };
    match r.get() {
        Some(resp) => {
            for entry in resp{
                let host: Host = serde_json::from_value(entry).unwrap();
                println!("delete {}", host);
            }
        },
        None => { }
    }

    /*
    match r.delete_object() {
        // need to decide how to reformat the url here for delete
        Some(_status) => { println!("deleted")},
        None => { println!("Error=Not Found." )}
    }
    */
}

/*
fn get_host(hostname: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name~={}&view={}", ENDPOINT, hostname, view);
    let url = format!("{}/{}", config.full_path(), search);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };
    match r.get_object() {
        Some(resp) => {
            for obj in resp.json {
                let c: Host = serde_json::from_value(obj).unwrap();
                println!("{}", c);
            }
        },
        None => { println!("Not Found.") }
    }
}
*/

fn get_host(hostname: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name~={}&view={}", ENDPOINT, hostname, view);
    let url = format!("{}/{}", config.full_path(), search);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };
    match r.get() {
        Some(resp) => {
            println!("{}", resp.len());
            for entry in resp{
                let host: Host = serde_json::from_value(entry).unwrap();
                println!("{}", host);
            }
        },
        None => { println!("Not Found.") }
    }
}

fn create_host(hostname: String, ipv4addr: String, view: String, mac: String, config: bloxconfig::Config) {
    let url = format!("{}/{}", config.full_path(), ENDPOINT);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };

    let mut addrobject = json!({
        "ipv4addr": ipv4addr,
    });

    // mac is an optional param, adding a blank one gets a complaint from the API
    if mac.len() > 0 {
        addrobject["mac"] = serde_json::Value::String(mac);
    }

    let post_data = json!({
        "name": hostname,
        "ipv4addrs": [ addrobject ],
        "view": view,
    });

    r.create_object(post_data);
}

#[test]
fn test_count_results_empty() {
    assert_eq!(0, 0);
}