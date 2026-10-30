#[allow(dead_code)]
#[path = "../../../adl/src/cli/resume_args.rs"]
mod resume_args;
#[allow(dead_code)]
#[path = "../../../adl/src/cli/usage.rs"]
mod usage;
#[path = "../../../adl/src/cli/workflow_args.rs"]
mod workflow_args;
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("workflow") => {
            if let Some(v) = workflow_args::parse(&args[1..], "legacy usage sentinel") {
                println!("{v:?}");
            }
        }
        Some("resume") => {
            if resume_args::parse(&args[1..], usage::resume_usage()).is_some() {
                std::process::exit(3);
            }
        }
        _ => std::process::exit(2),
    }
}
