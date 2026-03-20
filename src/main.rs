mod config;
mod profile;

use clap::Parser;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser)]
#[command(name = "cchange", about = "Switch Claude Code profiles")]
#[command(after_help = "All arguments after the profile name are passed through to claude.\n\
    Example: cchange company -r \"fix the bug\" --allowedTools Edit,Bash")]
struct Cli {
    /// Profile name, or a subcommand: "list", "init"
    profile: Option<String>,

    /// Arguments to pass through to claude
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}…{}", &key[..7], &key[key.len() - 4..])
}

fn list_profiles(cfg: &profile::Config) {
    let mut names: Vec<_> = cfg.profiles.keys().collect();
    names.sort();
    let default = cfg.default.as_deref().unwrap_or("");

    for name in names {
        let p = &cfg.profiles[name];
        let marker = if name == default { " (default)" } else { "" };

        let key_display = match &p.api_key {
            Some(k) => mask_key(k),
            None => "subscription".to_string(),
        };

        let url_display = match &p.api_url {
            Some(u) => format!(" → {u}"),
            None => String::new(),
        };

        println!("  {name}{marker}: {key_display}{url_display}");
    }
}

fn main() {
    let cli = Cli::parse();

    let profile_name = match cli.profile.as_deref() {
        Some("init") => {
            config::init_config();
            return;
        }
        Some("list") => {
            let cfg = config::load_config();
            list_profiles(&cfg);
            return;
        }
        Some(name) => name.to_string(),
        None => {
            let cfg = config::load_config();
            match &cfg.default {
                Some(d) => {
                    // Re-parse below with loaded config
                    let _ = cfg;
                    d.clone()
                }
                None => {
                    eprintln!("error: no profile specified and no default set");
                    eprintln!("usage: cchange <profile> [claude args...]");
                    eprintln!("       cchange list");
                    eprintln!("       cchange init");
                    std::process::exit(1);
                }
            }
        }
    };

    let cfg = config::load_config();

    let profile = match cfg.profiles.get(&profile_name) {
        Some(p) => p,
        None => {
            eprintln!("error: unknown profile \"{profile_name}\"");
            eprintln!("available profiles:");
            list_profiles(&cfg);
            std::process::exit(1);
        }
    };

    let mut cmd = Command::new("claude");

    match &profile.api_key {
        Some(key) => {
            cmd.env("ANTHROPIC_API_KEY", key);
        }
        None => {
            cmd.env_remove("ANTHROPIC_API_KEY");
        }
    }

    match &profile.api_url {
        Some(url) => {
            cmd.env("ANTHROPIC_BASE_URL", url);
        }
        None => {
            cmd.env_remove("ANTHROPIC_BASE_URL");
        }
    }

    cmd.args(&cli.args);

    let err = cmd.exec();
    eprintln!("error: failed to exec claude: {err}");
    std::process::exit(1);
}
