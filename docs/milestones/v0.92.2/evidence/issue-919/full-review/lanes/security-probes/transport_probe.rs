use std::{process::Command,path::{Path,PathBuf},fs};
#[derive(Debug,PartialEq)] enum ProcessStatus { Exit(i32), Cancelled }
#[derive(Debug)] struct ProcessOutput {status:ProcessStatus,stdout:String,stderr:String,truncated:bool}
struct CommandInvocation {program:String,args:Vec<String>}
impl CommandInvocation {fn argv(&self)-> &[String] {&self.args}}
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


impl ProcessOutput {
    fn redact_secret(mut self, secret: &str) -> Self {
        if !secret.is_empty() {
            self.stdout = self.stdout.replace(secret, "[REDACTED]");
            self.stderr = self.stderr.replace(secret, "[REDACTED]");
        }
        self
    }
}


fn main() {
 use std::os::unix::fs::PermissionsExt;
 let dir=PathBuf::from(std::env::args().nth(1).unwrap()); fs::create_dir_all(&dir).unwrap();
 let payload=dir.join("payload.txt");let redirected=dir.join("unexpected-output.txt");
 fs::write(&payload,"synthetic-payload").unwrap();
 fs::write(dir.join(".curlrc"),format!("output = \"{}\"\n",redirected.display())).unwrap();
 let wrapper=dir.join("curl-fixture");
 fs::write(&wrapper,format!("#!/bin/sh\nexport CURL_HOME='{}'\nexec /usr/bin/curl \"$@\"\n",dir.display())).unwrap();
 fs::set_permissions(&wrapper,fs::Permissions::from_mode(0o700)).unwrap();
 let invoke=|q:bool| {let mut args=vec![];if q {args.push("-q".into())}; args.extend(["--silent".into(),"--show-error".into(),format!("file://{}",payload.display())]);CommandInvocation{program:wrapper.display().to_string(),args}};
 let before=run_process(&invoke(false),None,None,1024);assert_eq!(before.status,ProcessStatus::Exit(0));assert!(before.stdout.is_empty());assert_eq!(fs::read_to_string(&redirected).unwrap(),"synthetic-payload");
 fs::remove_file(&redirected).unwrap();
 let after=run_process(&invoke(true),None,None,1024);assert_eq!(after.status,ProcessStatus::Exit(0));assert_eq!(after.stdout,"synthetic-payload");assert!(!redirected.exists());
 println!("PASS reproduced default-config write in exact run_process; -q negative control suppresses write");
 let echo=dir.join("synthetic-output");fs::write(&echo,"#!/bin/sh\nprintf %s \"$GITHUB_TOKEN\"\n").unwrap();fs::set_permissions(&echo,fs::Permissions::from_mode(0o700)).unwrap();
 let secret="synthetic-only-token-not-a-credential";let inv=CommandInvocation{program:echo.display().to_string(),args:vec![]};
 let output=run_process(&inv,Some(("GITHUB_TOKEN",secret)),None,8).redact_secret(secret);
 assert!(output.truncated);assert_eq!(output.stdout,&secret[..8]);
 println!("PASS reproduced partial secret retained after truncation-before-redaction using synthetic value");
}
