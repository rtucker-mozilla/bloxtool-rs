use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use serde_json::Value;
use std;
use std::process::exit;

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
struct Ipv6addr {
    _ref: String,
    #[serde(default)]
    duid: String,
    #[serde(default)]
    host: String,
    #[serde(default)]
    ipv6addr: String,
    #[serde(default)]
    configure_for_dhcp: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
struct Ipv4addr {
    _ref: String,
    #[serde(default)]
    mac: String,
    #[serde(default)]
    host: String,
    #[serde(default)]
    ipv4addr: String,
    #[serde(default)]
    configure_for_dhcp: bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct Host {
    _ref: String,
    name: String,
    #[serde(default)]
    ipv4addrs: Vec<Ipv4addr>,
    #[serde(default)]
    ipv6addrs: Vec<Ipv6addr>,
    view: String,
}
impl Host {
    fn build_display_string(&self) -> String {
        let ref_string = format!("_ref={}", self._ref);
        let name_string = format!("name={}", self.name);
        let view_string = format!("view={}", self.view);
        let mut outvec = vec![ref_string, name_string];

        if self.ipv4addrs.len() > 0 {
            let ipv4str = format!("ipv4addr={}", self.ipv4addrs[0].ipv4addr);
            outvec.push(ipv4str.to_string());
            if self.ipv4addrs[0].mac != "" {
                let ipv4macstr = format!("ipv4mac={}", self.ipv4addrs[0].mac);
                outvec.push(ipv4macstr.to_string());
            }
        }
        if self.ipv6addrs.len() > 0 {
            let ipv6str = format!("ipv6addr={}", self.ipv6addrs[0].ipv6addr);
            outvec.push(ipv6str.to_string());
            if self.ipv6addrs[0].duid != "" {
                let ipv6duidstr = format!("ipv6duid={}", self.ipv6addrs[0].duid);
                outvec.push(ipv6duidstr.to_string());
            }
        }
        outvec.push(view_string);
        return outvec.join(" ");
    }

}
impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let outstring = self.build_display_string();
        write!(f, "{}", outstring)
    }
}

const ENDPOINT: &'static str = "record:host";


// Main entry point for someone running bloxtool host <subcommand>
pub fn execute(host_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut host_search = "";
    let mut view = "";
    let mut ipv4addr = "";
    let mut search_string = "";
    // executed when someone does bloxtool host get <hostname> <view>
    if let Some(_get_matches) = host_matches.subcommand_matches("search") {
        match _get_matches.value_of("search_string"){
            Some(value) => { search_string = value },
            None => println!("Search String Required")
        }
        let search=format!("{}?{}", ENDPOINT, search_string);
        get_host(&search, view.to_string(), config.clone());
    }
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("hostname"){
            Some(value) => { host_search = value },
            None => println!("Host Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { view = value },
            None => println!("View Required")
        }
        let search=format!("{}?name={}&view={}", ENDPOINT, host_search, view);
        get_host(&search, view.to_string(), config.clone());
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

fn serialize_entries(entries: Vec<Value>) -> Vec<Host> {
    let entries: Vec<Value> = entries;
    let mut return_hosts = vec![];
    for entry in entries {
        let host: Host = serde_json::from_value(entry).unwrap();
        return_hosts.push(host);
    }
    return_hosts

}

fn delete_host(host_search: String, view: String, config: bloxconfig::Config) {
    let search=format!("{}?name={}&view={}", ENDPOINT, &host_search, view);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    if api_out.count == 0 {
        println!("Error: {} not found.", &host_search);
        exit(2);
    } else {
        let entries: Vec<Value> = api_out.response;
        for entry in entries {
            let host: Host = serde_json::from_value(entry).unwrap();
            match r.delete(host._ref) {
                Ok(_val) => {
                    println!("Success: Deleted {}", &host_search);
                },
                Err(_err) => {
                    println!("Error: {}", _err);
                    exit(2);
                }
            }
        }
    }
}


fn get_host(search: &str, _view: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search.to_string()));
    if api_out.count == 0 {
        println!("Error: {} not found.", search);
    } else {
        let entries = serialize_entries(api_out.response);
        for entry in entries {
            println!("{}", entry);
        }
    }
}

fn create_host(hostname: String, ipv4addr: String, view: String, mac: String, config: bloxconfig::Config) {
    let r = restapi::RESTApi {
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

#[cfg(test)]
mod test_host {
    use bloxconfig;
    use mockito::{Matcher, mock, reset};
    use host_execute::serialize_entries;
    use restapi::InfobloxResponse;
    use restapi;

    #[test]
    fn test_get_host_empty () {
        let out = r#"[]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let search=format!("record:host?name=foo&view=Public");
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
    fn test_get_host_single_response () {
        let out = r#"[
    {
        "_ref": "record:host/ZG5zLmhvc3QkLjE1LmNvbS5tb3ppbGxhLm1kYzEucHJpdmF0ZS5ydHVja2Vy:foo.mozilla.com/Private",
        "ipv4addrs": [
            {
                "_ref": "record:host_ipv4addr/ZG5zLmhvc3RfYWRkcmVzcyQuMTUuY29tLm1vemlsbGEubWRjMS5wcml2YXRlLnJ0dWNrZXIuMTAuNDguNzUuMjAyLg:10.0.0.1/foo.mozilla.com/Private",
                "configure_for_dhcp": false,
                "host": "foo.mozilla.com",
                "ipv4addr": "10.0.0.1"
            }
        ],
        "name": "foo.mozilla.com",
        "view": "Private"
    }
]"#;
        let url = mockito::server_url();
        let config = bloxconfig::Config{
            username: "admin".to_string(),
            password: "password".to_string(),
            allow_insecure_ssl: false,
            host: url
        };
        let mut api_out = InfobloxResponse{ ..Default::default() };
        let search=format!("record:host?name=foo&view=Public");
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
        assert_eq!(entries[0].name, "foo.mozilla.com");
        assert_eq!(entries[0].view, "Private");
        assert_eq!(entries[0]._ref, "record:host/ZG5zLmhvc3QkLjE1LmNvbS5tb3ppbGxhLm1kYzEucHJpdmF0ZS5ydHVja2Vy:foo.mozilla.com/Private");
        assert_eq!(entries[0].ipv4addrs[0]._ref, "record:host_ipv4addr/ZG5zLmhvc3RfYWRkcmVzcyQuMTUuY29tLm1vemlsbGEubWRjMS5wcml2YXRlLnJ0dWNrZXIuMTAuNDguNzUuMjAyLg:10.0.0.1/foo.mozilla.com/Private");
        assert_eq!(entries[0].ipv4addrs[0].ipv4addr, "10.0.0.1");
        assert_eq!(entries[0].ipv4addrs[0].host, "foo.mozilla.com");
        assert_eq!(entries[0].ipv4addrs[0].configure_for_dhcp, false);
        reset();
    }
}