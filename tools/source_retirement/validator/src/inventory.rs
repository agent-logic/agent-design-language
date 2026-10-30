use crate::*;
pub fn combined(
    original: &Value,
    combined: &Value,
    groups: &Value,
    frozen: &Tree,
    observed: &Tree,
    data: &Value,
) -> Result<()> {
    ensure!(
        original["new_removals_authorized"] == true && original["all_holds_resolved"] == false,
        "original scoped approval flags changed"
    );
    let original_rows = rows(&original["rows"])?;
    let current = rows(&combined["rows"])?;
    let additional = rows(&combined["additional_paths"])?;
    let captured: Vec<_> = current
        .iter()
        .chain(additional)
        .map(|r| json!([r["id"], r["path"], r["current_identity"]]))
        .collect();
    ensure!(
        canonical_sha(&json!(captured))? == data["CAPTURED_IDENTITIES_SHA256"],
        "captured current identities changed"
    );
    let tuples: Vec<_> = original_rows
        .iter()
        .map(|r| json!([r["id"], r["path"], r["original_mode"], r["original_blob"]]))
        .collect();
    ensure!(
        original_rows.len() == 4134
            && paths(original_rows)?.len() == 4134
            && canonical_sha(&json!(tuples))? == data["INVENTORY_SHA256"],
        "original inventory identity changed"
    );
    ensure!(
        original["original_revision"] == "248f00e359e412f6bc0061ac88be9ff374e3a212"
            && original["observed_main"] == "9cfd035aac2a44637b3c8026bd40b669579a8c7f",
        "reviewed baseline changed"
    );
    for r in original_rows {
        let p = string(&r["path"])?;
        ensure!(
            *at(frozen, p) == json!([r["original_mode"], r["original_blob"]]),
            "frozen identity mismatch: {p}"
        );
        ensure!(
            *at(observed, p) == r["observed_main"],
            "observed identity mismatch: {p}"
        );
    }
    let ordered = |rs: &[Value]| {
        rs.iter()
            .map(|r| json!([r["id"], r["path"]]))
            .collect::<Vec<_>>()
    };
    ensure!(
        ordered(current) == ordered(original_rows),
        "combined ordered inventory changed"
    );
    let added = paths(additional)?;
    ensure!(
        additional.len() == 22
            && added.len() == 22
            && combined["additional_current_source_paths"] == 22
            && added.is_disjoint(&paths(original_rows)?)
            && additional
                .iter()
                .all(|r| r["original_inventory_member"] == false),
        "additional inventory overlap or count drift"
    );
    let mut counts = BTreeMap::<String, usize>::new();
    let mut held = BTreeMap::new();
    for r in current {
        let a = string(&r["proposed_action"])?;
        *counts.entry(a.to_owned()).or_default() += 1;
        if a.starts_with("hold_") {
            held.insert(string(&r["id"])?, r);
        }
    }
    ensure!(
        json!(counts) == combined["counts"] && combined["original_count"] == 4134,
        "combined counts drift"
    );
    let mut grouped = BTreeMap::new();
    let mut count = 0;
    for g in rows(&groups["groups"])? {
        let rs = rows(&g["rows"])?;
        ensure!(number(&g["count"])? == rs.len(), "held group count changed");
        for r in rs {
            count += 1;
            grouped.insert(string(&r["id"])?, r);
        }
    }
    ensure!(
        count == grouped.len()
            && count == number(&groups["held_count"])?
            && grouped == held
            && groups["original_count"] == 4134
            && groups["additional_paths_separate"] == 22,
        "held groups are not complete and disjoint"
    );
    Ok(())
}
pub fn separately_observed(
    combined: &Value,
    frozen: &Tree,
    observed: &Tree,
    removed: &Paths,
    data: &Value,
) -> Result<()> {
    let records = &combined["separately_observed_retirements"];
    ensure!(
        *records == data["SEPARATELY_OBSERVED_RETIREMENTS"],
        "separately observed retirement scope or provenance changed"
    );
    let mut captured = paths(rows(&combined["rows"])?)?;
    captured.extend(paths(rows(&combined["additional_paths"])?)?);
    for r in rows(records)? {
        let p = string(&r["path"])?;
        ensure!(
            !captured.contains(p)
                && !frozen.contains_key(p)
                && *at(observed, p) == identity(&r["execution_baseline_identity"])
                && removed.contains(p),
            "separately observed retirement inventory/baseline/approval mismatch"
        );
    }
    Ok(())
}
