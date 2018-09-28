use clap;
use bloxconfig;
use restapi;


#[derive(Deserialize)]
#[allow(dead_code)]
struct Ipv4addr {
    _ref: String,
    #[serde(default)]
    mac: String,
    #[serde(default)]
    ipv4addr: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Host {
    _ref: String,
    name: String,
    ipv4addrs: Vec<Ipv4addr>,
}


// Main entry point for someone running bloxtool host <subcommand>
pub fn execute(host_matches: &clap::ArgMatches, config: bloxconfig::Config){
    let mut host_search = "";
    let mut view = "";
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
        get_host(host_search.to_string(), view.to_string(), config);
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
                    if array.len() == 0 {
                        println!("No Results Found");
                    } else {
                        let mut restoutput_array: Vec<restapi::RESTOutput> = Vec::new();
                        let delim = "=";
                        for item in array {
                            restoutput_array.push(
                                restapi::RESTOutput {
                                    line: format!("_ref{}{} hostname{}{} ipv4addr{}{} mac{}{}", delim, item._ref, delim,item.name, delim,item.ipv4addrs[0].ipv4addr, delim,item.ipv4addrs[0].mac)
                                }
                            )
                        }
                        for item in restoutput_array {
                            item.output();
                        }
                    }
                    },
                Err(err) => { println!("Error: {}", err)}
            }
        },
        Err(error) => { println!("Error: {}", error)}
    }
}