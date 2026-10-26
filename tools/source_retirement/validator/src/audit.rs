use crate::*;
pub fn run(repo: &Repo) -> Result<Value> {
    let data = identities()?;
    let ledger = repo.packet("DISPOSITION_REGISTER.json")?;
    let observed = repo.tree(string(&ledger["observed_main"])?)?;
    let frozen = repo.tree(string(&ledger["original_revision"])?)?;
    let candidate = repo.tree("HEAD")?;
    let combined = repo.packet("COMBINED_REMAINING_DELTA.json")?;
    let groups = repo.packet("GROUPED_REMAINING_HOLDS.json")?;
    inventory::combined(&ledger, &combined, &groups, &frozen, &observed, &data)?;
    ensure!(
        groups["ledger_sha256"]
            == sha(&repo.bytes(&format!("{PACKET}/COMBINED_REMAINING_DELTA.json"))?),
        "grouped ledger digest mismatch"
    );
    let approved = repo.pinned_packet(
        "APPROVED_COMPONENT_DELTA.json",
        "4b752d6ccb95bb7c2857540126f3d1c145c2220838a1a1df9e9c59bc0d94198c",
    )?;
    let mut removed = rows(&ledger["rows"])?
        .iter()
        .filter(|r| r["action"] == "existing_branch_removal")
        .map(|r| Ok(string(&r["path"])?.to_owned()))
        .collect::<Result<Paths>>()?;
    let components = strings(&data["COMPONENTS"])?;
    ensure!(
        removed.len() == 34
            && removed
                .iter()
                .all(|p| components.iter().any(|c| p.starts_with(&format!("{c}/")))),
        "original removal scope changed"
    );
    let component_removed = strings(&approved["removed"])?;
    ensure!(
        component_removed.len() == 36 && removed.is_disjoint(&component_removed),
        "component removal scope changed"
    );
    removed.extend(component_removed);
    let cutover = repo.pinned_packet(
        "PUBLIC_CUTOVER_APPLIED.json",
        "b4c8926946c54a5bca644b8c171b03995a404b8bcea55102ef45c5ec6ff12dcd",
    )?;
    let extra = "adl/tools/test_select_validation_lanes.sh";
    let deletes = rows(&cutover["paths"])?
        .iter()
        .filter(|r| r["action"] == "delete")
        .map(|r| r["path"].clone())
        .collect::<Vec<_>>();
    ensure!(
        deletes == vec![json!(extra)],
        "public cutover deletion scope changed"
    );
    ensure!(
        !repo
            .tree(string(&cutover["implementation_commit"])?)?
            .contains_key(extra)
            && repo
                .tree(string(&cutover["base_head"])?)?
                .contains_key(extra),
        "cutover removal custody mismatch"
    );
    removed.insert(extra.to_owned());
    let metadata = repo.text(&["rev-parse", "--path-format=absolute", "--git-common-dir"])?;
    let original =
        Path::new(&metadata).join("csdlc-v3/local/evidence/1223/retirement/baseline.tar.gz");
    let mut archives = custody::Archives::default();
    let families = [
        (
            "CODEFRIEND_SIX_HELPER_APPLIED.json",
            "CODEFRIEND_BATCH_SHA256",
            "CODEFRIEND_IMPLEMENTATION",
        ),
        (
            "CODEFRIEND_FAMILY_APPLIED.json",
            "CODEFRIEND_FAMILY_SHA256",
            "CODEFRIEND_FAMILY_IMPLEMENTATION",
        ),
        (
            "CSDLC_SOURCE_FAMILY_APPLIED.json",
            "CSDLC_FAMILY_SHA256",
            "CSDLC_FAMILY_IMPLEMENTATION",
        ),
        (
            "RUNTIME_SOURCE_FAMILY_APPLIED.json",
            "RUNTIME_FAMILY_SHA256",
            "RUNTIME_FAMILY_IMPLEMENTATION",
        ),
        (
            "INFRASTRUCTURE_SOURCE_FAMILY_APPLIED.json",
            "INFRASTRUCTURE_FAMILY_SHA256",
            "INFRASTRUCTURE_FAMILY_IMPLEMENTATION",
        ),
    ];
    let mut accepted = Vec::new();
    for (index, (name, pin, commit)) in families.iter().enumerate() {
        let b = repo.pinned_packet(name, string(&data[pin])?)?;
        let revision = string(&data[commit])?;
        let before = repo.tree(&format!("{revision}^"))?;
        let after = repo.tree(revision)?;
        let added = match index {
            0 => family::codefriend_batch(&b, &before, &after, &removed, &data)?,
            1 => family::codefriend(&b, &before, &after, &removed)?,
            2 => family::csdlc(&b, &before, &after, &removed, &data)?,
            3 => family::runtime(&b, &before, &after, &removed, &data)?,
            _ => family::infrastructure(&b, &before, &after, &removed, &data)?,
        };
        removed.extend(added);
        accepted.push(b);
    }
    for (key, conversion) in [
        ("CONVERSION_SKILLS_BATCHES", true),
        ("RECENT_OWNER_BATCHES", false),
    ] {
        for spec in rows(&data[key])? {
            let b = repo.pinned_packet(string(&spec["packet"])?, string(&spec["sha256"])?)?;
            let rev = string(&spec["commit"])?;
            let before = repo.tree(&format!("{rev}^"))?;
            let after = repo.tree(rev)?;
            removed.extend(family::recent(&b, &before, &after, &removed, spec, &data)?);
            if conversion {
                custody::conversion(&mut archives, &b, spec)?;
            } else {
                custody::recent(&mut archives, &b, spec)?;
            }
        }
    }
    ensure!(removed.len() == 3407, "executed removal count changed");
    inventory::separately_observed(&combined, &frozen, &observed, &removed, &data)?;
    let overlay = repo.pinned_packet(
        "EXECUTION_OVERLAY.json",
        string(&data["EXECUTION_APPROVAL_SHA256"])?,
    )?;
    ensure!(
        overlay["baseline"] == ledger["observed_main"],
        "execution baseline changed"
    );
    let mut scoped = paths(rows(&combined["rows"])?)?;
    scoped.extend(paths(rows(&combined["additional_paths"])?)?);
    scoped.extend(strings(&data["PROTECTED"])?);
    scoped.extend(paths(rows(&cutover["paths"])?)?);
    let historical = successors::normalized(repo, &candidate)?;
    overlay::check(&overlay, &observed, &historical, &removed, &scoped, &data)?;
    for (path, id) in &observed {
        if path.starts_with(".csdlc/")
            && ![
                ".csdlc/issues/1223/",
                ".csdlc/v3/issues/1223/",
                ".csdlc/evidence/1223/",
                ".csdlc/transactions/",
            ]
            .iter()
            .any(|p| path.starts_with(p))
        {
            ensure!(
                at(&historical, path) == id,
                "historical lifecycle evidence changed: {path}"
            );
        }
    }
    check_rollback(&original, string(&data["ARCHIVE_SHA256"])?)?;
    custody::codefriend_six(&mut archives, &accepted[0], &original, &data)?;
    custody::codefriend(&mut archives, &accepted[1], &original, &data)?;
    custody::csdlc(&mut archives, &accepted[2], &original, &data)?;
    custody::runtime(&mut archives, &accepted[3], &original, &data)?;
    custody::infrastructure(&mut archives, &accepted[4])?;
    let mut scanned = BTreeMap::new();
    let names = [
        "RETIREMENT_BATCH_01_DDNS",
        "RETIREMENT_BATCH_02_LOG_ARCHIVE",
        "RETIREMENT_BATCH_03_AGENT_CHECKPOINT_ARCHIVE",
    ];
    for component in &components {
        let mut manifest = None;
        for name in names {
            let m = repo.packet(&format!("{name}.json"))?;
            if m["component"] == *component {
                manifest = Some(m);
                break;
            }
        }
        let manifest = manifest.context("component manifest missing")?;
        let mut allowed =
            strings(&manifest["reference_scan"]["retained_historical_reference_files"])?;
        ensure!(allowed.iter().all(|p|[".csdlc/","docs/milestones/","docs/reviews/"].iter().any(|s|p.starts_with(s))||p=="adl/tools/skills/repo-packet-builder/tests/fixtures/issue-520-changed-paths.txt"),"historical classification contains operational path");
        for name in names {
            for suffix in [".json", "_RESULT.json"] {
                allowed.insert(format!("{PACKET}/{name}{suffix}"));
            }
        }
        for name in [
            "DISPOSITION_REGISTER.json",
            "CURRENT_SUCCESSOR_ROWS.jsonl",
            "INDEPENDENT_REPOSITORY_BUILDS.json",
            "REMAINING_COMPONENT_DECISIONS.json",
            "COMBINED_REMAINING_DELTA.json",
        ] {
            allowed.insert(format!("{PACKET}/{name}"));
        }
        for p in [
            "tools/source_retirement/README.md",
            "tools/source_retirement/validator/reviewed-identities.json",
        ] {
            allowed.insert(p.to_owned());
        }
        if component == "infra/aws/runtime/agent-checkpoint-archive" {
            let p = "infra/runtime-v3/runtime-init.toml";
            ensure!(
                repo.text(&["show", &format!("HEAD:{p}")])?
                    .contains("agent-logic-infrastructure"),
                "successor comment missing"
            );
            allowed.insert(p.to_owned());
        }
        let result = Command::new("git")
            .args([
                "--no-optional-locks",
                "grep",
                "-I",
                "-l",
                "-F",
                "-e",
                component,
                "HEAD",
                "--",
            ])
            .current_dir(&repo.root)
            .output()?;
        ensure!(
            matches!(result.status.code(), Some(0 | 1)),
            "reference inspection failed"
        );
        let refs = String::from_utf8(result.stdout)?
            .lines()
            .map(|s| s.strip_prefix("HEAD:").unwrap_or(s).to_owned())
            .collect::<Paths>();
        check_references(&refs, &allowed)?;
        scanned.insert(component.clone(), refs.len());
    }
    let mut report = data["REPORT_TEMPLATE"].clone();
    report["head"] = json!(repo.text(&["rev-parse", "HEAD"])?);
    report["executed_retirement_actions"] = json!(removed.len());
    report["baseline_removals"] = json!(removed.len() - 2);
    report["total_removals"] = json!(removed.len());
    report["direct_reference_files"] = json!(scanned);
    Ok(report)
}
