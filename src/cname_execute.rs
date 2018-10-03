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

#[test]
fn test_count_results_empty() {
    let data = r#"
        []
    "#;
    let v: Vec<Cname> = serde_json::from_str(data).unwrap();
    let res = count_results(v);
    assert_eq!(res, 0);
}

#[test]
fn test_count_results_1_entry() {
    let data = r#"
        [
            {
            "_ref": "foo.bar.baz/Public",
            "name": "foo.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.1",
                "mac": "00:00:00:00:00:00"
                }
            ]
            }
        ]
    "#;
    let v: Vec<Cname> = serde_json::from_str(data).unwrap();
    let res = count_results(v);
    assert_eq!(res, 1);
}
#[test]
fn test_count_results_2_entry() {
    let data = r#"
        [
            {
            "_ref": "foo.bar.baz/Public",
            "name": "foo.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.1",
                "mac": "00:00:00:00:00:00"
                }
            ]
            },
            {
            "_ref": "foo2.bar.baz/Public",
            "name": "foo2.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo2.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.2",
                "mac": "00:00:00:00:00:0A"
                }
            ]
            }
        ]
    "#;
    let v: Vec<Cname> = serde_json::from_str(data).unwrap();
    let res = count_results(v);
    assert_eq!(res, 2);
}

#[test]
fn test_list_to_struct() {
    let data = r#"
        [
            {
            "_ref": "foo.bar.baz/Public",
            "name": "foo.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.1",
                "mac": "00:00:00:00:00:00"
                }
            ]
            }
        ]
    "#;
    let v: Vec<Cname> = serde_json::from_str(data).unwrap();
    let line = list_to_struct(v);
    let proper_line = "_ref=foo.bar.baz/Public hostname=foo.bar.baz ipv4addr=10.0.0.1 mac=00:00:00:00:00:00".to_string();
    assert_eq!(line[0].output(), proper_line);
}

#[test]
fn test_list_to_struct_2_entries() {
    let data = r#"
        [
            {
            "_ref": "foo.bar.baz/Public",
            "name": "foo.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.1",
                "mac": "00:00:00:00:00:00"
                }
            ]
            },
            {
            "_ref": "foo2.bar.baz/Public",
            "name": "foo2.bar.baz",
            "ipv4addrs": [
                {
                "_ref": "foo2.bar.baz/10.0.0.1",
                "ipv4addr": "10.0.0.2",
                "mac": "00:00:00:00:00:0A"
                }
            ]
            }
        ]
    "#;
    let v: Vec<Cname> = serde_json::from_str(data).unwrap();
    let line = list_to_struct(v);
    let proper_line = "_ref=foo.bar.baz/Public hostname=foo.bar.baz ipv4addr=10.0.0.1 mac=00:00:00:00:00:00".to_string();
    let proper_line2 = "_ref=foo2.bar.baz/Public hostname=foo2.bar.baz ipv4addr=10.0.0.2 mac=00:00:00:00:00:0A".to_string();
    assert_eq!(line[0].output(), proper_line);
    assert_eq!(line[1].output(), proper_line2);
}
