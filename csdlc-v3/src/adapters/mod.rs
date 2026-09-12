use std::{
    fmt, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

const GITHUB_READ_ONLY_ADAPTER: &str = "github-api-read-only";
const GITHUB_OPERATIONAL_ADAPTER: &str = "github-api-operational";

#[derive(Clone, PartialEq, Eq)]
pub struct CommandInvocation {
    pub program: String,
    argv: Vec<String>,
    pub credential_scope: CredentialScope,
}

impl CommandInvocation {
    pub fn new(
        program: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, AdapterError> {
        let program = program.into();
        if program.trim().is_empty()
            || program.chars().any(char::is_whitespace)
            || looks_like_shell(&program)
        {
            return Err(AdapterError::ShellStringRejected);
        }
        let argv = args.into_iter().map(Into::into).collect::<Vec<_>>();
        if argv.iter().any(|arg| looks_like_shell(arg)) {
            return Err(AdapterError::ShellStringRejected);
        }
        if argv
            .iter()
            .any(|arg| is_secret_flag(arg) || redact(arg) != *arg)
        {
            return Err(AdapterError::SecretArgumentRejected);
        }
        Ok(Self {
            program,
            argv,
            credential_scope: CredentialScope::None,
        })
    }

    pub fn with_child_credential(mut self, name: impl Into<String>) -> Result<Self, AdapterError> {
        let name = name.into();
        if !is_safe_credential_name(&name) {
            return Err(AdapterError::CredentialResolutionFailed);
        }
        self.credential_scope = CredentialScope::ChildProcessOnly { name };
        Ok(self)
    }

    pub fn argv(&self) -> &[String] {
        &self.argv
    }

    pub fn child_credential_name(&self) -> Option<&str> {
        match &self.credential_scope {
            CredentialScope::ChildProcessOnly { name } => Some(name.as_str()),
            CredentialScope::None => None,
        }
    }

    pub fn inject_child_credential_for_process(
        &self,
        resolver: &impl CredentialResolver,
        injector: &mut impl ChildCredentialInjector,
    ) -> Result<(), AdapterError> {
        let Some(name) = self.child_credential_name() else {
            return Ok(());
        };
        resolver.inject_child_credential(name, injector)
    }

    pub fn redacted_argv(&self) -> Vec<String> {
        let mut redact_next = false;
        self.argv
            .iter()
            .map(|arg| {
                if redact_next {
                    redact_next = false;
                    return "[REDACTED]".to_owned();
                }
                if is_secret_flag(arg) {
                    redact_next = true;
                    return arg.clone();
                }
                redact(arg)
            })
            .collect::<Vec<_>>()
    }
}

impl fmt::Debug for CommandInvocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CommandInvocation")
            .field("program", &self.program)
            .field("argv", &self.redacted_argv())
            .field("credential_scope", &self.credential_scope)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialScope {
    None,
    ChildProcessOnly { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    ShellStringRejected,
    SecretArgumentRejected,
    CredentialResolutionFailed,
}

pub trait CredentialResolver {
    fn inject_child_credential(
        &self,
        name: &str,
        injector: &mut impl ChildCredentialInjector,
    ) -> Result<(), AdapterError>;
}

pub trait ChildCredentialInjector {
    fn inject_child_credential(&mut self, name: &str, value: &str);
}

#[derive(Clone, PartialEq, Eq)]
pub struct StaticCredentialResolver {
    name: String,
    value: String,
}

impl StaticCredentialResolver {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl CredentialResolver for StaticCredentialResolver {
    fn inject_child_credential(
        &self,
        name: &str,
        injector: &mut impl ChildCredentialInjector,
    ) -> Result<(), AdapterError> {
        if self.name == name {
            if self.value.trim().is_empty() {
                return Err(AdapterError::CredentialResolutionFailed);
            }
            injector.inject_child_credential(&self.name, &self.value);
            Ok(())
        } else {
            Err(AdapterError::CredentialResolutionFailed)
        }
    }
}

impl fmt::Debug for StaticCredentialResolver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StaticCredentialResolver")
            .field("name", &self.name)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentCredentialResolver;

impl CredentialResolver for EnvironmentCredentialResolver {
    fn inject_child_credential(
        &self,
        name: &str,
        injector: &mut impl ChildCredentialInjector,
    ) -> Result<(), AdapterError> {
        let value = std::env::var(name)
            .or_else(|_| read_approved_token_file(name))
            .map_err(|_| AdapterError::CredentialResolutionFailed)?;
        if value.trim().is_empty() {
            return Err(AdapterError::CredentialResolutionFailed);
        }
        injector.inject_child_credential(name, value.trim());
        Ok(())
    }
}

fn read_approved_token_file(name: &str) -> Result<String, AdapterError> {
    if !matches!(name, "GITHUB_TOKEN" | "GH_TOKEN") {
        return Err(AdapterError::CredentialResolutionFailed);
    }
    let path = std::env::var("ADL_GITHUB_TOKEN_FILE")
        .map_err(|_| AdapterError::CredentialResolutionFailed)?;
    fs::read_to_string(path).map_err(|_| AdapterError::CredentialResolutionFailed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    pub status: ProcessStatus,
    pub stdout: String,
    pub stderr: String,
    pub truncated: bool,
}

impl ProcessOutput {
    fn redact_secret(mut self, secret: &str) -> Self {
        if !secret.is_empty() {
            self.stdout = self.stdout.replace(secret, "[REDACTED]");
            self.stderr = self.stderr.replace(secret, "[REDACTED]");
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Exit(i32),
    TimedOut,
    Cancelled,
}

pub trait ProcessAdapter {
    fn preflight_child_credential(
        &mut self,
        invocation: &CommandInvocation,
    ) -> Result<(), AdapterError> {
        let _ = invocation;
        Ok(())
    }

    fn run(&mut self, invocation: CommandInvocation) -> ProcessOutput;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RealProcessAdapter<R> {
    resolver: R,
    max_output_bytes: usize,
}

impl<R> RealProcessAdapter<R> {
    pub fn new(resolver: R) -> Self {
        Self {
            resolver,
            max_output_bytes: 1024 * 1024,
        }
    }

    pub fn with_max_output_bytes(mut self, max_output_bytes: usize) -> Self {
        self.max_output_bytes = max_output_bytes;
        self
    }
}

impl<R: CredentialResolver> ProcessAdapter for RealProcessAdapter<R> {
    fn preflight_child_credential(
        &mut self,
        invocation: &CommandInvocation,
    ) -> Result<(), AdapterError> {
        let Some(name) = invocation.child_credential_name() else {
            return Ok(());
        };
        let mut captured = CapturedChildCredentials::default();
        self.resolver.inject_child_credential(name, &mut captured)?;
        captured
            .take_single(name)
            .map(|_| ())
            .ok_or(AdapterError::CredentialResolutionFailed)
    }

    fn run(&mut self, invocation: CommandInvocation) -> ProcessOutput {
        let mut captured = CapturedChildCredentials::default();
        let credential = match invocation.child_credential_name() {
            Some(name) => match self.resolver.inject_child_credential(name, &mut captured) {
                Ok(()) => captured.take_single(name),
                Err(_) => {
                    return ProcessOutput {
                        status: ProcessStatus::Exit(126),
                        stdout: String::new(),
                        stderr: "credential resolution failed".into(),
                        truncated: false,
                    };
                }
            },
            None => None,
        };
        let Some((credential_name, credential_value)) = credential else {
            if matches!(
                invocation.program.as_str(),
                GITHUB_READ_ONLY_ADAPTER | GITHUB_OPERATIONAL_ADAPTER
            ) {
                return ProcessOutput {
                    status: ProcessStatus::Exit(126),
                    stdout: String::new(),
                    stderr: "credential resolution failed".into(),
                    truncated: false,
                };
            }
            return run_process(&invocation, None, None, self.max_output_bytes);
        };
        if credential_value.contains(['\n', '\r', '"', '\\']) {
            return ProcessOutput {
                status: ProcessStatus::Exit(126),
                stdout: String::new(),
                stderr: "credential resolution failed".into(),
                truncated: false,
            };
        }
        let (process_invocation, curl_config_required) = match invocation.program.as_str() {
            GITHUB_READ_ONLY_ADAPTER => match github_read_only_curl_invocation(&invocation) {
                Ok(curl) => (curl, true),
                Err(output) => return output,
            },
            GITHUB_OPERATIONAL_ADAPTER => match github_operational_curl_invocation(&invocation) {
                Ok(curl) => (curl, true),
                Err(output) => return output,
            },
            _ => (invocation, false),
        };
        let curl_config = if curl_config_required || process_invocation.program == "curl" {
            match write_private_curl_config(&credential_value) {
                Ok(path) => Some(path),
                Err(_) => {
                    return ProcessOutput {
                        status: ProcessStatus::Exit(126),
                        stdout: String::new(),
                        stderr: "credential configuration failed".into(),
                        truncated: false,
                    };
                }
            }
        } else {
            None
        };
        let result = run_process(
            &process_invocation,
            Some((&credential_name, &credential_value)),
            curl_config.as_deref(),
            self.max_output_bytes,
        )
        .redact_secret(&credential_value);
        if let Some(path) = curl_config {
            let _ = fs::remove_file(path);
        }
        result
    }
}

/// Git branch syntax plus the existing typed-argv safety contract. Apply before
/// persisting a PR-create intent so every accepted head can be read back.
pub(crate) fn supported_pr_branch(value: &str) -> bool {
    !value.is_empty()
        && value != "@"
        && !value.starts_with('-')
        && !value.ends_with('.')
        && !value.contains("..")
        && !value.contains("@{")
        && !value
            .chars()
            .any(|c| c <= ' ' || c == '\u{7f}' || "~^:?*[\\".contains(c))
        && value
            .split('/')
            .all(|part| !part.is_empty() && !part.starts_with('.') && !part.ends_with(".lock"))
        && CommandInvocation::new(
            GITHUB_READ_ONLY_ADAPTER,
            ["pull-requests-by-head", "owner/repo", value],
        )
        .is_ok()
}

fn encode_query_value(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                char::from(byte).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn github_read_only_curl_invocation(
    invocation: &CommandInvocation,
) -> Result<CommandInvocation, ProcessOutput> {
    let (operation, repository, number, page) = match invocation.argv() {
        [operation, repository, number] => (operation, repository, number, None),
        [operation, repository, number, page] if operation == "issue-comments" => {
            (operation, repository, number, Some(page))
        }
        _ => return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter requires operation, repository, number, and optional comment page".into(),
            truncated: false,
        }),
    };
    let page = match page {
        None => 1,
        Some(page) => match page.parse::<u64>() {
            Ok(page @ 1..=100) => page,
            _ => {
                return Err(ProcessOutput {
                    status: ProcessStatus::Exit(2),
                    stdout: String::new(),
                    stderr: "github comment page must be within 1 through 100".into(),
                    truncated: false,
                })
            }
        },
    };
    if !matches!(
        operation.as_str(),
        "pull-request"
            | "pull-request-merge-state"
            | "pull-request-merge-linkage"
            | "branch-merge-rules"
            | "pull-requests-by-head"
            | "issue"
            | "issue-comments"
            | "issues-by-marker"
    ) || (!matches!(
        operation.as_str(),
        "issues-by-marker"
            | "pull-requests-by-head"
            | "branch-merge-rules"
            | "pull-request-merge-linkage"
    ) && number.parse::<u64>().is_err())
        || (operation == "issues-by-marker"
            && (number.is_empty()
                || number
                    .chars()
                    .any(|ch| !(ch.is_ascii_alphanumeric() || ch == '-'))))
        || (matches!(
            operation.as_str(),
            "pull-requests-by-head" | "branch-merge-rules"
        ) && !supported_pr_branch(number))
    {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter received unsupported request".into(),
            truncated: false,
        });
    }
    if operation == "branch-merge-rules" {
        let parts: Vec<_> = repository.split('/').collect();
        if parts.len() != 2
            || parts.iter().any(|s| {
                s.is_empty()
                    || s.chars()
                        .any(|c| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')))
            })
        {
            return Err(ProcessOutput {
                status: ProcessStatus::Exit(2),
                stdout: String::new(),
                stderr: "invalid repository".into(),
                truncated: false,
            });
        }
        return CommandInvocation::new("curl", [
            "--fail-with-body".to_owned(), "--silent".to_owned(), "--show-error".to_owned(),
            "--header".to_owned(), "Accept: application/vnd.github+json".to_owned(),
            "--header".to_owned(), "X-GitHub-Api-Version: 2022-11-28".to_owned(),
            format!("https://api.github.com/repos/{repository}/rules/branches/{}?per_page=100&page=1", encode_query_value(number)),
        ]).map_err(|_| ProcessOutput { status: ProcessStatus::Exit(2), stdout: String::new(), stderr: "invalid rule observation".into(), truncated: false });
    }
    if matches!(
        operation.as_str(),
        "pull-request-merge-state" | "pull-request-merge-linkage"
    ) {
        let Some((owner, name)) = repository.split_once('/') else {
            return Err(ProcessOutput {
                status: ProcessStatus::Exit(2),
                stdout: String::new(),
                stderr: "invalid repository".into(),
                truncated: false,
            });
        };
        if ![owner, name].iter().all(|s| {
            !s.is_empty()
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        }) {
            return Err(ProcessOutput {
                status: ProcessStatus::Exit(2),
                stdout: String::new(),
                stderr: "invalid repository".into(),
                truncated: false,
            });
        }
        let query = if operation == "pull-request-merge-linkage" {
            crate::commands::remote::merge_linkage_query(repository, number).ok_or_else(|| {
                ProcessOutput {
                    status: ProcessStatus::Exit(2),
                    stdout: String::new(),
                    stderr: "invalid qualified merge linkage target".into(),
                    truncated: false,
                }
            })?
        } else {
            crate::commands::remote::merge_state_query(owner, name, number)
        };
        return CommandInvocation::new(
            "curl",
            [
                "--fail-with-body".to_owned(),
                "--silent".to_owned(),
                "--show-error".to_owned(),
                "--get".to_owned(),
                "--data-urlencode".to_owned(),
                format!("query={query}"),
                "--header".to_owned(),
                "Accept: application/vnd.github+json".to_owned(),
                "https://api.github.com/graphql".to_owned(),
            ],
        )
        .map_err(|_| ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "invalid merge observation".into(),
            truncated: false,
        });
    }
    if operation == "issues-by-marker" {
        return CommandInvocation::new(
            "curl",
            [
                "--fail-with-body".to_owned(),
                "--silent".to_owned(),
                "--show-error".to_owned(),
                "--location".to_owned(),
                "--header".to_owned(),
                "Accept: application/vnd.github+json".to_owned(),
                "--header".to_owned(),
                "X-GitHub-Api-Version: 2022-11-28".to_owned(),
                format!(
                    "https://api.github.com/search/issues?q=repo:{repository}+type:issue+{number}"
                ),
            ],
        )
        .map_err(|_| ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter rejected unsafe request".into(),
            truncated: false,
        });
    }
    if operation == "pull-requests-by-head" {
        let Some((owner, _)) = repository.split_once('/') else {
            return Err(ProcessOutput {
                status: ProcessStatus::Exit(2),
                stdout: String::new(),
                stderr: "github read-only adapter rejected unsafe request".into(),
                truncated: false,
            });
        };
        return CommandInvocation::new(
            "curl",
            [
                "--fail-with-body".to_owned(),
                "--silent".to_owned(),
                "--show-error".to_owned(),
                "--location".to_owned(),
                "--header".to_owned(),
                "Accept: application/vnd.github+json".to_owned(),
                "--header".to_owned(),
                "X-GitHub-Api-Version: 2022-11-28".to_owned(),
                format!(
                    "https://api.github.com/repos/{repository}/pulls?head={}&state=all&per_page=100", encode_query_value(&format!("{owner}:{number}"))
                ),
            ],
        )
        .map_err(|_| ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter rejected unsafe request".into(),
            truncated: false,
        });
    }
    let resource = match operation.as_str() {
        "pull-request" => "pulls",
        "issue" => "issues",
        "issue-comments" => "issues",
        _ => unreachable!("operation checked above"),
    };
    CommandInvocation::new(
        "curl",
        [
            "--fail-with-body".to_owned(),
            "--silent".to_owned(),
            "--show-error".to_owned(),
            "--location".to_owned(),
            "--header".to_owned(),
            "Accept: application/vnd.github+json".to_owned(),
            "--header".to_owned(),
            "X-GitHub-Api-Version: 2022-11-28".to_owned(),
            if operation == "issue-comments" {
                format!("https://api.github.com/repos/{repository}/{resource}/{number}/comments?per_page=100&page={page}")
            } else {
                format!("https://api.github.com/repos/{repository}/{resource}/{number}")
            },
        ],
    )
    .map_err(|_| ProcessOutput {
        status: ProcessStatus::Exit(2),
        stdout: String::new(),
        stderr: "github read-only adapter rejected unsafe request".into(),
        truncated: false,
    })
}

fn github_operational_curl_invocation(
    invocation: &CommandInvocation,
) -> Result<CommandInvocation, ProcessOutput> {
    let [method, endpoint, input_path] = invocation.argv() else {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github operational adapter requires method, endpoint, and input path".into(),
            truncated: false,
        });
    };
    let parts: Vec<_> = endpoint.split('/').collect();
    let merge_put = method == "PUT"
        && parts.len() == 6
        && parts[0] == "repos"
        && parts[3] == "pulls"
        && parts[4].parse::<u64>().is_ok_and(|n| n > 0)
        && parts[5] == "merge";
    let graphql_ready = method == "GRAPHQL" && endpoint == "mark-pull-request-ready";
    if (!endpoint.starts_with("repos/") || !matches!(method.as_str(), "POST" | "PATCH"))
        && !graphql_ready
        && !merge_put
        || endpoint.contains("..")
        || endpoint
            .chars()
            .any(|ch| !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/')))
    {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github operational adapter received unsupported request".into(),
            truncated: false,
        });
    }
    let input = Path::new(input_path);
    if !input.is_absolute() || !input.is_file() {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github operational adapter requires an existing absolute input file".into(),
            truncated: false,
        });
    }
    CommandInvocation::new(
        "curl",
        [
            "--fail-with-body".to_owned(),
            "--silent".to_owned(),
            "--show-error".to_owned(),
            "--location".to_owned(),
            "--request".to_owned(),
            if graphql_ready {
                "POST".to_owned()
            } else {
                method.clone()
            },
            "--header".to_owned(),
            "Accept: application/vnd.github+json".to_owned(),
            "--header".to_owned(),
            "X-GitHub-Api-Version: 2022-11-28".to_owned(),
            "--header".to_owned(),
            "Content-Type: application/json".to_owned(),
            "--data-binary".to_owned(),
            format!("@{input_path}"),
            if graphql_ready {
                "https://api.github.com/graphql".to_owned()
            } else {
                format!("https://api.github.com/{endpoint}")
            },
        ],
    )
    .map_err(|_| ProcessOutput {
        status: ProcessStatus::Exit(2),
        stdout: String::new(),
        stderr: "github operational adapter rejected unsafe request".into(),
        truncated: false,
    })
}

#[derive(Default)]
struct CapturedChildCredentials {
    pairs: Vec<(String, String)>,
}

impl CapturedChildCredentials {
    fn take_single(&mut self, expected_name: &str) -> Option<(String, String)> {
        if self.pairs.len() == 1 && self.pairs[0].0 == expected_name {
            self.pairs.pop()
        } else {
            None
        }
    }
}

impl ChildCredentialInjector for CapturedChildCredentials {
    fn inject_child_credential(&mut self, name: &str, value: &str) {
        self.pairs.push((name.to_owned(), value.to_owned()));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FakeProcessAdapter {
    output: ProcessOutput,
    invocations: Vec<CommandInvocation>,
}

impl FakeProcessAdapter {
    pub fn new(output: ProcessOutput) -> Self {
        Self {
            output,
            invocations: Vec::new(),
        }
    }

    pub fn invocations(&self) -> &[CommandInvocation] {
        &self.invocations
    }
}

impl ProcessAdapter for FakeProcessAdapter {
    fn run(&mut self, invocation: CommandInvocation) -> ProcessOutput {
        self.invocations.push(invocation);
        self.output.clone()
    }
}

fn run_process(
    invocation: &CommandInvocation,
    credential: Option<(&str, &str)>,
    curl_config: Option<&Path>,
    max_output_bytes: usize,
) -> ProcessOutput {
    let mut command = Command::new(&invocation.program);
    command.args(invocation.argv());
    apply_minimal_child_environment(&mut command);
    if let Some(path) = curl_config {
        command.arg("--config").arg(path);
    }
    if let Some((name, value)) = credential {
        command.env(name, value);
    }
    match command.output() {
        Ok(output) => process_output(output, max_output_bytes),
        Err(error) => ProcessOutput {
            status: ProcessStatus::Exit(127),
            stdout: String::new(),
            stderr: format!("process execution failed: {error}"),
            truncated: false,
        },
    }
}

/// Child processes run without ambient parent credentials or shell startup
/// state. `PATH` is retained only as the explicit executable lookup trust
/// boundary, and `LC_ALL=C` fixes locale-sensitive output. `HOME`, proxy
/// variables, CA overrides, and provider configuration are intentionally not
/// inherited; GitHub credentials are passed through the typed child-credential
/// scope and private curl config.
fn apply_minimal_child_environment(command: &mut Command) {
    command.env_clear();
    command.env(
        "PATH",
        std::env::var("PATH").unwrap_or_else(|_| "/usr/bin:/bin:/usr/sbin:/sbin".to_owned()),
    );
    command.env("LC_ALL", "C");
    #[cfg(windows)]
    if let Ok(system_root) = std::env::var("SystemRoot") {
        command.env("SystemRoot", system_root);
    }
}

fn process_output(output: std::process::Output, max_output_bytes: usize) -> ProcessOutput {
    let mut stdout = output.stdout;
    let mut stderr = output.stderr;
    let truncated =
        truncate(&mut stdout, max_output_bytes) | truncate(&mut stderr, max_output_bytes);
    ProcessOutput {
        status: output
            .status
            .code()
            .map(ProcessStatus::Exit)
            .unwrap_or(ProcessStatus::Cancelled),
        stdout: String::from_utf8_lossy(&stdout).into_owned(),
        stderr: String::from_utf8_lossy(&stderr).into_owned(),
        truncated,
    }
}

fn truncate(bytes: &mut Vec<u8>, max_output_bytes: usize) -> bool {
    if bytes.len() > max_output_bytes {
        bytes.truncate(max_output_bytes);
        true
    } else {
        false
    }
}

fn write_private_curl_config(token: &str) -> std::io::Result<PathBuf> {
    let dir = runtime_dir()?;
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!(
        "curl-auth-{}-{}.config",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(&path)?;
    use std::io::Write;
    writeln!(file, "header = \"Authorization: Bearer {token}\"")?;
    Ok(path)
}

fn runtime_dir() -> std::io::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let git_dir = git_control_dir(&cwd).unwrap_or_else(|| cwd.join(".git"));
    Ok(git_dir.join("csdlc-v3/runtime"))
}

fn git_control_dir(root: &Path) -> Option<PathBuf> {
    for candidate in root.ancestors() {
        let dot_git = candidate.join(".git");
        if dot_git.is_dir() {
            return dot_git.canonicalize().ok();
        }
        let Ok(contents) = fs::read_to_string(&dot_git) else {
            continue;
        };
        let Some(gitdir) = contents.strip_prefix("gitdir:") else {
            continue;
        };
        let gitdir = gitdir.trim();
        let path = Path::new(gitdir);
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            candidate.join(path)
        };
        let git_dir = path.canonicalize().ok()?;
        if let Some(common_dir) = git_common_dir(&git_dir) {
            return Some(common_dir);
        }
        return Some(git_dir);
    }
    None
}

fn git_common_dir(git_dir: &Path) -> Option<PathBuf> {
    let contents = fs::read_to_string(git_dir.join("commondir")).ok()?;
    let path = Path::new(contents.trim());
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        git_dir.join(path)
    };
    path.canonicalize().ok()
}

pub trait GitAdapter {
    fn observe_branch(&mut self, invocation: CommandInvocation) -> GitObservation;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitObservation {
    pub branch: String,
    pub authorizes_lifecycle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FakeGitAdapter {
    observations: Vec<GitObservation>,
}

impl FakeGitAdapter {
    pub fn observations(&self) -> &[GitObservation] {
        &self.observations
    }
}

impl GitAdapter for FakeGitAdapter {
    fn observe_branch(&mut self, invocation: CommandInvocation) -> GitObservation {
        let branch = invocation.argv().last().cloned().unwrap_or_default();
        let observation = GitObservation {
            branch,
            authorizes_lifecycle: false,
        };
        self.observations.push(observation.clone());
        observation
    }
}

fn looks_like_shell(value: &str) -> bool {
    let program_name = value.rsplit(['/', '\\']).next().unwrap_or(value);
    let program_name = program_name
        .strip_suffix(".exe")
        .unwrap_or(program_name)
        .to_ascii_lowercase();
    if matches!(
        program_name.as_str(),
        "sh" | "bash" | "zsh" | "fish" | "cmd" | "powershell" | "pwsh"
    ) {
        return true;
    }
    ["&&", "||", ";", "|", "$(", "`"]
        .iter()
        .any(|needle| value.contains(needle))
}

fn redact(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    if is_inline_secret_assignment(value)
        || is_authorization_header(&lower)
        || is_option_assigned_authorization_header(&lower)
        || contains_url_userinfo(value)
        || lower.contains("token=")
        || lower.contains("secret=")
        || lower.contains("password=")
        || lower.contains("api-key=")
        || lower.contains("api_key=")
        || lower.contains("credential=")
        || lower.contains("authorization=")
    {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}

fn is_secret_flag(value: &str) -> bool {
    if value.contains('=') || !value.starts_with('-') {
        return false;
    }
    let key = value.trim_start_matches('-').to_ascii_lowercase();
    is_sensitive_key(&key)
}

fn is_safe_credential_name(value: &str) -> bool {
    !value.trim().is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
        && !looks_like_shell(value)
}

fn is_inline_secret_assignment(value: &str) -> bool {
    let Some((key, _)) = value.split_once('=') else {
        return false;
    };
    is_sensitive_key(&key.trim_start_matches('-').to_ascii_lowercase())
}

fn is_authorization_header(lowercase_value: &str) -> bool {
    lowercase_value.starts_with("authorization:")
}

fn is_option_assigned_authorization_header(lowercase_value: &str) -> bool {
    let Some((_, assigned_value)) = lowercase_value.split_once('=') else {
        return false;
    };
    is_authorization_header(assigned_value.trim_start())
}

fn contains_url_userinfo(value: &str) -> bool {
    let Some((_, after_scheme)) = value.split_once("://") else {
        return false;
    };
    let authority = after_scheme.split('/').next().unwrap_or(after_scheme);
    let Some((userinfo, _host)) = authority.rsplit_once('@') else {
        return false;
    };
    !userinfo.is_empty()
}

fn is_sensitive_key(key: &str) -> bool {
    [
        "token",
        "secret",
        "password",
        "key",
        "credential",
        "authorization",
        "auth",
    ]
    .iter()
    .any(|needle| key.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    // PVF: required deterministic small local argv contract; no credentials/network.
    #[test]
    fn comment_page_argv_is_bounded_and_reaches_curl_url() {
        for page in ["1", "2", "100"] {
            let request = CommandInvocation::new(
                GITHUB_READ_ONLY_ADAPTER,
                [
                    "issue-comments",
                    "agent-logic/agent-design-language",
                    "771",
                    page,
                ],
            )
            .unwrap();
            let curl = github_read_only_curl_invocation(&request).unwrap();
            assert_eq!(curl.argv().last().unwrap(), &format!("https://api.github.com/repos/agent-logic/agent-design-language/issues/771/comments?per_page=100&page={page}"));
        }
        for page in ["0", "101", "bad"] {
            let request = CommandInvocation::new(
                GITHUB_READ_ONLY_ADAPTER,
                [
                    "issue-comments",
                    "agent-logic/agent-design-language",
                    "771",
                    page,
                ],
            )
            .unwrap();
            assert!(github_read_only_curl_invocation(&request).is_err());
        }
    }

    #[test]
    fn github_read_only_adapter_supports_pull_request_head_reconciliation() {
        let invocation = CommandInvocation::new(
            GITHUB_READ_ONLY_ADAPTER,
            [
                "pull-requests-by-head",
                "agent-logic/agent-design-language",
                "codex/517-tail-01-quality-gate",
            ],
        )
        .expect("safe typed invocation");

        let curl = github_read_only_curl_invocation(&invocation).expect("supported readback");
        assert_eq!(curl.program, "curl");
        assert_eq!(
            curl.argv().last().map(String::as_str),
            Some(
                "https://api.github.com/repos/agent-logic/agent-design-language/pulls?head=agent-logic%3Acodex%2F517-tail-01-quality-gate&state=all&per_page=100"
            )
        );
    }

    // PVF: deterministic local CPU, real URL construction; no network.
    #[test]
    fn branch_query_preserves_special_characters() {
        for (head, encoded) in [
            ("codex/fix+retry", "codex%2Ffix%2Bretry"),
            ("codex/a&state=closed", "codex%2Fa%26state%3Dclosed"),
            ("codex/é#%", "codex%2F%C3%A9%23%25"),
        ] {
            assert!(supported_pr_branch(head));
            let input = CommandInvocation::new(
                GITHUB_READ_ONLY_ADAPTER,
                ["pull-requests-by-head", "owner/repo", head],
            )
            .unwrap();
            let output = github_read_only_curl_invocation(&input).unwrap();
            assert_eq!(output.argv().last().unwrap(), &format!("https://api.github.com/repos/owner/repo/pulls?head=owner%3A{encoded}&state=all&per_page=100"));
        }
        for invalid in [
            "", "@", "-bad", "a..b", "a@{b", "a//b", "a/.b", "a.lock", "a?b", "a b", "a;echo",
        ] {
            assert!(!supported_pr_branch(invalid), "{invalid}");
        }
    }

    #[test]
    fn github_read_only_adapter_rejects_unsafe_pull_request_head() {
        let invocation = CommandInvocation::new(
            GITHUB_READ_ONLY_ADAPTER,
            [
                "pull-requests-by-head",
                "agent-logic/agent-design-language",
                "codex/unsafe?state=open",
            ],
        )
        .expect("typed invocation construction");

        let rejected = github_read_only_curl_invocation(&invocation).expect_err("unsafe head");
        assert_eq!(rejected.status, ProcessStatus::Exit(2));
    }

    // PVF: deterministic local CPU argv contract; no credentials or network.
    #[test]
    fn github_operational_adapter_supports_only_the_typed_ready_graphql_mutation() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("csdlc-v3-ready-input-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let input = dir.join("request.json");
        fs::write(&input, b"{}").unwrap();
        let invocation = CommandInvocation::new(
            GITHUB_OPERATIONAL_ADAPTER,
            [
                "GRAPHQL".to_owned(),
                "mark-pull-request-ready".to_owned(),
                input.to_string_lossy().into_owned(),
            ],
        )
        .unwrap();
        let curl = github_operational_curl_invocation(&invocation).unwrap();
        assert!(curl
            .argv()
            .windows(2)
            .any(|args| args == ["--request", "POST"]));
        assert_eq!(
            curl.argv().last().map(String::as_str),
            Some("https://api.github.com/graphql")
        );

        let unsupported = CommandInvocation::new(
            GITHUB_OPERATIONAL_ADAPTER,
            [
                "GRAPHQL".to_owned(),
                "arbitrary-mutation".to_owned(),
                input.to_string_lossy().into_owned(),
            ],
        )
        .unwrap();
        assert!(github_operational_curl_invocation(&unsupported).is_err());
        fs::remove_file(input).unwrap();
        fs::remove_dir(dir).unwrap();
    }

    // PVF: deterministic local subprocess environment contract; no network.
    #[test]
    fn unscoped_child_process_does_not_inherit_parent_credentials_or_home() {
        unsafe {
            std::env::set_var("ADL_TEST_PARENT_CREDENTIAL_751", "must-not-leak");
            std::env::set_var("HTTPS_PROXY", "http://credential.example.invalid");
            std::env::set_var("HOME", "/tmp/adl-home-must-not-leak");
        }
        let invocation = CommandInvocation::new("/usr/bin/env", std::iter::empty::<&str>())
            .expect("env command");
        let output = run_process(&invocation, None, None, 1024 * 1024);
        unsafe {
            std::env::remove_var("ADL_TEST_PARENT_CREDENTIAL_751");
            std::env::remove_var("HTTPS_PROXY");
            std::env::remove_var("HOME");
        }
        assert_eq!(output.status, ProcessStatus::Exit(0));
        assert!(!output.stdout.contains("ADL_TEST_PARENT_CREDENTIAL_751="));
        assert!(!output.stdout.contains("must-not-leak"));
        assert!(!output.stdout.contains("HTTPS_PROXY="));
        assert!(!output.stdout.contains("HOME=/tmp/adl-home-must-not-leak"));
        assert!(output.stdout.contains("LC_ALL=C"));
        assert!(output.stdout.contains("PATH="));
    }

    // PVF: deterministic local subprocess credential-scope/redaction contract; no network.
    #[test]
    fn scoped_child_process_receives_only_selected_credential_and_minimal_environment() {
        let invocation = CommandInvocation::new("/usr/bin/env", std::iter::empty::<&str>())
            .expect("env command")
            .with_child_credential("GITHUB_TOKEN")
            .expect("safe credential name");
        let mut adapter =
            RealProcessAdapter::new(StaticCredentialResolver::new("GITHUB_TOKEN", "secret-751"));
        let output = adapter.run(invocation);
        assert_eq!(output.status, ProcessStatus::Exit(0));
        assert!(output.stdout.contains("GITHUB_TOKEN=[REDACTED]"));
        assert!(!output.stdout.contains("secret-751"));
        assert!(!output.stderr.contains("secret-751"));
        assert!(!output.stdout.contains("HOME="));
        assert!(!output.stdout.contains("HTTPS_PROXY="));
        assert!(output.stdout.contains("LC_ALL=C"));
        assert!(output.stdout.contains("PATH="));
    }
}

#[cfg(test)]
mod merge_adapter_tests {
    use super::*;
    #[test]
    fn merge_linkage_readback_admits_only_qualified_numeric_targets() {
        for (target, allowed) in [
            ("844:agent-logic/planning#505", true),
            ("844:agent-logic/agent-design-language#505", true),
            ("844:#505", false),
            ("844:agent-logic/planning#0", false),
            ("0:agent-logic/planning#505", false),
            ("844:agent-logic/planning/extra#505", false),
            ("844:agent-logic/planning#505) { viewer { login } }", false),
            ("844:agent-logic/planning\"#505", false),
        ] {
            let request = CommandInvocation::new(
                GITHUB_READ_ONLY_ADAPTER,
                [
                    "pull-request-merge-linkage",
                    "agent-logic/agent-design-language",
                    target,
                ],
            )
            .unwrap();
            let result = github_read_only_curl_invocation(&request);
            assert_eq!(result.is_ok(), allowed, "{target}");
            if let Ok(curl) = result {
                let query = curl
                    .argv
                    .iter()
                    .find(|arg| arg.starts_with("query="))
                    .unwrap();
                assert!(query.contains("closingIssuesReferences(first:100)"));
                assert!(query.contains("linkedRepository: repository"));
                assert!(query.contains("issue(number:505) { number url state }"));
            }
        }
    }
    // PVF: required deterministic owner adapter contract, small local CPU/filesystem.
    #[test]
    fn merge_put_is_narrow_and_readback_cannot_supply_arbitrary_query() {
        let path =
            std::env::temp_dir().join(format!("csdlc-merge-adapter-{}.json", std::process::id()));
        std::fs::write(&path, b"{}").unwrap();
        for (endpoint, allowed) in [
            (
                "repos/agent-logic/agent-design-language/pulls/844/merge",
                true,
            ),
            ("repos/agent-logic/agent-design-language/issues/844", false),
            ("repos/agent-logic/agent-design-language/pulls/844", false),
            (
                "repos/agent-logic/agent-design-language/pulls/0/merge",
                false,
            ),
            (
                "repos/agent-logic/agent-design-language/pulls/844/merge/other",
                false,
            ),
        ] {
            let invocation = CommandInvocation::new(
                GITHUB_OPERATIONAL_ADAPTER,
                ["PUT", endpoint, path.to_str().unwrap()],
            )
            .unwrap();
            let result = github_operational_curl_invocation(&invocation);
            assert_eq!(result.is_ok(), allowed, "{endpoint}");
        }
        for (target, allowed) in [("844", true), ("844) { viewer { login } }", false)] {
            let invocation = CommandInvocation::new(
                GITHUB_READ_ONLY_ADAPTER,
                [
                    "pull-request-merge-state",
                    "agent-logic/agent-design-language",
                    target,
                ],
            )
            .unwrap();
            let result = github_read_only_curl_invocation(&invocation);
            assert_eq!(result.is_ok(), allowed);
            if let Ok(invocation) = result {
                assert!(invocation
                    .argv()
                    .iter()
                    .any(|v| v.starts_with("query=query")));
                assert!(!invocation.argv().iter().any(|v| v.contains("mutation ")));
            }
        }
        std::fs::remove_file(path).unwrap();
    }
}
