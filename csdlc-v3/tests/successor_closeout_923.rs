//! PVF: deterministic local planning-contract check, bounded file reads only.
//! Verifies #923 tail/link/finish invariants; not release or product proof.
use std::path::Path;
fn check(text: &str, base: &Path) -> Result<(), String> {
    let rows: Vec<_> = text.lines().filter(|s| s.starts_with("| TAIL-")).collect();
    if rows.len() != 10 {
        return Err("ten tail stages required".into());
    }
    for (i, row) in rows.iter().enumerate() {
        if !row.starts_with(&format!("| TAIL-{:02} |", i + 1)) {
            return Err("tail order".into());
        }
        let cells: Vec<_> = row
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if cells.len() != 4 {
            return Err("owner/input/output required".into());
        }
    }
    for part in text.split("](").skip(1) {
        let link = part.split(')').next().ok_or("link syntax")?;
        if !link.starts_with("https://") && !base.join(link).is_file() {
            return Err(format!("missing {link}"));
        }
    }
    for required in [
        "not all workers finishing housekeeping",
        "`clean` is separate",
        "15-minute break",
        "All findings fixed",
        "Attempt failed",
        "#1148/#1149",
        "#1150",
        "Worker #1",
        "exact artifact identity",
    ] {
        if !text.contains(required) {
            return Err(format!("missing boundary {required}"));
        }
    }
    Ok(())
}
#[test]
fn complete_closeout_plan_and_negative_contract_cases() {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docs/milestones/v0.92.2/evidence/issue-923");
    let text = std::fs::read_to_string(base.join("SUCCESSOR_CLOSEOUT_PLAN.md")).unwrap();
    check(&text, &base).unwrap();
    for damaged in [
        text.replace("| TAIL-05 |", "| TAIL-06 |"),
        text.lines()
            .filter(|s| !s.starts_with("| TAIL-08"))
            .collect::<Vec<_>>()
            .join("\n"),
        text.replace(
            "not all workers finishing housekeeping",
            "only after every cleanup",
        ),
        text.replace("../../../v0.93/MILESTONE_SPLIT_v0.93.json", "missing.json"),
        text.replace("| Quality owner |", "| |"),
        text.replace("15-minute break", "immediate start"),
    ] {
        assert!(check(&damaged, &base).is_err());
    }
}
