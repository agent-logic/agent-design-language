use anyhow::{bail, Result};
use std::path::PathBuf;
fn main() {
    if let Err(e) = run() {
        if std::env::args().nth(1).as_deref() == Some("compatibility") {
            eprintln!("adl_event component=compatibility_lockset outcome=refused reason={e}");
            std::process::exit(2);
        }
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let action = a.first().map(String::as_str).unwrap_or("");
    if action == "adapter-evidence" && a.len() == 1 {
        let directory =
            adl_public_contracts::root().join("docs/repository-split/adapter-admission-1275");
        adl_public_contracts::adapter_evidence::verify(
            &adl_public_contracts::adapter_evidence::load(&directory)?,
        )?;
        println!("adapter evidence consistency: PASS (not lifecycle acceptance)");
        return Ok(());
    }
    if a.len() == 1 && matches!(action, "runtime-owner" | "codefriend-owner") {
        let probe = std::env::current_exe()?.with_file_name("adl-owner-probe");
        let result = if action == "runtime-owner" {
            adl_public_contracts::runtime_owner::run(&probe)?
        } else {
            adl_public_contracts::codefriend::run(&probe, false)?
        };
        println!("{result}");
        return Ok(());
    }
    if matches!(action, "metadata-check" | "metadata-binary") && a.len() == 3 && a[1] == "--input" {
        let metadata: serde_json::Value = serde_json::from_slice(&std::fs::read(&a[2])?)?;
        if action == "metadata-binary" {
            let target = metadata["target_directory"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("target_directory missing"))?;
            println!(
                "{}",
                std::path::Path::new(target).join("debug/adl").display()
            );
        } else {
            let packages = metadata["packages"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("packages missing"))?;
            let local: std::collections::BTreeSet<_> = packages
                .iter()
                .filter(|p| p["source"].is_null())
                .filter_map(|p| p["name"].as_str())
                .collect();
            if local
                != std::collections::BTreeSet::from(["adl", "adl-schema", "adl-legacy-contracts"])
            {
                bail!("public CLI local dependency closure changed: {local:?}");
            }
            println!("public CLI private dependency count: 0");
        }
        return Ok(());
    }
    if action == "uts-install" && a.len() == 1 {
        println!("{}", adl_public_contracts::uts_install::run()?);
        return Ok(());
    }
    if action == "docs" && (a.len() == 1 || (a.len() == 2 && a[1] == "--self-test")) {
        if a.len() == 2 {
            adl_public_contracts::public_docs::self_test()?;
        }
        let errors = adl_public_contracts::public_docs::check(&adl_public_contracts::root())?;
        if !errors.is_empty() {
            bail!("{}", errors.join("\n"));
        }
        println!(
            "PASS public docs contract: links, anchors, command delegation and version sources"
        );
        return Ok(());
    }
    if action == "remote-consumer" && a.len() == 3 && a[1] == "--output" {
        let report = adl_public_contracts::distribution::consumer(std::path::Path::new(&a[2]))?;
        println!("{report}");
        return Ok(());
    }
    if action == "install-validator" && a.len() == 3 && a[1] == "--issue" {
        let result = adl_public_contracts::install_validator::run(a[2].parse()?)?;
        println!("{result}");
        std::process::exit(if result["status"] == "passed" { 0 } else { 2 });
    }
    if action == "network-probe" && a.len() == 1 {
        std::net::TcpStream::connect_timeout(
            &"127.0.0.1:9".parse()?,
            std::time::Duration::from_secs(2),
        )?;
        return Ok(());
    }
    if action == "install-proof" && a.len() == 3 && a[1] == "--output" {
        let report = adl_public_contracts::install::run(std::path::Path::new(&a[2]))?;
        println!("{report}");
        return Ok(());
    }
    if action == "compatibility" {
        let qualified = a.iter().any(|v| v == "--require-qualified");
        let values: Vec<_> = a[1..]
            .iter()
            .filter(|v| v.as_str() != "--require-qualified")
            .cloned()
            .collect();
        let options = options(&values)?;
        allowed(
            &options,
            &["--lockset", "--graph", "--rollback", "--evidence-root"],
        )?;
        let load = |k: &str| -> Result<serde_json::Value> {
            adl_public_contracts::compatibility::load(std::path::Path::new(
                options
                    .get(k)
                    .ok_or_else(|| anyhow::anyhow!("missing {k}"))?,
            ))
        };
        let report = adl_public_contracts::compatibility::verify(
            &load("--lockset")?,
            &load("--graph")?,
            &load("--rollback")?,
            options.get("--evidence-root").map(std::path::Path::new),
            qualified,
        )?;
        println!("{report}");
        return Ok(());
    }
    if matches!(action, "godel" | "remote-distribution") {
        let d = adl_public_contracts::distribution::Distribution::load(action == "godel")?;
        let command = a.get(1).map(String::as_str).unwrap_or("");
        let options = options(&a[2.min(a.len())..])?;
        let required = |k: &str| -> Result<&str> {
            options
                .get(k)
                .map(String::as_str)
                .ok_or_else(|| anyhow::anyhow!("missing {k}"))
        };
        let result = match command {
            "verify" => {
                allowed(&options, &["--artifact", "--extract"])?;
                d.verify(
                    std::path::Path::new(required("--artifact")?),
                    options.get("--extract").map(std::path::Path::new),
                )?
            }
            "verify-root" => {
                allowed(&options, &["--root"])?;
                d.verify_root(std::path::Path::new(required("--root")?))?
            }
            "export" => {
                allowed(&options, &["--repository", "--output"])?;
                d.export(
                    std::path::Path::new(required("--repository")?),
                    std::path::Path::new(required("--output")?),
                )?
            }
            _ => bail!("unknown distribution action"),
        };
        println!("{result}");
        return Ok(());
    }
    if matches!(action, "transpiler-pack" | "transpiler-install") {
        let options = options(&a[1..])?;
        let required = |k: &str| -> Result<&str> {
            options
                .get(k)
                .map(String::as_str)
                .ok_or_else(|| anyhow::anyhow!("missing {k}"))
        };
        let result = if action == "transpiler-pack" {
            allowed(&options, &["--binary", "--output"])?;
            adl_public_contracts::transpiler::build(
                &adl_public_contracts::root(),
                std::path::Path::new(required("--binary")?),
                std::path::Path::new(required("--output")?),
            )?
        } else {
            allowed(&options, &["--archive", "--sha256", "--destination"])?;
            adl_public_contracts::transpiler::install(
                std::path::Path::new(required("--archive")?),
                required("--sha256")?,
                std::path::Path::new(required("--destination")?),
            )?
        };
        println!("{result}");
        return Ok(());
    }
    if a.len() != 3 || a[1] != "--binary" {
        bail!("usage: adl-public-contracts CONTRACT --binary PATH; distribution and transpiler actions accept explicit named paths");
    }
    let binary = std::fs::canonicalize(PathBuf::from(&a[2]))?;
    let result = match action {
        "portable-workflow" => adl_public_contracts::portable::workflow(&binary)?,
        "resume-args" => adl_public_contracts::portable::resume(
            &binary,
            &std::env::current_exe()?.with_file_name("adl-argument-probe"),
        )?,
        "transpiler-test" => adl_public_contracts::transpiler::regressions(&binary)?,
        "session" => adl_public_contracts::session::run(&binary)?,
        "codefriend-dispatch" => adl_public_contracts::codefriend::run(&binary, true)?,
        "codefriend-owner" => adl_public_contracts::codefriend::run(&binary, false)?,
        "runtime-owner" => adl_public_contracts::runtime_owner::run(&binary)?,
        "runtime-dispatch" => adl_public_contracts::dispatch::runtime(&binary)?,
        "cli-refusals" => adl_public_contracts::dispatch::refusals(&binary)?,
        _ => bail!("unknown contract"),
    };
    println!("{result}");
    Ok(())
}
fn options(args: &[String]) -> Result<std::collections::BTreeMap<String, String>> {
    if !args.len().is_multiple_of(2) {
        bail!("option requires a value");
    }
    let mut result = std::collections::BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if !pair[0].starts_with("--") || result.insert(pair[0].clone(), pair[1].clone()).is_some() {
            bail!("invalid or duplicate option");
        }
    }
    Ok(result)
}
fn allowed(options: &std::collections::BTreeMap<String, String>, names: &[&str]) -> Result<()> {
    if options.keys().any(|k| !names.contains(&k.as_str())) {
        bail!("unknown option");
    }
    Ok(())
}
