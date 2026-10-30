//! Current public documentation contract, deterministic repository-local checks.
use super::*;
use std::collections::{BTreeSet, HashMap};
pub fn prose(text: &str) -> String {
    let mut fence = None;
    let mut lines = vec![];
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(mark) = fence {
            if t.starts_with(mark) {
                fence = None;
            }
            continue;
        }
        if t.starts_with("```") {
            fence = Some("```");
            continue;
        }
        if t.starts_with("~~~") {
            fence = Some("~~~");
            continue;
        }
        lines.push(line);
    }
    lines.join("\n")
}
pub fn anchors(text: &str) -> BTreeSet<String> {
    let mut seen = HashMap::new();
    let mut out = BTreeSet::new();
    for line in prose(text).lines() {
        let hashes = line.chars().take_while(|c| *c == '#').count();
        if !(1..=6).contains(&hashes) || !line[hashes..].starts_with(char::is_whitespace) {
            continue;
        }
        let title = line[hashes..]
            .trim()
            .trim_end_matches('#')
            .trim()
            .to_lowercase();
        let slug: String = title
            .chars()
            .filter(|c| c.is_alphanumeric() || ['_', '-', ' '].contains(c))
            .map(|c| if c == ' ' { '-' } else { c })
            .collect();
        let count = seen.entry(slug.clone()).or_insert(0);
        out.insert(if *count == 0 {
            slug
        } else {
            format!("{slug}-{count}")
        });
        *count += 1;
    }
    out
}
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            c => out.push(c.as_os_str()),
        }
    }
    out
}
pub fn link_errors(root: &Path, relative: &str) -> Result<Vec<String>> {
    let root = fs::canonicalize(root)?;
    let path = root.join(relative);
    let pattern = regex::Regex::new(r#"\]\(([^\s)]+)(?:\s+"[^"]*")?\)"#)?;
    let scheme = regex::Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*:")?;
    let mut errors = vec![];
    for capture in pattern.captures_iter(&prose(&fs::read_to_string(&path)?)) {
        let target = &capture[1];
        if scheme.is_match(target) {
            continue;
        }
        let target = target.trim_matches(['<', '>']);
        let (file, anchor) = target.split_once('#').unwrap_or((target, ""));
        let dest = if file.is_empty() {
            path.clone()
        } else {
            normalize(&path.parent().context("doc parent")?.join(file))
        };
        let dest = fs::canonicalize(&dest).unwrap_or(dest);
        if !dest.starts_with(&root) {
            errors.push(format!("{relative}: outside repository link {target}"));
        } else if !dest.exists() {
            errors.push(format!("{relative}: missing target {target}"));
        } else if !anchor.is_empty()
            && dest.extension().is_some_and(|e| e == "md")
            && !anchors(&fs::read_to_string(&dest)?).contains(anchor)
        {
            errors.push(format!("{relative}: missing anchor {target}"));
        }
    }
    Ok(errors)
}
pub fn check(root: &Path) -> Result<Vec<String>> {
    let mut docs = vec![
        "README.md",
        "CODE_OF_CONDUCT.md",
        "CONTRIBUTING.md",
        "REVIEW.md",
        "AGENTS.md",
        "docs/onboarding.md",
        "docs/default_workflow.md",
        "docs/README.md",
        "docs/COMPATIBILITY.md",
        "docs/PUBLIC_BUILD.md",
        "demos/README.md",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    for name in [
        "README.md",
        "PACKAGE.md",
        "ADL_UNIVERSAL_TOOL_SCHEMA.md",
        "UTS_V1.1_SCHEMA.md",
    ] {
        docs.push(format!("docs/specs/uts/{name}"));
    }
    for name in ["README.md", "ACC_V1.0_SPEC.md", "ACC_V1.1_SPEC.md"] {
        docs.push(format!("docs/specs/acc/{name}"));
    }
    let mut errors = vec![];
    let retired = regex::Regex::new(r"adl/src/(?:uts|acc)\.rs")?;
    for name in docs {
        if !root.join(&name).is_file() {
            errors.push(format!("missing current doc {name}"));
            continue;
        }
        errors.extend(link_errors(root, &name)?);
        if retired.is_match(&fs::read_to_string(root.join(&name))?) {
            errors.push(format!("{name}: retired current implementation path"));
        }
    }
    let example = "adl/examples/v0-3-fork-join-seq-run.adl.yaml";
    if !root.join(example).is_file() {
        errors.push("missing public example input".into());
    }
    for(target,expected)in [("build","cargo build --locked --manifest-path adl/Cargo.toml --bin adl".to_owned()),("plan",format!("cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- {example} --print-plan")),("check","bash tools/public_adl/run_cli_ci.sh".into()),("packages","bash tools/public_adl/run_package_checks.sh".into()),("docs","cargo run --locked --quiet --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- docs --self-test".into())]{let result=Command::new("make").args(["--no-print-directory","-n",target]).current_dir(root).output()?;if !result.status.success()||text(&result).0.trim()!=expected{errors.push(format!("Make target {target} does not delegate the declared public command"));}}
    let manifest: toml::Value = fs::read_to_string(root.join("adl/Cargo.toml"))?.parse()?;
    let version = manifest["package"]["version"]
        .as_str()
        .context("package version")?;
    let compatibility = fs::read_to_string(root.join("docs/COMPATIBILITY.md"))?;
    if !compatibility.contains(&format!("`adl` {version}")) {
        errors.push("CLI version table differs from manifest".into());
    }
    let toolchain: toml::Value = fs::read_to_string(root.join("rust-toolchain.toml"))?.parse()?;
    let channel = toolchain["toolchain"]["channel"]
        .as_str()
        .context("toolchain channel")?;
    if !compatibility.contains(&format!("Rust {channel}")) {
        errors.push("Rust version table differs from toolchain".into());
    }
    for name in [
        ".github/ISSUE_TEMPLATE/bug_report.md",
        ".github/ISSUE_TEMPLATE/proposal.md",
        ".github/pull_request_template.md",
    ] {
        if !root.join(name).is_file() {
            errors.push(format!("missing public intake {name}"));
        }
    }
    Ok(errors)
}
pub fn self_test() -> Result<()> {
    use anyhow::ensure;
    let t = tempfile::tempdir()?;
    let root = t.path();
    fs::write(
        root.join("guide.md"),
        "# Guide\n[bad](absent.md)\n[bad](target.md#absent)\n",
    )?;
    fs::write(root.join("target.md"), "# Present\n")?;
    ensure!(
        link_errors(root, "guide.md")?.len() == 2,
        "missing file/anchor fixture"
    );
    fs::write(
        root.join("guide.md"),
        "# Guide\n[ok](target.md#same-1)\n```md\n[example](missing.md)\n```\n",
    )?;
    fs::write(root.join("target.md"), "# Same\n# Same\n")?;
    ensure!(
        link_errors(root, "guide.md")?.is_empty(),
        "duplicate heading/code fence fixture"
    );
    fs::write(root.join("guide.md"), "[outside](../outside.md)\n")?;
    ensure!(
        link_errors(root, "guide.md")?
            .iter()
            .any(|s| s.contains("outside repository")),
        "escape fixture"
    );
    Ok(())
}
