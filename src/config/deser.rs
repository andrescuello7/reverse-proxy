use std::fs;
use std::process::exit;
use crate::config::ProxyEnvirements;

pub fn to_toml() -> ProxyEnvirements {
    let filename: &'static str = "proxy.toml";
    let contents: String = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let proxy_envirements: ProxyEnvirements = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    return proxy_envirements;
}