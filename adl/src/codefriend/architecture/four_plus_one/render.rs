//! Deterministic, inert diagram/report artifacts; no external renderer execution.
use super::*;
use std::fmt::Write;

pub const RENDERER: &str = "codefriend.four_plus_one.render.v1";
fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
fn md(s: &str) -> String {
    xml(s)
        .replace('\\', "&#92;")
        .replace('|', "&#124;")
        .replace('[', "&#91;")
        .replace(']', "&#93;")
        .replace('*', "&#42;")
        .replace('_', "&#95;")
        .replace('`', "&#96;")
        .replace('\n', " ")
}
fn name(v: View) -> &'static str {
    match v {
        View::Logical => "logical",
        View::Development => "development",
        View::Process => "process",
        View::Deployment => "deployment",
    }
}
fn basis(b: &Basis) -> &'static str {
    match b {
        Basis::SourceDeclaration => "source declaration",
        Basis::Inference => "inference",
        Basis::Assumption => "assumption",
    }
}
fn refs(citations: &[Citation]) -> String {
    citations
        .iter()
        .map(|c| {
            format!(
                "{}:{}–{} [{}]",
                md(&c.path),
                c.first_line,
                c.last_line,
                md(&c.evidence_id)
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}
fn wrapped(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for ch in s.chars() {
        if ch == '\n' || line.chars().count() >= width {
            lines.push(std::mem::take(&mut line));
        }
        if ch != '\n' {
            line.push(ch);
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}
fn svg_label(out: &mut String, label: &str, x: usize, y: usize, width: usize) {
    for (i, line) in wrapped(label, width).iter().enumerate() {
        let _ = write!(
            out,
            "<text x=\"{x}\" y=\"{}\">{}</text>",
            y + i * 18,
            xml(line)
        );
    }
}

/// Relative output paths and deterministic bytes for manifest inclusion. Evidence
/// excerpts are in the JSON package; readable source references accompany every claim.
pub fn artifacts(
    package: &Package,
    admission: &Admission,
    now: u64,
) -> Result<BTreeMap<String, Vec<u8>>> {
    package.validate(admission, now)?;
    let mut files = BTreeMap::new();
    files.insert("package.json".into(), serde_json::to_vec_pretty(package)?);
    let mut report=format!("# 4+1 architecture package\n\nRepository: {}\n\nRevision: {}\n\nCoverage: {}. Source declarations and inference are not verified running topology.\n\n",md(&package.repository),package.revision,if package.complete{"all views populated"}else{"incomplete"});
    report.push_str("Views: [Logical](#logical), [Development](#development), [Process](#process), [Deployment](#deployment), [Scenarios](#scenarios).\n\n");
    for v in View::ALL {
        let view = &package.views[&v];
        let key = name(v);
        write!(report, "## {key}\n\n").unwrap();
        for missing in &view.missing_inputs {
            writeln!(report, "Missing input: {}\n", md(missing)).unwrap();
        }
        let mut mermaid = String::from("flowchart TD\n");
        let mut ids = BTreeMap::new();
        report.push_str(
            "| ID | Name and responsibility | Basis | Evidence |\n| --- | --- | --- | --- |\n",
        );
        for (i, id) in view.entities.iter().enumerate() {
            let e = package
                .entities
                .iter()
                .find(|e| &e.id == id)
                .expect("validated entity");
            ids.insert(id.as_str(), i);
            writeln!(mermaid, "  n{i}[\"entity_{i}\"]").unwrap();
            writeln!(
                report,
                "| {} | {}: {} | {} | {} |",
                md(id),
                md(&e.name),
                md(&e.responsibility),
                basis(&e.basis),
                refs(&e.citations)
            )
            .unwrap();
        }
        report.push_str(
            "\n| Relationship | From → to | Basis | Evidence |\n| --- | --- | --- | --- |\n",
        );
        for (i, r) in view.relationships.iter().enumerate() {
            writeln!(
                mermaid,
                "  n{} -->|relationship_{i}| n{}",
                ids[r.from.as_str()],
                ids[r.to.as_str()]
            )
            .unwrap();
            writeln!(
                report,
                "| R{i}: {} | {} → {} | {} | {} |",
                md(&r.description),
                md(&r.from),
                md(&r.to),
                basis(&r.basis),
                refs(&r.citations)
            )
            .unwrap();
        }
        files.insert(format!("{key}.mmd"), mermaid.into_bytes());
        writeln!(report,"\n[Editable diagram source]({key}.mmd). In that source, entity_N follows the entity table order; relationship_N is RN above.\n").unwrap();
        for (i, id) in view.entities.iter().enumerate() {
            if view
                .relationships
                .iter()
                .any(|r| &r.from == id || &r.to == id)
            {
                continue;
            }
            let entity = package
                .entities
                .iter()
                .find(|e| &e.id == id)
                .expect("validated entity");
            let height = 80 + wrapped(&entity.name, 80).len() * 18;
            let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"760\" height=\"{height}\" role=\"img\"><title>{key} isolated entity {i}</title><rect width=\"760\" height=\"{height}\" fill=\"white\"/><rect x=\"10\" y=\"30\" width=\"740\" height=\"{}\" fill=\"#edf2f8\" stroke=\"#24354b\"/><g fill=\"#172234\" font-family=\"sans-serif\" font-size=\"13\">",height-40);
            svg_label(
                &mut svg,
                &format!("{key}: {} (no admitted relationship)", entity.id),
                10,
                18,
                100,
            );
            svg_label(&mut svg, &entity.name, 20, 52, 80);
            svg.push_str("</g></svg>\n");
            let path = format!("{key}-entity-{i:03}.svg");
            writeln!(report, "![{key} isolated entity {i}]({path})\n").unwrap();
            files.insert(path, svg.into_bytes());
        }
        // One relationship per diagram: long names wrap rather than clipping,
        // and each diagram is independently embeddable in a PDF page.
        for (i, r) in view.relationships.iter().enumerate() {
            let from = package.entities.iter().find(|e| e.id == r.from).unwrap();
            let to = package.entities.iter().find(|e| e.id == r.to).unwrap();
            let fl = wrapped(&from.name, 38);
            let tl = wrapped(&to.name, 38);
            let rows = fl.len().max(tl.len());
            let height = 100 + rows * 18;
            let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"760\" height=\"{height}\" viewBox=\"0 0 760 {height}\" role=\"img\"><title>{key} relationship R{i}</title><rect width=\"760\" height=\"{height}\" fill=\"white\"/><g stroke=\"#24354b\" fill=\"#edf2f8\"><rect x=\"10\" y=\"35\" width=\"320\" height=\"{}\" rx=\"6\"/><rect x=\"430\" y=\"35\" width=\"320\" height=\"{}\" rx=\"6\"/><path d=\"M330 60 H420 M410 54 L420 60 L410 66\" fill=\"none\"/></g><g fill=\"#172234\" font-family=\"sans-serif\" font-size=\"13\">",rows*18+20,rows*18+20);
            svg_label(
                &mut svg,
                &format!("{key}: R{i} ({})", basis(&r.basis)),
                10,
                20,
                100,
            );
            svg_label(&mut svg, &from.name, 20, 55, 38);
            svg_label(&mut svg, &to.name, 440, 55, 38);
            svg.push_str("</g></svg>\n");
            let path = format!("{key}-{i:03}.svg");
            writeln!(report, "![{key} R{i}]({path})\n").unwrap();
            files.insert(path, svg.into_bytes());
        }
    }
    report.push_str("## scenarios\n\n| Scenario | Logical | Development | Process | Deployment |\n| --- | --- | --- | --- | --- |\n");
    for s in &package.scenarios {
        write!(report, "| [{}](#scenario-{})", md(&s.id), s.id).unwrap();
        for v in View::ALL {
            write!(
                report,
                " | {}",
                s.trace
                    .get(&v)
                    .map(|t| t.iter().map(|id| md(id)).collect::<Vec<_>>().join(" → "))
                    .unwrap_or_else(|| "Missing trace".into())
            )
            .unwrap();
        }
        report.push_str(" |\n");
    }
    for s in &package.scenarios {
        write!(
            report,
            "\n### scenario-{}\n\n{}\n\nBasis: {}; failure/recovery: {}.\n\nEvidence: {}\n",
            s.id,
            md(&s.description),
            basis(&s.basis),
            s.failure_recovery,
            refs(&s.citations)
        )
        .unwrap();
    }
    report.push_str("\n## Conflicts and missing inputs\n\n");
    for c in &package.conflicts {
        writeln!(
            report,
            "- Conflict: {}. Evidence: {}",
            md(&c.description),
            refs(&c.citations)
        )
        .unwrap();
    }
    for missing in &package.missing_inputs {
        writeln!(report, "- Missing input: {}", md(missing)).unwrap();
    }
    files.insert("architecture.md".into(), report.into_bytes());
    ensure!(
        files.values().map(Vec::len).sum::<usize>() <= 16 * 1024 * 1024,
        "four_plus_one_render_bounds"
    );
    Ok(files)
}

/// Create a new private artifact directory beneath an owner-validated output root.
pub fn write_artifacts(
    package: &Package,
    admission: &Admission,
    now: u64,
    output: &std::path::Path,
) -> Result<()> {
    use std::io::Write;
    use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
    super::generation::validate_artifact_path(output)?;
    let files = artifacts(package, admission, now)?;
    ensure!(files.len() <= 1536, "four_plus_one_artifact_count");
    std::fs::DirBuilder::new().mode(0o700).create(output)?;
    for (path, bytes) in files {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(output.join(path))?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    std::fs::File::open(output)?.sync_all()?;
    Ok(())
}
