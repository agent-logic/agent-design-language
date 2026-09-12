//! Gate A semantic foundation on the single DurableTransactionStore owner.
//! Constructors validate data, not operational authority. Native authority/topology
//! bridges must admit production callers before invoking these storage primitives.
//! Effect reservation/attachment and executable recovery belong to later slices.
use super::{codec, DurableTransactionStore};
use crate::lifecycle::{
    semantic::{self as policy, Invalidation, SemanticCommand},
    LifecycleState,
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(String),
    InvalidEncoding(String),
    InvalidDigest,
    WrongRepository,
    LegacyMigrationRequired,
    AlreadyExists,
    StaleVersion,
    Busy,
    RecoveryRequired,
    UnsafePath,
    ExhaustedVersion,
    Io(String),
}
fn io(e: std::io::Error) -> Error {
    Error::Io(e.to_string())
}
fn encoding(e: String) -> Error {
    Error::InvalidEncoding(e)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Digest(String);
impl TryFrom<String> for Digest {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        let (domain, hex) = value.split_once(':').ok_or("digest domain missing")?;
        if !matches!(
            domain,
            "semantic-input-v1"
                | "semantic-state-v1"
                | "semantic-audit-v1"
                | "semantic-authority-v1"
        ) || hex.len() != 64
            || !hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("invalid digest".into());
        }
        Ok(Self(value))
    }
}
impl From<Digest> for String {
    fn from(value: Digest) -> Self {
        value.0
    }
}
impl Digest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn authority(bytes: &[u8]) -> Self {
        hash_bytes("semantic-authority-v1", bytes)
    }
}
fn hash_bytes(domain: &str, bytes: &[u8]) -> Digest {
    let mut hash = blake3::Hasher::new();
    hash.update(domain.as_bytes());
    hash.update(&[0]);
    hash.update(bytes);
    Digest(format!("{domain}:{}", hash.finalize().to_hex()))
}
fn hash<T: Serialize>(domain: &str, value: &T) -> Result<Digest, Error> {
    Ok(hash_bytes(domain, &codec::bytes(value).map_err(encoding)?))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawKey", into = "RawKey")]
pub struct IssueKey {
    repository: String,
    issue: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawKey {
    repository: String,
    issue: u64,
}
impl TryFrom<RawKey> for IssueKey {
    type Error = String;
    fn try_from(raw: RawKey) -> Result<Self, String> {
        let parts: Vec<_> = raw.repository.split('/').collect();
        if raw.issue == 0
            || parts.len() != 2
            || parts.iter().any(|p| {
                p.is_empty()
                    || *p == "."
                    || *p == ".."
                    || !p
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
            })
        {
            return Err("nonzero issue and owner/repository required".into());
        }
        Ok(Self {
            repository: raw.repository,
            issue: raw.issue,
        })
    }
}
impl From<IssueKey> for RawKey {
    fn from(k: IssueKey) -> Self {
        Self {
            repository: k.repository,
            issue: k.issue,
        }
    }
}
impl IssueKey {
    pub fn new(repository: impl Into<String>, issue: u64) -> Result<Self, Error> {
        Self::try_from(RawKey {
            repository: repository.into(),
            issue,
        })
        .map_err(Error::InvalidInput)
    }
    pub fn repository(&self) -> &str {
        &self.repository
    }
    pub fn issue(&self) -> u64 {
        self.issue
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticVersion {
    generation: u64,
    digest: Digest,
}
impl SemanticVersion {
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceInputVersion {
    revision: u64,
    digest: Digest,
}
impl EvidenceInputVersion {
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Explicit typed input shape; ordered plan steps and validator argv remain ordered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanStep {
    pub id: String,
    pub acceptance: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validator {
    pub id: String,
    pub argv: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub branch: String,
    pub head: String,
    pub worktree: PathBuf,
    pub registration: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueInputs {
    intent: String,
    cards: BTreeMap<String, String>,
    plan: Vec<PlanStep>,
    validation: Vec<Validator>,
    binding: Option<Binding>,
    authority: Digest,
}
impl IssueInputs {
    pub fn new(
        intent: String,
        cards: BTreeMap<String, String>,
        plan: Vec<PlanStep>,
        validation: Vec<Validator>,
        binding: Option<Binding>,
        authority: Digest,
    ) -> Result<Self, Error> {
        let value = Self {
            intent,
            cards,
            plan,
            validation,
            binding,
            authority,
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), Error> {
        fn nonempty(v: &str) -> bool {
            !v.trim().is_empty() && !v.contains('\0')
        }
        if !nonempty(&self.intent)
            || self.cards.is_empty()
            || self.cards.iter().any(|(k, v)| !nonempty(k) || !nonempty(v))
            || self.plan.is_empty()
            || self.validation.is_empty()
            || self
                .plan
                .iter()
                .any(|p| !nonempty(&p.id) || !nonempty(&p.acceptance))
            || self.validation.iter().any(|v| {
                !nonempty(&v.id) || v.argv.is_empty() || v.argv.iter().any(|a| !nonempty(a))
            })
            || !self.authority.0.starts_with("semantic-authority-v1:")
        {
            return Err(Error::InvalidInput("incomplete issue inputs".into()));
        }
        let plan: std::collections::BTreeSet<_> = self.plan.iter().map(|p| &p.id).collect();
        let validators: std::collections::BTreeSet<_> =
            self.validation.iter().map(|p| &p.id).collect();
        if plan.len() != self.plan.len() || validators.len() != self.validation.len() {
            return Err(Error::InvalidInput("duplicate input identity".into()));
        }
        if let Some(b) = &self.binding {
            if !b.branch.starts_with("codex/")
                || b.head.len() != 40
                || !b.head.bytes().all(|b| b.is_ascii_hexdigit())
                || !b.worktree.is_absolute()
                || !nonempty(&b.registration)
            {
                return Err(Error::InvalidInput("invalid binding".into()));
            }
        }
        Ok(())
    }
    pub fn intent(&self) -> &str {
        &self.intent
    }
    pub fn authority(&self) -> &Digest {
        &self.authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    key: IssueKey,
    generation: u64,
    phase: LifecycleState,
    inputs: IssueInputs,
    input_version: EvidenceInputVersion,
    invalidations: Vec<Invalidation>,
    projection_required: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Audit {
    before: Option<SemanticVersion>,
    after: SemanticVersion,
    previous: Option<Digest>,
    command: SemanticCommand,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Schema {
    #[serde(rename = "csdlc.v3.semantic_commit.v1")]
    CommitV1,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    schema: Schema,
    payload: Payload,
    audit: Audit,
    audit_digest: Digest,
}
impl Snapshot {
    pub fn key(&self) -> &IssueKey {
        &self.payload.key
    }
    pub fn version(&self) -> &SemanticVersion {
        &self.audit.after
    }
    pub fn inputs_version(&self) -> &EvidenceInputVersion {
        &self.payload.input_version
    }
    pub fn inputs(&self) -> &IssueInputs {
        &self.payload.inputs
    }
    pub fn phase(&self) -> LifecycleState {
        self.payload.phase
    }
    pub fn projection_required(&self) -> bool {
        self.payload.projection_required
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, Error> {
        codec::bytes(self).map_err(encoding)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let value: Self = codec::decode(bytes).map_err(encoding)?;
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), Error> {
        self.payload.inputs.validate()?;
        if self.payload.generation == 0
            || self.payload.input_version.revision == 0
            || self.payload.input_version.revision > self.payload.generation
            || self.payload.input_version.digest != hash("semantic-input-v1", &self.payload.inputs)?
            || self.audit.after.generation != self.payload.generation
            || self.audit.after.digest != hash("semantic-state-v1", &self.payload)?
            || self.audit_digest != hash("semantic-audit-v1", &self.audit)?
        {
            return Err(Error::InvalidDigest);
        }
        let mut ordered = self.payload.invalidations.clone();
        ordered.sort();
        ordered.dedup();
        if ordered != self.payload.invalidations {
            return Err(Error::InvalidDigest);
        }
        match (&self.audit.before, &self.audit.previous) {
            (None, None)
                if self.payload.generation == 1
                    && self.audit.command == SemanticCommand::Prepare => {}
            (Some(before), Some(_))
                if before.generation.checked_add(1) == Some(self.payload.generation) => {}
            _ => return Err(Error::InvalidDigest),
        }
        Ok(())
    }
}

/// Gate A only supports local amendments and projection acknowledgements.
/// No proof/review/remote outcome can be forged through this API.
#[derive(Debug, Clone)]
pub enum LocalChange {
    AmendCards(BTreeMap<String, String>),
    AmendPlan(Vec<PlanStep>),
    AmendValidation(Vec<Validator>),
    AcknowledgeProjection(ProjectionWriteProof),
}
/// Readback proof of the exact generated view. Rechecked under the mutation lock.
#[derive(Debug, Clone)]
pub struct ProjectionWriteProof {
    version: SemanticVersion,
}
impl ProjectionWriteProof {
    pub fn verify(root: &SemanticRoot, snapshot: &Snapshot) -> Result<Self, Error> {
        let path = root.projection_path(snapshot)?;
        reject_symlinks(&path)?;
        if fs::read(path).map_err(io)? != snapshot.canonical_bytes()? {
            return Err(Error::InvalidInput(
                "projection differs from committed view".into(),
            ));
        }
        Ok(Self {
            version: snapshot.version().clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Admission {
    key: IssueKey,
    expected: SemanticVersion,
    authority: Digest,
}
impl Admission {
    pub fn new(key: IssueKey, expected: SemanticVersion, authority: Digest) -> Self {
        Self {
            key,
            expected,
            authority,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Observation {
    Absent,
    Current(Box<Snapshot>),
    RecoveryRequired,
    LegacyMigrationRequired,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitOutcome {
    Committed(Box<Snapshot>),
    Unchanged(Box<Snapshot>),
}

/// Canonical Git-common identity. No constructor creates state or locks.
#[derive(Debug, Clone)]
pub struct SemanticRoot {
    common: PathBuf,
    repository: String,
}
impl SemanticRoot {
    pub fn from_git_common(
        common: impl AsRef<Path>,
        repository: impl Into<String>,
    ) -> Result<Self, Error> {
        let common = common.as_ref();
        reject_symlinks(common)?;
        let common = fs::canonicalize(common).map_err(io)?;
        if !common.join("HEAD").is_file()
            || !common.join("objects").is_dir()
            || !common.join("config").is_file()
        {
            return Err(Error::UnsafePath);
        }
        let repository = repository.into();
        IssueKey::new(repository.clone(), 1)?;
        Ok(Self { common, repository })
    }
    pub fn projection_path(&self, snapshot: &Snapshot) -> Result<PathBuf, Error> {
        self.directory(snapshot.key())?;
        let base = if let Some(binding) = &snapshot.inputs().binding {
            binding.worktree.join(".csdlc/v3/issues")
        } else {
            self.common.join("csdlc-v3/local/projections")
        };
        Ok(base
            .join(snapshot.key().issue.to_string())
            .join("state.json"))
    }
    fn directory(&self, key: &IssueKey) -> Result<PathBuf, Error> {
        if key.repository != self.repository {
            return Err(Error::WrongRepository);
        }
        let path = self
            .common
            .join("csdlc-v3/semantic/issues")
            .join(key.issue.to_string());
        reject_symlinks(&path)?;
        Ok(path)
    }
    fn legacy(&self, key: &IssueKey) -> Result<bool, Error> {
        let local = self.common.join("csdlc-v3/local");
        let issue = key.issue.to_string();
        let mut paths = vec![
            local.join("issues").join(&issue),
            local.join("evidence").join(&issue),
            local
                .join("transactions/pending")
                .join(format!("{issue}.json")),
        ];
        if let Some(primary) = self.common.parent() {
            paths.push(primary.join(".csdlc/issues").join(&issue));
        }
        let registrations = self.common.join("worktrees");
        if registrations.exists() {
            reject_symlinks(&registrations)?;
            for entry in fs::read_dir(registrations).map_err(io)? {
                let gitdir = entry.map_err(io)?.path().join("gitdir");
                reject_symlinks(&gitdir)?;
                let target = fs::read_to_string(gitdir).map_err(io)?;
                if let Some(checkout) = Path::new(target.trim()).parent() {
                    paths.push(checkout.join(".csdlc/issues").join(&issue));
                }
            }
        }
        for path in paths {
            reject_symlinks(&path)?;
            if path.exists() {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

// These associated methods deliberately do not construct the historical session,
// whose retained construction API holds a lock for its lifetime.
impl DurableTransactionStore {
    pub fn observe_issue(root: &SemanticRoot, key: &IssueKey) -> Result<Observation, Error> {
        let directory = root.directory(key)?;
        if !directory.exists() {
            return Ok(if root.legacy(key)? {
                Observation::LegacyMigrationRequired
            } else {
                Observation::Absent
            });
        }
        let lock = match acquire(&directory, false) {
            Err(Error::Io(_)) if !directory.join("state.lock").exists() => {
                return Ok(Observation::RecoveryRequired)
            }
            other => other?,
        };
        let result = read_current(&directory, key);
        drop(lock);
        match result {
            Err(Error::RecoveryRequired) => Ok(Observation::RecoveryRequired),
            other => other.map(|s| Observation::Current(Box::new(s))),
        }
    }
    pub fn prepare_issue(
        root: &SemanticRoot,
        key: IssueKey,
        inputs: IssueInputs,
    ) -> Result<CommitOutcome, Error> {
        inputs.validate()?;
        if inputs.binding.is_some() {
            return Err(Error::InvalidInput(
                "prepare cannot pre-bind topology".into(),
            ));
        }
        let directory = root.directory(&key)?;
        if root.legacy(&key)? {
            return Err(Error::LegacyMigrationRequired);
        }
        // Parent creation is serialized through stable advisory locking before the
        // issue directory becomes visible. An interrupted directory is not reset.
        let parent = directory.parent().ok_or(Error::UnsafePath)?;
        create_directories(parent, &root.common)?;
        let _parent = acquire(parent, true)?;
        if directory.exists() {
            return Err(Error::AlreadyExists);
        }
        fs::create_dir(&directory).map_err(io)?;
        sync_chain(&directory, &root.common)?;
        let _lock = acquire(&directory, true)?;
        let decision = policy::decide(
            None,
            SemanticCommand::Prepare,
            policy::Outcome::Success,
            &policy::Facts {
                prepared: true,
                ..Default::default()
            },
        )
        .map_err(|_| Error::InvalidInput("prepare rejected".into()))?;
        let input_version = EvidenceInputVersion {
            revision: 1,
            digest: hash("semantic-input-v1", &inputs)?,
        };
        let payload = Payload {
            key,
            generation: 1,
            phase: decision.phase,
            inputs,
            input_version,
            invalidations: vec![],
            projection_required: true,
        };
        let next = make_snapshot(payload, None, SemanticCommand::Prepare)?;
        activate(&directory, &root.common, &next)?;
        Ok(CommitOutcome::Committed(Box::new(next)))
    }
    pub fn commit_issue_local(
        root: &SemanticRoot,
        admission: Admission,
        change: LocalChange,
    ) -> Result<CommitOutcome, Error> {
        let directory = root.directory(&admission.key)?;
        if !directory.exists() {
            return Err(if root.legacy(&admission.key)? {
                Error::LegacyMigrationRequired
            } else {
                Error::InvalidInput("issue absent".into())
            });
        }
        let _lock = acquire(&directory, true)?;
        let current = read_current(&directory, &admission.key)?;
        if current.version() != &admission.expected {
            return Err(Error::StaleVersion);
        }
        if current.inputs().authority() != &admission.authority {
            return Err(Error::InvalidInput("authority changed".into()));
        }
        let mut payload = current.payload.clone();
        let command = match change {
            LocalChange::AmendCards(cards) => {
                payload.inputs.cards = cards;
                SemanticCommand::AmendCards
            }
            LocalChange::AmendPlan(plan) => {
                payload.inputs.plan = plan;
                SemanticCommand::AmendPlan
            }
            LocalChange::AmendValidation(validation) => {
                payload.inputs.validation = validation;
                SemanticCommand::AmendValidation
            }
            LocalChange::AcknowledgeProjection(proof) => {
                if proof.version != *current.version() {
                    return Err(Error::StaleVersion);
                }
                ProjectionWriteProof::verify(root, &current)?;
                payload.projection_required = false;
                SemanticCommand::AcknowledgeProjection
            }
        };
        payload.inputs.validate()?;
        if payload == current.payload {
            return Ok(CommitOutcome::Unchanged(Box::new(current)));
        }
        let decision = policy::decide(
            Some(payload.phase),
            command,
            policy::Outcome::Success,
            &policy::Facts::default(),
        )
        .map_err(|_| Error::InvalidInput("local transition rejected".into()))?;
        payload.generation = payload
            .generation
            .checked_add(1)
            .ok_or(Error::ExhaustedVersion)?;
        if payload.inputs != current.payload.inputs {
            payload.input_version = EvidenceInputVersion {
                revision: payload
                    .input_version
                    .revision
                    .checked_add(1)
                    .ok_or(Error::ExhaustedVersion)?,
                digest: hash("semantic-input-v1", &payload.inputs)?,
            };
            payload.projection_required = true;
        }
        payload.phase = decision.phase;
        payload.invalidations.extend(decision.invalidations);
        payload.invalidations.sort();
        payload.invalidations.dedup();
        let next = make_snapshot(payload, Some(&current), command)?;
        activate(&directory, &root.common, &next)?;
        Ok(CommitOutcome::Committed(Box::new(next)))
    }
}
fn make_snapshot(
    payload: Payload,
    previous: Option<&Snapshot>,
    command: SemanticCommand,
) -> Result<Snapshot, Error> {
    let after = SemanticVersion {
        generation: payload.generation,
        digest: hash("semantic-state-v1", &payload)?,
    };
    let audit = Audit {
        before: previous.map(|s| s.version().clone()),
        after,
        previous: previous.map(|s| s.audit_digest.clone()),
        command,
    };
    let audit_digest = hash("semantic-audit-v1", &audit)?;
    let next = Snapshot {
        schema: Schema::CommitV1,
        payload,
        audit,
        audit_digest,
    };
    next.validate()?;
    Ok(next)
}

struct Lock(File);
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}
fn acquire(directory: &Path, create: bool) -> Result<Lock, Error> {
    let path = directory.join("state.lock");
    reject_symlinks(&path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(create)
        .create(create)
        .truncate(false)
        .open(path)
        .map_err(io)?;
    let result = if create {
        FileExt::try_lock_exclusive(&file)
    } else {
        FileExt::try_lock_shared(&file)
    };
    result.map_err(|e| {
        if e.kind() == std::io::ErrorKind::WouldBlock {
            Error::Busy
        } else {
            io(e)
        }
    })?;
    Ok(Lock(file))
}
fn reject_symlinks(path: &Path) -> Result<(), Error> {
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(meta) if meta.file_type().is_symlink() => return Err(Error::UnsafePath),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(io(e)),
        }
    }
    Ok(())
}
fn sync_chain(path: &Path, root: &Path) -> Result<(), Error> {
    let mut path = path;
    loop {
        File::open(path).and_then(|f| f.sync_all()).map_err(io)?;
        if path == root {
            return Ok(());
        }
        path = path.parent().ok_or(Error::UnsafePath)?;
    }
}
fn create_directories(path: &Path, root: &Path) -> Result<(), Error> {
    reject_symlinks(path)?;
    fs::create_dir_all(path).map_err(io)?;
    sync_chain(path, root)
}
fn create_only(path: &Path, bytes: &[u8], root: &Path) -> Result<(), Error> {
    reject_symlinks(path)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(io)?;
    file.write_all(bytes).map_err(io)?;
    file.sync_all().map_err(io)?;
    sync_chain(path.parent().ok_or(Error::UnsafePath)?, root)
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pointer {
    generation: u64,
    digest: Digest,
    audit: Digest,
}
fn name(version: &SemanticVersion) -> String {
    format!(
        "{}-{}.json",
        version.generation,
        version
            .digest
            .0
            .rsplit(':')
            .next()
            .expect("validated digest")
    )
}
fn activate(directory: &Path, common: &Path, next: &Snapshot) -> Result<(), Error> {
    let commits = directory.join("commits");
    let intents = directory.join("intents");
    create_directories(&commits, common)?;
    create_directories(&intents, common)?;
    let pointer = Pointer {
        generation: next.version().generation,
        digest: next.version().digest.clone(),
        audit: next.audit_digest.clone(),
    };
    let bytes = codec::bytes(&pointer).map_err(encoding)?;
    create_only(
        &intents.join(format!("{}.json", pointer.generation)),
        &bytes,
        common,
    )?;
    create_only(
        &commits.join(name(next.version())),
        &next.canonical_bytes()?,
        common,
    )?;
    create_only(&directory.join("current.next"), &bytes, common)?;
    fs::rename(
        directory.join("current.next"),
        directory.join("current.json"),
    )
    .map_err(io)?;
    sync_chain(directory, common)
}
fn read_current(directory: &Path, key: &IssueKey) -> Result<Snapshot, Error> {
    reject_symlinks(directory)?;
    if directory.join("current.next").exists() || !directory.join("current.json").exists() {
        return Err(Error::RecoveryRequired);
    }
    let current = directory.join("current.json");
    reject_symlinks(&current)?;
    let pointer: Pointer = codec::decode(&fs::read(current).map_err(io)?).map_err(encoding)?;
    let version = SemanticVersion {
        generation: pointer.generation,
        digest: pointer.digest,
    };
    let mut snapshot = load_commit(directory, &version)?;
    if snapshot.key() != key || snapshot.audit_digest != pointer.audit {
        return Err(Error::InvalidDigest);
    }
    let newest = snapshot.clone();
    // Validate every retained predecessor and matching intent, not only the tip.
    loop {
        let intent = directory
            .join("intents")
            .join(format!("{}.json", snapshot.version().generation));
        reject_symlinks(&intent)?;
        let p: Pointer = codec::decode(&fs::read(intent).map_err(io)?).map_err(encoding)?;
        if p.generation != snapshot.version().generation
            || p.digest != snapshot.version().digest
            || p.audit != snapshot.audit_digest
        {
            return Err(Error::InvalidDigest);
        }
        let Some(before) = &snapshot.audit.before else {
            break;
        };
        let previous = load_commit(directory, before)?;
        if previous.key() != key || Some(&previous.audit_digest) != snapshot.audit.previous.as_ref()
        {
            return Err(Error::InvalidDigest);
        }
        snapshot = previous;
    }
    for entry in fs::read_dir(directory.join("intents")).map_err(io)? {
        let entry = entry.map_err(io)?;
        reject_symlinks(&entry.path())?;
        let file = entry
            .file_name()
            .into_string()
            .map_err(|_| Error::RecoveryRequired)?;
        let generation = file
            .strip_suffix(".json")
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or(Error::RecoveryRequired)?;
        if generation == 0 || generation > newest.version().generation {
            return Err(Error::RecoveryRequired);
        }
    }
    Ok(newest)
}
fn load_commit(directory: &Path, version: &SemanticVersion) -> Result<Snapshot, Error> {
    let path = directory.join("commits").join(name(version));
    reject_symlinks(&path)?;
    let snapshot = Snapshot::from_bytes(&fs::read(path).map_err(io)?)?;
    if snapshot.version() != version {
        return Err(Error::InvalidDigest);
    }
    Ok(snapshot)
}
