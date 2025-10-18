use std::fs;
use std::process::exit;
use crate::config::ProxyConfig;

// Read text of config un proxy.toml
// If have configuration for servers
pub fn read_config() -> String {
    let filename: &'static str = "rpx.toml";
    let content: String = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    return content;
}

// Parser and creation of models
pub fn parser_data(content: String) -> ProxyConfig {
    let proxy_config: ProxyConfig = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from");
            exit(1);
        }
    };
    return proxy_config;
}