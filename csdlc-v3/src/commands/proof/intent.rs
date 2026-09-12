//! Bounded Cargo validator execution owned by the existing proof command.
//! A marker alone is never proof: require an actual nonzero successful test result.
use super::*;
use crate::application::intent::{Context, Validator};
use serde_json::{json, Value};

pub fn execute(context: &Context, validators: &[Validator]) -> Result<Value, String> {
    #[cfg(not(unix))]
    {
        let _ = (context, validators);
        return Err("intent_validator_platform_not_supported".into());
    }
    #[cfg(unix)]
    {
        execute_unix(context, validators)
    }
}

#[cfg(unix)]
fn execute_unix(context: &Context, validators: &[Validator]) -> Result<Value, String> {
    let binding = ProofWorktreeBinding {
        worktree: context.root.clone(),
        branch: context.branch.clone(),
        exact_head: context.head.clone(),
        git_common_dir: PathBuf::from(git_read(
            &context.root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?),
        generation: context.index["generation"]
            .as_u64()
            .ok_or("intent_issue_generation_missing")?,
        lifecycle_digest: context.index["digest"]
            .as_str()
            .ok_or("intent_issue_digest_missing")?
            .into(),
    };
    let request = ProofRouteRequest {
        issue: context.issue,
        repository: context.repository.clone(),
        binding: Some(binding),
        cutover_issue: None,
        operator_approval: None,
        evidence_root: Some(context.root.to_string_lossy().into_owned()),
        proof: None,
        shadow: None,
        soak: None,
        install: None,
    };
    authorize_worktree(&request, Some(&context.root)).map_err(|finding| finding.code)?;
    if validators.is_empty() {
        return Err("intent_validators_missing".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    for validator in validators {
        if validator.program != "cargo"
            || validator.args.first().map(String::as_str) != Some("test")
            || validator.success_marker != "test result: ok."
            || !ids.insert(&validator.id)
        {
            return Err("intent_validator_not_admitted".into());
        }
        if !(1..=300).contains(&validator.timeout_seconds) {
            return Err("intent_validator_timeout_not_admitted".into());
        }
        safe_component(&validator.id).map_err(|finding| finding.code)?;
        let mut args = validator.args.iter().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--offline" | "--locked" | "--lib" | "--all-targets" => {}
                "--manifest-path" => {
                    let path = args.next().ok_or("intent_validator_manifest_missing")?;
                    resolve_repo_path(&context.root, path, true).map_err(|finding| finding.code)?;
                }
                "--test" => {
                    safe_component(args.next().ok_or("intent_validator_test_missing")?)
                        .map_err(|finding| finding.code)?;
                }
                _ => return Err("intent_validator_argument_not_admitted".into()),
            }
        }
    }
    let input_digest = tracked_input_digest(&context.root, validators)?;
    let mut outcomes = Vec::new();
    let mut execution_finding: Option<String> = None;
    for validator in validators {
        let admission = context.fresh().and_then(|()| {
            authorize_worktree(&request, Some(&context.root))
                .map(|_| ())
                .map_err(|finding| finding.code.to_owned())
        });
        if let Err(code) = admission {
            if outcomes.is_empty() {
                return Err(code);
            }
            execution_finding = Some(code);
            break;
        }
        let execution = match run_validator(&context.root, validator) {
            Ok(execution) => execution,
            Err(code) if outcomes.is_empty() => return Err(code),
            Err(code) => {
                execution_finding = Some(code);
                break;
            }
        };
        let text = String::from_utf8_lossy(&execution.stdout.bytes);
        let tests_passed = text
            .lines()
            .filter_map(|line| {
                line.trim()
                    .strip_prefix("test result: ")
                    .and_then(|value| value.split_once(". ").map(|(_, summary)| summary))
                    .and_then(|value| value.split_once(" passed;"))
                    .and_then(|(count, _)| count.parse::<u64>().ok())
            })
            .sum::<u64>();
        let tests_failed = text
            .lines()
            .filter(|line| line.trim().starts_with("test result: "))
            .filter_map(|line| {
                line.split_once(" passed; ")
                    .and_then(|(_, rest)| rest.split_once(" failed;"))
                    .and_then(|(count, _)| count.parse::<u64>().ok())
            })
            .sum::<u64>();
        let passed = execution.success
            && !execution.timed_out
            && !execution.cancelled
            && execution.cleanup_complete
            && !execution.stdout.truncated
            && !execution.stderr.truncated
            && tests_passed > 0;
        outcomes.push(json!({"id":validator.id,"program":validator.program,"args":validator.args,"timeout_seconds":validator.timeout_seconds,
            "compiler_artifacts":compiler_artifacts(&context.root,&text),"executed_args":validator.args.iter().cloned().chain(std::iter::once("--message-format=json".to_owned())).collect::<Vec<_>>(),"input_digest":input_digest,"exit_code":execution.exit_code,"tests_passed":tests_passed,"tests_failed":tests_failed,
            "stdout_digest":blake3::hash(&execution.stdout.bytes).to_hex().to_string(),"stderr_digest":blake3::hash(&execution.stderr.bytes).to_hex().to_string(),
            "stdout_evidence":diagnostic_excerpt(&context.root,&execution.stdout.bytes),"stderr_evidence":diagnostic_excerpt(&context.root,&execution.stderr.bytes),
            "timed_out":execution.timed_out,"cancelled":execution.cancelled,"cleanup_complete":execution.cleanup_complete,
            "truncated":execution.stdout.truncated||execution.stderr.truncated,"passed":passed,"elapsed_ms":execution.elapsed_ms}));
        if !passed {
            break;
        }
    }
    let input_revalidation = context
        .fresh_integrity()
        .and_then(|()| {
            outcomes.iter().try_for_each(|record| {
                compiler_inputs_tracked(&context.root, &record["compiler_artifacts"])
            })
        })
        .and_then(|()| tracked_input_digest(&context.root, validators));
    let unchanged = input_revalidation
        .as_ref()
        .is_ok_and(|digest| digest == &input_digest);
    let input_revalidation = match input_revalidation {
        Ok(digest) => {
            json!({"status":if unchanged {"unchanged"} else {"changed"},"input_digest":digest})
        }
        Err(code) => json!({"status":"failed","finding":code}),
    };
    let passed = unchanged
        && outcomes.len() == validators.len()
        && outcomes.iter().all(|value| value["passed"] == true);
    let mut receipt = json!({"schema":"csdlc.v3.intent_proof.v1","issue":context.issue,"repository":context.repository,"head":context.head,"issue_digest":context.index["digest"],"validators":outcomes,"status":if passed{"passed"}else{"failed"},"inputs_unchanged":unchanged,"input_revalidation":input_revalidation,"execution_finding":execution_finding});
    receipt["payload_digest"] = blake3::hash(&canonical_json(&receipt))
        .to_hex()
        .to_string()
        .into();
    let reference = format!(".csdlc/evidence/{}/intent-proof.json", context.issue);
    if let Err(finding) = write_canonical_evidence(&request, &reference, &receipt) {
        return Err(
            json!({"schema":"csdlc.v3.intent_proof_result.v1","read_only":false,
            "performed_mutation":true,"operational_authority":false,"status":"failed",
            "proof":receipt,"evidence_ref":reference,"evidence_persisted":false,
            "persistence_finding":finding.code})
            .to_string(),
        );
    }
    let result = json!({"schema":"csdlc.v3.intent_proof_result.v1","read_only":false,"performed_mutation":true,"operational_authority":true,"status":if passed{"completed"}else{"failed"},"proof":receipt,"evidence_ref":reference,"evidence_persisted":true});
    if passed {
        Ok(result)
    } else {
        Err(result.to_string())
    }
}

fn diagnostic_excerpt(root: &Path, bytes: &[u8]) -> Value {
    let mut excerpt = String::new();
    let mut redacted_lines = 0usize;
    let mut truncated = false;
    for line in String::from_utf8_lossy(bytes).lines() {
        let lower = line.to_ascii_lowercase();
        let sensitive = [
            "authorization",
            "password",
            "secret",
            "token",
            "credential",
            "api_key",
            "api-key",
            "ghp_",
            "github_pat_",
            "sk-",
        ]
        .iter()
        .any(|key| lower.contains(key));
        let mut line = if sensitive {
            redacted_lines += 1;
            "[REDACTED SENSITIVE LINE]".to_owned()
        } else {
            line.replace(root.to_string_lossy().as_ref(), "<repository>")
        };
        if let Some(home) = std::env::var_os("HOME").filter(|home| !home.is_empty()) {
            line = line.replace(home.to_string_lossy().as_ref(), "<home>");
        }
        line.retain(|c| !c.is_control() || c == '\t');
        if excerpt.len() + line.len() + 1 > 32 * 1024 {
            truncated = true;
            break;
        }
        excerpt.push_str(&line);
        excerpt.push('\n');
    }
    json!({"schema":"csdlc.v3.validator_diagnostic_excerpt.v1","captured_bytes":bytes.len(),"excerpt":excerpt,"redacted_lines":redacted_lines,"excerpt_truncated":truncated,"lossless":false,"policy":"bounded UTF-8 diagnostic excerpt; sensitive-key lines omitted and repository/HOME paths replaced; exact captured-byte digest is recorded separately"})
}

#[cfg(unix)]
#[derive(Default)]
struct Capture {
    bytes: Vec<u8>,
    truncated: bool,
    eof: bool,
}

#[cfg(unix)]
impl Capture {
    fn drain(&mut self, reader: &mut std::os::unix::net::UnixStream) -> std::io::Result<()> {
        let mut buffer = [0u8; 8192];
        // Bound each drain so a continuously writing child cannot starve its deadline.
        for _ in 0..8 {
            match reader.read(&mut buffer) {
                Ok(0) => {
                    self.eof = true;
                    break;
                }
                Ok(count) => {
                    let keep = (1024 * 1024usize)
                        .saturating_sub(self.bytes.len())
                        .min(count);
                    self.bytes.extend_from_slice(&buffer[..keep]);
                    self.truncated |= keep < count;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}

#[cfg(unix)]
struct ValidatorExecution {
    stdout: Capture,
    stderr: Capture,
    exit_code: Option<i32>,
    success: bool,
    timed_out: bool,
    cancelled: bool,
    cleanup_complete: bool,
    elapsed_ms: u128,
}

#[cfg(unix)]
struct OwnedValidator {
    child: std::process::Child,
    pgid: i32,
    disarmed: bool,
}

#[cfg(unix)]
impl OwnedValidator {
    fn terminate(&self) -> bool {
        unsafe extern "C" {
            fn kill(pid: i32, signal: i32) -> i32;
        }
        // process_group(0) created this exact child-owned process group. Never
        // enumerate host processes or signal a caller-provided PID/group.
        let result = unsafe { kill(-self.pgid, 9) };
        result == 0 || std::io::Error::last_os_error().raw_os_error() == Some(3)
    }
}
#[cfg(unix)]
impl Drop for OwnedValidator {
    fn drop(&mut self) {
        if !self.disarmed {
            let _ = self.terminate();
            let _ = self.child.try_wait();
        }
    }
}

#[cfg(unix)]
static VALIDATOR_CANCELLATION: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
#[cfg(unix)]
static VALIDATOR_SIGNAL_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
#[cfg(unix)]
extern "C" fn cancel_validator(_signal: i32) {
    VALIDATOR_CANCELLATION.store(true, std::sync::atomic::Ordering::SeqCst);
}
#[cfg(unix)]
unsafe extern "C" {
    fn signal(number: i32, handler: usize) -> usize;
}

#[cfg(unix)]
struct CancellationGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
    previous_interrupt: usize,
    previous_terminate: usize,
}
#[cfg(unix)]
impl CancellationGuard {
    fn install() -> Result<Self, String> {
        let lock = VALIDATOR_SIGNAL_LOCK
            .try_lock()
            .map_err(|_| "intent_validator_signal_scope_busy")?;
        VALIDATOR_CANCELLATION.store(false, std::sync::atomic::Ordering::SeqCst);
        let handler = cancel_validator as *const () as usize;
        // POSIX signal dispositions are scoped to this serialized owner invocation.
        // The handler only performs a lock-free atomic store; cleanup stays in Rust.
        let previous_interrupt = unsafe { signal(2, handler) };
        if previous_interrupt == usize::MAX {
            return Err("intent_validator_signal_setup_failed".into());
        }
        let previous_terminate = unsafe { signal(15, handler) };
        if previous_terminate == usize::MAX {
            unsafe { signal(2, previous_interrupt) };
            return Err("intent_validator_signal_setup_failed".into());
        }
        Ok(Self {
            _lock: lock,
            previous_interrupt,
            previous_terminate,
        })
    }
    fn cancelled(&self) -> bool {
        VALIDATOR_CANCELLATION.load(std::sync::atomic::Ordering::SeqCst)
    }
}
#[cfg(unix)]
impl Drop for CancellationGuard {
    fn drop(&mut self) {
        unsafe {
            signal(15, self.previous_terminate);
            signal(2, self.previous_interrupt);
        }
    }
}

#[cfg(unix)]
fn run_validator(root: &Path, validator: &Validator) -> Result<ValidatorExecution, String> {
    use std::os::{
        fd::OwnedFd,
        unix::{net::UnixStream, process::CommandExt},
    };
    let start = Instant::now();
    let cancellation = CancellationGuard::install()?;
    let (mut stdout, stdout_writer) =
        UnixStream::pair().map_err(|_| "intent_validator_capture_failed")?;
    let (mut stderr, stderr_writer) =
        UnixStream::pair().map_err(|_| "intent_validator_capture_failed")?;
    stdout
        .set_nonblocking(true)
        .map_err(|_| "intent_validator_capture_failed")?;
    stderr
        .set_nonblocking(true)
        .map_err(|_| "intent_validator_capture_failed")?;
    let mut command = Command::new("cargo");
    command
        .current_dir(root)
        .args(&validator.args)
        .arg("--message-format=json")
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("HOME", std::env::var_os("HOME").unwrap_or_default())
        .env("CARGO_TARGET_DIR", root.join("target/intent-validation"))
        .stdin(Stdio::null())
        .stdout(Stdio::from(OwnedFd::from(stdout_writer)))
        .stderr(Stdio::from(OwnedFd::from(stderr_writer)))
        .process_group(0);
    let child = command
        .spawn()
        .map_err(|_| "intent_validator_spawn_failed")?;
    // Command retains configured descriptors; release them so EOF reflects the
    // owned process group, not a writer accidentally retained in the parent.
    drop(command);
    let pgid = i32::try_from(child.id()).map_err(|_| "intent_validator_process_group_invalid")?;
    let mut owned = OwnedValidator {
        child,
        pgid,
        disarmed: false,
    };
    let mut out = Capture::default();
    let mut err = Capture::default();
    let mut status = None;
    let mut timed_out = false;
    let mut cancelled = false;
    let deadline = Duration::from_secs(validator.timeout_seconds);
    loop {
        out.drain(&mut stdout)
            .map_err(|_| "intent_validator_reader_failed")?;
        err.drain(&mut stderr)
            .map_err(|_| "intent_validator_reader_failed")?;
        if status.is_none() {
            status = owned
                .child
                .try_wait()
                .map_err(|_| "intent_validator_wait_failed")?;
        }
        if cancellation.cancelled() {
            cancelled = true;
            break;
        }
        if start.elapsed() >= deadline {
            timed_out = true;
            break;
        }
        if status.is_some() && out.eof && err.eof {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    // Kill surviving descendants even when Cargo exited first. Neither inherited
    // output descriptors nor an uninterruptible child may make cleanup unbounded.
    let terminated = owned.terminate();
    let cleanup_start = Instant::now();
    while cleanup_start.elapsed() < Duration::from_secs(2) {
        out.drain(&mut stdout)
            .map_err(|_| "intent_validator_reader_failed")?;
        err.drain(&mut stderr)
            .map_err(|_| "intent_validator_reader_failed")?;
        if status.is_none() {
            status = owned
                .child
                .try_wait()
                .map_err(|_| "intent_validator_wait_failed")?;
        }
        if status.is_some() && out.eof && err.eof {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    cancelled |= cancellation.cancelled();
    let cleanup_complete = terminated && status.is_some() && out.eof && err.eof;
    owned.disarmed = cleanup_complete;
    Ok(ValidatorExecution {
        stdout: out,
        stderr: err,
        exit_code: status.and_then(|status| status.code()),
        success: status.is_some_and(|status| status.success()),
        timed_out,
        cancelled,
        cleanup_complete,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

fn git_read(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|_| "intent_proof_git_failed")?;
    if !output.status.success() {
        return Err("intent_proof_git_failed".into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}
fn manifest_inputs(root: &Path, validators: &[Validator]) -> Result<(), String> {
    let tracked: std::collections::BTreeSet<String> = git_read(root, &["ls-files", "-z"])?
        .split('\0')
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
        .collect();
    let require = |path: &Path| -> Result<PathBuf, String> {
        let path = path
            .canonicalize()
            .map_err(|_| "intent_validator_input_unavailable")?;
        let relative = path
            .strip_prefix(root)
            .map_err(|_| "intent_validator_input_outside_repository")?;
        if !tracked.contains(&relative.to_string_lossy().into_owned()) {
            return Err("intent_validator_input_not_tracked".into());
        }
        Ok(path)
    };
    let mut queue = Vec::new();
    for validator in validators {
        let selection = ValidatorTargetSelection::from_args(&validator.args);
        let manifest = validator
            .args
            .windows(2)
            .find(|pair| pair[0] == "--manifest-path")
            .map(|pair| root.join(&pair[1]))
            .unwrap_or_else(|| root.join("Cargo.toml"));
        queue.push((require(&manifest)?, Some(selection.clone())));
        // Cargo resolves inherited dependencies and patches from ancestor
        // workspace manifests, even when --manifest-path selects one member.
        for ancestor in manifest.parent().into_iter().flat_map(Path::ancestors) {
            if !ancestor.starts_with(root) {
                break;
            }
            let workspace = ancestor.join("Cargo.toml");
            if workspace.is_file() {
                queue.push((require(&workspace)?, None));
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    while let Some((manifest, selection)) = queue.pop() {
        if !seen.insert((manifest.clone(), selection.clone())) {
            continue;
        }
        let directory = manifest
            .parent()
            .ok_or("intent_validator_manifest_invalid")?;
        let value: toml::Value = toml::from_str(
            &fs::read_to_string(&manifest).map_err(|_| "intent_validator_manifest_unreadable")?,
        )
        .map_err(|_| "intent_validator_manifest_invalid")?;
        let mut selected_here = selection.is_some();
        if let (Some(workspace), Some(selection)) = (value.get("workspace"), selection.as_ref()) {
            selected_here = false;
            let member_key = if workspace.get("default-members").is_some() {
                "default-members"
            } else if value.get("package").is_some() {
                ""
            } else {
                "members"
            };
            let members = if member_key.is_empty() {
                vec![".".to_owned()]
            } else {
                workspace
                    .get(member_key)
                    .and_then(toml::Value::as_array)
                    .ok_or("intent_validator_workspace_selection_not_admitted")?
                    .iter()
                    .map(|member| {
                        member
                            .as_str()
                            .map(str::to_owned)
                            .ok_or("intent_validator_workspace_selection_not_admitted")
                    })
                    .collect::<Result<Vec<_>, _>>()?
            };
            let excluded = workspace
                .get("exclude")
                .and_then(toml::Value::as_array)
                .map(|members| {
                    members
                        .iter()
                        .map(|member| {
                            member
                                .as_str()
                                .map(str::to_owned)
                                .ok_or("intent_validator_workspace_selection_not_admitted")
                        })
                        .collect::<Result<std::collections::BTreeSet<_>, _>>()
                })
                .transpose()?
                .unwrap_or_default();
            for member in members {
                if member.contains(['*', '?', '[', ']'])
                    || excluded
                        .iter()
                        .any(|path| path.contains(['*', '?', '[', ']']))
                {
                    return Err("intent_validator_workspace_selection_not_admitted".into());
                }
                if excluded.contains(&member) {
                    continue;
                }
                let member = directory.join(member);
                let member_manifest = if member.file_name().is_some_and(|name| name == "Cargo.toml")
                {
                    member
                } else {
                    member.join("Cargo.toml")
                };
                let member_manifest = require(&member_manifest)?;
                if member_manifest == manifest {
                    selected_here = true;
                } else {
                    queue.push((member_manifest, Some(selection.clone())));
                }
            }
        }
        if selected_here
            && selection.as_ref().is_some_and(|selection| {
                manifest_declares_selected_custom_harness(&value, selection)
            })
        {
            return Err("intent_validator_custom_harness_not_admitted".into());
        }
        for default in ["src/lib.rs", "src/main.rs", "build.rs"] {
            let path = directory.join(default);
            if path.exists() {
                require(&path)?;
            }
        }
        let mut stack = vec![&value];
        while let Some(value) = stack.pop() {
            match value {
                toml::Value::Table(table) => {
                    for (key, value) in table {
                        if key == "path" {
                            if let Some(path) = value.as_str() {
                                let path = directory.join(path);
                                if path.is_dir() {
                                    queue.push((require(&path.join("Cargo.toml"))?, None));
                                } else {
                                    require(&path)?;
                                }
                            }
                        }
                        if key == "build" {
                            if let Some(path) = value.as_str() {
                                require(&directory.join(path))?;
                            }
                        }
                        stack.push(value);
                    }
                }
                toml::Value::Array(values) => stack.extend(values),
                _ => {}
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ValidatorTargetSelection {
    all_targets: bool,
    default_targets: bool,
    lib: bool,
    tests: Vec<String>,
}

impl ValidatorTargetSelection {
    fn from_args(args: &[String]) -> Self {
        let lib = args.iter().any(|arg| arg == "--lib");
        let tests = args
            .windows(2)
            .filter(|pair| pair[0] == "--test")
            .map(|pair| pair[1].clone())
            .collect::<Vec<_>>();
        Self {
            all_targets: args.iter().any(|arg| arg == "--all-targets"),
            default_targets: !lib
                && tests.is_empty()
                && !args.iter().any(|arg| arg == "--all-targets"),
            lib,
            tests,
        }
    }
}

fn manifest_declares_selected_custom_harness(
    manifest: &toml::Value,
    selection: &ValidatorTargetSelection,
) -> bool {
    let custom =
        |target: &toml::Value| target.get("harness").and_then(toml::Value::as_bool) == Some(false);
    if selection.all_targets {
        return ["lib", "bin", "example", "test", "bench"]
            .into_iter()
            .filter_map(|kind| manifest.get(kind))
            .any(|targets| match targets {
                toml::Value::Table(target) => {
                    target.get("harness").and_then(toml::Value::as_bool) == Some(false)
                }
                toml::Value::Array(targets) => targets.iter().any(custom),
                _ => false,
            });
    }
    if selection.default_targets {
        return [
            ("lib", true),
            ("bin", true),
            ("example", false),
            ("test", true),
            ("bench", false),
        ]
        .into_iter()
        .filter_map(|(kind, default_test)| manifest.get(kind).map(|value| (value, default_test)))
        .any(|(targets, default_test)| match targets {
            toml::Value::Table(target) => {
                target
                    .get("test")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(default_test)
                    && target.get("harness").and_then(toml::Value::as_bool) == Some(false)
            }
            toml::Value::Array(targets) => targets.iter().any(|target| {
                target
                    .get("test")
                    .and_then(toml::Value::as_bool)
                    .unwrap_or(default_test)
                    && custom(target)
            }),
            _ => false,
        });
    }
    let custom_lib = selection.lib
        && manifest
            .get("lib")
            .is_some_and(|target| target.is_table() && custom(target));
    let custom_test = !selection.tests.is_empty()
        && manifest
            .get("test")
            .and_then(toml::Value::as_array)
            .is_some_and(|targets| {
                targets.iter().any(|target| {
                    target
                        .get("name")
                        .and_then(toml::Value::as_str)
                        .is_some_and(|name| selection.tests.iter().any(|selected| selected == name))
                        && custom(target)
                })
            });
    custom_lib || custom_test
}

// Rust dependency records expose #[path], include! and include_bytes! inputs
// which a Cargo manifest alone cannot enumerate. Repository-crate compiler
// inputs must belong to the exact tracked candidate, including excluded trees.
fn compiler_artifacts(root: &Path, stdout: &str) -> Value {
    let mut artifacts = Vec::new();
    for line in stdout.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if value["reason"] != "compiler-artifact" {
            continue;
        }
        let Some(manifest) = value["manifest_path"].as_str().map(Path::new) else {
            continue;
        };
        let Ok(manifest) = manifest.strip_prefix(root) else {
            // Cargo's actual package source identity distinguishes registry
            // packages from local paths; outside is not itself provenance.
            // Unknown and external local/git inputs fail closed below.
            if value["package_id"].as_str().is_some_and(|id| {
                id.starts_with("registry+https://")
                    || id.starts_with("registry+http://")
                    || id.starts_with("sparse+https://")
            }) {
                continue;
            }
            artifacts.push(json!({"external_input_denied":true}));
            continue;
        };
        let source = value["target"]["src_path"]
            .as_str()
            .map(Path::new)
            .and_then(|path| path.strip_prefix(root).ok());
        let files: Vec<_> = value["filenames"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|value| {
                value
                    .as_str()
                    .map(Path::new)
                    .and_then(|path| path.strip_prefix(root).ok())
                    .map(|path| path.to_string_lossy().into_owned())
            })
            .collect();
        artifacts.push(json!({"manifest":manifest,"source":source,"name":value["target"]["name"],"kind":value["target"]["kind"],"files":files}));
    }
    json!(artifacts)
}

fn compiler_inputs_tracked(root: &Path, artifacts: &Value) -> Result<(), String> {
    let artifacts = artifacts
        .as_array()
        .filter(|values| !values.is_empty())
        .ok_or("intent_validator_compiler_inputs_missing")?;
    let tracked: std::collections::BTreeSet<PathBuf> = git_read(root, &["ls-files", "-z"])?
        .split('\0')
        .filter(|v| !v.is_empty())
        .map(|v| root.join(v))
        .collect();
    for artifact in artifacts {
        if artifact["external_input_denied"] == true {
            return Err("intent_validator_input_outside_repository".into());
        }
        let manifest = root
            .join(
                artifact["manifest"]
                    .as_str()
                    .ok_or("intent_validator_compiler_artifact_invalid")?,
            )
            .canonicalize()
            .map_err(|_| "intent_validator_compiler_input_unavailable")?;
        let source = root
            .join(
                artifact["source"]
                    .as_str()
                    .ok_or("intent_validator_compiler_artifact_invalid")?,
            )
            .canonicalize()
            .map_err(|_| "intent_validator_compiler_input_unavailable")?;
        if !tracked.contains(&manifest) || !tracked.contains(&source) {
            return Err("intent_validator_compiler_input_not_tracked".into());
        }
        let directory = manifest
            .parent()
            .ok_or("intent_validator_compiler_artifact_invalid")?;
        let target = artifact["name"]
            .as_str()
            .ok_or("intent_validator_compiler_artifact_invalid")?
            .replace('-', "_");
        let mut records = std::collections::BTreeSet::new();
        for file in artifact["files"]
            .as_array()
            .ok_or("intent_validator_compiler_artifact_invalid")?
        {
            let file = root.join(
                file.as_str()
                    .ok_or("intent_validator_compiler_artifact_invalid")?,
            );
            let parent = file
                .parent()
                .ok_or("intent_validator_compiler_artifact_invalid")?
                .canonicalize()
                .map_err(|_| "intent_validator_dependency_inventory_unreadable")?;
            if !parent.starts_with(root.join("target/intent-validation")) {
                return Err("intent_validator_compiler_artifact_invalid".into());
            }
            let filename = file
                .file_stem()
                .and_then(|v| v.to_str())
                .ok_or("intent_validator_compiler_artifact_invalid")?;
            let stem = filename
                .strip_prefix("lib")
                .filter(|v| v.starts_with(&format!("{target}-")))
                .unwrap_or(filename)
                .replace('-', "_");
            for entry in fs::read_dir(&parent)
                .map_err(|_| "intent_validator_dependency_inventory_unreadable")?
            {
                let entry =
                    entry.map_err(|_| "intent_validator_dependency_inventory_unreadable")?;
                let path = entry.path();
                let name = path
                    .file_stem()
                    .and_then(|v| v.to_str())
                    .unwrap_or_default()
                    .replace('-', "_");
                let is_build =
                    target == "build_script_build" && name.starts_with("build_script_build");
                if path.extension().is_some_and(|v| v == "d") && (name == stem || is_build) {
                    let metadata = entry
                        .file_type()
                        .map_err(|_| "intent_validator_dependency_inventory_unreadable")?;
                    if !metadata.is_file() || metadata.is_symlink() {
                        return Err("intent_validator_dependency_symlink".into());
                    }
                    records.insert(path);
                }
            }
        }
        // Every actual repository artifact needs its own dependency record.
        if records.is_empty() {
            return Err("intent_validator_compiler_inputs_missing".into());
        }
        for path in records {
            let text = fs::read_to_string(path)
                .map_err(|_| "intent_validator_dependency_record_invalid")?;
            let dependencies = text
                .lines()
                .next()
                .and_then(|line| line.split_once(": "))
                .map(|(_, v)| v)
                .ok_or("intent_validator_dependency_record_invalid")?;
            let mut tokens = Vec::new();
            let mut token = String::new();
            let mut escaped = false;
            for c in dependencies.chars() {
                if escaped {
                    token.push(c);
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c.is_whitespace() {
                    if !token.is_empty() {
                        tokens.push(std::mem::take(&mut token));
                    }
                } else {
                    token.push(c);
                }
            }
            if !token.is_empty() {
                tokens.push(token);
            }
            if tokens.is_empty() {
                return Err("intent_validator_compiler_inputs_missing".into());
            }
            for token in tokens {
                let candidate = directory
                    .join(token)
                    .canonicalize()
                    .map_err(|_| "intent_validator_compiler_input_unavailable")?;
                if !tracked.contains(&candidate) {
                    return Err("intent_validator_compiler_input_not_tracked".into());
                }
            }
        }
    }
    Ok(())
}

fn tracked_input_digest(root: &Path, validators: &[Validator]) -> Result<String, String> {
    manifest_inputs(root, validators)?;
    if !git_read(root, &["diff", "--name-only", "HEAD"])?.is_empty() {
        return Err("intent_candidate_tracked_changes".into());
    }
    if git_read(root, &["ls-files", "--others", "--exclude-standard", "-z"])?
        .split('\0')
        .any(|name| !name.is_empty() && !name.starts_with(".csdlc/"))
    {
        return Err("intent_candidate_untracked_source".into());
    }
    // Ignoring source/configuration does not remove its influence on Cargo.
    // Only native artifacts and the same explicit disposable Rust cache roots
    // admitted by terminal archival are exempt from candidate admission.
    if git_read(
        root,
        &[
            "ls-files",
            "--others",
            "--ignored",
            "--exclude-standard",
            "-z",
        ],
    )?
    .split('\0')
    .any(|name| {
        !name.is_empty()
            && ![".csdlc/", "target/", "csdlc-v3/target/", "adl/target/"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
    }) {
        return Err("intent_candidate_ignored_source".into());
    }
    let files = git_read(root, &["ls-files", "-z"])?;
    let mut hash = blake3::Hasher::new();
    hash.update(
        &serde_json::to_vec(validators).map_err(|_| "intent_validator_serialization_failed")?,
    );
    for name in files.split('\0').filter(|name| !name.is_empty()) {
        let path = resolve_repo_path(root, name, true).map_err(|finding| finding.code)?;
        hash.update(name.as_bytes());
        hash.update(&fs::read(path).map_err(|_| "intent_validator_input_unreadable")?);
    }
    Ok(hash.finalize().to_hex().to_string())
}

/// Recheck the actual current candidate rather than treating equal HEAD as proof
/// that worktree bytes or declared validator inputs stayed unchanged.
pub fn verify_current_inputs(root: &Path, proof: &Value) -> Result<(), String> {
    let issue = proof["issue"]
        .as_u64()
        .ok_or("intent_proof_issue_missing")?;
    let context = Context::load(root, issue)?;
    let mut payload = proof.clone();
    let claimed = payload
        .as_object_mut()
        .and_then(|value| value.remove("payload_digest"))
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or("intent_proof_payload_missing")?;
    if proof["schema"] != "csdlc.v3.intent_proof.v1"
        || proof["repository"] != context.repository
        || proof["head"] != context.head
        || proof["issue_digest"] != context.index["digest"]
        || proof["status"] != "passed"
        || blake3::hash(&canonical_json(&payload)).to_hex().as_str() != claimed
    {
        return Err("intent_proof_identity_or_digest_mismatch".into());
    }
    let plan = context.plan()?;
    let digest = tracked_input_digest(root, &plan.validators)?;
    let records = proof["validators"]
        .as_array()
        .ok_or("intent_proof_validators_missing")?;
    for record in records {
        compiler_inputs_tracked(root, &record["compiler_artifacts"])?;
    }
    if records.is_empty()
        || records.len() != plan.validators.len()
        || records
            .iter()
            .zip(&plan.validators)
            .any(|(record, validator)| {
                record["id"] != validator.id
                    || record["program"] != validator.program
                    || record["args"] != json!(validator.args)
                    || record["timeout_seconds"] != validator.timeout_seconds
                    || record["input_digest"] != digest
                    || record["exit_code"] != 0
                    || record["tests_passed"]
                        .as_u64()
                        .is_none_or(|count| count == 0)
                    || record["tests_failed"] != 0
                    || record["passed"] != true
                    || record["timed_out"] != false
                    || record["cancelled"] != false
                    || record["truncated"] != false
                    || record["cleanup_complete"] != true
            })
    {
        return Err("intent_proof_current_inputs_mismatch".into());
    }
    Ok(())
}
