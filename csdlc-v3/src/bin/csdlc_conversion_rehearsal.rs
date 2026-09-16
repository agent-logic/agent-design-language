use csdlc_v3::conversion::{
    convert, inspect_conversion_operation, observe, relocate_current_observation_copy,
    restore_conversion_pre_effect, run_writer_fence_guardian, ConversionRequest,
    CurrentObservationRelocationRequest,
};
use serde_json::json;
use std::env;
use std::fs;
use std::path::PathBuf;

fn argument(args: &[String], name: &str) -> Result<String, String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
        .ok_or_else(|| format!("missing {name}"))
}

fn run() -> Result<serde_json::Value, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("writer-fence-guardian") => {
            let path = PathBuf::from(argument(&args, "--request")?);
            run_writer_fence_guardian(&path)?;
            Ok(json!({"status":"released"}))
        }
        Some("convert") => {
            let path = PathBuf::from(argument(&args, "--request")?);
            let request: ConversionRequest = serde_json::from_slice(
                &fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?
            ).map_err(|error| error.to_string())?;
            let records = convert(&request)?;
            Ok(json!({"schema":"csdlc.v3.copied_record_conversion_result.v1","status":"completed","records":records}))
        }
        Some("relocate-current") => {
            let path = PathBuf::from(argument(&args, "--request")?);
            let request: CurrentObservationRelocationRequest = serde_json::from_slice(
                &fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?,
            )
            .map_err(|error| error.to_string())?;
            let result = relocate_current_observation_copy(&request)?;
            Ok(serde_json::to_value(result).map_err(|error| error.to_string())?)
        }
        Some("operation-evidence") | Some("restore-pre-effect") => {
            let path = PathBuf::from(argument(&args, "--request")?);
            let request: ConversionRequest = serde_json::from_slice(
                &fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?,
            )
            .map_err(|error| error.to_string())?;
            let result = if args[0] == "operation-evidence" {
                serde_json::to_value(inspect_conversion_operation(&request)?)
            } else {
                serde_json::to_value(restore_conversion_pre_effect(&request)?)
            };
            result.map_err(|error| error.to_string())
        }
        Some("status") | Some("validate") => {
            let git_common = PathBuf::from(argument(&args, "--git-common")?);
            let repository = argument(&args, "--repository")?;
            let issue = argument(&args, "--issue")?.parse::<u64>().map_err(|error| error.to_string())?;
            let mut result = observe(&git_common, &repository, issue)?;
            result["command"] = json!(args[0]);
            Ok(result)
        }
        _ => Err("usage: csdlc-conversion-rehearsal convert|relocate-current|operation-evidence|restore-pre-effect --request FILE | status|validate --git-common PATH --repository OWNER/REPO --issue N".to_owned()),
    }
}

fn main() {
    match run() {
        Ok(value) => {
            let restore_refused = value.get("schema").and_then(|item| item.as_str())
                == Some("csdlc.v3.copied_record_conversion_restore_result.v1")
                && value.get("allowed").and_then(|item| item.as_bool()) == Some(false);
            println!("{value}");
            if restore_refused {
                std::process::exit(2);
            }
        }
        Err(error) => {
            println!(
                "{}",
                json!({"schema":"csdlc.v3.copied_record_conversion_failure.v1","status":"failed","finding":error})
            );
            std::process::exit(2);
        }
    }
}
