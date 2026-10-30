//! Public planning depends only on portable contracts. Runtime-owned expansion,
//! tracing, overlays and execution are delegated before their effects begin.
use adl_legacy_contracts::{adl, resolve};
use anyhow::{Context, Result};

#[path = "resume_args.rs"]
#[allow(dead_code)] // Runtime consumes the parsed values after local diagnostics.
mod resume_args;
#[path = "workflow_args.rs"]
#[allow(dead_code)] // The shared parser retains options forwarded to Runtime unchanged.
mod workflow_args;
#[path = "workflow_display.rs"]
mod workflow_display;

fn delegate(args: &[String], command: &str) -> Result<()> {
    let mut forwarded = vec![command.to_string()];
    forwarded.extend_from_slice(args);
    super::runtime_owner::SelectedRuntime::from_environment()?.exec(&forwarded)
}

pub(super) fn run(args: &[String]) -> Result<()> {
    let Some(parsed) = workflow_args::parse(args, super::usage()) else {
        return Ok(());
    };
    if parsed.steer_path.is_some() && parsed.resume_path.is_none() {
        anyhow::bail!("--steer requires --resume <run.json> so steering is only applied at a checkpoint boundary");
    }
    // Trace includes malformed-document diagnostics; let its owner load the input.
    if parsed.do_trace || parsed.overlay_path.is_some() {
        return delegate(args, "run");
    }
    let path = parsed
        .adl_path
        .to_str()
        .context("ADL path must be valid UTF-8")?;
    let doc = adl::AdlDoc::load_from_file(path)
        .with_context(|| format!("failed to load ADL document: {path}"))?;
    let explicit_presentation = parsed.print_plan || parsed.print_prompts;
    if parsed.do_run
        || (!explicit_presentation && doc.version.trim() != "0.1")
        || doc
            .providers
            .values()
            .any(|provider| provider.profile.is_some())
    {
        return delegate(args, "run");
    }
    let resolved = resolve::resolve_run(&doc)?;
    workflow_display::render(
        &resolved,
        parsed.print_plan || !explicit_presentation,
        parsed.print_prompts,
    )
}

pub(super) fn resume(args: &[String]) -> Result<()> {
    if resume_args::parse(args, super::resume_usage()).is_none() {
        return Ok(());
    }
    delegate(args, "resume")
}
