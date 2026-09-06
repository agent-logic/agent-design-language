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

fn github_read_only_curl_invocation(
    invocation: &CommandInvocation,
) -> Result<CommandInvocation, ProcessOutput> {
    let [operation, repository, number] = invocation.argv() else {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter requires operation, repository, and number".into(),
            truncated: false,
        });
    };
    if !matches!(
        operation.as_str(),
        "pull-request" | "issue" | "issue-comments"
    ) || number.parse::<u64>().is_err()
    {
        return Err(ProcessOutput {
            status: ProcessStatus::Exit(2),
            stdout: String::new(),
            stderr: "github read-only adapter received unsupported request".into(),
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
                format!("https://api.github.com/repos/{repository}/{resource}/{number}/comments?per_page=100")
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
    if !matches!(method.as_str(), "POST" | "PATCH")
        || !endpoint.starts_with("repos/")
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
            method.clone(),
            "--header".to_owned(),
            "Accept: application/vnd.github+json".to_owned(),
            "--header".to_owned(),
            "X-GitHub-Api-Version: 2022-11-28".to_owned(),
            "--header".to_owned(),
            "Content-Type: application/json".to_owned(),
            "--data-binary".to_owned(),
            format!("@{input_path}"),
            format!("https://api.github.com/{endpoint}"),
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
