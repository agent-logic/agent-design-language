//! Public ADL facade: portable presentation and repository-local coordination,
//! with product execution delegated to explicitly qualified owners.
use anyhow::Result;

#[path = "codefriend_owner.rs"]
mod codefriend_owner;
#[path = "observability.rs"]
#[allow(dead_code)] // Preserve the shared logging contract while product events move to owners.
mod observability;
#[path = "process_cmd.rs"]
#[allow(dead_code)]
mod process_cmd;
#[path = "public_workflow.rs"]
mod public_workflow;
#[path = "runtime_owner.rs"]
mod runtime_owner;
#[path = "session_cmd.rs"]
mod session_cmd;
#[path = "usage.rs"]
mod usage;

fn usage() -> &'static str {
    usage::usage()
}
fn resume_usage() -> &'static str {
    usage::resume_usage()
}

pub fn run_main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(error) = dispatch(&args) {
        eprintln!("Error: {error}");
        for (index, cause) in error.chain().skip(1).enumerate() {
            eprintln!("  {index}: {cause}");
        }
        std::process::exit(1);
    }
}

fn dispatch(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => {
            println!("{}", usage());
            return Ok(());
        }
        Some("--version" | "-V") => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => {}
    }
    observability::emit_event(
        "adl",
        "dispatch",
        "started",
        &[(
            "subcommand",
            args.first().map(String::as_str).unwrap_or("workflow"),
        )],
    );
    match args.first().map(String::as_str) {
        Some("codefriend") => {
            codefriend_owner::SelectedCodeFriend::from_environment()?.exec(&args[1..])
        }
        Some("process") => process_cmd::real_process(&args[1..]),
        Some("session") => session_cmd::real_session(&args[1..]),
        Some("resume") => public_workflow::resume(&args[1..]),
        Some(
            "artifact" | "agent" | "csm" | "demo" | "godel" | "identity" | "provider"
            | "runtime-v2" | "runtime-v3" | "scheduler" | "keygen" | "sign" | "instrument"
            | "learn" | "verify",
        ) => runtime_owner::SelectedRuntime::from_environment()?.exec(args),
        Some("pr") => {
            anyhow::bail!("the v1 `adl pr` control plane was removed; use native C-SDLC v3 csdlc")
        }
        Some("tooling") => {
            anyhow::bail!("the v1 tooling multiplexer was removed; use native C-SDLC v3 csdlc")
        }
        _ => public_workflow::run(args),
    }
}
