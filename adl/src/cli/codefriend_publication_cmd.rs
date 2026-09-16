use adl::codefriend::publication::{
    admit_local, read_publication, read_review, verify_artifacts, write_json_create_only,
    DecisionKind, DecisionRecord, ManifestInput,
};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};

pub(super) fn run(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "missing_publication_command");
    match args[0].as_str() {
        "prepare" => prepare(&args[1..]),
        "approve" => decide(&args[1..], DecisionKind::Approved),
        "withhold" => decide(&args[1..], DecisionKind::Withheld),
        "invalidate" => invalidate(&args[1..]),
        "inspect" => inspect(&args[1..]),
        "admit-local" => admit(&args[1..]),
        _ => anyhow::bail!("unsupported_publication_command"),
    }
}

fn prepare(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &["--review-record", "--manifest", "--artifact-root", "--out"],
    )?;
    let review = read_review(Path::new(flags["--review-record"]))?;
    let input = ManifestInput::read(Path::new(flags["--manifest"]))?;
    verify_artifacts(
        Path::new(flags["--artifact-root"]),
        &input.artifact_manifest,
    )?;
    let publication = input.publication(&review)?;
    write_json_create_only(Path::new(flags["--out"]), &publication)?;
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "schema": "codefriend.publication_prepare_result.v1",
            "binding_digest": publication.binding_digest()?,
            "state": publication.state,
            "target": publication.target,
            "artifact_count": publication.artifact_manifest.len()
        }))?
    );
    Ok(())
}

fn decide(args: &[String], kind: DecisionKind) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--publication",
            "--actor",
            "--reason",
            "--out",
        ],
    )?;
    let review = read_review(Path::new(flags["--review-record"]))?;
    let publication = read_publication(Path::new(flags["--publication"]))?;
    let decision = DecisionRecord::new(
        &review,
        &publication,
        kind,
        flags["--actor"],
        flags["--reason"],
        unix_time(),
        None,
    )?;
    write_json_create_only(Path::new(flags["--out"]), &decision)?;
    println!("{}", serde_json::to_string(&decision)?);
    Ok(())
}

fn invalidate(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--decision",
            "--actor",
            "--reason",
            "--out",
        ],
    )?;
    let review = read_review(Path::new(flags["--review-record"]))?;
    let previous = DecisionRecord::read(Path::new(flags["--decision"]), &review)?;
    let decision = DecisionRecord::new(
        &review,
        &previous.publication,
        DecisionKind::Invalidated,
        flags["--actor"],
        flags["--reason"],
        unix_time(),
        Some(&previous),
    )?;
    write_json_create_only(Path::new(flags["--out"]), &decision)?;
    println!("{}", serde_json::to_string(&decision)?);
    Ok(())
}

fn inspect(args: &[String]) -> Result<()> {
    let flags = exact_flags(args, &["--review-record", "--decision"])?;
    let review = read_review(Path::new(flags["--review-record"]))?;
    let decision = DecisionRecord::read(Path::new(flags["--decision"]), &review)?;
    println!("{}", serde_json::to_string(&decision)?);
    Ok(())
}

fn admit(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--decision",
            "--artifact-root",
            "--destination-root",
        ],
    )?;
    let review = read_review(Path::new(flags["--review-record"]))?;
    let decision = DecisionRecord::read(Path::new(flags["--decision"]), &review)?;
    let receipt = admit_local(
        &review,
        &decision,
        Path::new(flags["--artifact-root"]),
        Path::new(flags["--destination-root"]),
        unix_time(),
    )?;
    println!("{}", serde_json::to_string(&receipt)?);
    Ok(())
}

fn exact_flags<'a>(args: &'a [String], expected: &[&str]) -> Result<BTreeMap<&'a str, &'a str>> {
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_publication_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_publication_argument"
        );
    }
    ensure!(
        flags.len() == expected.len() && expected.iter().all(|flag| flags.contains_key(flag)),
        "missing_publication_argument"
    );
    Ok(flags)
}

fn unix_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
