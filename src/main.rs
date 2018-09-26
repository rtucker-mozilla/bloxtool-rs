extern crate dirs;
mod bloxconfig;
extern crate ini;

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
fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = bloxconfig::get_config(home_path);
    println!("updated: {}", config.host);
    //let file_path = get_ini_path(home_path);
    //println!("file_path={}", file_path);
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
}