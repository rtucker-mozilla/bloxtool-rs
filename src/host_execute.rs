use clap;
use bloxconfig;
use restapi;
use serde_json;


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
}


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

fn count_results(input_json: Vec<Host>) -> usize {
    let array: Vec<Host> = input_json;
    return array.len();
}

fn list_to_struct(i_list: Vec<Host>) -> Vec<restapi::InfobloxResponse>{
    let mut restoutput_array: Vec<restapi::InfobloxResponse> = Vec::new();
    let delim = "=";
    for item in i_list {
        restoutput_array.push(
            restapi::InfobloxResponse {
                line: format!("_ref{}{} hostname{}{} ipv4addr{}{} mac{}{}", delim, item._ref, delim,item.name, delim,item.ipv4addrs[0].ipv4addr, delim,item.ipv4addrs[0].mac)
            }
        )
    }
    return restoutput_array;
}

fn delete_host(hostname: String, view: String, config: bloxconfig::Config) {
    let host_search=format!("record:host?name={}&view={}", hostname, view);
    let url = format!("{}/{}", &config.full_path(), host_search);
    let r = restapi::RESTApi {
        url: url,
        config: config.clone()
    };
    match r.get() { 
        // MAke sure we are able to talk to the server
        Ok(mut data) => { 
            // Make sure the json response is valid
            match data.json() {
                Ok(inside) => { 
                    // convert json list to array of Host structs
                    let resp_obj: serde_json::Value = inside;
                    let _ref = &resp_obj[0]["_ref"].to_string();
                    if _ref.len() == 0 || _ref == "null" {
                        println!("Error: Host not found. {}", hostname);
                    } else {
                        let delete_url = format!("{}/{}", config.clone().full_path(), _ref.trim_matches('"'));
                        let d = restapi::RESTApi {
                            url: delete_url,
                            config: config.clone()
                        };

                        match d.delete() {
                            Ok(mut resp) => { 
                                let resp_obj: serde_json::Value = resp.json().unwrap();
                                if resp.status() == 400 {
                                    println!("Error: {}", resp_obj["text"]);
                                }
                                if resp.status() == 200 {
                                    println!("Success {}", resp_obj);
                                }
                            },
                            Err(error) => { println!("Error: {}", error)}
                        }
                    }
                },
                Err(error) => { println!("Error: {}", error)}
            }
        },
        Err(error) => { println!("Error: {}", error)}
    }

}
fn create_host(hostname: String, ipv4addr: String, view: String, mac: String, config: bloxconfig::Config) {
    let host_search="record:host";
    let url = format!("{}/{}", config.full_path(), host_search);
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

    match r.post(post_data) {
        Ok(mut resp) => { 
            let resp_obj: serde_json::Value = resp.json().unwrap();
            if resp.status() == 400 {
                println!("Error: {}", resp_obj["text"]);
            }
            if resp.status() == 201 {
                println!("Success {}", resp_obj);
            }
        },
        Err(error) => { println!("Error: {}", error)}
    }
}

fn get_host(host_search: String, view: String, config: bloxconfig::Config) {
    let host_search=format!("record:host?name~={}&view={}", host_search, view);
    let url = format!("{}/{}", config.full_path(), host_search);
    let r = restapi::RESTApi {
        url: url,
        config: config
    };
    match r.get() { 
        // MAke sure we are able to talk to the server
        Ok(mut data) => { 
            // Make sure the json response is valid
            match data.json() {
                Ok(inside) => { 
                    // convert json list to array of Host structs
                    let array: Vec<Host> = inside;
                    let result_count = count_results(array.clone());

                    if result_count == 0 {
                        println!("No Results Found");
                    } else {
                        let lines = list_to_struct(array.clone());
                        for item in lines {
                            println!("{}", item.output());
                        }
                    }
                    },
                Err(error) => { println!("Error: {}", error)}
            }
        },
        Err(error) => { println!("Error: {}", error)}
    }
}

#[test]
fn test_count_results_empty() {
    let data = r#"
        []
    "#;
    let v: Vec<Host> = serde_json::from_str(data).unwrap();
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
    let v: Vec<Host> = serde_json::from_str(data).unwrap();
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
    let v: Vec<Host> = serde_json::from_str(data).unwrap();
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
    let v: Vec<Host> = serde_json::from_str(data).unwrap();
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
    let v: Vec<Host> = serde_json::from_str(data).unwrap();
    let line = list_to_struct(v);
    let proper_line = "_ref=foo.bar.baz/Public hostname=foo.bar.baz ipv4addr=10.0.0.1 mac=00:00:00:00:00:00".to_string();
    let proper_line2 = "_ref=foo2.bar.baz/Public hostname=foo2.bar.baz ipv4addr=10.0.0.2 mac=00:00:00:00:00:0A".to_string();
    assert_eq!(line[0].output(), proper_line);
    assert_eq!(line[1].output(), proper_line2);
}
