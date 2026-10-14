//! Portable workflow presentation; execution and provider admission stay with the caller.
use adl_legacy_contracts::{prompt, resolve};
use anyhow::Result;

pub(super) fn render(
    resolved: &resolve::AdlResolved,
    print_plan: bool,
    print_prompts: bool,
) -> Result<()> {
    if print_plan {
        resolve::print_resolved_plan(resolved);
    }
    if print_prompts {
        prompt::print_prompts(resolved)?;
    }
    Ok(())
}
