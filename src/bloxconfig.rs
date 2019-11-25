use ini::Ini;
use std::path::PathBuf;
use std::process;
static VERSION: &'static str = "2.6";

#[derive(Clone)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub allow_insecure_ssl: bool,
    pub host: String,
}

impl Config {
    pub fn full_path(& self) -> String{
        let mut hostname = self.host.clone();
        if hostname.ends_with("/") == false {
            hostname = format!("{}/", hostname);
        }
        return format!("{}wapi/v{}", hostname, VERSION);
    }
}

pub fn get_config(path: PathBuf) -> Config {
    let path_string = path.as_path().to_str().unwrap();
    let _file_name = ".bloxtool.cfg";
    let full_file_path = format!("{}/{}",path_string.to_string(), _file_name);
    let conf = match Ini::load_from_file(full_file_path) {
        Ok(value) => value,
        Err(_error) => {
            println!("Error: unable to read config file.");
            process::exit(2);
        }
    };
    let main_config = match conf.section(Some("InfoBlox")) {
        Some(value) => {value},
        None => { 
            println!("Error: unable to read config section.");
            process::exit(2);
        }
    };

    let l_username = match main_config.get("username") {
        Some(value) => { value.to_string() },
        None => { 
            println!("Error: username required in .bloxtool.cfg");
            process::exit(2);
        }
    };

    let l_allow_insecure_ssl = match main_config.get("allow_insecure_ssl") {
        Some(value) => { 
            if value.to_string().to_uppercase() == "TRUE" {
                true
            } else {
                false
            }
        },
        None => { 
            false
        }
    };

    let l_password = match main_config.get("password") {
        Some(value) => { value.to_string() },
        None => { 
            println!("Error: password required in .bloxtool.cfg");
            process::exit(2);
        },
    };

    let l_host = match main_config.get("host") {
        Some(value) => { value.to_string() },
        None => { 
            println!("Error: host required in .bloxtool.cfg");
            process::exit(2);
        },
    };

    let config = Config{
        username: l_username,
        password: l_password,
        allow_insecure_ssl: l_allow_insecure_ssl,
        host: l_host
    };
    return config;
}

#[test]
fn test_full_path() {
    let config = Config{
        username: "username".to_string(),
        password: "password".to_string(),
        allow_insecure_ssl: false,
        host: "https://localhost/".to_string(),
    };
    assert_eq!(config.full_path(), format!("https://localhost/wapi/v{}", VERSION));
}
#[test]
fn test_full_path_hostname_not_having_trailing_slash() {
    let config = Config{
        username: "username".to_string(),
        password: "password".to_string(),
        allow_insecure_ssl: false,
        host: "https://localhost".to_string(),
    };
    assert_eq!(config.full_path(), format!("https://localhost/wapi/v{}", VERSION));
}
/*
#[test]
fn test_Config_struct_empty() {
    let foo = Config{};;
    assert_eq!(foo.username, "");
}
*/