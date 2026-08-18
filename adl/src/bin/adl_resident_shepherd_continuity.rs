use std::{env, fs, path::PathBuf, time::Duration};

use adl::resident_shepherd_spot_continuity::{
    dehydrate, preflight, restore_and_admit, validate_completed_continuation,
    validate_habitability_receipt, ContinuationInput, DehydrationInput, RestoreInput,
};
use anyhow::{bail, Context, Result};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("resident continuity failed: {error:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .context("expected dehydrate, restore, or complete")?;
    let mut input: Option<PathBuf> = None;
    let mut runtime_root: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--runtime-root" => runtime_root = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            _ => bail!("unknown argument {arg}"),
        }
    }
    let input = input.context("--input is required")?;
    let runtime_root = runtime_root.context("--runtime-root is required")?;
    let output = output.context("--output is required")?;
    let bytes = match command.as_str() {
        "preflight" => {
            let value: DehydrationInput = serde_json::from_slice(&fs::read(input)?)?;
            serde_json::to_vec_pretty(&preflight(&value)?)?
        }
        "dehydrate" => {
            let value: DehydrationInput = serde_json::from_slice(&fs::read(input)?)?;
            if value.retained_runtime_root != runtime_root {
                bail!("input retained_runtime_root must equal --runtime-root");
            }
            let deadline = if let Some(notice) = &value.spot_notice {
                let deadline = chrono::DateTime::parse_from_rfc3339(&notice.deadline_utc)
                    .context("Spot notice deadline must be RFC3339")?
                    .with_timezone(&chrono::Utc);
                (deadline - chrono::Utc::now())
                    .to_std()
                    .context("Spot notice deadline has already expired")?
            } else {
                Duration::from_secs(30)
            };
            serde_json::to_vec_pretty(&dehydrate(&value, deadline).await?)?
        }
        "restore" => {
            let value: RestoreInput = serde_json::from_slice(&fs::read(input)?)?;
            serde_json::to_vec_pretty(&restore_and_admit(&runtime_root, &value).await?)?
        }
        "complete" => {
            let value: ContinuationInput = serde_json::from_slice(&fs::read(input)?)?;
            serde_json::to_vec_pretty(
                &validate_completed_continuation(&runtime_root, &value).await?,
            )?
        }
        "validate-receipt" => serde_json::to_vec_pretty(&validate_habitability_receipt(&input)?)?,
        _ => bail!("expected dehydrate, restore, complete, or validate-receipt"),
    };
    fs::write(output, bytes)?;
    Ok(())
}
