use std::{
    fs,
    path::{Component, Path},
    process::ExitCode,
};

use adl_runtime_kernel::{run_first_birthday_demo, BirthdayDemoCase, EvidenceKind};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("adl-runtime-birthday-demo: {error}");
            ExitCode::from(65)
        }
    }
}

async fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut case = BirthdayDemoCase::Positive;
    let mut output = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--case" => case = parse_case(&args.next().ok_or("--case requires a value")?)?,
            "--output" => output = Some(args.next().ok_or("--output requires a value")?),
            _ => return Err(format!("unsupported argument {arg}")),
        }
    }
    let packet = run_first_birthday_demo(case)
        .await
        .map_err(|e| e.to_string())?;
    let bytes = serde_jcs::to_vec(&packet).map_err(|e| e.to_string())?;
    if let Some(output) = output {
        let relative = Path::new(&output);
        if relative.is_absolute()
            || relative
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err("output must be a normalized relative path".to_owned());
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .ok_or("repository root unavailable")?;
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, bytes).map_err(|e| e.to_string())?;
    } else {
        println!("{}", String::from_utf8(bytes).map_err(|e| e.to_string())?);
    }
    Ok(())
}

fn parse_case(value: &str) -> Result<BirthdayDemoCase, String> {
    Ok(match value {
        "positive" => BirthdayDemoCase::Positive,
        "startup" => BirthdayDemoCase::Startup,
        "wake" => BirthdayDemoCase::Wake,
        "restore" => BirthdayDemoCase::Restore,
        "snapshot" => BirthdayDemoCase::Snapshot,
        "admission" => BirthdayDemoCase::Admission,
        "copied_state" => BirthdayDemoCase::CopiedState,
        "simulation" => BirthdayDemoCase::Simulation,
        "named_fixture" => BirthdayDemoCase::NamedFixture,
        "interrupted" => BirthdayDemoCase::Interrupted,
        "missing_identity_root" => BirthdayDemoCase::MissingEvidence(EvidenceKind::IdentityRoot),
        "missing_continuity_head" => {
            BirthdayDemoCase::MissingEvidence(EvidenceKind::ContinuityHead)
        }
        "missing_memory_grounding" => {
            BirthdayDemoCase::MissingEvidence(EvidenceKind::MemoryGrounding)
        }
        "missing_capability_envelope" => {
            BirthdayDemoCase::MissingEvidence(EvidenceKind::CapabilityEnvelope)
        }
        "missing_cognitive_profile" => {
            BirthdayDemoCase::MissingEvidence(EvidenceKind::CognitiveProfile)
        }
        "missing_witness_set" => BirthdayDemoCase::MissingEvidence(EvidenceKind::WitnessSet),
        "missing_receipt" => BirthdayDemoCase::MissingEvidence(EvidenceKind::Receipt),
        "missing_reviewer_validation" => {
            BirthdayDemoCase::MissingEvidence(EvidenceKind::ReviewerValidation)
        }
        _ => return Err(format!("unsupported case {value}")),
    })
}
