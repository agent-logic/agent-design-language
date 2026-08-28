use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct CommandInvocation {
    pub program: String,
    argv: Vec<String>,
    pub credential_scope: CredentialScope,
    child_credential: Option<ChildCredential>,
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
            child_credential: None,
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

    pub fn with_resolved_child_credential(
        mut self,
        name: impl Into<String>,
        resolver: &impl CredentialResolver,
    ) -> Result<Self, AdapterError> {
        let name = name.into();
        if !is_safe_credential_name(&name) {
            return Err(AdapterError::CredentialResolutionFailed);
        }
        let value = resolver.resolve(&name)?;
        self.credential_scope = CredentialScope::ChildProcessOnly { name: name.clone() };
        self.child_credential = Some(ChildCredential { name, value });
        Ok(self)
    }

    pub fn argv(&self) -> &[String] {
        &self.argv
    }

    pub fn child_credential_name(&self) -> Option<&str> {
        self.child_credential
            .as_ref()
            .map(|credential| credential.name.as_str())
    }

    pub fn child_credential_value_for_process(&self) -> Option<&str> {
        self.child_credential
            .as_ref()
            .map(|credential| credential.value.as_str())
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
            .field(
                "child_credential",
                &self
                    .child_credential
                    .as_ref()
                    .map(|credential| (&credential.name, "[REDACTED]")),
            )
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
    fn resolve(&self, name: &str) -> Result<String, AdapterError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn resolve(&self, name: &str) -> Result<String, AdapterError> {
        if self.name == name {
            Ok(self.value.clone())
        } else {
            Err(AdapterError::CredentialResolutionFailed)
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ChildCredential {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    pub status: ProcessStatus,
    pub stdout: String,
    pub stderr: String,
    pub truncated: bool,
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
