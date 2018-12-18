extern crate dirs;
extern crate ini;
extern crate reqwest;
extern crate serde;
extern crate mockito;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

mod bloxconfig;
mod host_execute;
mod cname_execute;
mod a_execute;
mod txt_execute;
mod mx_execute;
mod search_execute;
mod restapi;
mod domain_authoratative_execute;
mod domain_delegated_execute;
mod network_execute;
use clap::App;

fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = bloxconfig::get_config(home_path);
    let _yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(_yaml).get_matches();

    match matches.subcommand_matches("record:host") {
        Some(value) => { host_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("record:cname") {
        Some(value) => { cname_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("record:a") {
        Some(value) => { a_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("record:txt") {
        Some(value) => { txt_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("record:mx") {
        Some(value) => { mx_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("search") {
        Some(value) => { search_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("domain:authoratative") {
        Some(value) => { domain_authoratative_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("domain:delegated") {
        Some(value) => { domain_delegated_execute::execute(value, config.clone()) },
        None => {}
    }

    match matches.subcommand_matches("network") {
        Some(value) => { network_execute::execute(value, config.clone()) },
        None => {}
    }
}