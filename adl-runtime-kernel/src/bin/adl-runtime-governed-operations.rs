#[path = "../governed_operations.rs"]
mod governed_operations;

use std::process::ExitCode;

use governed_operations::{execute, GovernedCommand, RuntimeConfig};

#[tokio::main]
async fn main() -> ExitCode {
    let config = match RuntimeConfig::from_env() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(78);
        }
    };
    let command = match read_command() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    let outcome = execute(config, command).await;
    match serde_json::to_string(&outcome) {
        Ok(json) => println!("{json}"),
        Err(error) => {
            eprintln!("outcome encoding failed: {error}");
            return ExitCode::from(70);
        }
    }
    if outcome.status == "completed" {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(77)
    }
}

fn read_command() -> Result<GovernedCommand, String> {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .take(1_048_577)
        .read_to_string(&mut input)
        .map_err(|_| "command_read_failed".to_owned())?;
    if input.is_empty() || input.len() > 1_048_576 {
        return Err("command_size_invalid".to_owned());
    }
    serde_json::from_str(&input).map_err(|_| "command_invalid".to_owned())
}
