extern crate ini;
use std::path::PathBuf;

pub fn get_ini_path(path: PathBuf) -> String {
    let _file_name = ".bloxtool.cfg";
    let path_string = path.as_path().to_str().unwrap();
    return format!("{}/{}",path_string.to_string(), _file_name); //format!("{}/.bloxtool.cfg", path.as_path().to_str())
}

#[test]
fn test_get_ini_path() {
    let mut path = PathBuf::new();
    path.push("/foo");
    assert_eq!(get_ini_path(path), "/foo/.bloxtool.cfg");
}