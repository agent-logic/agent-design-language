use source_retirement_validator::{audit, cleanup, Repo};
fn main() {
    let repo = Repo::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.."));
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let result = if args.is_empty() {
        audit::run(&repo).map(|v| println!("{v}"))
    } else if args == ["--cleanup-contracts"] {
        cleanup::run(&repo)
    } else {
        eprintln!("usage: source-retirement-validator [--cleanup-contracts]");
        std::process::exit(2)
    };
    if let Err(e) = result {
        println!(
            "{}",
            serde_json::json!({"status":"failed","error":format!("{e:#}")})
        );
        std::process::exit(1);
    }
}
