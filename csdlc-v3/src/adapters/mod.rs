#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInvocation {
    pub program: String,
    pub argv: Vec<String>,
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
        Ok(Self {
            program,
            argv,
            credential_scope: CredentialScope::None,
        })
    }

    pub fn with_child_credential(mut self, name: impl Into<String>) -> Self {
        self.credential_scope = CredentialScope::ChildProcessOnly { name: name.into() };
        self
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialScope {
    None,
    ChildProcessOnly { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    ShellStringRejected,
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
        let branch = invocation.argv.last().cloned().unwrap_or_default();
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
    if value.contains("token=")
        || value.contains("secret=")
        || value.contains("password=")
        || is_inline_secret_assignment(value)
    {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}

fn is_secret_flag(value: &str) -> bool {
    matches!(
        value,
        "--token"
            | "--secret"
            | "--password"
            | "--api-key"
            | "--access-token"
            | "-token"
            | "-secret"
            | "-password"
    )
}

fn is_inline_secret_assignment(value: &str) -> bool {
    let Some((key, _)) = value.split_once('=') else {
        return false;
    };
    matches!(
        key.trim_start_matches('-').to_ascii_lowercase().as_str(),
        "token" | "secret" | "password" | "api-key" | "access-token"
    )
}
