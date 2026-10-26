use crate::*;
fn removals(batch: &Value) -> Result<Vec<Value>> {
    Ok(rows(&batch["files"])?
        .iter()
        .filter(|r| r["action"] == "remove")
        .cloned()
        .collect())
}
fn exact_removed(rows: &[Value], before: &Tree, after: &Tree) -> Result<()> {
    for r in rows {
        let p = string(&r["path"])?;
        ensure!(
            *at(before, p) == identity(&r["before"])
                && r["after"].is_null()
                && !after.contains_key(p),
            "applied identity mismatch: {p}"
        );
    }
    Ok(())
}
fn scope(rs: &[Value], n: usize, prior: &Paths) -> Result<Paths> {
    let p = paths(rs)?;
    ensure!(
        rs.len() == n && p.len() == n && p.is_disjoint(prior),
        "removal scope changed"
    );
    Ok(p)
}
fn id_scope(rs: &[Value]) -> Result<String> {
    let mut pairs = rs
        .iter()
        .map(|r| Ok((string(&r["id"])?.to_owned(), string(&r["path"])?.to_owned())))
        .collect::<Result<Vec<_>>>()?;
    pairs.sort();
    canonical_sha(&json!(pairs))
}
fn indexed(rs: &[Value]) -> Result<BTreeMap<&str, &Value>> {
    rs.iter().map(|r| Ok((string(&r["path"])?, r))).collect()
}
pub fn codefriend_batch(
    batch: &Value,
    before: &Tree,
    after: &Tree,
    prior: &Paths,
    data: &Value,
) -> Result<Paths> {
    let rs = rows(&batch["removed"])?;
    let p = scope(rs, 6, prior)?;
    let map = rs
        .iter()
        .map(|r| Ok((string(&r["id"])?, &r["path"])))
        .collect::<Result<BTreeMap<_, _>>>()?;
    ensure!(
        json!(map) == data["CODEFRIEND_REMOVALS"],
        "CodeFriend removal scope changed"
    );
    ensure!(
        batch["rollback_archive"]["sha256"] == data["ARCHIVE_SHA256"],
        "CodeFriend archive changed"
    );
    exact_removed(rs, before, after)?;
    for r in rs {
        let c = &r["rollback"];
        ensure!(
            c["archive_member"] == format!("source/{}", string(&r["path"])?)
                && c["archive_member_sha256"] == r["before"]["sha256"]
                && c["current_bytes_match_archive"] == true
                && c["supplement_required"] == false,
            "CodeFriend rollback identity mismatch"
        );
    }
    Ok(p)
}
pub fn codefriend(batch: &Value, before: &Tree, after: &Tree, prior: &Paths) -> Result<Paths> {
    let rs = removals(batch)?;
    let p = scope(&rs, 198, prior)?;
    ensure!(
        batch["counts"]["product_removals"] == 196 && batch["counts"]["workflow_removals"] == 2,
        "CodeFriend family denominator changed"
    );
    ensure!(
        !p.contains("adl/src/bin/adl_review.rs") && !p.contains("adl/src/cli/codefriend_owner.rs"),
        "retained review or public selector removed"
    );
    let c = indexed(rows(&batch["rollback_custody"]["rows"])?)?;
    ensure!(
        c.keys().map(|p| p.to_string()).collect::<Paths>() == p,
        "CodeFriend family custody scope changed"
    );
    exact_removed(&rs, before, after)?;
    for r in &rs {
        ensure!(
            c[string(&r["path"])?]["preimage_sha256"] == r["before"]["sha256"],
            "CodeFriend family custody identity mismatch"
        );
    }
    Ok(p)
}
pub fn csdlc(
    batch: &Value,
    before: &Tree,
    after: &Tree,
    prior: &Paths,
    data: &Value,
) -> Result<Paths> {
    let rs = removals(batch)?;
    let p = paths(&rs)?;
    for path in &p {
        ensure!(!protected(path, data)?, "C-SDLC protected path removal");
    }
    scope(&rs, 815, prior)?;
    ensure!(
        batch["counts"] == json!({"source_removals":815,"caller_modifications":12,"additions":3}),
        "C-SDLC family denominator changed"
    );
    let cr = rows(&batch["rollback"]["rows"])?;
    let c = indexed(cr)?;
    ensure!(
        cr.len() == 815
            && c.keys().map(|p| p.to_string()).collect::<Paths>() == p
            && id_scope(cr)? == data["CSDLC_REMOVAL_SCOPE_SHA256"],
        "C-SDLC family custody scope changed"
    );
    exact_removed(&rs, before, after)?;
    for r in &rs {
        let path = string(&r["path"])?;
        let x = c[path];
        ensure!(
            x["sha256"] == r["before"]["sha256"]
                && x["member"] == format!("source/{path}")
                && number(&x["mode"])? == custody::mode(&r["before"]["mode"])? as usize,
            "C-SDLC family custody identity mismatch"
        );
    }
    Ok(p)
}
pub fn runtime(
    batch: &Value,
    before: &Tree,
    after: &Tree,
    prior: &Paths,
    data: &Value,
) -> Result<Paths> {
    let rs = removals(batch)?;
    let p = paths(&rs)?;
    let retains = strings(&data["RUNTIME_PUBLIC_RETAINS"])?;
    for path in &p {
        ensure!(
            !protected(path, data)? && !retains.contains(path),
            "Runtime protected path removal"
        );
    }
    scope(&rs, 896, prior)?;
    ensure!(
        batch["counts"] == json!({"source_removals":896,"caller_changes":54,"public_retains":8}),
        "Runtime family denominator changed"
    );
    let cr = rows(&batch["prepared_patch"]["deletions"])?;
    let c = indexed(cr)?;
    ensure!(
        cr.len() == 896
            && c.keys().map(|p| p.to_string()).collect::<Paths>() == p
            && id_scope(cr)? == data["RUNTIME_REMOVAL_SCOPE_SHA256"],
        "Runtime family custody scope changed"
    );
    let retained = rows(&batch["prepared_patch"]["retains"])?;
    ensure!(
        retained.len() == 8 && paths(retained)? == retains,
        "Runtime public retain scope changed"
    );
    for r in retained {
        let path = string(&r["path"])?;
        ensure!(
            *at(before, path) == r["before"] && at(before, path) == at(after, path),
            "Runtime public retained identity changed"
        );
    }
    exact_removed(&rs, before, after)?;
    for r in &rs {
        let path = string(&r["path"])?;
        let x = c[path];
        ensure!(
            x["id"] == r["id"]
                && x["before"] == identity(&r["before"])
                && x["current_sha256"] == r["before"]["sha256"]
                && x["rollback"]["member"] == format!("source/{path}"),
            "Runtime family custody identity mismatch"
        );
    }
    Ok(p)
}
pub fn infrastructure(
    batch: &Value,
    before: &Tree,
    after: &Tree,
    prior: &Paths,
    data: &Value,
) -> Result<Paths> {
    let rs = removals(batch)?;
    let p = scope(&rs, 212, prior)?;
    ensure!(
        id_scope(&rs)? == data["INFRASTRUCTURE_REMOVAL_SCOPE_SHA256"]
            && p.iter().all(|p| p.starts_with("infra/")),
        "Infrastructure removal scope changed"
    );
    ensure!(
        batch["counts"] == json!({"source_removals":212,"caller_changes":20}),
        "Infrastructure family denominator changed"
    );
    let all = rows(&batch["files"])?;
    ensure!(
        all.len() == 232 && paths(all)?.len() == 232,
        "Infrastructure caller scope changed"
    );
    for r in all {
        let path = string(&r["path"])?;
        ensure!(
            *at(before, path) == identity(&r["before"])
                && *at(after, path) == identity(&r["after"]),
            "Infrastructure applied identity mismatch"
        );
        let c = &r["rollback"];
        ensure!(
            c["member"] == path
                && c["sha256"] == r["before"]["sha256"]
                && custody::mode(&c["mode"])? == custody::mode(&r["before"]["mode"])?,
            "Infrastructure custody identity mismatch"
        );
    }
    Ok(p)
}
pub fn recent(
    batch: &Value,
    before: &Tree,
    after: &Tree,
    prior: &Paths,
    spec: &Value,
    data: &Value,
) -> Result<Paths> {
    let all = rows(&batch["files"])?;
    let rs = removals(batch)?;
    let p = scope(&rs, number(&spec["removals"])?, prior)?;
    ensure!(
        all.len() == number(&spec["files"])?
            && paths(all)?.len() == all.len()
            && canonical_sha(&json!(p))? == spec["scope_sha256"],
        "Recent owner exact scope changed"
    );
    for path in &p {
        ensure!(!protected(path, data)?, "Recent owner protected removal");
    }
    for r in all {
        let path = string(&r["path"])?;
        require_action(string(&r["action"])?, &r["before"], &r["after"])?;
        ensure!(
            *at(before, path) == identity(&r["before"])
                && *at(after, path) == identity(&r["after"]),
            "Recent owner applied identity mismatch: {path}"
        );
    }
    Ok(p)
}
