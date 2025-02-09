use std::fs;
use wg_2024::config::Config;

/// Try to parse a file into a Config.
/// # Panics
/// - Unable to read the file
/// - Unable to parse to Config struct
pub fn load_from_file(path: &str) -> Config {
    let config_str = fs::read_to_string(path).expect(
        "ERROR[Network Initializer]: is unable to read config from '{path}' relative from the root of the project",
    );

    toml::from_str(&config_str).expect("ERROR[Network Initializer]: Unable to parse config.toml")
}
