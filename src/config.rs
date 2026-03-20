use crate::profile::Config;
use std::path::PathBuf;

const SAMPLE_CONFIG: &str = r#"default = "pro"

[profiles.pro]
# No api_key = subscription auth

[profiles.company]
api_key = "sk-ant-api03-YOUR_KEY"
api_url = "https://api-proxy.company.com/v1"

[profiles.personal]
api_key = "sk-ant-api03-YOUR_KEY"
"#;

pub fn config_path() -> PathBuf {
    if let Ok(p) = std::env::var("CCHANGE_CONFIG") {
        return PathBuf::from(p);
    }
    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".config")
        .join("cchange")
        .join("config.toml")
}

pub fn init_config() {
    let path = config_path();
    if path.exists() {
        eprintln!("config already exists at {}", path.display());
        eprintln!("edit it directly to make changes");
        std::process::exit(1);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            eprintln!("error: could not create {}: {e}", parent.display());
            std::process::exit(1);
        });
    }
    std::fs::write(&path, SAMPLE_CONFIG).unwrap_or_else(|e| {
        eprintln!("error: could not write {}: {e}", path.display());
        std::process::exit(1);
    });
    println!("created {}", path.display());
    println!("edit it to add your API keys and profiles");
}

pub fn load_config() -> Config {
    let path = config_path();
    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("error: config file not found at {}", path.display());
            eprintln!();
            eprintln!("run `cchange init` to create it, or create it manually:");
            eprintln!();
            eprint!("{SAMPLE_CONFIG}");
            std::process::exit(1);
        }
    };

    match toml::from_str(&contents) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("error: failed to parse {}: {e}", path.display());
            std::process::exit(1);
        }
    }
}
