use clap;
use bloxconfig;
use restapi;
use restapi::InfobloxResponse;
use serde_json;
use host_execute::Host;
use cname_execute::Cname;
use mx_execute::MX;
use aaaa_execute::AAAA;
use a_execute::AddressRecord;
use txt_execute::TxtRecord;
use domain_authoratative_execute::AuthoratativeDomain;
use domain_delegated_execute::DelegatedDomain;

const ENDPOINT: &'static str = "search";
// Main entry point for someone running bloxtool host <subcommand>
// curl -u "username:password" https://<hostname>/wapi/v2.9/search\?search_string~\=foo.domain.com

fn extract_type_from_ref(_ref_val: String) -> Result<String, String> {
    let _ref = _ref_val.trim_matches('"');
    let _split: Vec<&str> = _ref.split("/").collect();
    if _split.len() >= 2 {
        let ret_val = String::from(_split[0]);
        return Ok(ret_val);
    }
    return Err(String::from("Unable to extract type from ref"))

}

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
        let obj_type = extract_type_from_ref(obj["_ref"].to_string()).unwrap();
        match obj_type.as_str() {
            "record:host" => { 
                let host: Host = serde_json::from_value(clone).unwrap();
                println!("type=record:host {}", host);
            },
            "record:mx" => { 
                let mx: MX = serde_json::from_value(clone).unwrap();
                println!("type=record:mx {}", mx);
            },
            "record:a" => { 
                let a: AddressRecord = serde_json::from_value(clone).unwrap();
                println!("type=record:a {}", a);
            },
            "record:cname" => { 
                let cname: Cname = serde_json::from_value(clone).unwrap();
                println!("type=record:cname {}", cname);
            },
            "record:aaaa" => { 
                let aaaa: AAAA = serde_json::from_value(clone).unwrap();
                println!("type=record:aaaa {}", aaaa);
            },
            "record:txt" => { 
                let txt: TxtRecord = serde_json::from_value(clone).unwrap();
                println!("type=record:txt {}", txt);
            },
            "zone_auth" => {
                let zone: AuthoratativeDomain = serde_json::from_value(clone).unwrap();
                println!("type=domain:authoratative {}", zone);
            },
            "zone_delegated" => {
                let zone: DelegatedDomain = serde_json::from_value(clone).unwrap();
                println!("type=domain:delegated {}", zone);
            },
            _ => { println!("type=unclassified {:?}", entry) }
        }
    }
}
#[cfg(test)]
#[test]
fn test_extract_type_from_ref_proper_response() {
    let input = String::from("record:host/foo.bar.baz");
    let proper = "record:host";
    match extract_type_from_ref(input) {
        Ok(_msg) => { assert_eq!(_msg, proper) },
        Err(_err) => {} 
    }
    
}

#[test]
fn test_extract_type_from_ref_improper_response() {
    let input = String::from("invalid response");
    let proper = "Unable to extract type from ref";
    match extract_type_from_ref(input) {
        Ok(_msg) => {},
        Err(_err) => { assert_eq!(_err, proper )} 
    }
}