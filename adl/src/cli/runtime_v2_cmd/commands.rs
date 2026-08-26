use anyhow::{anyhow, Context, Result};
use serde_json::to_string_pretty;
use std::fs;
use std::path::{Path, PathBuf};

use super::helpers::{resolve_relative_input_path, resolve_relative_output_path};
use crate::cli::usage;
use ::adl::runtime_v2::{
    runtime_v2_cognitive_being_flagship_demo_contract,
    runtime_v2_constructability_anchor_validator_contract,
    runtime_v2_contract_market_demo_contract, runtime_v2_curiosity_engine_contract,
    runtime_v2_feature_proof_coverage_contract, runtime_v2_foundation_demo_contract,
    runtime_v2_godel_agent_runtime_contract_for, runtime_v2_loop_runtime_contract,
    runtime_v2_observatory_flagship_contract, runtime_v2_operator_control_report_contract,
    runtime_v2_reasoning_graph_contract, runtime_v2_security_boundary_proof_contract,
    RuntimeV2ConstructabilityAnchorValidatorPacket,
};

pub(crate) fn real_runtime_v2_operator_controls(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 operator-controls requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 operator-controls: {other}"
                ))
            }
        }
        i += 1;
    }

    let report = runtime_v2_operator_control_report_contract()?;
    let json = to_string_pretty(&report)?;
    let Some(out_path) = out_path else {
        println!("{json}");
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "operator-controls")?;
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "runtime-v2 operator-controls --out path must have a parent directory"
        ));
    };
    fs::create_dir_all::<&Path>(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write Runtime v2 operator control report to {}",
            resolved.display()
        )
    })?;
    println!(
        "RUNTIME_V2_OPERATOR_CONTROL_REPORT_PATH={}",
        resolved.display()
    );
    Ok(())
}

pub(crate) fn real_runtime_v2_security_boundary(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 security-boundary requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 security-boundary: {other}"
                ))
            }
        }
        i += 1;
    }

    let proof = runtime_v2_security_boundary_proof_contract()?;
    let json = to_string_pretty(&proof)?;
    let Some(out_path) = out_path else {
        println!("{json}");
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "security-boundary")?;
    let Some(parent) = resolved.parent() else {
        return Err(anyhow!(
            "runtime-v2 security-boundary --out path must have a parent directory"
        ));
    };
    fs::create_dir_all::<&Path>(parent)
        .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    fs::write(&resolved, json.as_bytes()).with_context(|| {
        format!(
            "failed to write Runtime v2 security boundary proof to {}",
            resolved.display()
        )
    })?;
    println!(
        "RUNTIME_V2_SECURITY_BOUNDARY_PROOF_PATH={}",
        resolved.display()
    );
    Ok(())
}

pub(crate) fn real_runtime_v2_foundation_demo(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v2 foundation-demo requires --out <dir>"));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 foundation-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_foundation_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "foundation-demo")?;
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 foundation demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    artifacts
        .proof_packet
        .validate_packaging_artifacts(&resolved)?;
    println!("RUNTIME_V2_FOUNDATION_DEMO_ROOT={}", resolved.display());
    Ok(())
}

pub(crate) fn real_runtime_v2_integrated_csm_run_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 integrated-csm-run-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--prototype-only" => {
                return Err(anyhow!(
                    "runtime-v2 integrated-csm-run-demo no longer supports prototype-only execution; use --out to produce the reconciled Runtime v2 plus current-runtime proof bundle"
                ));
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 integrated-csm-run-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    if let Some(out_path) = out_path {
        resolve_relative_output_path(repo_root, &out_path, "integrated-csm-run-demo")?;
    }
    Err(anyhow!(
        "runtime-v2 integrated-csm-run-demo production execution is unavailable: its historical governed adapter is test-only"
    ))
}

pub(crate) fn real_runtime_v2_minimal_integrated_runtime_path(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 minimal-integrated-runtime-path requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 minimal-integrated-runtime-path: {other}"
                ))
            }
        }
        i += 1;
    }

    if let Some(out_path) = out_path {
        resolve_relative_output_path(repo_root, &out_path, "minimal-integrated-runtime-path")?;
    }
    Err(anyhow!(
        "runtime-v2 minimal-integrated-runtime-path production execution is unavailable: its historical governed adapter is test-only"
    ))
}

pub(crate) fn real_runtime_v2_aee_obsmem_pvf_handoff(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 aee-obsmem-pvf-handoff requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 aee-obsmem-pvf-handoff: {other}"
                ))
            }
        }
        i += 1;
    }

    if let Some(out_path) = out_path {
        resolve_relative_output_path(repo_root, &out_path, "aee-obsmem-pvf-handoff")?;
    }
    Err(anyhow!(
        "runtime-v2 aee-obsmem-pvf-handoff production execution is unavailable: its historical governed adapter is test-only"
    ))
}

pub(crate) fn real_runtime_v2_unified_runtime_kernel(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 unified-runtime-kernel requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 unified-runtime-kernel: {other}"
                ))
            }
        }
        i += 1;
    }

    if let Some(out_path) = out_path {
        resolve_relative_output_path(repo_root, &out_path, "unified-runtime-kernel")?;
    }
    Err(anyhow!(
        "runtime-v2 unified-runtime-kernel production execution is unavailable: its historical governed adapter is test-only"
    ))
}

pub(crate) fn real_runtime_v2_observatory_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 observatory-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 observatory-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let resolved = match out_path.as_ref() {
        Some(out_path) => Some(resolve_relative_output_path(
            repo_root,
            out_path,
            "observatory-flagship-demo",
        )?),
        None => None,
    };

    let artifacts = runtime_v2_observatory_flagship_contract()?;
    let Some(resolved) = resolved else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 Observatory flagship demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!(
        "{}",
        observatory_flagship_demo_stdout_line(
            out_path
                .as_ref()
                .expect("resolved D12 output path should preserve requested --out")
        )
    );
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.operator_report_markdown);
    Ok(())
}

pub(crate) fn real_runtime_v2_cognitive_being_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 cognitive-being-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 cognitive-being-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_cognitive_being_flagship_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved =
        resolve_relative_output_path(repo_root, &out_path, "cognitive-being-flagship-demo")?;
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 cognitive-being flagship demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!("{}", cognitive_being_flagship_demo_stdout_line(&out_path));
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.reviewer_report_markdown);
    Ok(())
}

pub(crate) fn real_runtime_v2_feature_proof_coverage(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 feature-proof-coverage requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 feature-proof-coverage: {other}"
                ))
            }
        }
        i += 1;
    }

    let packet = runtime_v2_feature_proof_coverage_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "feature-proof-coverage")?;
    packet.write_to_path(&resolved)?;
    println!("{}", feature_proof_coverage_stdout_line(&out_path));
    Ok(())
}

pub(crate) fn real_runtime_v2_reasoning_graph(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v2 reasoning-graph requires --out <path>"));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 reasoning-graph: {other}"
                ))
            }
        }
        i += 1;
    }

    let packet = runtime_v2_reasoning_graph_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet.canonicalized()?)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "reasoning-graph")?;
    packet.write_to_path(&resolved)?;
    println!("{}", reasoning_graph_stdout_line(&out_path));
    Ok(())
}

pub(crate) fn real_runtime_v2_curiosity_engine(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v2 curiosity-engine requires --out <path>"));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 curiosity-engine: {other}"
                ))
            }
        }
        i += 1;
    }

    let packet = runtime_v2_curiosity_engine_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet.canonicalized()?)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "curiosity-engine")?;
    packet.write_to_path(&resolved)?;
    println!("{}", curiosity_engine_stdout_line(&out_path));
    Ok(())
}

pub(crate) fn real_runtime_v2_constructability_anchor_validator(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut input_path: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 constructability-anchor-validator requires --input <packet.json>"
                    ));
                };
                input_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 constructability-anchor-validator requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 constructability-anchor-validator: {other}"
                ))
            }
        }
        i += 1;
    }

    let resolved_out = out_path
        .as_ref()
        .map(|out_path| {
            resolve_relative_output_path(repo_root, out_path, "constructability-anchor-validator")
        })
        .transpose()?;
    let packet = if let Some(input_path) = input_path {
        let resolved = resolve_relative_input_path(
            repo_root,
            &input_path,
            "constructability-anchor-validator",
        )?;
        if resolved_out
            .as_ref()
            .is_some_and(|resolved_out| resolved_out == &resolved)
        {
            return Err(anyhow!(
                "runtime-v2 constructability-anchor-validator --input and --out must be different paths"
            ));
        }
        let bytes = fs::read(&resolved).with_context(|| {
            format!(
                "read Runtime v2 Constructability Anchor Validator input {}",
                input_path.display()
            )
        })?;
        if let Some(resolved_out) = &resolved_out {
            match fs::remove_file(resolved_out) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!(
                            "remove stale Runtime v2 Constructability Anchor Validator output {}",
                            out_path
                                .as_ref()
                                .expect("resolved output has requested path")
                                .display()
                        )
                    })
                }
            }
        }
        let packet: RuntimeV2ConstructabilityAnchorValidatorPacket = serde_json::from_slice(&bytes)
            .with_context(|| {
                format!(
                    "parse Runtime v2 Constructability Anchor Validator input {}",
                    input_path.display()
                )
            })?;
        packet.canonicalized()?
    } else {
        runtime_v2_constructability_anchor_validator_contract()?
    };
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet)?);
        return Ok(());
    };
    let resolved = resolved_out.expect("output path was resolved above");
    packet.write_to_path(&resolved)?;
    println!(
        "{}",
        constructability_anchor_validator_stdout_line(&out_path)
    );
    Ok(())
}

pub(crate) fn real_runtime_v2_loop_runtime(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v2 loop-runtime requires --out <path>"));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => return Err(anyhow!("unknown arg for runtime-v2 loop-runtime: {other}")),
        }
        i += 1;
    }

    let packet = runtime_v2_loop_runtime_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet.canonicalized()?)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "loop-runtime")?;
    packet.write_to_path(&resolved)?;
    println!("{}", loop_runtime_stdout_line(&out_path));
    Ok(())
}

pub(crate) fn real_runtime_v2_godel_agent_runtime(repo_root: &Path, args: &[String]) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut agent_count: usize = 10;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 godel-agent-runtime requires --out <path>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--agents" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 godel-agent-runtime requires --agents <count>"
                    ));
                };
                agent_count = value.parse::<usize>().with_context(|| {
                    format!("parse runtime-v2 godel-agent-runtime --agents value '{value}'")
                })?;
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 godel-agent-runtime: {other}"
                ))
            }
        }
        i += 1;
    }

    let graph = runtime_v2_reasoning_graph_contract()?;
    let loop_runtime = runtime_v2_loop_runtime_contract()?;
    let packet = runtime_v2_godel_agent_runtime_contract_for(
        agent_count,
        &graph.graph_id,
        &loop_runtime.runtime_id,
    )?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&packet.canonicalized()?)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "godel-agent-runtime")?;
    packet.write_to_path(&resolved)?;
    println!("{}", godel_agent_runtime_stdout_line(&out_path));
    Ok(())
}

pub(crate) fn real_runtime_v2_contract_market_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 contract-market-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 contract-market-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    let artifacts = runtime_v2_contract_market_demo_contract()?;
    let Some(out_path) = out_path else {
        println!("{}", to_string_pretty(&artifacts.proof_packet)?);
        return Ok(());
    };
    let resolved = resolve_relative_output_path(repo_root, &out_path, "contract-market-demo")?;
    fs::create_dir_all::<&Path>(&resolved).with_context(|| {
        format!(
            "failed to create Runtime v2 contract-market demo root {}",
            resolved.display()
        )
    })?;
    artifacts.write_to_root(&resolved)?;
    println!("{}", contract_market_demo_stdout_line(&out_path));
    println!();
    println!("{}", artifacts.execution_summary()?);
    println!();
    println!("{}", artifacts.operator_report_markdown);
    Ok(())
}

pub(crate) fn real_runtime_v2_governed_tools_flagship_demo(
    repo_root: &Path,
    args: &[String],
) -> Result<()> {
    let mut out_path: Option<PathBuf> = None;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!(
                        "runtime-v2 governed-tools-flagship-demo requires --out <dir>"
                    ));
                };
                out_path = Some(PathBuf::from(value));
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", usage::usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!(
                    "unknown arg for runtime-v2 governed-tools-flagship-demo: {other}"
                ))
            }
        }
        i += 1;
    }

    if let Some(out_path) = out_path {
        resolve_relative_output_path(repo_root, &out_path, "governed-tools-flagship-demo")?;
    }
    Err(anyhow!(
        "runtime-v2 governed-tools-flagship-demo production execution is unavailable: its historical governed adapter is test-only"
    ))
}

pub(crate) fn observatory_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_OBSERVATORY_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

pub(crate) fn feature_proof_coverage_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_FEATURE_PROOF_COVERAGE_PATH={}",
        out_path.display()
    )
}

pub(crate) fn reasoning_graph_stdout_line(out_path: &Path) -> String {
    format!("RUNTIME_V2_REASONING_GRAPH_PATH={}", out_path.display())
}

pub(crate) fn curiosity_engine_stdout_line(out_path: &Path) -> String {
    format!("RUNTIME_V2_CURIOSITY_ENGINE_PATH={}", out_path.display())
}

pub(crate) fn constructability_anchor_validator_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_PATH={}",
        out_path.display()
    )
}

pub(crate) fn loop_runtime_stdout_line(out_path: &Path) -> String {
    format!("RUNTIME_V2_LOOP_RUNTIME_PATH={}", out_path.display())
}

pub(crate) fn godel_agent_runtime_stdout_line(out_path: &Path) -> String {
    format!("RUNTIME_V2_GODEL_AGENT_RUNTIME_PATH={}", out_path.display())
}

pub(crate) fn cognitive_being_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_COGNITIVE_BEING_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}

pub(crate) fn contract_market_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_CONTRACT_MARKET_DEMO_ROOT={}",
        out_path.display()
    )
}

#[cfg(test)]
pub(crate) fn governed_tools_flagship_demo_stdout_line(out_path: &Path) -> String {
    format!(
        "RUNTIME_V2_GOVERNED_TOOLS_FLAGSHIP_DEMO_ROOT={}",
        out_path.display()
    )
}
