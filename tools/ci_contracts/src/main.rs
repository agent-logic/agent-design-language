use std::path::PathBuf;
fn main() {
    let mut args = std::env::args().skip(1);
    let action = args.next().unwrap_or_default();
    if action == "ci" {
        if let Err(e) = adl_ci_contracts::ci::cli(&args.collect::<Vec<_>>()) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
    if action == "coverage" || action == "coverage-report" {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let result = if action == "coverage" {
            adl_ci_contracts::coverage::cli(&root, &args.collect::<Vec<_>>()).map(|()| true)
        } else {
            let env = |k| {
                std::env::var(k)
                    .map(PathBuf::from)
                    .map_err(|e| e.to_string())
            };
            (|| {
                adl_ci_contracts::coverage::report(
                    &env("COVERAGE_OUTPUT")?,
                    &env("RUNNER_TEMP")?,
                    &env("GITHUB_STEP_SUMMARY")?,
                    &std::env::var("COVERAGE_OUTCOME").unwrap_or_default(),
                )
            })()
        };
        match result {
            Ok(true) => {}
            Ok(false) => std::process::exit(1),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        return;
    }
    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    if args.next().is_some() {
        eprintln!("unexpected argument");
        std::process::exit(2);
    }
    let result = match action.as_str() {
        "action-pins" => adl_ci_contracts::actions::check(&root).map(|()| {
            println!("PASS canonical action pins across every workflow");
            true
        }),
        "planning-contract" => adl_ci_contracts::planning::run(&root).map(|v| {
            println!("{v}");
            true
        }),
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
