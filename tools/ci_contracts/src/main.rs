use std::path::PathBuf;
fn main() {
    let mut args = std::env::args().skip(1);
    let action = args.next().unwrap_or_default();
    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    if args.next().is_some() {
        eprintln!("unexpected argument");
        std::process::exit(2);
    }
    let result = match action.as_str() {
        "workflow-policy" => adl_ci_contracts::workflow_policy(&root).map(|v| {
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
            v["status"] == "pass"
        }),
        "aggregation-contract" => adl_ci_contracts::aggregation_contract(&root).map(|()| {
            println!("PASS UTS public producer gate: six outcomes and declaration scope");
            true
        }),
        _ => Err(
            "usage: adl-ci-contracts <workflow-policy|aggregation-contract> [repository-root]"
                .into(),
        ),
    };
    match result {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2)
        }
    }
}
