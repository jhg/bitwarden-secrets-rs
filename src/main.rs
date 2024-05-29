use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::process::exit;
use rpassword::read_password;
use clap::{Parser, ValueEnum};
use zeroize::Zeroize;

mod helpers;
mod tools;
mod config;

use helpers::BwObject;
use tools::BitwardenCli;

#[derive(ValueEnum, Clone)]
#[clap(rename_all = "lowercase")]
enum OutputFormat {
    Bash,
    PowerShell,
    Json,
}

#[derive(Parser)]
struct Opts {
    #[clap(short, long)]
    #[cfg_attr(target_os = "windows", clap(default_value = "powershell"))]
    #[cfg_attr(not(target_os = "windows"), clap(default_value = "bash"))]
    format: OutputFormat,
}

fn main() -> io::Result<()> {
    let opts = Opts::parse();

    let config = config::Config::find()
        .expect("Failed to load .bw-secrets.json config file");

    if atty::is(atty::Stream::Stdout) {
        let current_exe = std::env::current_exe()
            .expect("Failed to get current executable path");
        let exe_name = current_exe.file_name()
            .expect("Failed to get current executable name")
            .to_string_lossy();

        #[cfg(target_os = "windows")]
        eprintln!("For security and platform limitations use it as:\niex ({exe_name} | Out-String)");
        #[cfg(not(target_os = "windows"))]
        eprintln!("For security and platform limitations use it as:\nsource <({exe_name})");

        eprintln!("Also check the help with --help to ensure you are using the correct format");
        exit(1);
    }

    write!(io::stderr(), "Enter Bitwarden password [will be hidden]: ")?;
    let mut password = read_password()?;
    let session_result = BitwardenCli::unlock(&password);
    password.zeroize();

    let Ok(session) = session_result
        else {
            if !BitwardenCli::is_installed() {
                eprintln!("Bitwarden CLI is not installed");
            } else {
                eprintln!("Failed to unlock Bitwarden");
            }
            exit(1);
        };

    let mut secrets = HashMap::new();
    for (id, config) in config {
        if !config.object.is_a_secret() {
            eprintln!("Skipping non-secret object: {}", config.object.as_str());
            eprintln!("Allowed objects are username, password, totp, uri and notes");
            continue;
        }

        let Ok(value) = session.get_object(config.object, &id) else {
            eprintln!("Failed to get {}: {}", config.name, id);
            continue;
        };
        secrets.insert(config.name, value);
    }

    match opts.format {
        OutputFormat::Bash => {
            for (name, value) in secrets {
                let value = value.replace("'", "\\'");
                println!("export {}='{}'", name, value);
            }
        },
        OutputFormat::PowerShell => {
            for (name, value) in secrets {
                let value = value.replace("\"", "\\\"");
                println!("$Env:{} = \"{}\"", name, value);
            }
        },
        OutputFormat::Json => {
            serde_json::to_writer(io::stdout(), &secrets)?;
        },
    }

    return Ok(());
}
