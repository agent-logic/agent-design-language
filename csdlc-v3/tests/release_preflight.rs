//! PVF: deterministic local Git/CLI contract proof, small CPU/disk, required for #856.
//! Fixtures prove consistency and nonmutation, never live release approval.
use serde_json::{json, Value};
#[path = "support/observation.rs"]
mod observation;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string()
}
fn hash(root: &Path, path: &str) -> String {
    blake3::hash(&fs::read(root.join(path)).unwrap())
        .to_hex()
        .to_string()
}
struct Fixture {
    root: PathBuf,
    request: Value,
    gate: Value,
}
impl Fixture {
    fn new() -> Self {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let root = source.join(format!(
            "csdlc-v3/target/release-preflight-{}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        assert!(Command::new("git")
            .args(["clone", "--quiet", "--shared"])
            .arg(&source)
            .arg(&root)
            .status()
            .unwrap()
            .success());
        git(&root, &["config", "user.name", "Release fixture"]);
        git(&root, &["config", "user.email", "fixture@example.invalid"]);
        let canonical = git(&source, &["rev-parse", "origin/main"]);
        git(
            &root,
            &["update-ref", "refs/remotes/origin/main", &canonical],
        );
        // Copy candidate inputs under development, independent of the implementer's commit state.
        for path in git(&source, &["ls-files"])
            .lines()
            .filter(|p| p.ends_with("Cargo.toml") || p.ends_with("Cargo.lock"))
        {
            fs::copy(source.join(path), root.join(path)).unwrap();
        }
        let inventory = "docs/milestones/v0.92.1/RELEASE_ARTIFACTS.json";
        fs::copy(source.join(inventory), root.join(inventory)).unwrap();
        // This is a historical release fixture, not a claim that today's merge
        // checkout is releasable as v0.92.1. Exercise post-release package drift
        // on every host, then retain only the inventory's declared Cargo inputs.
        let later_package = root.join("fixture-post-release");
        fs::create_dir_all(&later_package).unwrap();
        fs::write(
            later_package.join("Cargo.toml"),
            "[package]\nname = 'fixture-post-release'\nversion = '0.1.0'\n",
        )
        .unwrap();
        fs::write(later_package.join("Cargo.lock"), "version = 4\n").unwrap();
        git(&root, &["add", "fixture-post-release"]);
        let declared: Value =
            serde_json::from_slice(&fs::read(root.join(inventory)).unwrap()).unwrap();
        let mut retained = BTreeSet::new();
        for package in declared["packages"].as_array().unwrap() {
            retained.insert(package["manifest"].as_str().unwrap());
            if let Some(workspace) = package["workspace"].as_str() {
                retained.insert(workspace);
            }
        }
        for lock in declared["lockfiles"].as_array().unwrap() {
            retained.insert(lock.as_str().unwrap());
        }
        for lock in declared["historical_lockfiles"].as_array().unwrap() {
            retained.insert(lock["path"].as_str().unwrap());
        }
        for path in git(&root, &["ls-files"]).lines().filter(|path| {
            (path.ends_with("Cargo.toml") || path.ends_with("Cargo.lock"))
                && !retained.contains(path)
        }) {
            fs::remove_file(root.join(path)).unwrap();
        }
        assert!(!later_package.join("Cargo.toml").exists());
        assert!(!later_package.join("Cargo.lock").exists());
        // Exercise future drift in an existing package, workspace declaration,
        // and shared lock even when the current checkout has not changed them.
        fs::write(
            root.join("adl/Cargo.toml"),
            "[package]\nname = 'future-adl'\nversion = '9.9.9'\n",
        )
        .unwrap();
        fs::write(
            root.join("adl-v2/Cargo.toml"),
            "[workspace.package]\nversion = '9.9.9'\n",
        )
        .unwrap();
        fs::write(
            root.join("adl/Cargo.lock"),
            "version = 4\n[[package]]\nname = 'fixture-later-local'\nversion = '9.9.9'\n",
        )
        .unwrap();
        let mut workspaces: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
        for package in declared["packages"].as_array().unwrap() {
            let manifest = package["manifest"].as_str().unwrap();
            let version = package["version"].as_str().unwrap();
            let version_field = if let Some(workspace) = package["workspace"].as_str() {
                let parent = Path::new(workspace).parent().unwrap();
                let member = Path::new(manifest)
                    .parent()
                    .unwrap()
                    .strip_prefix(parent)
                    .unwrap();
                let entry = workspaces
                    .entry(workspace.to_owned())
                    .or_insert_with(|| (version.to_owned(), Vec::new()));
                assert_eq!(entry.0, version, "fixture workspace versions disagree");
                entry.1.push(member.to_str().unwrap().to_owned());
                "version.workspace = true".to_owned()
            } else {
                format!("version = {}", package["version"])
            };
            fs::write(
                root.join(manifest),
                format!(
                    "[package]\nname = {}\n{version_field}\nedition = \"2021\"\n",
                    package["package"]
                ),
            )
            .unwrap();
        }
        for (path, (version, members)) in workspaces {
            fs::write(
                root.join(path),
                format!(
                    "[workspace]\nmembers = {}\n[workspace.package]\nversion = {}\n",
                    json!(members),
                    json!(version)
                ),
            )
            .unwrap();
        }
        // Lockfile package records are synthetic release-metadata inputs. Do
        // not inherit later local packages through today's shared lockfiles;
        // this fixture does not prove Cargo dependency resolution.
        for lock in declared["lockfiles"].as_array().unwrap() {
            let lock = lock.as_str().unwrap();
            let owner = lock.strip_suffix("Cargo.lock").unwrap().to_owned() + "Cargo.toml";
            let owners: Vec<_> = declared["packages"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|package| {
                    package["manifest"].as_str() == Some(&owner)
                        || package["workspace"].as_str() == Some(&owner)
                })
                .collect();
            assert!(
                !owners.is_empty(),
                "fixture lock has no declared owner: {lock}"
            );
            let mut text = String::from("version = 4\n\n[[package]]\nname = \"fixture-registry-dependency\"\nversion = \"1.0.0\"\nsource = \"registry+https://example.invalid/index\"\n");
            for package in owners {
                text.push_str(&format!(
                    "\n[[package]]\nname = {}\nversion = {}\n",
                    package["package"], package["version"]
                ));
            }
            fs::write(root.join(lock), text).unwrap();
        }
        assert!(!fs::read_to_string(root.join("adl/Cargo.lock"))
            .unwrap()
            .contains("fixture-later-local"));
        fs::copy(
            source.join("adl/tools/release_ceremony.sh"),
            root.join("adl/tools/release_ceremony.sh"),
        )
        .unwrap();
        for issue in [522, 525, 526] {
            fs::write(
                root.join(format!("docs/milestones/v0.92.1/fixture-{issue}.md")),
                format!("# Explicit fixture evidence for {issue}\nNot live acceptance.\n"),
            )
            .unwrap();
        }
        git(&root, &["add", "."]);
        git(
            &root,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "-qm",
                "candidate fixture",
            ],
        );
        let sha = git(&root, &["rev-parse", "HEAD"]);
        let notes = "docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md";
        let notes_digest = hash(&root, notes);
        let gate = json!({"schema":"csdlc.v3.release_gate.v1","repository":"agent-logic/agent-design-language","version":"v0.92.1","candidate_sha":sha,"notes_path":notes,"notes_digest":notes_digest,"inventory_digest":hash(&root,inventory),"status":"ready_for_preflight","evidence":([522,525,526].map(|issue| {let path=format!("docs/milestones/v0.92.1/fixture-{issue}.md");json!({"issue":issue,"digest":hash(&root,&path),"path":path})}))});
        let request = json!({"repository":"agent-logic/agent-design-language","version":"v0.92.1","candidate_sha":sha,"notes_path":notes,"notes_digest":notes_digest,"gate_path":root.join(".git/gate.json"),"gate_digest":""});
        let mut fixture = Self {
            root,
            request,
            gate,
        };
        fixture.write_gate();
        fixture
    }
    fn write_gate(&mut self) {
        fs::write(
            self.root.join(".git/gate.json"),
            serde_json::to_vec(&self.gate).unwrap(),
        )
        .unwrap();
        self.request["gate_digest"] = hash(&self.root, ".git/gate.json").into();
    }
    fn run(&self, expected: Option<&str>) {
        self.run_at(&self.root, expected);
    }
    fn run_at(&self, checkout: &Path, expected: Option<&str>) {
        fs::write(
            self.root.join(".git/request.json"),
            serde_json::to_vec(&self.request).unwrap(),
        )
        .unwrap();
        let refs = git(checkout, &["show-ref"]);
        let status = git(checkout, &["status", "--porcelain"]);
        let candidate = self.root.join(".git/observation-candidate/csdlc");
        if !candidate.exists() {
            observation::install_candidate(&candidate);
        }
        // Same bytes with new stat metadata expose optional Git index refresh,
        // which a content-only working-tree status comparison cannot detect.
        let notes = checkout.join("docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md");
        fs::write(&notes, fs::read(&notes).unwrap()).unwrap();
        let before = observation::inventory(&self.root.join(".git"));
        let output = Command::new(&candidate)
            .current_dir(checkout)
            .args(["release-preflight", "--request"])
            .arg(self.root.join(".git/request.json"))
            .output()
            .unwrap();
        let after = observation::inventory(&self.root.join(".git"));
        let changed: std::collections::BTreeSet<_> = before
            .keys()
            .chain(after.keys())
            .filter(|path| before.get(*path) != after.get(*path))
            .collect();
        assert!(
            changed.is_empty(),
            "release-preflight mutated Git metadata: {changed:?}"
        );
        let report: Value = serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&output.stderr)));
        assert_eq!(output.status.success(), expected.is_none(), "{report}");
        assert_eq!(report["eligible"], expected.is_none());
        assert_eq!(report["mutation_allowed"], false);
        assert_eq!(report["release_authorized"], false);
        if let Some(code) = expected {
            assert!(
                report["findings"][0].as_str().unwrap().contains(code),
                "{report}"
            );
            assert!(!output.stderr.is_empty());
        } else {
            assert!(output.stderr.is_empty());
        }
        assert_eq!(refs, git(checkout, &["show-ref"]));
        assert_eq!(status, git(checkout, &["status", "--porcelain"]));
    }
    fn commit_input(&mut self) {
        git(&self.root, &["add", "."]);
        git(
            &self.root,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "-qm",
                "changed candidate",
            ],
        );
        let sha = git(&self.root, &["rev-parse", "HEAD"]);
        self.request["candidate_sha"] = sha.clone().into();
        self.gate["candidate_sha"] = sha.into();
        self.write_gate();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
#[test]
fn exact_candidate_preflight_and_negative_matrix() {
    let mut f = Fixture::new();
    f.run(None);
    // Isolation must not weaken the production omission guard. A new package
    // introduced after fixture setup is still rejected without an inventory row.
    let unlisted = f.root.join("fixture-unlisted");
    fs::create_dir_all(&unlisted).unwrap();
    fs::write(
        unlisted.join("Cargo.toml"),
        "[package]\nname = 'fixture-unlisted'\nversion = '0.1.0'\n",
    )
    .unwrap();
    f.commit_input();
    f.run(Some("release_inventory_omits_manifest"));
    fs::remove_dir_all(unlisted).unwrap();
    f.commit_input();
    f.run(None);
    let shared_lock = f.root.join("adl/Cargo.lock");
    let declared_lock = fs::read_to_string(&shared_lock).unwrap();
    fs::write(
        &shared_lock,
        format!("{declared_lock}\n[[package]]\nname = \"fixture-unlisted\"\nversion = \"0.1.0\"\n"),
    )
    .unwrap();
    f.commit_input();
    f.run(Some("unlisted_local_lock_package"));
    fs::write(shared_lock, declared_lock).unwrap();
    f.commit_input();
    f.run(None);
    let linked = f.root.with_extension("linked");
    git(
        &f.root,
        &[
            "worktree",
            "add",
            "--quiet",
            "--detach",
            linked.to_str().unwrap(),
            "HEAD",
        ],
    );
    f.run_at(&linked, None);
    let gate_bytes = fs::read(f.root.join(".git/gate.json")).unwrap();
    fs::remove_file(f.root.join(".git/gate.json")).unwrap();
    f.run_at(&linked, Some("native_v3_gate_missing"));
    fs::write(f.root.join(".git/gate.json"), b"corrupt gate").unwrap();
    f.request["gate_digest"] = hash(&f.root, ".git/gate.json").into();
    f.run_at(&linked, Some("native_v3_gate_malformed"));
    let original_gate = f.gate.clone();
    f.gate["status"] = "pending_qualification".into();
    f.write_gate();
    f.run_at(&linked, Some("native_v3_gate_stale_or_ineligible"));
    f.gate = original_gate;
    f.write_gate();
    assert_eq!(fs::read(f.root.join(".git/gate.json")).unwrap(), gate_bytes);
    git(
        &f.root,
        &["worktree", "remove", "--force", linked.to_str().unwrap()],
    );
    let original = f.request.clone();
    for (key, value, error) in [
        ("candidate_sha", "abc", "full_candidate_sha_required"),
        ("candidate_sha", &"0".repeat(40), "candidate_head_mismatch"),
        ("notes_digest", "wrong", "release_notes_digest_mismatch"),
        ("notes_path", "../notes", "release_notes_path_mismatch"),
        ("gate_digest", "wrong", "native_v3_gate_digest_mismatch"),
        ("repository", "wrong/repo", "unsupported_release_identity"),
    ] {
        f.request[key] = value.into();
        f.run(Some(error));
        f.request = original.clone();
    }
    fs::remove_file(f.root.join(".git/gate.json")).unwrap();
    f.run(Some("native_v3_gate_missing"));
    f.write_gate();
    fs::write(f.root.join(".git/gate.json"), b"malformed").unwrap();
    f.request["gate_digest"] = hash(&f.root, ".git/gate.json").into();
    f.run(Some("native_v3_gate_malformed"));
    f.write_gate();
    let original_gate = f.gate.clone();
    for (key, value) in [
        ("candidate_sha", "stale"),
        ("version", "v0.92.0"),
        ("repository", "wrong/repo"),
        ("status", "blocked"),
        ("notes_digest", "changed"),
    ] {
        f.gate[key] = value.into();
        f.write_gate();
        f.run(Some("native_v3_gate_stale_or_ineligible"));
        f.gate = original_gate.clone();
        f.write_gate();
    }
    f.gate["evidence"] = json!([]);
    f.write_gate();
    f.run(Some("release_tail_evidence_required"));
    f.gate = original_gate.clone();
    f.write_gate();
    f.gate["evidence"][0]["digest"] = "wrong".into();
    f.write_gate();
    f.run(Some("release_tail_evidence_mismatch"));
    f.gate = original_gate.clone();
    f.write_gate();
    f.gate["inventory_digest"] = "wrong".into();
    f.write_gate();
    f.run(Some("release_inventory_digest_mismatch"));
    f.gate = original_gate;
    f.write_gate();
    let manifest = f.root.join("adl/Cargo.toml");
    let bytes = fs::read_to_string(&manifest).unwrap();
    fs::write(&manifest, bytes.replace("0.92.1", "0.92.0")).unwrap();
    f.run(Some("candidate_checkout_dirty"));
    f.commit_input();
    f.run(Some("release_package_version_mismatch"));
    fs::write(&manifest, bytes).unwrap();
    f.commit_input();
    f.run(None);
    let lock = f.root.join("adl/Cargo.lock");
    let bytes = fs::read_to_string(&lock).unwrap();
    fs::write(
        &lock,
        bytes.replacen(
            "name = \"adl\"\nversion = \"0.92.1\"",
            "name = \"adl\"\nversion = \"0.92.0\"",
            1,
        ),
    )
    .unwrap();
    f.commit_input();
    f.run(Some("release_lock_version_mismatch"));
    fs::write(&lock, bytes).unwrap();
    f.commit_input();
    f.run(None);
    let lock = f.root.join("adl/Cargo.lock");
    let lock_bytes = fs::read_to_string(&lock).unwrap();
    fs::write(&lock, "version = 4\npackage = []\n").unwrap();
    f.commit_input();
    f.run(Some("release_lock_owner_missing"));
    let without_owner = lock_bytes
        .split("[[package]]")
        .filter(|part| !part.trim_start().starts_with("name = \"adl\"\n"))
        .collect::<Vec<_>>()
        .join("[[package]]");
    fs::write(&lock, without_owner).unwrap();
    f.commit_input();
    f.run(Some("release_lock_owner_missing"));
    let sourced_owner = lock_bytes.replacen(
        "name = \"adl\"\nversion = \"0.92.1\"",
        "name = \"adl\"\nversion = \"0.92.1\"\nsource = \"registry+https://example.invalid\"",
        1,
    );
    fs::write(&lock, sourced_owner).unwrap();
    f.commit_input();
    f.run(Some("release_lock_owner_missing"));
    fs::write(&lock, lock_bytes).unwrap();
    f.commit_input();
    f.run(None);
    let notes = f.root.join(f.request["notes_path"].as_str().unwrap());
    fs::write(notes, "changed notes").unwrap();
    f.commit_input();
    f.run(Some("release_notes_digest_mismatch"));
    git(&f.root, &["update-ref", "-d", "refs/remotes/origin/main"]);
    f.run(Some("native_v3_authority_suspended"));
}
