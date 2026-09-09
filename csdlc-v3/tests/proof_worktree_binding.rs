//! PVF: deterministic local integration; required identity/confinement proof;
//! medium CPU/disk profile, no credentials or network. Fixtures model native
//! bound records directly; they do not claim to validate the separate binder.
use serde_json::{json, Value};
#[cfg(unix)]
use std::os::unix::fs::{symlink, PermissionsExt};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().into()
}
fn write(root: &Path, name: &str, value: &Value) {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
}
fn hash(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
struct Fixture {
    base: PathBuf,
    primary: PathBuf,
    worktree: PathBuf,
    binary: PathBuf,
    request: Value,
}
impl Fixture {
    fn new() -> Self {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let base = source
            .join("csdlc-v3/target/proof-binding-tests")
            .join(format!("{}", std::process::id()));
        if base.exists() {
            fs::remove_dir_all(&base).unwrap();
        }
        fs::create_dir_all(&base).unwrap();
        let base = base.canonicalize().unwrap();
        let primary = base.join("primary");
        git(
            &base,
            &[
                "clone",
                "--shared",
                "--no-checkout",
                source.to_str().unwrap(),
                primary.to_str().unwrap(),
            ],
        );
        let baseline = git(&source, &["rev-parse", "HEAD"]);
        git(&primary, &["checkout", "-B", "main", &baseline]);
        git(&primary, &["config", "user.name", "Proof Test"]);
        git(&primary, &["config", "user.email", "proof@example.invalid"]);
        let parent = base.join("worktrees");
        fs::create_dir_all(&parent).unwrap();
        write(
            &primary,
            ".adl/worktree-policy.json",
            &json!({"schema":"adl.worktree_policy.v1","required_parent":parent}),
        );
        git(&primary, &["add", "-f", ".adl/worktree-policy.json"]);
        git(&primary, &["commit", "-m", "fixture policy"]);
        git(
            &primary,
            &["update-ref", "refs/remotes/origin/main", "HEAD"],
        );
        git(
            &primary,
            &[
                "remote",
                "set-url",
                "origin",
                "https://github.com/agent-logic/agent-design-language.git",
            ],
        );
        let worktree = parent.join("issue-762");
        git(
            &primary,
            &[
                "worktree",
                "add",
                "-b",
                "codex/762-fixture",
                worktree.to_str().unwrap(),
                "HEAD",
            ],
        );
        let binary = primary.join(".adl/bin/native-v3/csdlc");
        fs::create_dir_all(binary.parent().unwrap()).unwrap();
        fs::copy(env!("CARGO_BIN_EXE_csdlc"), &binary).unwrap();
        // One stable driver binary stays in primary for every invocation.
        let issue_dir = worktree.join(".csdlc/issues/762");
        fs::create_dir_all(issue_dir.join("cards")).unwrap();
        let mut index = json!({"schema":"csdlc.v3.local_state.v1","issue":762,"phase":"bound","generation":1,"repository":"agent-logic/agent-design-language","branch":"codex/762-fixture","worktree":worktree,"template_registry_version":"1.0.4","operational_authority":true});
        let bound = json!({"schema":"csdlc.v3.binding.v1","issue":762,"branch":"codex/762-fixture","worktree":worktree});
        write(&issue_dir, "binding.json", &bound);
        let mut hasher = blake3::Hasher::new();
        hasher.update(&serde_json::to_vec(&index).unwrap());
        for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            for suffix in ["values.json", "md"] {
                let bytes = if suffix == "md" {
                    format!("# {card} fixture\n").into_bytes()
                } else {
                    serde_json::to_vec(&json!({"issue":762,"card":card})).unwrap()
                };
                fs::write(issue_dir.join(format!("cards/{card}.{suffix}")), &bytes).unwrap();
                hasher.update(&bytes);
            }
        }
        hasher.update(&fs::read(issue_dir.join("binding.json")).unwrap());
        let digest = hasher.finalize().to_hex().to_string();
        index["digest"] = json!(digest);
        write(&issue_dir, "index.json", &index);
        write(
            &worktree,
            ".csdlc/evidence/762/input/request.json",
            &json!({"issue":762}),
        );
        write(
            &worktree,
            ".csdlc/evidence/762/input/source.json",
            &json!({"result":"pass"}),
        );
        let evidence_digest =
            hash(&fs::read(worktree.join(".csdlc/evidence/762/input/source.json")).unwrap());
        let observer = worktree.join(".csdlc/evidence/762/input/observer");
        fs::write(&observer, "#!/bin/sh\nprintf '%s\\n' '{\"schema\":\"csdlc.v3.operational_local.v1\",\"command\":\"doctor\",\"result\":{\"issue\":762,\"phase\":\"bound\"}}'\n").unwrap();
        #[cfg(unix)]
        fs::set_permissions(&observer, fs::Permissions::from_mode(0o755)).unwrap();
        let request = json!({"issue":762,"repository":"agent-logic/agent-design-language","binding":{"worktree":worktree,"branch":"codex/762-fixture","exact_head":git(&worktree,&["rev-parse","HEAD"]),"git_common_dir":primary.join(".git"),"generation":1,"lifecycle_digest":digest},"evidence_root":worktree,"proof":{"manifest_id":"bound-proof","lane":"worktree","deterministic":true,"evidence_ref":".csdlc/evidence/762/input/source.json","evidence_digest":evidence_digest,"observed_digest":evidence_digest,"stale":false,"normalization":"doctor_issue_phase_v1","command":{"generation":"v3","binary_ref":".csdlc/evidence/762/input/observer","argv":["--request",".csdlc/evidence/762/input/request.json"],"request_ref":".csdlc/evidence/762/input/request.json","timeout_millis":1000,"side_effect_boundary_refs":[".csdlc/issues/762"],"provider_side_effects":false}}});
        Self {
            base,
            primary,
            worktree,
            binary,
            request,
        }
    }
    fn run(&self, route: &str, cwd: &Path, request: &Value) -> Value {
        let path = self.base.join("invocation.json");
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        let out = Command::new(&self.binary)
            .current_dir(cwd)
            .args([route, "--request"])
            .arg(&path)
            .output()
            .unwrap();
        let value: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|_| {
            panic!(
                "stdout={} stderr={}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
        });
        assert_eq!(out.status.success(), value["status"] == "ready");
        value
    }
    fn denied(&self, route: &str, cwd: &Path, request: &Value, code: &str) {
        let primary_before = snapshot(&self.primary);
        let worktree_before = snapshot(&self.worktree);
        let value = self.run(route, cwd, request);
        assert_eq!(value["status"], "blocked", "{value}");
        assert_eq!(value["performed_mutation"], false);
        assert!(
            value["findings"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v["code"] == code),
            "{value}"
        );
        assert_eq!(
            primary_before,
            snapshot(&self.primary),
            "rejected request wrote primary"
        );
        assert_eq!(
            worktree_before,
            snapshot(&self.worktree),
            "rejected request wrote worktree"
        );
    }
}
// Snapshot issue-owned mutation surfaces, including existing binary bytes and
// symlink targets. Git's read-only inspections must not change these surfaces.
fn snapshot(root: &Path) -> Vec<(String, String)> {
    fn walk(root: &Path, path: &Path, out: &mut Vec<(String, String)>) {
        let Ok(meta) = path.symlink_metadata() else {
            return;
        };
        let name = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if meta.file_type().is_symlink() {
            out.push((
                name,
                format!("link:{}", fs::read_link(path).unwrap().display()),
            ));
        } else if meta.is_file() {
            out.push((name, hash(&fs::read(path).unwrap())));
        } else {
            out.push((name, "directory".into()));
            for entry in fs::read_dir(path).unwrap() {
                walk(root, &entry.unwrap().path(), out);
            }
        }
    }
    let mut out = Vec::new();
    for name in [".adl/bin", ".csdlc/evidence/762", ".csdlc/issues/762"] {
        walk(root, &root.join(name), &mut out);
    }
    out.sort();
    out
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

#[test]
fn stable_binary_authenticates_invoking_worktree_before_every_operational_write() {
    let f = Fixture::new();
    f.denied("proof", &f.primary, &f.request, "proof_worktree_mismatch");
    let mut primary = f.request.clone();
    primary["binding"]["worktree"] = json!(f.primary);
    primary["evidence_root"] = json!(f.primary);
    f.denied(
        "proof",
        &f.primary,
        &primary,
        "proof_primary_checkout_denied",
    );
    for (field, value, code) in [
        ("branch", json!("codex/wrong"), "proof_branch_mismatch"),
        ("exact_head", json!("0".repeat(40)), "proof_head_mismatch"),
        ("generation", json!(999), "proof_lifecycle_stale"),
        (
            "lifecycle_digest",
            json!("0".repeat(64)),
            "proof_lifecycle_stale",
        ),
        ("git_common_dir", json!(f.base), "proof_common_dir_mismatch"),
        ("worktree", json!(f.primary), "proof_worktree_mismatch"),
    ] {
        let mut q = f.request.clone();
        q["binding"][field] = value;
        f.denied("proof", &f.worktree, &q, code);
    }
    let mut q = f.request.clone();
    q["binding"] = Value::Null;
    f.denied("install", &f.worktree, &q, "proof_binding_missing");
    q = f.request.clone();
    q["evidence_root"] = json!(f.primary);
    f.denied("proof", &f.worktree, &q, "proof_worktree_mismatch");
    for route in ["shadow", "soak"] {
        f.denied(route, &f.worktree, &f.request, "historical_route_disabled");
    }
    let card = f.worktree.join(".csdlc/issues/762/cards/sip.md");
    let saved = fs::read(&card).unwrap();
    fs::write(&card, "tampered").unwrap();
    f.denied("proof", &f.worktree, &f.request, "proof_lifecycle_stale");
    fs::write(&card, saved).unwrap();
    #[cfg(unix)]
    {
        let escape = f.worktree.join(".csdlc/evidence/762/v3-proof");
        fs::create_dir_all(escape.parent().unwrap()).unwrap();
        symlink(&f.primary, &escape).unwrap();
        f.denied("proof", &f.worktree, &f.request, "proof_path_symlink");
        fs::remove_file(escape).unwrap();
    }
    let registration =
        PathBuf::from(git(&f.worktree, &["rev-parse", "--absolute-git-dir"])).join("gitdir");
    let saved_registration = fs::read(&registration).unwrap();
    fs::write(
        &registration,
        format!("{}/missing/.git\n", f.base.display()),
    )
    .unwrap();
    f.denied(
        "proof",
        &f.worktree,
        &f.request,
        "proof_registration_mismatch",
    );
    fs::write(&registration, saved_registration).unwrap();
    let selector = f.worktree.join("csdlc-v3/operator/authority-selector.json");
    let saved_selector = fs::read(&selector).unwrap();
    fs::write(&selector, "{}").unwrap();
    f.denied("proof", &f.worktree, &f.request, "proof_authority_invalid");
    fs::write(&selector, saved_selector).unwrap();
    let head = git(&f.worktree, &["rev-parse", "HEAD"]);
    git(&f.worktree, &["checkout", "--detach", &head]);
    f.denied("proof", &f.worktree, &f.request, "proof_branch_mismatch");
    git(&f.worktree, &["checkout", "codex/762-fixture"]);
    let mut wrong_issue = f.request.clone();
    wrong_issue["issue"] = json!(999999);
    f.denied("proof", &f.worktree, &wrong_issue, "proof_issue_unbound");
    let mut stale = f.request.clone();
    stale["proof"]["stale"] = json!(true);
    f.denied("proof", &f.worktree, &stale, "proof_evidence_stale");
    for (field, value, code) in [
        (
            "deterministic",
            json!(false),
            "proof_lane_not_deterministic",
        ),
        (
            "observed_digest",
            json!("0".repeat(64)),
            "proof_observed_digest_mismatch",
        ),
    ] {
        let mut request = f.request.clone();
        request["proof"][field] = value;
        f.denied("proof", &f.worktree, &request, code);
    }
    let before = snapshot(&f.primary);
    let result = f.run("proof", &f.worktree, &f.request);
    assert_eq!(result["status"], "ready", "{result}");
    assert_eq!(result["performed_mutation"], true);
    assert!(f
        .worktree
        .join(".csdlc/evidence/762/v3-proof/bound-proof.json")
        .is_file());
    assert_eq!(before, snapshot(&f.primary));
    // A second invocation from a nested directory resolves the same registered checkout.
    let result = f.run(
        "proof",
        &f.worktree.join(".csdlc/evidence/762/input"),
        &f.request,
    );
    assert_eq!(result["status"], "ready", "{result}");
    let artifact = fs::read(&f.binary).unwrap();
    fs::write(
        f.worktree.join(".csdlc/evidence/762/input/csdlc"),
        &artifact,
    )
    .unwrap();
    write(
        &f.worktree,
        ".csdlc/evidence/762/input/provenance.json",
        &json!({"schema":"csdlc.v3.install_provenance.v1","source":"fixture exact build"}),
    );
    let selector_ref = ".csdlc/evidence/762/input/selector.json";
    fs::copy(
        f.worktree.join("csdlc-v3/operator/authority-selector.json"),
        f.worktree.join(selector_ref),
    )
    .unwrap();
    let selector_digest = hash(&fs::read(f.worktree.join(selector_ref)).unwrap());
    let approval = json!({"schema":"csdlc.v3.cutover_approval.v1","authority_issue":505,"decision":"approved","repository":"agent-logic/agent-design-language","exact_head":f.request["binding"]["exact_head"],"selected_binary_digest":hash(&artifact),"selector_metadata_digest":selector_digest});
    write(
        &f.worktree,
        ".csdlc/evidence/762/input/approval.json",
        &approval,
    );
    let mut install = f.request.clone();
    install["proof"] = Value::Null;
    install["cutover_issue"] = json!(505);
    install["install"] = json!({"artifact_name":"csdlc","artifact_ref":".csdlc/evidence/762/input/csdlc","source_provenance_ref":".csdlc/evidence/762/input/provenance.json","selector_metadata_ref":selector_ref,"source_provenance":"fixture exact build","selected_binary_digest":hash(&artifact),"observed_binary_digest":hash(&artifact),"selector_metadata_digest":selector_digest,"destination":".adl/bin/csdlc","stable_destination":true,"executes_install":true,"exact_head":f.request["binding"]["exact_head"],"cutover_approval_ref":".csdlc/evidence/762/input/approval.json","cutover_approval_digest":hash(&fs::read(f.worktree.join(".csdlc/evidence/762/input/approval.json")).unwrap())});
    for (field, value, code) in [
        (
            "artifact_name",
            json!("other"),
            "install_artifact_not_one_binary",
        ),
        (
            "observed_binary_digest",
            json!("0".repeat(64)),
            "install_observed_binary_digest_mismatch",
        ),
        (
            "source_provenance",
            json!("wrong source"),
            "install_source_provenance_mismatch",
        ),
        (
            "cutover_approval_ref",
            Value::Null,
            "install_typed_authority_missing",
        ),
        (
            "cutover_approval_digest",
            json!("0".repeat(64)),
            "install_cutover_approval_digest_mismatch",
        ),
        (
            "destination",
            json!("../primary/escape"),
            "proof_path_invalid",
        ),
    ] {
        let mut request = install.clone();
        request["install"][field] = value;
        f.denied("install", &f.worktree, &request, code);
    }
    #[cfg(unix)]
    {
        let bin = f.worktree.join(".adl/bin");
        fs::create_dir_all(&bin).unwrap();
        symlink(&f.binary, bin.join("csdlc")).unwrap();
        f.denied("install", &f.worktree, &install, "proof_path_symlink");
        fs::remove_file(bin.join("csdlc")).unwrap();
    }
    let before = snapshot(&f.primary);
    let result = f.run("install", &f.worktree, &install);
    assert_eq!(result["status"], "ready", "{result}");
    assert_eq!(result["performed_mutation"], true);
    assert_eq!(
        fs::read(f.worktree.join(".adl/bin/csdlc")).unwrap(),
        artifact
    );
    assert_eq!(before, snapshot(&f.primary));
}
