use adl_remote_validation::{
    adapter_plan, read_request, read_result, resolve_repository_root, run_local, validate_request,
    validate_result, AdapterKind,
};
use std::env;
use std::path::{Path, PathBuf};

fn usage() -> ! {
    eprintln!("usage: adl-remote-validation validate-request <request.json> | validate-result <request.json> <result.json> | adapter-plan <local|nessus|aws> <request.json> | run-local <request.json> [--repo <path>]");
    std::process::exit(2);
}

fn adapter(value: &str) -> Result<AdapterKind, String> {
    match value {
        "local" => Ok(AdapterKind::Local),
        "nessus" => Ok(AdapterKind::Nessus),
        "aws" => Ok(AdapterKind::Aws),
        _ => Err(format!("unsupported adapter: {value}")),
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command, request] if command == "validate-request" => {
            let request = read_request(Path::new(request))?;
            validate_request(&request)?;
            println!(
                "{{\"schema\":\"adl.remote_validation.validation.v1\",\"status\":\"passed\"}}"
            );
        }
        [command, request, result] if command == "validate-result" => {
            let request = read_request(Path::new(request))?;
            let result = read_result(Path::new(result))?;
            validate_result(&request, &result)?;
            println!(
                "{{\"schema\":\"adl.remote_validation.validation.v1\",\"status\":\"passed\"}}"
            );
        }
        [command, adapter_name, request] if command == "adapter-plan" => {
            let request = read_request(Path::new(request))?;
            let plan = adapter_plan(&request, adapter(adapter_name)?)?;
            println!(
                "{}",
                serde_json::to_string(&plan).map_err(|error| error.to_string())?
            );
        }
        [command, request] if command == "run-local" => {
            let request = read_request(Path::new(request))?;
            let root = resolve_repository_root(Path::new("."))?;
            let result = run_local(&request, &root)?;
            println!(
                "{}",
                serde_json::to_string(&result).map_err(|error| error.to_string())?
            );
        }
        [command, request, flag, root] if command == "run-local" && flag == "--repo" => {
            let request = read_request(Path::new(request))?;
            let result = run_local(&request, &PathBuf::from(root))?;
            println!(
                "{}",
                serde_json::to_string(&result).map_err(|error| error.to_string())?
            );
        }
        _ => usage(),
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!(
            "adl_event component=remote_validation status=failed error={}",
            error.replace(['\n', '\r'], " ")
        );
        std::process::exit(1);
    }
}
