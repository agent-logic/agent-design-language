use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_json::json;

use super::csm_cmd::real_csm_standalone;
use super::process_cmd::real_process;
use adl::long_lived_agent::load_spec;
use adl_runtime::runtime_api_auth::RuntimeApiCredentialStore;
use adl_runtime_kernel::{is_canonical_agent_name, AGENT_ADMISSION_SCHEMA};
use adl_runtime_kernel::RuntimeInitConfig;

static AGENT_ARTIFACT_TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);
const AGENT_CONFIG_SCHEMA: &str = "adl.csm.agent_config.v1";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentAddConfig {
    schema: String,
    runtime: AgentRuntimeConfig,
    identity: AgentIdentityConfig,
    office: String,
    provider: AgentProviderConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentRuntimeConfig {
    init: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentIdentityConfig {
    id: String,
    name: String,
    display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentProviderConfig {
    kind: String,
    model: String,
    endpoint: String,
}

pub(crate) fn real_csmctl(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("runtime") => real_runtime(&args[1..]),
        Some("status") => real_status(&args[1..]),
        Some("diagnostics") => real_diagnostics(&args[1..]),
        Some("api") => real_api(&args[1..]),
        Some("cloud") => real_cloud(&args[1..]),
        Some("agent") => real_agent(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl module '{other}'. Expected runtime, agent, api, status, diagnostics, cloud, help, or --version.\n\n{}",
            csmctl_usage()
        )),
    }
}

pub(crate) fn csmctl_usage() -> &'static str {
    "csmctl - CSM runtime administration control plane\n\n\
Usage:\n\
  csmctl runtime service <install|start|status|stop|remove> ...\n\
  csmctl runtime governed-stop --spec <agent-spec.yaml> --reason <text> ...\n\
  csmctl runtime continuity <capture|stage|restore|drill> ...\n\
  csmctl runtime backpressure prove ...\n\
  csmctl runtime storage prove-s3 ...\n\
  csmctl runtime observatory --packet <visibility-packet.json> ...\n\
  csmctl agent add --config <agent.yaml>\n\
  csmctl api get --spec <agent-spec.yaml> [--path /status] [--bind 127.0.0.1:19997]\n\
  csmctl api credential <status|rotate|revoke> --spec <agent-spec.yaml>\n\
  csmctl status [--pid <pid>|--pid-file <path>|--port <port> [--host 127.0.0.1]] [--json]\n\
  csmctl diagnostics process status [--pid <pid>|--pid-file <path>|--port <port>] [--json]\n\
  csmctl cloud aws-signal acip-sns-proof ...\n\
  csmctl cloud cloud-control cloudfront-status ...\n\
  csmctl --help\n\
  csmctl --version\n\n\
Modules:\n\
  runtime      Administer the CSM service, governed stop, embedded API bind, continuity, backpressure, storage, and observatory surfaces.\n\
  agent        Add a provider-verified agent to a running Runtime without editing its init file or restarting it.\n\
  api          Authenticated client and credential lifecycle for the embedded runtime API.\n\
  status       Permission-safe liveness checks for CSM process metadata or loopback ports.\n\
  diagnostics  Explicit diagnostic wrappers around permission-safe process probes.\n\
  cloud        Governed runtime cloud-control and signal proof surfaces.\n\n\
Boundaries:\n\
  - csm is the runtime owner and executes the permanent daemon loop.\n\
  - csmctl is the operator/admin control plane for that runtime.\n\
  - adl remains ADL language authoring, compilation, validation, and runtime workflow tooling.\n\
  - C-SDLC issue execution resolves through csdlc-install and the independent typed v2 binaries."
}

fn real_agent(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("add") => csmctl_agent_add(&args[1..]),
        Some("list") => csmctl_agent_read(&args[1..], None),
        Some("get") => csmctl_agent_read(&args[1..], Some(required_arg(&args[1..], "--id")?)),
        Some("remove") => csmctl_agent_remove(&args[1..]),
        Some("checkpoint") => csmctl_agent_checkpoint(&args[1..]),
        Some("dehydrate") => csmctl_agent_dehydrate(&args[1..]),
        Some("migrate") => csmctl_agent_migrate(&args[1..]),
        Some("rehydrate") => csmctl_agent_rehydrate(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_agent_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl agent command '{other}'. Expected add, list, get, remove, checkpoint, dehydrate, migrate, rehydrate, or help"
        )),
    }
}

fn csmctl_agent_usage() -> &'static str {
    "csmctl agent - manage the Runtime v3 agent lifecycle\n\n\
Usage:\n\
  csmctl agent add --config <agent.yaml>\n\
  csmctl agent list --init <init>\n\
  csmctl agent get --init <init> --id <id>\n\
  csmctl agent checkpoint --init <init> --id <id> [--out <checkpoint.json>]\n\
  csmctl agent dehydrate --init <init> --id <id> --out <freeze-dried-agent.json>\n\
  csmctl agent migrate --init <init> --id <id> --out <freeze-dried-agent.json>\n\
  csmctl agent rehydrate --init <init> --bundle <freeze-dried-agent.json>\n\
  csmctl agent remove --init <init> --id <id>\n\n\
Migration checkpoints and freeze-dried bundles are written atomically. migrate removes the source only after the bundle is durable."
}

struct RuntimeAgentClient {
    client: reqwest::blocking::Client,
    base_url: String,
    write_token: String,
}

impl RuntimeAgentClient {
    fn from_args(args: &[String]) -> Result<Self> {
        let init_path = required_path_arg(args, "--init")?;
        Self::from_init_path(init_path)
    }

    fn from_init_path(init_path: PathBuf) -> Result<Self> {
        let init = RuntimeInitConfig::from_path(init_path.clone())
            .with_context(|| format!("load Runtime v3 init {}", init_path.display()))?;
        let address: SocketAddr = init
            .api
            .address
            .parse()
            .with_context(|| format!("parse Runtime API address {}", init.api.address))?;
        let roots = fs::read(&init.api.tls.trust_roots_path).with_context(|| {
            format!(
                "read Runtime trust roots {}",
                init.api.tls.trust_roots_path.display()
            )
        })?;
        let certificates =
            reqwest::Certificate::from_pem_bundle(&roots).context("parse Runtime trust roots")?;
        let write_token = fs::read_to_string(&init.credentials.acip_write_token_path)
            .with_context(|| {
                format!(
                    "read Runtime write credential {}",
                    init.credentials.acip_write_token_path.display()
                )
            })?;
        let write_token = write_token.trim().to_owned();
        if write_token.is_empty() || write_token.chars().any(char::is_whitespace) {
            return Err(anyhow!("Runtime write credential is invalid"));
        }
        let mut builder = reqwest::blocking::Client::builder()
            .tls_built_in_root_certs(false)
            .resolve(&init.api.tls.server_name, address)
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(15));
        for certificate in certificates {
            builder = builder.add_root_certificate(certificate);
        }
        let client = builder
            .build()
            .context("build authenticated Runtime agent lifecycle client")?;
        Ok(Self {
            client,
            base_url: format!("https://{}:{}", init.api.tls.server_name, address.port()),
            write_token,
        })
    }

    fn call(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut request = self
            .client
            .request(method, format!("{}{}", self.base_url, path))
            .bearer_auth(&self.write_token);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .context("call authenticated Runtime agent lifecycle API")?;
        let status = response.status();
        let text = response
            .text()
            .context("read Runtime agent lifecycle response")?;
        if !status.is_success() {
            return Err(anyhow!(
                "Runtime agent lifecycle returned HTTP {status}: {text}"
            ));
        }
        serde_json::from_str(&text).context("parse Runtime agent lifecycle response")
    }
}

fn csmctl_agent_add(args: &[String]) -> Result<()> {
    if args.len() != 2 || args.first().map(String::as_str) != Some("--config") {
        return Err(anyhow!(
            "csmctl agent add requires exactly --config <agent.yaml>"
        ));
    }
    let config_path = PathBuf::from(&args[1]);
    let config = load_agent_add_config(&config_path)?;
    let value = RuntimeAgentClient::from_init_path(config.runtime.init)?.call(
        reqwest::Method::POST,
        "/v1/agents",
        Some(&json!({
            "schema": AGENT_ADMISSION_SCHEMA,
            "id": config.identity.id,
            "name": config.identity.name,
            "display_name": config.identity.display_name,
            "office": config.office,
            "provider": config.provider.kind,
            "model": config.provider.model,
            "endpoint": config.provider.endpoint
        })),
    )?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn load_agent_add_config(path: &std::path::Path) -> Result<AgentAddConfig> {
    let bytes = fs::read(path).with_context(|| format!("read agent config {}", path.display()))?;
    let mut config: AgentAddConfig = serde_yaml::from_slice(&bytes)
        .with_context(|| format!("parse agent config {}", path.display()))?;
    if config.schema != AGENT_CONFIG_SCHEMA {
        return Err(anyhow!("agent config schema must be {AGENT_CONFIG_SCHEMA}"));
    }
    validate_canonical_agent_name(&config.identity.name)?;
    for (field, value) in [
        ("identity.id", config.identity.id.as_str()),
        (
            "identity.display_name",
            config.identity.display_name.as_str(),
        ),
        ("office", config.office.as_str()),
        ("provider.kind", config.provider.kind.as_str()),
        ("provider.model", config.provider.model.as_str()),
        ("provider.endpoint", config.provider.endpoint.as_str()),
    ] {
        if value.trim().is_empty() || value.chars().any(char::is_control) {
            return Err(anyhow!("agent config {field} is invalid"));
        }
    }
    if config.runtime.init.is_relative() {
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
        config.runtime.init = parent.join(&config.runtime.init);
    }
    Ok(config)
}

fn validate_canonical_agent_name(name: &str) -> Result<()> {
    if !is_canonical_agent_name(name) {
        return Err(anyhow!(
            "agent identity.name must contain exactly two lowercase dot-separated neutral name segments"
        ));
    }
    Ok(())
}

fn csmctl_agent_read(args: &[String], agent_id: Option<&str>) -> Result<()> {
    let client = RuntimeAgentClient::from_args(args)?;
    let path = match agent_id {
        Some(id) => format!("/v1/agents/{}", safe_agent_id(id)?),
        None => "/v1/agents".to_owned(),
    };
    let value = client.call(reqwest::Method::GET, &path, None)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn csmctl_agent_remove(args: &[String]) -> Result<()> {
    let id = safe_agent_id(required_arg(args, "--id")?)?;
    let value = RuntimeAgentClient::from_args(args)?.call(
        reqwest::Method::DELETE,
        &format!("/v1/agents/{id}"),
        None,
    )?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn csmctl_agent_checkpoint(args: &[String]) -> Result<()> {
    let id = safe_agent_id(required_arg(args, "--id")?)?;
    let value = RuntimeAgentClient::from_args(args)?.call(
        reqwest::Method::POST,
        &format!("/v1/agents/{id}/checkpoint"),
        None,
    )?;
    if let Some(path) = optional_arg(args, "--out") {
        write_json_atomically(PathBuf::from(path).as_path(), &value)?;
    }
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn csmctl_agent_dehydrate(args: &[String]) -> Result<()> {
    let id = safe_agent_id(required_arg(args, "--id")?)?;
    let output = PathBuf::from(required_arg(args, "--out")?);
    let value = RuntimeAgentClient::from_args(args)?.call(
        reqwest::Method::POST,
        &format!("/v1/agents/{id}/dehydrate"),
        None,
    )?;
    write_json_atomically(&output, &value)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn csmctl_agent_migrate(args: &[String]) -> Result<()> {
    let id = safe_agent_id(required_arg(args, "--id")?)?;
    let output = PathBuf::from(required_arg(args, "--out")?);
    let client = RuntimeAgentClient::from_args(args)?;
    let bundle = client.call(
        reqwest::Method::POST,
        &format!("/v1/agents/{id}/dehydrate"),
        None,
    )?;
    write_json_atomically(&output, &bundle)?;
    let digest = bundle["bundle_digest"]
        .as_str()
        .ok_or_else(|| anyhow!("Runtime migration bundle omitted bundle_digest"))?;
    let committed = client.call(
        reqwest::Method::POST,
        &format!("/v1/agents/{id}/dehydrate/commit"),
        Some(&json!({"bundle_digest":digest})),
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"schema":"adl.csmctl.agent_migration.v1","status":"migrated","bundle":output,"source_commit":committed})
        )?
    );
    Ok(())
}

fn csmctl_agent_rehydrate(args: &[String]) -> Result<()> {
    let bundle_path = PathBuf::from(required_arg(args, "--bundle")?);
    let bundle: serde_json::Value =
        serde_json::from_slice(&fs::read(&bundle_path).with_context(|| {
            format!("read freeze-dried agent bundle {}", bundle_path.display())
        })?)
        .context("parse freeze-dried agent bundle")?;
    let value = RuntimeAgentClient::from_args(args)?.call(
        reqwest::Method::POST,
        "/v1/agents/rehydrate",
        Some(&bundle),
    )?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn safe_agent_id(value: &str) -> Result<&str> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b':' | b'_' | b'-'))
    {
        return Err(anyhow!("agent id is invalid"));
    }
    Ok(value)
}

fn write_json_atomically(path: &std::path::Path, value: &serde_json::Value) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("output path has no parent"))?;
    fs::create_dir_all(parent)
        .with_context(|| format!("create output directory {}", parent.display()))?;
    let sequence = AGENT_ARTIFACT_TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = path.with_extension(format!("json.{}.{}.tmp", std::process::id(), sequence));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)
        .with_context(|| format!("create temporary artifact {}", temp.display()))?;
    let commit = (|| -> Result<()> {
        file.write_all(&serde_json::to_vec_pretty(value)?)?;
        file.sync_all()?;
        fs::hard_link(&temp, path).with_context(|| {
            format!(
                "commit new artifact {} without replacing an existing artifact",
                path.display()
            )
        })?;
        fs::remove_file(&temp)
            .with_context(|| format!("remove temporary artifact {}", temp.display()))?;
        Ok(())
    })();
    if commit.is_err() {
        let _ = fs::remove_file(&temp);
    }
    commit?;
    File::open(parent)
        .with_context(|| format!("open output directory {}", parent.display()))?
        .sync_all()
        .with_context(|| format!("fsync output directory {}", parent.display()))?;
    Ok(())
}

fn required_arg<'a>(args: &'a [String], flag: &str) -> Result<&'a str> {
    optional_arg(args, flag)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow!("missing required {flag} <value>"))
}

fn real_api(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("get") => csmctl_api_get(&args[1..]),
        Some("credential") => csmctl_api_credential(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_api_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl api command '{other}'. Expected get, credential, or help.\n\n{}",
            csmctl_api_usage()
        )),
    }
}

fn csmctl_api_usage() -> &'static str {
    "csmctl api - authenticated CSM runtime API control plane\n\n\
Usage:\n\
  csmctl api get --spec <agent-spec.yaml> [--path /status] [--bind 127.0.0.1:19997]\n\
  csmctl api credential status --spec <agent-spec.yaml>\n\
  csmctl api credential rotate --spec <agent-spec.yaml>\n\
  csmctl api credential revoke --spec <agent-spec.yaml>\n\n\
Notes:\n\
  Credentials are read from the runtime state root, sent only in the Authorization header, and never printed.\n\
  Rotation and revocation are observed by the running CSM API without a restart."
}

fn csmctl_api_get(args: &[String]) -> Result<()> {
    const CONNECT_RETRY_ATTEMPTS: usize = 200;
    const CONNECT_RETRY_DELAY: Duration = Duration::from_millis(25);
    const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

    let spec = required_path_arg(args, "--spec")?;
    let bind = optional_arg(args, "--bind").unwrap_or("127.0.0.1:19997");
    let path = optional_arg(args, "--path").unwrap_or("/status");
    if !path.starts_with('/') || path.contains(['\r', '\n']) {
        return Err(anyhow!("csmctl api --path must be an absolute HTTP path"));
    }
    let addr: SocketAddr = bind
        .parse()
        .with_context(|| format!("parse csmctl API bind {bind}"))?;
    if !addr.ip().is_loopback() {
        return Err(anyhow!(
            "csmctl refuses to send the runtime API credential to a non-loopback address"
        ));
    }
    let loaded = load_spec(&spec).context("load CSM spec for authenticated API client")?;
    let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let url = format!("http://{bind}{path}");
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(REQUEST_TIMEOUT)
        .build()
        .context("build csmctl runtime API client")?;
    let response = store
        .with_bearer_token(|token| {
            for attempt in 0..CONNECT_RETRY_ATTEMPTS {
                match client.get(&url).bearer_auth(token).send() {
                    Ok(response) => return Ok(response),
                    Err(err) if err.is_connect() && attempt + 1 < CONNECT_RETRY_ATTEMPTS => {
                        std::thread::sleep(CONNECT_RETRY_DELAY);
                    }
                    Err(err) => return Err(err),
                }
            }
            unreachable!("bounded runtime API connection retry loop always returns")
        })
        .map_err(anyhow::Error::msg)?
        .context("call authenticated CSM runtime API")?;
    let status = response.status();
    let body = response.text().context("read CSM runtime API response")?;
    if !status.is_success() {
        return Err(anyhow!("CSM runtime API returned HTTP {status}: {body}"));
    }
    println!("{body}");
    Ok(())
}

fn csmctl_api_credential(args: &[String]) -> Result<()> {
    let action = args
        .first()
        .map(String::as_str)
        .ok_or_else(|| anyhow!("csmctl api credential requires status, rotate, or revoke"))?;
    let spec = required_path_arg(&args[1..], "--spec")?;
    let loaded = load_spec(&spec).context("load CSM spec for credential administration")?;
    let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
    let metadata = match action {
        "status" => {
            let metadata = store.metadata().map_err(anyhow::Error::msg)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "schema": "adl.csmctl.runtime_api_credential.v1",
                    "action": action,
                    "status": if metadata.is_some() { "present" } else { "missing" },
                    "credential": metadata,
                    "secret_printed": false
                }))?
            );
            return Ok(());
        }
        "rotate" => store.rotate(),
        "revoke" => store.revoke(),
        other => return Err(anyhow!("unknown credential action '{other}'")),
    }
    .map_err(anyhow::Error::msg)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema": "adl.csmctl.runtime_api_credential.v1",
            "action": action,
            "status": "completed",
            "credential": metadata,
            "secret_printed": false
        }))?
    );
    Ok(())
}

fn required_path_arg(args: &[String], flag: &str) -> Result<PathBuf> {
    optional_arg(args, flag)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing required {flag} <path>"))
}

fn optional_arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].as_str())
}

fn real_runtime(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("service") => {
            let mapped = runtime_service_args(args)?;
            delegate_to_csm(&mapped)
        }
        Some("governed-stop") | Some("continuity") | Some("backpressure") | Some("storage")
        | Some("observatory") => delegate_to_csm(args),
        Some("daemon") => Err(anyhow!(
            "csmctl does not execute the runtime daemon loop. Use `csm daemon ...` for direct runtime execution or `csmctl runtime service ...` for administered service control."
        )),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_runtime_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl runtime command '{other}'. Expected service, governed-stop, continuity, backpressure, storage, observatory, help, or --version.\n\n{}",
            csmctl_runtime_usage()
        )),
    }
}

fn csmctl_runtime_usage() -> &'static str {
    "csmctl runtime - administer CSM runtime-owned local surfaces\n\n\
Usage:\n\
  csmctl runtime service <install|start|status|stop|remove> ...\n\
  csmctl runtime governed-stop --spec <agent-spec.yaml> --reason <text> ...\n\
  csmctl runtime continuity <capture|stage|restore|drill> ...\n\
  csmctl runtime backpressure prove ...\n\
  csmctl runtime storage prove-s3 ...\n\
  csmctl runtime observatory --packet <visibility-packet.json> ...\n\n\
Notes:\n\
  csmctl runtime delegates administered operations to CSM-owned parsers so runtime semantics stay single-sourced.\n\
  The runtime API is embedded in csm daemon and administered through csmctl runtime service ... --api-bind.\n\
  Direct daemon-loop execution remains owned by `csm daemon`, not csmctl."
}

fn real_status(args: &[String]) -> Result<()> {
    if matches!(
        args.first().map(String::as_str),
        Some("--help" | "-h" | "help")
    ) {
        println!("{}", csmctl_status_usage());
        return Ok(());
    }
    let mut mapped = Vec::with_capacity(args.len() + 1);
    mapped.push("status".to_string());
    mapped.extend(args.iter().cloned());
    real_process(&mapped)
}

fn csmctl_status_usage() -> &'static str {
    "csmctl status - permission-safe CSM liveness check\n\n\
Usage:\n\
  csmctl status --pid <pid> [--json]\n\
  csmctl status --pid-file <path> [--json]\n\
  csmctl status --port <port> [--host 127.0.0.1|::1|localhost] [--json]\n\n\
Notes:\n\
  This is a thin control-plane alias for `adl process status` using exact metadata or exact loopback probes only."
}

fn real_diagnostics(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("process") => real_process(&args[1..]),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_diagnostics_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl diagnostics command '{other}'. Expected process or help.\n\n{}",
            csmctl_diagnostics_usage()
        )),
    }
}

fn csmctl_diagnostics_usage() -> &'static str {
    "csmctl diagnostics - CSM runtime diagnostic probes\n\n\
Usage:\n\
  csmctl diagnostics process status [--pid <pid>|--pid-file <path>|--port <port>] [--json]\n\n\
Notes:\n\
  Diagnostics are intentionally permission-safe and do not use broad host process scans."
}

fn real_cloud(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("aws-signal") | Some("cloud-control") => delegate_to_csm(args),
        Some("--help" | "-h" | "help") | None => {
            println!("{}", csmctl_cloud_usage());
            Ok(())
        }
        Some(other) => Err(anyhow!(
            "unknown csmctl cloud command '{other}'. Expected aws-signal, cloud-control, or help.\n\n{}",
            csmctl_cloud_usage()
        )),
    }
}

fn csmctl_cloud_usage() -> &'static str {
    "csmctl cloud - governed CSM cloud-control surfaces\n\n\
Usage:\n\
  csmctl cloud aws-signal acip-sns-proof --out <proof-dir> ...\n\
  csmctl cloud cloud-control cloudfront-status --out <proof-dir> ...\n\n\
Notes:\n\
  Cloud operations use the same CSM runtime-owned parsers and Agent Logic AWS guardrails as `csm`."
}

fn delegate_to_csm(args: &[String]) -> Result<()> {
    real_csm_standalone(args)
}

fn runtime_service_args(args: &[String]) -> Result<Vec<String>> {
    if args.get(1).map(String::as_str) != Some("install") || has_flag(args, "--csm-bin") {
        return Ok(args.to_vec());
    }
    let mut mapped = args.to_vec();
    mapped.push("--csm-bin".to_string());
    mapped.push(default_csm_owner_binary()?.display().to_string());
    Ok(mapped)
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn default_csm_owner_binary() -> Result<PathBuf> {
    let current = std::env::current_exe().context("resolve current csmctl executable")?;
    let parent = current
        .parent()
        .ok_or_else(|| anyhow!("current executable has no parent directory"))?;
    Ok(parent.join(format!("csm{}", std::env::consts::EXE_SUFFIX)))
}

#[cfg(test)]
mod tests {
    use super::{
        csmctl_agent_usage, csmctl_api_get, csmctl_api_usage, csmctl_cloud_usage,
        csmctl_diagnostics_usage, csmctl_runtime_usage, csmctl_status_usage, csmctl_usage,
        load_agent_add_config, real_csmctl, runtime_service_args, safe_agent_id,
        validate_canonical_agent_name, write_json_atomically, RuntimeAgentClient,
    };
    use adl::csm_runtime_api::{serve_runtime_api, CsmRuntimeApiOptions};
    use adl_runtime::runtime_api_auth::RuntimeApiCredentialStore;
    use serde_json::{json, Value};
    use std::fs;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn temp_root(prefix: &str) -> PathBuf {
        let seq = TEMP_SEQ.fetch_add(1, Ordering::SeqCst);
        let root =
            std::env::temp_dir().join(format!("adl-csmctl-{prefix}-{}-{seq}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp root");
        root
    }

    fn write_spec(root: &std::path::Path) -> PathBuf {
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: csmctl-service-agent
display_name: CSMCTL Service Agent
state_root: state
workflow:
  kind: demo_adapter
  name: csmctl_service_probe
  run_args: {}
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
"#,
        )
        .expect("write spec");
        spec
    }

    struct GovernedCsmTestPort {
        bind: String,
        lock_dir: PathBuf,
    }

    impl Drop for GovernedCsmTestPort {
        fn drop(&mut self) {
            let _ = fs::remove_dir(&self.lock_dir);
        }
    }

    fn reserve_governed_csm_test_port(label: &str) -> GovernedCsmTestPort {
        let start = ((std::process::id() as u64)
            .wrapping_add(TEMP_SEQ.fetch_add(1, Ordering::SeqCst)))
            % 50;
        let lock_root = std::env::current_dir()
            .expect("resolve current test directory")
            .join(".adl")
            .join("test-port-locks")
            .join("csm");
        fs::create_dir_all(&lock_root).expect("create governed CSM test port lock root");
        for offset in 0..50 {
            let port = 19_950 + ((start + offset) % 50) as u16;
            let lock_dir = lock_root.join(format!("port-{port}.lock"));
            if fs::create_dir(&lock_dir).is_err() {
                continue;
            }
            match TcpListener::bind(("127.0.0.1", port)) {
                Ok(listener) => {
                    let bind = listener.local_addr().expect("read governed CSM test port");
                    drop(listener);
                    return GovernedCsmTestPort {
                        bind: bind.to_string(),
                        lock_dir,
                    };
                }
                Err(_) => {
                    let _ = fs::remove_dir(&lock_dir);
                }
            }
        }
        panic!("could not bind one governed CSM test port for {label} in 19950-19999");
    }

    fn assert_err_contains(result: anyhow::Result<()>, needle: &str) {
        let err = result.expect_err("expected error");
        assert!(
            err.to_string().contains(needle),
            "expected {needle:?} in {err}"
        );
    }

    #[test]
    fn csmctl_usage_documents_modular_runtime_control_plane() {
        let usage = csmctl_usage();
        assert!(usage.contains("csmctl runtime service"));
        assert!(usage.contains("csmctl api get"));
        assert!(usage.contains("csmctl status"));
        assert!(usage.contains("csmctl diagnostics process status"));
        assert!(usage.contains("csmctl cloud aws-signal"));
        assert!(usage.contains("csm is the runtime owner"));
        assert!(usage.contains("adl remains ADL language"));
        assert!(!usage.contains("adl compile"));
        assert!(usage.contains("csdlc-install"));
        assert!(usage.contains("independent typed v2 binaries"));
    }

    #[test]
    fn csmctl_authenticated_api_client_uses_runtime_owned_credential() {
        let root = temp_root("api-client");
        let spec = write_spec(&root);
        let port = reserve_governed_csm_test_port("api-client");
        let bind = port.bind.clone();
        let server_spec = spec.clone();
        let server_bind = bind.clone();
        let server = std::thread::spawn(move || {
            serve_runtime_api(CsmRuntimeApiOptions {
                spec_path: server_spec,
                bind: server_bind,
                test_max_requests: Some(1),
                idle_timeout_ms: Some(5_000),
                shutdown_file: None,
                otel_status_path: None,
                otel_log_path: None,
            })
        });
        let loaded = adl::long_lived_agent::load_spec(&spec).unwrap();
        let store = RuntimeApiCredentialStore::for_state_root(&loaded.state_root);
        for _ in 0..100 {
            if store.path().exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        csmctl_api_get(&[
            "--spec".to_string(),
            spec.display().to_string(),
            "--bind".to_string(),
            bind,
            "--path".to_string(),
            "/status".to_string(),
        ])
        .expect("csmctl authenticated API request");
        let result = server.join().unwrap().unwrap();
        assert_eq!(result.served_requests, 1);
    }

    #[test]
    fn csmctl_authenticated_api_client_waits_for_slow_listener_startup() {
        let root = temp_root("api-client-slow-listener");
        let spec = write_spec(&root);
        let port = reserve_governed_csm_test_port("api-client-slow-listener");
        let bind = port.bind.clone();
        let loaded = adl::long_lived_agent::load_spec(&spec).unwrap();
        RuntimeApiCredentialStore::for_state_root(&loaded.state_root)
            .ensure()
            .expect("pre-create runtime API credential");

        let client_spec = spec.clone();
        let client_bind = bind.clone();
        let client = std::thread::spawn(move || {
            csmctl_api_get(&[
                "--spec".to_string(),
                client_spec.display().to_string(),
                "--bind".to_string(),
                client_bind,
                "--path".to_string(),
                "/status".to_string(),
            ])
        });

        std::thread::sleep(std::time::Duration::from_millis(750));
        let result = serve_runtime_api(CsmRuntimeApiOptions {
            spec_path: spec,
            bind,
            test_max_requests: Some(1),
            idle_timeout_ms: Some(5_000),
            shutdown_file: None,
            otel_status_path: None,
            otel_log_path: None,
        })
        .expect("serve delayed runtime API request");

        client
            .join()
            .expect("join delayed csmctl API client")
            .expect("csmctl API client waits for listener startup");
        assert_eq!(result.served_requests, 1);
    }

    #[test]
    fn csmctl_api_refuses_to_send_credentials_off_loopback() {
        let root = temp_root("api-non-loopback");
        let spec = write_spec(&root);
        let err = csmctl_api_get(&[
            "--spec".to_string(),
            spec.display().to_string(),
            "--bind".to_string(),
            "192.0.2.1:19997".to_string(),
        ])
        .unwrap_err();
        assert!(err.to_string().contains("non-loopback"));
        assert!(csmctl_api_usage().contains("never printed"));
    }

    #[test]
    fn csmctl_help_and_unknown_module_paths_are_bounded_to_admin_surface() {
        assert!(real_csmctl(&args(&[])).is_ok());
        assert!(real_csmctl(&args(&["help"])).is_ok());
        assert!(real_csmctl(&args(&["--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["compile"])),
            "unknown csmctl module 'compile'",
        );
    }

    #[test]
    fn csmctl_agent_lifecycle_is_complete_and_bounded() {
        let usage = csmctl_agent_usage();
        for command in [
            "agent add",
            "agent list",
            "agent get",
            "agent checkpoint",
            "agent dehydrate",
            "agent migrate",
            "agent rehydrate",
            "agent remove",
        ] {
            assert!(
                usage.contains(command),
                "missing lifecycle command {command}"
            );
        }
        assert!(usage.contains("fsynced") || usage.contains("written atomically"));
        assert_err_contains(
            real_csmctl(&args(&["agent", "add", "--id", "incomplete"])),
            "requires exactly --config",
        );
        assert!(safe_agent_id("gemma-e4b").is_ok());
        assert!(safe_agent_id("../shepherd").is_err());
    }

    #[test]
    fn csmctl_agent_commands_reject_incomplete_or_unsafe_requests() {
        for (command, expected) in [
            ("list", "missing required --init"),
            ("get", "missing required --id"),
            ("remove", "missing required --id"),
            ("checkpoint", "missing required --id"),
            ("dehydrate", "missing required --id"),
            ("migrate", "missing required --id"),
            ("rehydrate", "missing required --bundle"),
        ] {
            assert_err_contains(real_csmctl(&args(&["agent", command])), expected);
        }
        for command in ["remove", "checkpoint", "dehydrate", "migrate"] {
            assert_err_contains(
                real_csmctl(&args(&["agent", command, "--id", "../escape"])),
                "agent id is invalid",
            );
        }
        assert!(real_csmctl(&args(&["agent"])).is_ok());
        assert!(real_csmctl(&args(&["agent", "help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["agent", "invent"])),
            "unknown csmctl agent command 'invent'",
        );
    }

    #[test]
    fn csmctl_agent_artifacts_are_committed_atomically_in_repo_fixture() {
        let root = temp_root("agent-artifact-atomic");
        let path = root.join("freeze-dried-agent.json");
        let value = json!({"schema":"test","bundle_digest":"abc"});
        write_json_atomically(&path, &value).expect("atomic artifact write");
        let observed: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(observed, value);
        assert!(write_json_atomically(&path, &json!({"replacement":true})).is_err());
        let observed: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(observed, value);
        assert!(fs::read_dir(&root).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")));
    }

    #[test]
    fn csmctl_agent_add_config_separates_identity_from_runtime_binding() {
        let root = temp_root("agent-config");
        let config_path = root.join("ember.axioma.yaml");
        fs::write(
            &config_path,
            r#"schema: adl.csm.agent_config.v1
runtime:
  init: runtime-init.toml
identity:
  id: ember-axioma
  name: ember.axioma
  display_name: Ember Axioma
office: shepherd
provider:
  kind: ollama
  model: gemma4:e4b-mlx
  endpoint: http://nessus.local:11434
"#,
        )
        .expect("write agent config");
        let config = load_agent_add_config(&config_path).expect("load agent config");
        assert_eq!(config.identity.name, "ember.axioma");
        assert_eq!(config.identity.display_name, "Ember Axioma");
        assert_eq!(config.office, "shepherd");
        assert_eq!(config.provider.kind, "ollama");
        assert_eq!(config.provider.model, "gemma4:e4b-mlx");
        assert_eq!(config.runtime.init, root.join("runtime-init.toml"));

        fs::write(
            &config_path,
            r#"schema: adl.csm.agent_config.v1
runtime: { init: runtime-init.toml }
identity: { id: gemma-e4b, name: Gemma, display_name: Gemma }
office: assistant
provider: { kind: ollama, model: gemma4:e4b-mlx, endpoint: http://localhost:11434 }
"#,
        )
        .expect("replace invalid config");
        assert!(load_agent_add_config(&config_path)
            .expect_err("single model-bound name rejected")
            .to_string()
            .contains("exactly two lowercase dot-separated"));
        for invalid_name in [
            "ember.axioma.local",
            "ember.",
            "ember.-axioma",
            "ember.axioma-",
        ] {
            assert!(
                validate_canonical_agent_name(invalid_name).is_err(),
                "{invalid_name}"
            );
        }
    }

    #[test]
    fn csmctl_agent_add_config_rejects_invalid_schema_and_fields() {
        let root = temp_root("agent-config-invalid");
        let config_path = root.join("agent.yaml");
        let valid = |schema: &str,
                     id: &str,
                     display: &str,
                     office: &str,
                     kind: &str,
                     model: &str,
                     endpoint: &str| {
            format!(
                "schema: {schema}\nruntime: {{ init: runtime-init.toml }}\nidentity: {{ id: {id:?}, name: ember.axioma, display_name: {display:?} }}\noffice: {office:?}\nprovider: {{ kind: {kind:?}, model: {model:?}, endpoint: {endpoint:?} }}\n"
            )
        };
        for (contents, expected) in [
            (
                valid(
                    "wrong.schema",
                    "ember",
                    "Ember",
                    "assistant",
                    "ollama",
                    "gemma",
                    "http://localhost",
                ),
                "agent config schema",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "",
                    "Ember",
                    "assistant",
                    "ollama",
                    "gemma",
                    "http://localhost",
                ),
                "identity.id",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "ember",
                    "",
                    "assistant",
                    "ollama",
                    "gemma",
                    "http://localhost",
                ),
                "identity.display_name",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "ember",
                    "Ember",
                    "",
                    "ollama",
                    "gemma",
                    "http://localhost",
                ),
                "office",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "ember",
                    "Ember",
                    "assistant",
                    "",
                    "gemma",
                    "http://localhost",
                ),
                "provider.kind",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "ember",
                    "Ember",
                    "assistant",
                    "ollama",
                    "",
                    "http://localhost",
                ),
                "provider.model",
            ),
            (
                valid(
                    "adl.csm.agent_config.v1",
                    "ember",
                    "Ember",
                    "assistant",
                    "ollama",
                    "gemma",
                    "",
                ),
                "provider.endpoint",
            ),
        ] {
            fs::write(&config_path, contents).expect("write invalid config");
            let error = load_agent_add_config(&config_path).expect_err("reject invalid config");
            assert!(
                error.to_string().contains(expected),
                "expected {expected:?} in {error}"
            );
        }
        fs::write(&config_path, "not: [valid").expect("write malformed yaml");
        assert!(load_agent_add_config(&config_path)
            .expect_err("reject malformed yaml")
            .to_string()
            .contains("parse agent config"));
    }

    #[test]
    fn csmctl_agent_client_validates_init_tls_and_write_credential() {
        let root = temp_root("agent-client-config");
        let init_path = root.join("runtime-init.toml");
        let root_text = root.display().to_string();
        let init = include_str!("../../../infra/runtime-v3/runtime-init.toml")
            .replace("/var/lib/adl/runtime-v3", &root_text)
            .replace(
                "/opt/adl/bin/adl-runtime-kernel",
                &root.join("adl-runtime-kernel").display().to_string(),
            )
            .replace(
                "/opt/adl/bin/vector",
                &root.join("vector").display().to_string(),
            );
        fs::write(&init_path, init).expect("write Runtime init");

        let error = RuntimeAgentClient::from_init_path(init_path.clone())
            .err()
            .expect("missing trust roots must fail");
        assert!(error.to_string().contains("read Runtime trust roots"));

        let tls = root.join("tls");
        fs::create_dir_all(&tls).expect("create tls directory");
        let certified =
            rcgen::generate_simple_self_signed(vec!["runtime.dev.agent-logic.ai".to_owned()])
                .expect("generate test certificate");
        fs::write(tls.join("trust-roots.pem"), certified.cert.pem()).expect("write valid roots");
        let error = RuntimeAgentClient::from_init_path(init_path.clone())
            .err()
            .expect("missing write credential must fail");
        assert!(error.to_string().contains("read Runtime write credential"));

        let credentials = root.join("credentials");
        fs::create_dir_all(&credentials).expect("create credentials directory");
        fs::write(credentials.join("acip-write-token.txt"), "bad token\nvalue")
            .expect("write invalid token");
        let error = RuntimeAgentClient::from_init_path(init_path.clone())
            .err()
            .expect("whitespace-bearing write credential must fail");
        assert!(error.to_string().contains("write credential is invalid"));

        fs::write(
            credentials.join("acip-write-token.txt"),
            "test-write-token\n",
        )
        .expect("write valid token");
        let client = RuntimeAgentClient::from_init_path(init_path)
            .expect("build lifecycle client from governed init");
        assert_eq!(client.write_token, "test-write-token");
        assert!(client
            .base_url
            .starts_with("https://runtime.dev.agent-logic.ai:"));
    }

    #[test]
    fn csmctl_agent_lifecycle_commands_call_the_authenticated_runtime_api() {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let root = temp_root("agent-lifecycle-api");
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind lifecycle API fixture");
        let address = listener.local_addr().expect("read lifecycle API address");
        let certified =
            rcgen::generate_simple_self_signed(vec!["runtime.dev.agent-logic.ai".to_owned()])
                .expect("generate lifecycle API certificate");
        let certificate = rustls::pki_types::CertificateDer::from(certified.cert.der().to_vec());
        let private_key = rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der()),
        );
        let server_config = std::sync::Arc::new(
            rustls::ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(vec![certificate], private_key)
                .expect("build lifecycle API TLS config"),
        );
        listener
            .set_nonblocking(true)
            .expect("bound fixture accept timeout");
        let server = std::thread::spawn(move || {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
            let mut observed = Vec::new();
            while observed.len() < 9 && std::time::Instant::now() < deadline {
                let (stream, _) = match listener.accept() {
                    Ok(accepted) => accepted,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        continue;
                    }
                    Err(error) => panic!("accept lifecycle API request: {error}"),
                };
                stream
                    .set_nonblocking(false)
                    .expect("restore blocking fixture stream");
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                    .expect("bound fixture read timeout");
                let connection =
                    rustls::ServerConnection::new(server_config.clone()).expect("TLS connection");
                let mut tls = rustls::StreamOwned::new(connection, stream);
                let mut request = Vec::new();
                let mut buffer = [0_u8; 4096];
                loop {
                    let count = tls.read(&mut buffer).expect("read lifecycle API request");
                    if count == 0 {
                        break;
                    }
                    request.extend_from_slice(&buffer[..count]);
                    if let Some(header_end) = request
                        .windows(4)
                        .position(|window| window == b"\r\n\r\n")
                        .map(|position| position + 4)
                    {
                        let headers = String::from_utf8_lossy(&request[..header_end]);
                        let content_length = headers
                            .lines()
                            .find_map(|line| {
                                line.to_ascii_lowercase()
                                    .strip_prefix("content-length:")
                                    .and_then(|value| value.trim().parse::<usize>().ok())
                            })
                            .unwrap_or(0);
                        if request.len() >= header_end + content_length {
                            break;
                        }
                    }
                }
                let request = String::from_utf8(request).expect("UTF-8 lifecycle request");
                let request_lower = request.to_ascii_lowercase();
                assert!(request_lower.contains("authorization: bearer test-write-token"));
                assert!(!request
                    .split("\r\n\r\n")
                    .nth(1)
                    .unwrap_or("")
                    .contains("test-write-token"));
                observed.push(request);
                let body = r#"{"schema":"test","status":"ok","bundle_digest":"abc"}"#;
                write!(
                    tls,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
                .expect("write lifecycle API response");
                tls.flush().expect("flush lifecycle API response");
            }
            observed
        });

        let tls = root.join("tls");
        let credentials = root.join("credentials");
        fs::create_dir_all(&tls).expect("create lifecycle tls directory");
        fs::create_dir_all(&credentials).expect("create lifecycle credentials directory");
        fs::write(tls.join("trust-roots.pem"), certified.cert.pem())
            .expect("write lifecycle trust root");
        fs::write(credentials.join("acip-write-token.txt"), "test-write-token")
            .expect("write lifecycle token");
        let root_text = root.display().to_string();
        let init = include_str!("../../../infra/runtime-v3/runtime-init.toml")
            .replace("/var/lib/adl/runtime-v3", &root_text)
            .replace("127.0.0.1:20997", &address.to_string())
            .replace(
                "/opt/adl/bin/adl-runtime-kernel",
                &root.join("adl-runtime-kernel").display().to_string(),
            )
            .replace(
                "/opt/adl/bin/vector",
                &root.join("vector").display().to_string(),
            );
        let init_path = root.join("runtime-init.toml");
        fs::write(&init_path, init).expect("write lifecycle Runtime init");
        let init_arg = init_path.display().to_string();
        let config_path = root.join("ember.axioma.yaml");
        fs::write(
            &config_path,
            format!(
                "schema: adl.csm.agent_config.v1\nruntime: {{ init: {:?} }}\nidentity: {{ id: ember-axioma, name: ember.axioma, display_name: Ember Axioma }}\noffice: assistant\nprovider: {{ kind: ollama, model: gemma4:e4b-mlx, endpoint: http://nessus.local:11434 }}\n",
                init_path
            ),
        )
        .expect("write lifecycle agent config");
        let bundle = root.join("bundle.json");
        fs::write(&bundle, r#"{"schema":"test","bundle_digest":"abc"}"#)
            .expect("write rehydrate bundle");
        let checkpoint = root.join("checkpoint.json");
        let dehydration = root.join("dehydrated.json");
        let migration = root.join("migrated.json");
        let checkpoint_arg = checkpoint.display().to_string();
        let dehydration_arg = dehydration.display().to_string();
        let migration_arg = migration.display().to_string();
        let bundle_arg = bundle.display().to_string();

        real_csmctl(&args(&[
            "agent",
            "add",
            "--config",
            &config_path.display().to_string(),
        ]))
        .expect("add agent");
        for command in [
            vec!["agent", "list", "--init", &init_arg],
            vec!["agent", "get", "--init", &init_arg, "--id", "ember-axioma"],
            vec![
                "agent",
                "checkpoint",
                "--init",
                &init_arg,
                "--id",
                "ember-axioma",
                "--out",
                &checkpoint_arg,
            ],
            vec![
                "agent",
                "dehydrate",
                "--init",
                &init_arg,
                "--id",
                "ember-axioma",
                "--out",
                &dehydration_arg,
            ],
            vec![
                "agent",
                "migrate",
                "--init",
                &init_arg,
                "--id",
                "ember-axioma",
                "--out",
                &migration_arg,
            ],
            vec![
                "agent",
                "rehydrate",
                "--init",
                &init_arg,
                "--bundle",
                &bundle_arg,
            ],
            vec![
                "agent",
                "remove",
                "--init",
                &init_arg,
                "--id",
                "ember-axioma",
            ],
        ] {
            real_csmctl(&args(&command)).expect("execute lifecycle command");
        }
        let observed = server.join().expect("join lifecycle API fixture");
        assert_eq!(observed.len(), 9, "fixture timed out before all requests");
        let request_parts = observed
            .iter()
            .map(|request| {
                let (headers, body) = request.split_once("\r\n\r\n").expect("request framing");
                (headers.lines().next().expect("request line"), body)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            request_parts
                .iter()
                .map(|(line, _)| *line)
                .collect::<Vec<_>>(),
            [
                "POST /v1/agents HTTP/1.1",
                "GET /v1/agents HTTP/1.1",
                "GET /v1/agents/ember-axioma HTTP/1.1",
                "POST /v1/agents/ember-axioma/checkpoint HTTP/1.1",
                "POST /v1/agents/ember-axioma/dehydrate HTTP/1.1",
                "POST /v1/agents/ember-axioma/dehydrate HTTP/1.1",
                "POST /v1/agents/ember-axioma/dehydrate/commit HTTP/1.1",
                "POST /v1/agents/rehydrate HTTP/1.1",
                "DELETE /v1/agents/ember-axioma HTTP/1.1",
            ]
        );
        let add: Value = serde_json::from_str(request_parts[0].1).expect("parse add body");
        assert_eq!(add["id"], "ember-axioma");
        assert_eq!(add["name"], "ember.axioma");
        assert_eq!(add["model"], "gemma4:e4b-mlx");
        let commit: Value =
            serde_json::from_str(request_parts[6].1).expect("parse migration commit body");
        assert_eq!(commit, json!({"bundle_digest":"abc"}));
        let rehydrate: Value =
            serde_json::from_str(request_parts[7].1).expect("parse rehydrate body");
        assert_eq!(rehydrate["bundle_digest"], "abc");
        assert!(checkpoint.exists());
        assert!(dehydration.exists());
        assert!(migration.exists());
    }

    #[test]
    fn csmctl_module_usage_surfaces_document_owned_boundaries() {
        assert!(csmctl_runtime_usage().contains("Direct daemon-loop execution"));
        assert!(csmctl_status_usage().contains("exact metadata or exact loopback probes"));
        assert!(csmctl_diagnostics_usage().contains("permission-safe"));
        assert!(csmctl_cloud_usage().contains("Agent Logic AWS guardrails"));
    }

    #[test]
    fn csmctl_rejects_direct_daemon_execution() {
        assert_err_contains(
            real_csmctl(&args(&["runtime", "daemon", "--help"])),
            "does not execute the runtime daemon loop",
        );
    }

    #[test]
    fn csmctl_rejects_removed_standalone_runtime_api_route() {
        assert_err_contains(
            real_csmctl(&args(&["runtime", "api", "--help"])),
            "unknown csmctl runtime command 'api'",
        );
    }

    #[test]
    fn csmctl_runtime_help_and_unknown_paths_stay_runtime_scoped() {
        assert!(real_csmctl(&args(&["runtime"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["runtime", "compile"])),
            "unknown csmctl runtime command 'compile'",
        );
    }

    #[test]
    fn csmctl_runtime_help_paths_delegate_to_csm_owned_parsers() {
        assert!(real_csmctl(&args(&["runtime", "service", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "governed-stop", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "continuity", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "backpressure", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "storage", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["runtime", "observatory", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "aws-signal", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "cloud-control", "--help"])).is_ok());
    }

    #[test]
    fn csmctl_status_and_diagnostics_help_paths_are_permission_safe() {
        assert!(real_csmctl(&args(&["status", "--help"])).is_ok());
        assert!(real_csmctl(&args(&["diagnostics"])).is_ok());
        assert!(real_csmctl(&args(&["diagnostics", "help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["diagnostics", "scan"])),
            "unknown csmctl diagnostics command 'scan'",
        );
    }

    #[test]
    fn csmctl_status_requires_exact_target() {
        assert_err_contains(
            real_csmctl(&args(&["status", "--json"])),
            "requires exactly one of --pid, --pid-file, --port, or --name",
        );
    }

    #[test]
    fn csmctl_cloud_help_and_unknown_paths_stay_cloud_scoped() {
        assert!(real_csmctl(&args(&["cloud"])).is_ok());
        assert!(real_csmctl(&args(&["cloud", "--help"])).is_ok());
        assert_err_contains(
            real_csmctl(&args(&["cloud", "billing"])),
            "unknown csmctl cloud command 'billing'",
        );
    }

    #[test]
    fn csmctl_service_args_only_default_install_without_explicit_csm_bin() {
        let explicit = args(&["service", "install", "--csm-bin", "/tmp/csm"]);
        assert_eq!(
            runtime_service_args(&explicit).expect("explicit csm-bin args"),
            explicit
        );

        let status = args(&["service", "status"]);
        assert_eq!(
            runtime_service_args(&status).expect("service status args"),
            status
        );

        let install = args(&["service", "install"]);
        let mapped = runtime_service_args(&install).expect("default install args");
        assert_eq!(&mapped[..2], &install[..]);
        assert_eq!(mapped[mapped.len() - 2], "--csm-bin");
        assert!(
            mapped
                .last()
                .expect("default csm binary")
                .ends_with(&format!("csm{}", std::env::consts::EXE_SUFFIX)),
            "install should default to csm owner binary: {mapped:?}"
        );
    }

    #[test]
    fn csmctl_service_install_defaults_managed_binary_to_csm_owner() {
        let root = temp_root("service-default-csm");
        let spec = write_spec(&root);
        let service_root = root.join("service");
        let args = vec![
            "runtime".to_string(),
            "service".to_string(),
            "install".to_string(),
            "--spec".to_string(),
            spec.display().to_string(),
            "--service-root".to_string(),
            service_root.display().to_string(),
            "--manager".to_string(),
            "local".to_string(),
            "--json".to_string(),
        ];
        real_csmctl(&args).expect("install service through csmctl");

        let manifest: Value = serde_json::from_str(
            &fs::read_to_string(service_root.join("service_manifest.json"))
                .expect("read service manifest"),
        )
        .expect("parse service manifest");
        let csm_bin = manifest["csm_bin"].as_str().expect("manifest csm_bin");
        assert!(
            csm_bin.ends_with(&format!("csm{}", std::env::consts::EXE_SUFFIX)),
            "csmctl service install must configure csm daemon owner, got {csm_bin}"
        );
        assert!(
            !csm_bin.ends_with(&format!("csmctl{}", std::env::consts::EXE_SUFFIX)),
            "csmctl must not become the managed daemon executable"
        );
    }
}
