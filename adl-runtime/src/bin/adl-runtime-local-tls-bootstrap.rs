use std::{path::PathBuf, process::ExitCode};

use adl_runtime::local_tls::{bootstrap_runtime_tls, RuntimeTlsBootstrapConfig};

#[tokio::main]
async fn main() -> ExitCode {
    let args = match Args::parse(std::env::args().skip(1).collect()) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    let text = match std::fs::read_to_string(&args.config) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed reading local TLS bootstrap config: {error}");
            return ExitCode::from(66);
        }
    };
    let config = if args
        .config
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("json")
    {
        RuntimeTlsBootstrapConfig::from_json_str(&text)
    } else {
        RuntimeTlsBootstrapConfig::from_toml_str(&text)
    };
    let config = match config {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    match bootstrap_runtime_tls(&config).await {
        Ok(outcome) => match serde_json::to_string_pretty(&outcome) {
            Ok(json) => {
                println!("{json}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("failed encoding local TLS bootstrap outcome: {error}");
                ExitCode::from(70)
            }
        },
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(75)
        }
    }
}

struct Args {
    config: PathBuf,
}

impl Args {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut config = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--config" => {
                    let value = iter
                        .next()
                        .ok_or_else(|| "--config requires a path".to_owned())?;
                    config = Some(PathBuf::from(value));
                }
                "--help" | "-h" => {
                    return Err(
                        "Usage: adl-runtime-local-tls-bootstrap --config <config.toml|config.json>"
                            .to_owned(),
                    );
                }
                _ => return Err(format!("unknown argument: {arg}")),
            }
        }
        Ok(Self {
            config: config.ok_or_else(|| "--config is required".to_owned())?,
        })
    }
}
