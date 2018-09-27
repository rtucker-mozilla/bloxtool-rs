extern crate dirs;
extern crate ini;
mod bloxconfig;
mod host_execute;
use clap::App;

#[macro_use]
extern crate clap;

//use std::path::PathBuf;
//use ini::Ini;

/*
    match dirs::home_dir() {
        Some(path) => {
            let config_path = format!("{}/{}", home_path.to_string(), ".bloxtool.cfg");
            
            return Some(config_path); //format!("{}/.bloxtool.cfg", path.as_path().to_str())
        },
        None => return None
    }
*/
//#[cfg(feature = "yaml")]
fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = bloxconfig::get_config(home_path);
    let _yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(_yaml).get_matches();
    match matches.subcommand_matches("host") {
        Some(value) => { host_execute::execute(value, config) },
        None => {}
    }

    //println!("updated: {}", matches);
    //let file_path = get_ini_path(home_path);
    //println!("file_path={}", file_path);
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
}