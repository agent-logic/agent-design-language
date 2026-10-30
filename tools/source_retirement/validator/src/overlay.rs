use crate::*;
pub fn check(
    manifest: &Value,
    baseline: &Tree,
    candidate: &Tree,
    removed: &Paths,
    scoped: &Paths,
    data: &Value,
) -> Result<Paths> {
    ensure!(
        manifest["schema"] == "adl.source_retirement.execution_overlay.v1"
            && manifest["issue"] == 1223,
        "invalid execution manifest"
    );
    ensure!(
        manifest["status"] == "complete" && rows(&manifest["missing_entries"])?.is_empty(),
        "execution manifest incomplete"
    );
    let mut expected = baseline.clone();
    let mut seen = Paths::new();
    let mut transient = Paths::new();
    for e in rows(&manifest["entries"])? {
        let p = string(&e["path"])?;
        ensure!(seen.insert(p.to_owned()), "duplicate execution path: {p}");
        for field in [
            "review_reference",
            "implementation_commit",
            "approval_reference",
        ] {
            ensure!(
                e[field].as_str().is_some_and(|s| !s.is_empty()),
                "execution provenance missing: {p}"
            );
        }
        ensure!(
            e["before"] == *at(baseline, p),
            "execution before identity mismatch: {p}"
        );
        let after = &e["after"];
        let action = string(&e["action"])?;
        if action == "remove_added" {
            ensure!(
                e["before"].is_null() && after.is_null(),
                "invalid execution transition"
            );
            let (add, remove, id) = if p == data["TRANSIENT_CONVERSION_BRIDGE"] {
                (
                    "f598c0711d9b270a5b304c7cde64dcdf3068619e",
                    "d293b17fe05f70633826017eb581f5315359f80f",
                    &data["TRANSIENT_BRIDGE_IDENTITY"],
                )
            } else if p == data["TRANSIENT_RUNTIME_RESOURCES"] {
                (
                    "4df3ff232116341f2dfecb4dfd4e5448157aed63",
                    "7783ee5303651ace42d5d3fecd4055aa83a0a3a5",
                    &data["TRANSIENT_RUNTIME_IDENTITY"],
                )
            } else {
                bail!("unapproved transient removal")
            };
            ensure!(
                !e["rollback_custody"].is_null(),
                "unapproved transient removal"
            );
            let chain = e["source_chain"]
                .as_array()
                .into_iter()
                .flatten()
                .chain(
                    e["supplemental_source_chain"]
                        .as_array()
                        .into_iter()
                        .flatten(),
                )
                .collect::<Vec<_>>();
            let adds = chain
                .iter()
                .filter(|c| c["commit"] == add)
                .collect::<Vec<_>>();
            let deletes = chain
                .iter()
                .filter(|c| c["commit"] == remove)
                .collect::<Vec<_>>();
            ensure!(
                adds.len() == 1
                    && deletes.len() == 1
                    && adds[0]["before"].is_null()
                    && adds[0]["after"] == *id
                    && deletes[0]["before"] == *id
                    && deletes[0]["after"].is_null()
                    && identity(&e["rollback_custody"]) == *id,
                "transient add/removal chain mismatch"
            );
            transient.insert(p.to_owned());
        } else {
            require_action(action, &e["before"], after)?;
        }
        if !after.is_null() {
            let v = rows(after)?;
            ensure!(
                v.len() == 2
                    && ["100644", "100755", "120000"].iter().any(|m| v[0] == *m)
                    && v[1].as_str().is_some_and(|s| s.len() == 40
                        && s.bytes()
                            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))),
                "invalid after identity"
            );
        } else {
            ensure!(
                removed.contains(p)
                    && !e["rollback_custody"].is_null()
                    && e["rollback_custody"] != false,
                "unapproved removal or missing custody: {p}"
            );
        }
        replace(&mut expected, p, after);
    }
    let deleted: Paths = baseline
        .keys()
        .filter(|p| !candidate.contains_key(*p))
        .cloned()
        .collect();
    ensure!(
        deleted == removed.difference(&transient).cloned().collect(),
        "new deletion outside executed approval set"
    );
    for p in scoped.union(&seen) {
        if removed.contains(p) {
            ensure!(
                !candidate.contains_key(p),
                "approved removal still present: {p}"
            );
        } else {
            ensure!(
                at(candidate, p) == at(&expected, p),
                "retained source exceeds execution identity: {p}"
            );
        }
    }
    Ok(removed.clone())
}
