use clap;
use bloxconfig;

pub fn execute(host_matches: &clap::ArgMatches, config: bloxconfig::Config){
    if let Some(_get_matches) = host_matches.subcommand_matches("get") {
        match _get_matches.value_of("hostname"){
            Some(value) => { println!("WOOT HAVE GET HOST: {}", value) },
            None => println!("Host Required")
        }
        match _get_matches.value_of("view"){
            Some(value) => { println!("WOOT HAVE GET view: {}", value) },
            None => println!("View Required")
        }
    }
    println!("{}", config.host);

}