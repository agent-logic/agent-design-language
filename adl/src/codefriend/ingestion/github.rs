//! Read-only GitHub Git-data acquisition. No checkout, hooks, filters or tools.
//! Repository instructions are inert source bytes. Provenance is separate from
//! the transport-independent packet identity.
use super::{
    digest, unsafe_content, validate_repository, AdmissionInput, GitObjectFormat, Object, Packet,
    Scope, SCHEMA,
};
use anyhow::{bail, ensure, Result};
use base64::Engine;
use reqwest::{blocking::Client, header, redirect::Policy};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
    time::{Duration, Instant},
};

const META_LIMIT: u64 = 2 * 1024 * 1024;
const MAX_REQUESTS: usize = 4096;
const MAX_TRANSPORT_BYTES: u64 = 8 * 1024 * 1024;
const CAPTURE_TIMEOUT: Duration = Duration::from_secs(120);

/// Tokens are borrowed from the caller's approved resolver, never serialized.
/// The fixture constructor deliberately cannot receive credentials.
pub struct Transport {
    client: Client,
    base: String,
    fixture: bool,
    started: Instant,
    requests: usize,
    response_bytes: u64,
}
impl Transport {
    pub fn github(token: Option<&str>) -> Result<Self> {
        Self::build("https://api.github.com", token, false)
    }
    pub fn fixture(base: &str) -> Result<Self> {
        let url =
            reqwest::Url::parse(base).map_err(|_| anyhow::anyhow!("invalid_fixture_endpoint"))?;
        ensure!(
            url.scheme() == "http"
                && url
                    .host_str()
                    .is_some_and(|h| h == "127.0.0.1" || h == "[::1]")
                && url.port().is_some()
                && url.path() == "/"
                && url.username().is_empty()
                && url.password().is_none()
                && url.query().is_none()
                && url.fragment().is_none(),
            "invalid_fixture_endpoint"
        );
        Self::build(base.trim_end_matches('/'), None, true)
    }
    fn build(base: &str, token: Option<&str>, fixture: bool) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static("2022-11-28"),
        );
        if let Some(token) = token {
            ensure!(
                !token.is_empty()
                    && token.len() <= 4096
                    && token.bytes().all(|b| b.is_ascii_graphic()),
                "invalid_github_credential"
            );
            let mut value = header::HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|_| anyhow::anyhow!("invalid_github_credential"))?;
            value.set_sensitive(true);
            headers.insert(header::AUTHORIZATION, value);
        }
        let client = Client::builder()
            .default_headers(headers)
            .user_agent("adl-codefriend-readonly")
            .redirect(Policy::none())
            .no_proxy()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(20))
            .build()
            .map_err(|_| anyhow::anyhow!("github_transport_setup_failed"))?;
        Ok(Self {
            client,
            base: base.into(),
            fixture,
            started: Instant::now(),
            requests: 0,
            response_bytes: 0,
        })
    }
    fn get(&mut self, path: &str, limit: u64) -> Result<Value> {
        ensure!(
            self.requests < MAX_REQUESTS && self.started.elapsed() < CAPTURE_TIMEOUT,
            "github_capture_limit_exceeded"
        );
        self.requests += 1;
        let timeout = CAPTURE_TIMEOUT
            .saturating_sub(self.started.elapsed())
            .min(Duration::from_secs(20));
        let response = self
            .client
            .get(format!("{}{path}", self.base))
            .timeout(timeout)
            .send()
            .map_err(|_| anyhow::anyhow!("github_transport_failed_or_timeout"))?;
        match response.status().as_u16() {
            200 => (),
            401 => bail!("github_authentication_failed"),
            403 if response
                .headers()
                .get("x-ratelimit-remaining")
                .is_some_and(|v| v == "0") =>
            {
                bail!("github_rate_limited")
            }
            403 => bail!("github_forbidden"),
            404 => bail!("github_not_found"),
            429 => bail!("github_rate_limited"),
            300..=399 => bail!("github_redirect_rejected"),
            _ => bail!("github_http_failure"),
        }
        ensure!(
            !response.headers().contains_key(header::LINK),
            "github_pagination_rejected"
        );
        ensure!(
            response.content_length().is_none_or(|n| n <= limit),
            "github_response_limit_exceeded"
        );
        let remaining = MAX_TRANSPORT_BYTES.saturating_sub(self.response_bytes);
        ensure!(
            response.content_length().is_none_or(|n| n <= remaining),
            "github_capture_byte_limit_exceeded"
        );
        let mut bytes = Vec::new();
        response
            .take(limit.min(remaining) + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("github_response_read_failed"))?;
        ensure!(
            bytes.len() as u64 <= limit,
            "github_response_limit_exceeded"
        );
        ensure!(
            bytes.len() as u64 <= remaining,
            "github_capture_byte_limit_exceeded"
        );
        self.response_bytes += bytes.len() as u64;
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("github_invalid_response"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Input {
    Commit(String),
    PullRequest(u64),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub schema: String,
    pub transport: String,
    pub requested_repository: String,
    pub original_ref: String,
    pub source_repository: String,
    pub resolved_commit: String,
    pub packet_id: String,
    pub head_rechecked: bool,
    pub requests: usize,
}
pub struct Acquisition {
    pub packet: Packet,
    pub provenance: Provenance,
}
fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("github_incomplete_response"))
}
fn sha(value: &str) -> Result<()> {
    ensure!(
        value.len() == 40 && super::object_id(value),
        "github_exact_sha1_required"
    );
    Ok(())
}
fn repository_slug(repository: &str) -> Result<&str> {
    validate_repository(repository)?;
    let slug = repository
        .strip_prefix("https://github.com/")
        .ok_or_else(|| anyhow::anyhow!("github_repository_required"))?;
    ensure!(slug.split('/').count() == 2, "github_repository_required");
    Ok(slug)
}
fn verify_repository(transport: &mut Transport, repository: &str) -> Result<()> {
    let slug = repository_slug(repository)?;
    let value = transport.get(&format!("/repos/{slug}"), META_LIMIT)?;
    ensure!(
        string(&value, "full_name")? == slug && string(&value, "html_url")? == repository,
        "github_repository_mismatch"
    );
    Ok(())
}
fn pull_head(transport: &mut Transport, repository: &str, number: u64) -> Result<(String, String)> {
    ensure!(number > 0, "invalid_pull_request_number");
    let value = transport.get(
        &format!("/repos/{}/pulls/{number}", repository_slug(repository)?),
        META_LIMIT,
    )?;
    ensure!(
        value["number"].as_u64() == Some(number)
            && value["base"]["repo"]["html_url"].as_str() == Some(repository),
        "github_pull_request_mismatch"
    );
    let source = string(&value["head"]["repo"], "html_url")?.to_string();
    let slug = repository_slug(&source)?;
    ensure!(
        value["head"]["repo"]["full_name"].as_str() == Some(slug),
        "github_repository_mismatch"
    );
    let revision = string(&value["head"], "sha")?.to_string();
    sha(&revision)?;
    Ok((source, revision))
}

pub fn acquire(
    transport: &mut Transport,
    repository: &str,
    input: Input,
    scope: Scope,
) -> Result<Acquisition> {
    scope.validate()?;
    repository_slug(repository)?;
    if let Input::Commit(ref revision) = input {
        sha(revision)?;
    }
    if let Input::PullRequest(number) = input {
        ensure!(number > 0, "invalid_pull_request_number");
    }
    verify_repository(transport, repository)?;
    let (source, revision, original_ref) = match &input {
        Input::Commit(revision) => (repository.into(), revision.clone(), revision.clone()),
        Input::PullRequest(number) => {
            let (source, revision) = pull_head(transport, repository, *number)?;
            verify_repository(transport, &source)?;
            (source, revision, format!("refs/pull/{number}/head"))
        }
    };
    let prefix = format!("/repos/{}", repository_slug(&source)?);
    let commit = transport.get(&format!("{prefix}/git/commits/{revision}"), META_LIMIT)?;
    ensure!(
        string(&commit, "sha")? == revision,
        "github_revision_mismatch"
    );
    let root_tree = string(&commit["tree"], "sha")?;
    sha(root_tree)?;
    let mut trees = BTreeMap::<String, Vec<Value>>::new();
    let mut objects = Vec::new();
    let mut total = 0u64;
    for path in scope.paths() {
        let support = if scope.analysis.iter().any(|p| p == path) && path.ends_with(".rs") {
            "rust_source_not_yet_analyzed"
        } else {
            "context_or_unsupported_analysis"
        };
        let mut object = Object {
            path: path.into(),
            source_object: None,
            source_bytes: 0,
            content_digest: None,
            content: None,
            disposition: "missing".into(),
            analysis_support: support.into(),
        };
        let mut tree_id = root_tree.to_owned();
        let components: Vec<_> = path.split('/').collect();
        for (index, name) in components.iter().enumerate() {
            if !trees.contains_key(&tree_id) {
                let tree = transport.get(&format!("{prefix}/git/trees/{tree_id}"), META_LIMIT)?;
                ensure!(string(&tree, "sha")? == tree_id, "github_tree_mismatch");
                ensure!(
                    tree["truncated"].as_bool() == Some(false),
                    "github_tree_truncated_or_incomplete"
                );
                let entries = tree["tree"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("github_incomplete_tree"))?;
                let mut names = std::collections::BTreeSet::new();
                for entry in entries {
                    let name = string(entry, "path")?;
                    ensure!(
                        !name.is_empty()
                            && !name.contains('/')
                            && name != "."
                            && name != ".."
                            && names.insert(name),
                        "github_invalid_tree_entry"
                    );
                    sha(string(entry, "sha")?)?;
                    ensure!(
                        matches!(
                            (string(entry, "mode")?, string(entry, "type")?),
                            ("100644" | "100755" | "120000", "blob")
                                | ("040000", "tree")
                                | ("160000", "commit")
                        ),
                        "github_invalid_tree_entry"
                    );
                }
                trees.insert(tree_id.clone(), entries.clone());
            }
            let Some(entry) = trees[&tree_id]
                .iter()
                .find(|e| e["path"].as_str() == Some(name))
            else {
                break;
            };
            let mode = string(entry, "mode")?;
            ensure!(mode != "120000", "symlink_input_rejected");
            let id = string(entry, "sha")?.to_string();
            if index + 1 != components.len() {
                if mode != "040000" {
                    break;
                }
                tree_id = id;
                continue;
            }
            object.source_object = Some(id.clone());
            if !matches!(mode, "100644" | "100755") {
                object.disposition = "omitted_unsupported_object".into();
                break;
            }
            let size = entry["size"]
                .as_u64()
                .ok_or_else(|| anyhow::anyhow!("github_missing_blob_size"))?;
            total = total
                .checked_add(size)
                .ok_or_else(|| anyhow::anyhow!("byte_limit_exceeded"))?;
            ensure!(
                size <= scope.max_file_bytes && total <= scope.max_bytes,
                "byte_limit_exceeded"
            );
            let blob = transport.get(
                &format!("{prefix}/git/blobs/{id}"),
                size.saturating_mul(2).saturating_add(4096),
            )?;
            ensure!(
                string(&blob, "sha")? == id
                    && blob["size"].as_u64() == Some(size)
                    && string(&blob, "encoding")? == "base64",
                "github_blob_mismatch"
            );
            let encoded: String = string(&blob, "content")?
                .chars()
                .filter(|c| *c != '\n' && *c != '\r')
                .collect();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|_| anyhow::anyhow!("github_invalid_blob_encoding"))?;
            ensure!(
                bytes.len() as u64 == size && GitObjectFormat::Sha1.blob_digest(&bytes) == id,
                "github_blob_digest_mismatch"
            );
            object.source_bytes = size;
            match String::from_utf8(bytes) {
                Ok(content) if !content.contains('\0') => {
                    if unsafe_content(path, &content) {
                        object.disposition = "omitted_unsafe".into();
                    } else {
                        object.disposition = "included".into();
                        object.content_digest = Some(digest(content.as_bytes()));
                        object.content = Some(content);
                    }
                }
                _ => object.disposition = "omitted_binary".into(),
            }
            break;
        }
        objects.push(object);
    }
    if let Input::PullRequest(number) = input {
        ensure!(
            pull_head(transport, repository, number)? == (source.clone(), revision.clone()),
            "github_pull_request_changed"
        );
    }
    // Identity is re-observed even for an exact-commit input; no cached remote identity.
    verify_repository(transport, &source)?;
    let mut packet = Packet {
        schema: SCHEMA.into(),
        repository: source.clone(),
        revision: revision.clone(),
        object_format: GitObjectFormat::Sha1,
        scope,
        scope_digest: String::new(),
        completeness: if objects.iter().all(|o| o.disposition == "included") {
            "complete_scoped_acquisition"
        } else {
            "partial"
        }
        .into(),
        objects,
        excluded_surfaces: "all_paths_outside_declared_scope".into(),
        checkout_policy: "committed_blobs_only_dirty_and_untracked_excluded".into(),
        review_state: "not_reviewed".into(),
        packet_id: String::new(),
    };
    packet.seal()?;
    let provenance = Provenance {
        schema: "codefriend.github_acquisition.v1".into(),
        transport: if transport.fixture {
            "controlled_loopback_git_data_http"
        } else {
            "github_git_data_https"
        }
        .into(),
        requested_repository: repository.into(),
        original_ref,
        source_repository: source,
        resolved_commit: revision,
        packet_id: packet.packet_id.clone(),
        head_rechecked: matches!(input, Input::PullRequest(_)),
        requests: transport.requests,
    };
    Ok(Acquisition { packet, provenance })
}
impl Acquisition {
    fn validate(&self) -> Result<()> {
        self.packet.validate()?;
        let p = &self.provenance;
        repository_slug(&p.requested_repository)?;
        repository_slug(&p.source_repository)?;
        ensure!(
            p.schema == "codefriend.github_acquisition.v1"
                && matches!(
                    p.transport.as_str(),
                    "controlled_loopback_git_data_http" | "github_git_data_https"
                )
                && p.source_repository == self.packet.repository
                && p.resolved_commit == self.packet.revision
                && p.packet_id == self.packet.packet_id
                && p.requests > 0
                && p.requests <= MAX_REQUESTS,
            "github_provenance_mismatch"
        );
        if p.head_rechecked {
            let number = p
                .original_ref
                .strip_prefix("refs/pull/")
                .and_then(|v| v.strip_suffix("/head"))
                .and_then(|v| v.parse::<u64>().ok())
                .filter(|v| *v > 0)
                .ok_or_else(|| anyhow::anyhow!("github_provenance_mismatch"))?;
            ensure!(
                p.original_ref == format!("refs/pull/{number}/head"),
                "github_provenance_mismatch"
            );
        } else {
            ensure!(
                p.original_ref == p.resolved_commit
                    && p.requested_repository == p.source_repository,
                "github_provenance_mismatch"
            );
        }
        Ok(())
    }
    pub fn read(output: &Path) -> Result<Self> {
        let packet = AdmissionInput::read(&output.join("packet.json"))?
            .packet()
            .clone();
        let mut bytes = Vec::new();
        fs::File::open(output.join("provenance.json"))
            .map_err(|_| anyhow::anyhow!("github_provenance_open_failed"))?
            .take(16385)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("github_provenance_read_failed"))?;
        ensure!(bytes.len() <= 16384, "github_provenance_limit_exceeded");
        let provenance = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("github_provenance_parse_failed"))?;
        let result = Self { packet, provenance };
        result.validate()?;
        Ok(result)
    }
    /// Create-only bundle. A failed acquisition never creates output; write failures
    /// are reported, never promoted as complete. The packet remains directly readable.
    pub fn write(&self, output: &Path) -> Result<()> {
        self.validate()?;
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder
            .create(output)
            .map_err(|_| anyhow::anyhow!("github_output_exists_or_unavailable"))?;
        let result = (|| {
            for (name, bytes) in [
                ("packet.json", serde_json::to_vec_pretty(&self.packet)?),
                (
                    "provenance.json",
                    serde_json::to_vec_pretty(&self.provenance)?,
                ),
            ] {
                let mut options = fs::OpenOptions::new();
                options.write(true).create_new(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.mode(0o600);
                }
                let mut file = options
                    .open(output.join(name))
                    .map_err(|_| anyhow::anyhow!("github_artifact_create_failed"))?;
                file.write_all(&bytes)
                    .and_then(|_| file.sync_all())
                    .map_err(|_| anyhow::anyhow!("github_artifact_write_failed"))?;
            }
            ensure!(
                AdmissionInput::read(&output.join("packet.json"))?.packet() == &self.packet,
                "github_packet_readback_mismatch"
            );
            let readback = Self::read(output)?;
            ensure!(
                readback.provenance == self.provenance,
                "github_provenance_readback_mismatch"
            );
            Ok(())
        })();
        if result.is_err() {
            for name in ["packet.json", "provenance.json"] {
                let _ = fs::remove_file(output.join(name));
            }
            let _ = fs::remove_dir(output);
        }
        result
    }
}
