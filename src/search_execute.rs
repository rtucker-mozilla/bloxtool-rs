use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use host_execute::Host;
use cname_execute::Cname;
use mx_execute::MX;
use a_execute::AddressRecord;
use txt_execute::TxtRecord;

const ENDPOINT: &'static str = "search";

// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v2.9/search\?search_string~\=foo.domain.com

pub fn execute(search_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let search_string = search_matches.value_of("search_string").unwrap();
    let mut objtype_string = String::from("");
    match search_matches.value_of("objtype") {
        Some(_val) => {
            objtype_string = format!("&objtype={}", _val).to_string();
        },
        None => {}
    }
    let search=format!("{}?search_string~={}{}", ENDPOINT, search_string, objtype_string);
    let r = restapi::RESTApi {
        config: config
    };
    let mut api_out = InfobloxResponse{ ..Default::default() };
    api_out.process(r.get(search));
    for entry in api_out.response {
        let clone = entry.clone();
        let obj = &entry.as_object().unwrap();
        let _ref_val = obj["_ref"].to_string();
        let _ref = _ref_val.trim_matches('"');
        let _split: Vec<&str> = _ref.split("/").collect();
        if _split.len() > 0 {
            match &_split[0] {
                &"record:host" => { 
                    let host: Host = serde_json::from_value(clone).unwrap();
                    println!("type=record:host {}", host);
                },
                &"record:mx" => { 
                    let mx: MX = serde_json::from_value(clone).unwrap();
                    println!("type=record:mx {}", mx);
                },
                &"record:a" => { 
                    let a: AddressRecord = serde_json::from_value(clone).unwrap();
                    println!("type=record:a {}", a);
                },
                &"record:cname" => { 
                    let cname: Cname = serde_json::from_value(clone).unwrap();
                    println!("type=record:cname {}", cname);
                },
                &"record:txt" => { 
                    let txt: TxtRecord = serde_json::from_value(clone).unwrap();
                    println!("type=record:txt {}", txt);
                },
                _ => { println!("{:?}", entry) }
            }
        }
    }
}