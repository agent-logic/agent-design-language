//! Optional 4+1 artifacts participate in the existing exact-artifact approval.
use super::manifest::VerifiedArtifact;
use crate::codefriend::{
    architecture::{
        artifact::StructureArtifact,
        four_plus_one::{
            generation::{self, Generation},
            render,
        },
    },
    evidence::{contracts::ReviewRecord, hash},
};
use anyhow::{ensure, Result};
use std::collections::BTreeMap;

pub(crate) fn validate_files(
    files: &BTreeMap<String, Vec<u8>>,
    review: &ReviewRecord,
    now: u64,
) -> Result<()> {
    let get = |name: &str| {
        files
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("architecture_publication_missing_{name}"))
    };
    let generation: Generation = serde_json::from_slice(get("generation.json")?)?;
    let graph: StructureArtifact = serde_json::from_slice(get("graph.json")?)?;
    graph.record().validate()?;
    ensure!(
        graph.record().admission == review.admission,
        "architecture_publication_admission_changed"
    );
    let response: String = serde_json::from_slice(get("response.json")?)?;
    let rebuilt = generation::accept_response(&review.admission, &graph, &response, now)?;
    ensure!(
        hash(&generation)? == hash(&rebuilt)?,
        "architecture_publication_generation_changed"
    );
    let mut expected = render::artifacts(&generation.package, &review.admission, now)?;
    for name in ["generation.json", "graph.json", "response.json"] {
        expected.insert(name.into(), get(name)?.clone());
    }
    ensure!(
        &expected == files,
        "architecture_publication_render_changed"
    );
    Ok(())
}

pub(crate) fn from_snapshot(
    artifacts: &[VerifiedArtifact],
    review: &ReviewRecord,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let files: BTreeMap<_, _> = artifacts
        .iter()
        .filter_map(|a| {
            a.path
                .strip_prefix("architecture/")
                .map(|p| (p.to_owned(), a.bytes.clone()))
        })
        .collect();
    if !files.is_empty() {
        validate_files(
            &files,
            review,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        )?;
    }
    Ok(files)
}

pub(crate) fn markdown(files: &BTreeMap<String, Vec<u8>>) -> Result<String> {
    let Some(bytes) = files.get("architecture.md") else {
        return Ok(String::new());
    };
    let mut text = std::str::from_utf8(bytes)?.to_owned();
    for name in files.keys() {
        text = text.replace(&format!("]({name})"), &format!("](architecture-{name})"));
    }
    Ok(text)
}

pub(crate) fn html(files: &BTreeMap<String, Vec<u8>>) -> Result<String> {
    let mut html = ::markdown::to_html(&markdown(files)?);
    for view in [
        "logical",
        "development",
        "process",
        "deployment",
        "scenarios",
    ] {
        html = html.replace(
            &format!("<h2>{view}</h2>"),
            &format!("<h2 id=\"{view}\">{view}</h2>"),
        );
    }
    if let Some(bytes) = files.get("package.json") {
        let package: crate::codefriend::architecture::four_plus_one::Package =
            serde_json::from_slice(bytes)?;
        for scenario in &package.scenarios {
            let name = format!("scenario-{}", scenario.id);
            html = html.replace(
                &format!("<h3>{name}</h3>"),
                &format!("<h3 id=\"{name}\">{name}</h3>"),
            );
        }
    }
    Ok(format!(
        "<section aria-label=\"4+1 architecture\">{html}</section>"
    ))
}

pub(crate) fn attachments(files: &BTreeMap<String, Vec<u8>>) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut result: BTreeMap<_, _> = files
        .iter()
        .map(|(name, bytes)| (format!("architecture-{name}"), bytes.clone()))
        .collect();
    if files.contains_key("architecture.md") {
        result.insert(
            "architecture-architecture.md".into(),
            markdown(files)?.into_bytes(),
        );
    }
    Ok(result)
}
