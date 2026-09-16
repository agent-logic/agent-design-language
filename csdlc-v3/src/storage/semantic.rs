//! Gate A semantic foundation on the single DurableTransactionStore owner.
//! Constructors validate data, not operational authority. Native authority/topology
//! bridges must admit production callers before invoking these storage primitives.
//! Effect reservation/attachment and executable recovery belong to later slices.
pub mod journal_recovery;
pub mod protocol;
use super::{codec, DurableTransactionStore};
use crate::lifecycle::{
    semantic::{
        self as policy, AmendmentClass, AmendmentFacts, AmendmentOutcome, CausalInvalidation,
        Invalidation, SemanticCommand,
    },
    LifecycleState,
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
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
    PendingOperation,
    ConflictingReplay,
    EvidenceMismatch,
    AdmissionChanged,
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
                | "semantic-operation-v1"
                | "semantic-request-v1"
                | "semantic-evidence-v1"
                | "semantic-recovery-v1"
                | "semantic-projection-v1"
                | "card-projection-v1"
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
    pub fn projection(bytes: &[u8]) -> Self {
        hash_bytes("card-projection-v1", bytes)
    }
    pub fn semantic_projection(bytes: &[u8]) -> Self {
        hash_bytes("semantic-projection-v1", bytes)
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
    pub program: String,
    pub args: Vec<String>,
    pub success_marker: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}
fn default_timeout() -> u64 {
    300
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub base: String,
    pub title: String,
    pub body: String,
    pub draft: bool,
}
/// Lossless wire-equivalent of application::intent::IntentPlan. Keeping this DTO
/// here avoids a storage-to-application dependency; a round-trip test pins parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptedIntentPlan {
    pub schema: String,
    pub slug: String,
    pub cards: BTreeMap<String, serde_json::Value>,
    pub validators: Vec<Validator>,
    pub publication: Publication,
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
    intent_plan: AcceptedIntentPlan,
    plan: Vec<PlanStep>,
    binding: Option<Binding>,
    authority: Digest,
}
impl IssueInputs {
    pub fn new(
        intent: String,
        intent_plan: AcceptedIntentPlan,
        plan: Vec<PlanStep>,
        binding: Option<Binding>,
        authority: Digest,
    ) -> Result<Self, Error> {
        let value = Self {
            intent,
            intent_plan,
            plan,
            binding,
            authority,
        };
        value.validate()?;
        Ok(value)
    }
    pub fn from_intent_plan_bytes(
        intent: String,
        bytes: &[u8],
        plan: Vec<PlanStep>,
        binding: Option<Binding>,
        authority: Digest,
    ) -> Result<Self, Error> {
        Self::new(
            intent,
            codec::decode(bytes).map_err(encoding)?,
            plan,
            binding,
            authority,
        )
    }
    fn validate(&self) -> Result<(), Error> {
        fn nonempty(v: &str) -> bool {
            !v.trim().is_empty() && !v.contains('\0')
        }
        let accepted = &self.intent_plan;
        if !nonempty(&self.intent)
            || accepted.schema != "csdlc.v3.intent_plan.v1"
            || accepted.slug.is_empty()
            || accepted.slug.len() > 100
            || !accepted
                .slug
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || accepted.cards.len() != 6
            || ["sip", "stp", "spp", "vpp", "srp", "sor"].iter().any(|k| {
                !accepted
                    .cards
                    .get(*k)
                    .is_some_and(serde_json::Value::is_object)
            })
            || self.plan.is_empty()
            || self
                .plan
                .iter()
                .any(|p| !nonempty(&p.id) || !nonempty(&p.acceptance))
            || !self.authority.0.starts_with("semantic-authority-v1:")
        {
            return Err(Error::InvalidInput("incomplete issue inputs".into()));
        }
        let plan: std::collections::BTreeSet<_> = self.plan.iter().map(|p| &p.id).collect();
        if plan.len() != self.plan.len() {
            return Err(Error::InvalidInput("duplicate plan identity".into()));
        }
        // Validator admission/execution rules remain the proof owner's job. Preserve
        // timeout and marker semantics even when that owner would reject execution.
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
    pub fn accepted_plan(&self) -> &AcceptedIntentPlan {
        &self.intent_plan
    }
    pub fn slug(&self) -> &str {
        &self.intent_plan.slug
    }
    pub fn cards(&self) -> &BTreeMap<String, serde_json::Value> {
        &self.intent_plan.cards
    }
    pub fn plan(&self) -> &[PlanStep] {
        &self.plan
    }
    pub fn validation(&self) -> &[Validator] {
        &self.intent_plan.validators
    }
    pub fn publication(&self) -> &Publication {
        &self.intent_plan.publication
    }
    pub fn binding(&self) -> Option<&Binding> {
        self.binding.as_ref()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    causal_invalidations: Vec<CausalInvalidation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    acknowledged_card_projection: Option<Digest>,
    projection_required: bool,
    pending: Option<protocol::PendingOperation>,
    completed: Vec<protocol::CompletedOperation>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Audit {
    before: Option<SemanticVersion>,
    after: SemanticVersion,
    previous: Option<Digest>,
    command: SemanticCommand,
    operation: Option<protocol::OperationId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Schema {
    #[serde(rename = "csdlc.v3.semantic_commit.v3")]
    CommitV3,
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
    pub fn pending(&self) -> Option<&protocol::PendingOperation> {
        self.payload.pending.as_ref()
    }
    pub fn completed(&self) -> &[protocol::CompletedOperation] {
        &self.payload.completed
    }
    pub fn invalidations(&self) -> &[Invalidation] {
        &self.payload.invalidations
    }
    pub fn causal_invalidations(&self) -> &[CausalInvalidation] {
        &self.payload.causal_invalidations
    }
    pub fn acknowledged_card_projection(&self) -> Option<&Digest> {
        self.payload.acknowledged_card_projection.as_ref()
    }
    pub fn audit_identity(&self) -> &Digest {
        &self.audit_digest
    }
    pub fn audit_command(&self) -> SemanticCommand {
        self.audit.command
    }
    /// Stable projection identity: CAS generation and projection acknowledgement are
    /// deliberately absent, so writing the view of G also describes G+1's content.
    pub fn projection_bytes(&self) -> Result<Vec<u8>, Error> {
        #[derive(Serialize)]
        struct View<'a> {
            schema: &'static str,
            key: &'a IssueKey,
            phase: LifecycleState,
            inputs: &'a IssueInputs,
            input_version: &'a EvidenceInputVersion,
            invalidations: &'a [Invalidation],
            #[serde(skip_serializing_if = "<[CausalInvalidation]>::is_empty")]
            causal_invalidations: &'a [CausalInvalidation],
        }
        codec::bytes(&View {
            schema: "csdlc.v3.semantic_projection.v1",
            key: self.key(),
            phase: self.phase(),
            inputs: self.inputs(),
            input_version: self.inputs_version(),
            invalidations: self.invalidations(),
            causal_invalidations: self.causal_invalidations(),
        })
        .map_err(encoding)
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
        let mut causal = self.payload.causal_invalidations.clone();
        causal.sort();
        causal.dedup();
        if causal != self.payload.causal_invalidations
            || causal
                .iter()
                .any(|item| !self.payload.invalidations.contains(&item.evidence))
        {
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

/// Local amendments and projection acknowledgements.
/// No proof/review/remote outcome can be forged through this API.
#[derive(Debug, Clone)]
pub enum LocalChange {
    AmendCards(BTreeMap<String, serde_json::Value>),
    AmendPlan(Vec<PlanStep>),
    AmendValidation(Vec<Validator>),
    AmendBinding(VerifiedBindingAmendment),
    AcknowledgeProjection(ProjectionWriteProof),
}
/// Native owner attests the exact replacement binding after topology validation.
/// Parsing a binding does not grant this capability.
#[derive(Debug, Clone)]
pub struct VerifiedBindingAmendment {
    binding: Binding,
}
impl VerifiedBindingAmendment {
    pub(crate) fn from_native_owner(binding: Binding) -> Self {
        Self { binding }
    }
}
/// Readback proof of the exact generated view. Rechecked under the mutation lock.
#[derive(Debug, Clone)]
pub struct ProjectionWriteProof {
    version: SemanticVersion,
    card_projection: Option<CardProjectionBundle>,
}
impl ProjectionWriteProof {
    pub fn verify(root: &SemanticRoot, snapshot: &Snapshot) -> Result<Self, Error> {
        let path = root.projection_path(snapshot)?;
        reject_symlinks(&path)?;
        if fs::read(path).map_err(io)? != snapshot.projection_bytes()? {
            return Err(Error::InvalidInput(
                "projection differs from committed view".into(),
            ));
        }
        Ok(Self {
            version: snapshot.version().clone(),
            card_projection: None,
        })
    }

    fn verify_current(&self, root: &SemanticRoot, snapshot: &Snapshot) -> Result<(), Error> {
        if self.version != *snapshot.version() {
            return Err(Error::StaleVersion);
        }
        Self::verify(root, snapshot)?;
        if let Some(bundle) = &self.card_projection {
            bundle.validate(snapshot)?;
            if observe_card_projection_unlocked(root, snapshot, bundle)?
                != CardProjectionObservation::Healthy
            {
                return Err(Error::EvidenceMismatch);
            }
        }
        Ok(())
    }
}

pub const SEMANTIC_CARD_KINDS: [&str; 6] = ["sip", "stp", "spp", "vpp", "srp", "sor"];

/// One deterministic generated card. The bytes are derived by the application
/// owner from the accepted semantic input and active prompt registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardProjectionArtifact {
    kind: String,
    template_ref: String,
    values: Vec<u8>,
    rendered: Vec<u8>,
    values_digest: Digest,
    rendered_digest: Digest,
}

impl CardProjectionArtifact {
    pub(crate) fn new(
        kind: impl Into<String>,
        template_ref: impl Into<String>,
        values: Vec<u8>,
        rendered: Vec<u8>,
    ) -> Result<Self, Error> {
        let artifact = Self {
            kind: kind.into(),
            template_ref: template_ref.into(),
            values_digest: Digest::projection(&values),
            rendered_digest: Digest::projection(&rendered),
            values,
            rendered,
        };
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }
    pub fn template_ref(&self) -> &str {
        &self.template_ref
    }
    pub fn values(&self) -> &[u8] {
        &self.values
    }
    pub fn rendered(&self) -> &[u8] {
        &self.rendered
    }
    pub fn values_digest(&self) -> &Digest {
        &self.values_digest
    }
    pub fn rendered_digest(&self) -> &Digest {
        &self.rendered_digest
    }

    fn validate(&self) -> Result<(), Error> {
        if !SEMANTIC_CARD_KINDS.contains(&self.kind.as_str())
            || self.template_ref.trim().is_empty()
            || self.template_ref.contains('\0')
            || self.values_digest != Digest::projection(&self.values)
            || self.rendered_digest != Digest::projection(&self.rendered)
        {
            return Err(Error::InvalidInput(
                "invalid card projection artifact".into(),
            ));
        }
        serde_json::from_slice::<serde_json::Value>(&self.values)
            .map_err(|_| Error::InvalidInput("card projection values must be valid JSON".into()))?;
        std::str::from_utf8(&self.rendered)
            .map_err(|_| Error::InvalidInput("rendered card must be UTF-8".into()))?;
        Ok(())
    }
}

/// Complete six-card projection derived from one exact semantic version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardProjectionBundle {
    schema: String,
    semantic_digest: Digest,
    registry_version: String,
    cards: BTreeMap<String, CardProjectionArtifact>,
    projection_digest: Digest,
}

impl CardProjectionBundle {
    pub(crate) fn new(
        snapshot: &Snapshot,
        registry_version: impl Into<String>,
        cards: BTreeMap<String, CardProjectionArtifact>,
    ) -> Result<Self, Error> {
        let mut bundle = Self {
            schema: "csdlc.v3.semantic_card_projection.v1".into(),
            semantic_digest: Digest::semantic_projection(&snapshot.projection_bytes()?),
            registry_version: registry_version.into(),
            cards,
            projection_digest: Digest::projection(b"pending"),
        };
        bundle.projection_digest = bundle.computed_digest()?;
        bundle.validate(snapshot)?;
        Ok(bundle)
    }

    pub fn semantic_digest(&self) -> &Digest {
        &self.semantic_digest
    }
    pub fn registry_version(&self) -> &str {
        &self.registry_version
    }
    pub fn cards(&self) -> &BTreeMap<String, CardProjectionArtifact> {
        &self.cards
    }
    pub fn projection_digest(&self) -> &Digest {
        &self.projection_digest
    }
    pub fn manifest_bytes(&self) -> Result<Vec<u8>, Error> {
        #[derive(Serialize)]
        struct CardIndex<'a> {
            template_ref: &'a str,
            values_digest: &'a Digest,
            rendered_digest: &'a Digest,
        }
        #[derive(Serialize)]
        struct Manifest<'a> {
            schema: &'static str,
            semantic_digest: &'a Digest,
            registry_version: &'a str,
            projection_digest: &'a Digest,
            cards: BTreeMap<&'a str, CardIndex<'a>>,
        }
        let cards = self
            .cards
            .iter()
            .map(|(kind, artifact)| {
                (
                    kind.as_str(),
                    CardIndex {
                        template_ref: &artifact.template_ref,
                        values_digest: &artifact.values_digest,
                        rendered_digest: &artifact.rendered_digest,
                    },
                )
            })
            .collect();
        codec::bytes(&Manifest {
            schema: "csdlc.v3.semantic_card_projection_manifest.v1",
            semantic_digest: &self.semantic_digest,
            registry_version: &self.registry_version,
            projection_digest: &self.projection_digest,
            cards,
        })
        .map_err(encoding)
    }

    fn computed_digest(&self) -> Result<Digest, Error> {
        let cards = self
            .cards
            .iter()
            .map(|(kind, artifact)| {
                (
                    kind.as_str(),
                    (
                        artifact.template_ref.as_str(),
                        &artifact.values_digest,
                        &artifact.rendered_digest,
                    ),
                )
            })
            .collect::<BTreeMap<_, _>>();
        hash(
            "card-projection-v1",
            &(
                &self.schema,
                &self.semantic_digest,
                &self.registry_version,
                cards,
            ),
        )
    }

    fn validate(&self, snapshot: &Snapshot) -> Result<(), Error> {
        if self.schema != "csdlc.v3.semantic_card_projection.v1"
            || self.semantic_digest != Digest::semantic_projection(&snapshot.projection_bytes()?)
            || self.registry_version.trim().is_empty()
            || self.cards.len() != SEMANTIC_CARD_KINDS.len()
            || SEMANTIC_CARD_KINDS.iter().any(|kind| {
                self.cards
                    .get(*kind)
                    .is_none_or(|artifact| artifact.kind != *kind || artifact.validate().is_err())
            })
            || self.projection_digest != self.computed_digest()?
        {
            return Err(Error::InvalidInput(
                "invalid semantic card projection".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CardProjectionObservation {
    Healthy,
    Missing { paths: Vec<String> },
    Altered { paths: Vec<String> },
    Interrupted { staged_paths: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct Admission {
    key: IssueKey,
    expected: SemanticVersion,
    authority: Digest,
}

/// Authenticated input to the isolated copied-record conversion owner. The
/// caller must hold its fixture writer fence and prove that `source_digest`
/// identifies the complete retained source record before calling this API.
#[derive(Debug, Clone)]
pub struct CopiedRecordConversion {
    pub key: IssueKey,
    pub inputs: IssueInputs,
    pub phase: LifecycleState,
    pub source_generation: u64,
    pub source_digest: Digest,
}

/// In-process proof that the conversion owner holds the exact archived native
/// writer locks. Only conversion admission can use this token; ordinary legacy
/// classification continues to treat every native lock as residue.
pub struct NativeWriterFenceGuard {
    common: PathBuf,
    issues: BTreeSet<u64>,
    paths: Vec<PathBuf>,
    _locks: Vec<File>,
}

impl NativeWriterFenceGuard {
    pub fn acquire(common: &Path, issues: impl IntoIterator<Item = u64>) -> Result<Self, Error> {
        let common = fs::canonicalize(common).map_err(io)?;
        let state_root = common.join("csdlc-v3/local");
        let lock_root = state_root.join("locks");
        create_directories(&lock_root, &common)?;
        let issues = issues.into_iter().collect::<BTreeSet<_>>();
        if issues.is_empty() || issues.contains(&0) {
            return Err(Error::InvalidInput(
                "invalid writer-fence denominator".into(),
            ));
        }
        let mut locks = Vec::new();
        let mut paths = Vec::new();
        for issue in &issues {
            let path = lock_root.join(format!("{issue}.lock"));
            reject_symlinks(&path)?;
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(&path)
                .map_err(io)?;
            FileExt::lock_exclusive(&file).map_err(io)?;
            paths.push(path);
            locks.push(file);
        }
        rebarrier(paths.clone(), &common)?;
        Ok(Self {
            common,
            issues,
            paths,
            _locks: locks,
        })
    }

    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }

    /// Construct the in-process admission token after the conversion owner has
    /// authenticated that its durable guardian process holds the same native
    /// lock denominator. The guardian owns the file descriptors so an abrupt
    /// converter exit cannot release the archived writer fence.
    pub(crate) fn authenticated_guardian(
        common: &Path,
        issues: impl IntoIterator<Item = u64>,
    ) -> Result<Self, Error> {
        let common = fs::canonicalize(common).map_err(io)?;
        let issues = issues.into_iter().collect::<BTreeSet<_>>();
        if issues.is_empty() || issues.contains(&0) {
            return Err(Error::InvalidInput(
                "invalid writer-fence denominator".into(),
            ));
        }
        let paths = issues
            .iter()
            .map(|issue| {
                common
                    .join("csdlc-v3/local/locks")
                    .join(format!("{issue}.lock"))
            })
            .collect();
        Ok(Self {
            common,
            issues,
            paths,
            _locks: Vec::new(),
        })
    }

    fn authenticates(&self, root: &SemanticRoot, issue: u64) -> bool {
        self.common == root.common && self.issues.contains(&issue)
    }
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
    ProjectionRepairRequired(Box<Snapshot>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitOutcome {
    Committed(Box<Snapshot>),
    Unchanged(Box<Snapshot>),
}

/// Validated Git-common location plus caller-supplied repository label.
/// This is NOT authenticated repository/selector/topology admission. Production
/// callers must establish those guards in the native owner bridge. No constructor
/// creates state/locks or consults credentials/network.
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
        let cleaned = protocol::has_performed_successful_command(
            self,
            snapshot,
            SemanticCommand::RecordCleanup,
        )?;
        let base = if let Some(binding) = &snapshot.inputs().binding {
            if cleaned {
                self.common.join("csdlc-v3/local/projections")
            } else {
                binding.worktree.join(".csdlc/v3/issues")
            }
        } else {
            self.common.join("csdlc-v3/local/projections")
        };
        Ok(base
            .join(snapshot.key().issue.to_string())
            .join("state.json"))
    }
    pub fn card_projection_directory(&self, snapshot: &Snapshot) -> Result<PathBuf, Error> {
        Ok(self
            .projection_path(snapshot)?
            .parent()
            .ok_or(Error::UnsafePath)?
            .join("cards"))
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
        self.legacy_with_fence(key, None)
    }

    fn legacy_with_fence(
        &self,
        key: &IssueKey,
        fence: Option<&NativeWriterFenceGuard>,
    ) -> Result<bool, Error> {
        let mut roots = vec![self.common.join("csdlc-v3/local")];
        if let Some(primary) = self.common.parent() {
            roots.push(primary.join(".csdlc"));
        }
        let registrations = self.common.join("worktrees");
        if registrations.exists() {
            reject_symlinks(&registrations)?;
            for entry in fs::read_dir(registrations).map_err(io)? {
                let gitdir = entry.map_err(io)?.path().join("gitdir");
                reject_symlinks(&gitdir)?;
                let target = fs::read_to_string(gitdir).map_err(io)?;
                if let Some(checkout) = Path::new(target.trim()).parent() {
                    roots.push(checkout.join(".csdlc"));
                }
            }
        }
        for root in roots {
            let ignored_lock = fence
                .filter(|guard| guard.authenticates(self, key.issue))
                .and_then(|guard| {
                    let expected = self
                        .common
                        .join("csdlc-v3/local/locks")
                        .join(format!("{}.lock", key.issue));
                    guard
                        .paths
                        .iter()
                        .any(|path| path == &expected)
                        .then_some(expected)
                });
            if legacy_residue(&root, key.issue, ignored_lock.as_deref())? {
                return Ok(true);
            }
        }
        remote_residue(&self.common.join("csdlc-v3/remote"), key)
    }
}

/// Enumerates issue-scoped native local, preparation, mutation and terminal roots.
/// Empty directories still count as ambiguous residue. Do not parse a damaged
/// receipt to decide whether it is safe to overwrite its namespace.
fn legacy_residue(root: &Path, issue: u64, ignored_lock: Option<&Path>) -> Result<bool, Error> {
    let exact = [
        format!("issues/{issue}"),
        format!("locks/{issue}.lock"),
        format!("prepared/issues/{issue}"),
        format!("v3/issues/{issue}"),
        format!("bindings/{issue}.json"),
        format!("transactions/{issue}.json"),
        format!("transactions/completed/{issue}"),
        format!("transactions/pending/{issue}.json"),
    ];
    for relative in exact {
        let path = root.join(relative);
        reject_symlinks(&path)?;
        if ignored_lock == Some(path.as_path()) {
            continue;
        }
        if path.try_exists().map_err(io)? {
            return Ok(true);
        }
    }
    let evidence = root.join(format!("evidence/{issue}"));
    reject_symlinks(&evidence)?;
    if evidence.try_exists().map_err(io)? {
        let mut entries = fs::read_dir(&evidence)
            .map_err(io)?
            .map(|entry| entry.map_err(io).map(|entry| entry.file_name()))
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort();
        let authority_only = entries == [std::ffi::OsString::from("terminal-receipt.json")]
            && root.file_name().and_then(|name| name.to_str()) == Some(".csdlc")
            && root.parent().is_some_and(|repository| {
                let selector =
                    fs::read(repository.join("csdlc-v3/operator/authority-selector.json"))
                        .ok()
                        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok());
                selector.is_some_and(|selector| {
                    selector["authority_issue"].as_u64() == Some(issue)
                        && fs::read(
                            repository.join("csdlc-v3/operator/native-authority-receipt.json"),
                        )
                        .ok()
                        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
                        .is_some_and(|receipt| {
                            receipt["authority_issue"].as_u64() == Some(issue)
                                && receipt["terminal_receipt_path"]
                                    == format!(".csdlc/evidence/{issue}/terminal-receipt.json")
                        })
                })
            });
        if !authority_only {
            return Ok(true);
        }
    }
    for (parent, prefixes) in [
        (
            "issues",
            vec![format!(".issue-{issue}-"), format!(".issue-{issue}.")],
        ),
        (
            "prepared/issues",
            vec![format!(".issue-{issue}-"), format!(".issue-{issue}.")],
        ),
        ("archives", vec![format!("{issue}-"), format!("{issue}.")]),
    ] {
        let parent = root.join(parent);
        reject_symlinks(&parent)?;
        if !parent.try_exists().map_err(io)? {
            continue;
        }
        for entry in fs::read_dir(parent).map_err(io)? {
            let entry = entry.map_err(io)?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| Error::UnsafePath)?;
            if name == issue.to_string() || prefixes.iter().any(|prefix| name.starts_with(prefix)) {
                reject_symlinks(&entry.path())?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn remote_identity(value: &serde_json::Value) -> Result<(String, u64), Error> {
    let schema = value["schema"].as_str().ok_or(Error::RecoveryRequired)?;
    let identity = match schema {
        "csdlc.v3.github_mutation_intent.v1"
        | "csdlc.v3.github_mutation_intent.v2"
        | "csdlc.v3.merge_intent.v1" => &value["request"],
        "csdlc.v3.github_mutation_receipt.v1"
        | "csdlc.v3.github_mutation_receipt.v2"
        | "csdlc.v3.github_mutation_recovery.v1"
        | "csdlc.v3.github_mutation_reconciliation.v1" => value,
        _ => return Err(Error::RecoveryRequired),
    };
    let repository = identity["repository"]
        .as_str()
        .ok_or(Error::RecoveryRequired)?
        .to_owned();
    let issue = identity["issue"].as_u64().ok_or(Error::RecoveryRequired)?;
    IssueKey::new(repository.clone(), 1).map_err(|_| Error::RecoveryRequired)?;
    Ok((repository, issue))
}
fn read_remote_json(path: &Path) -> Result<serde_json::Value, Error> {
    reject_symlinks(path)?;
    if !path.is_file() {
        return Err(Error::RecoveryRequired);
    }
    codec::decode(&fs::read(path).map_err(io)?).map_err(|_| Error::RecoveryRequired)
}

fn repository_scoped_issue_creation_result(
    remote: &Path,
    namespace: &str,
    path: &Path,
    key: &IssueKey,
) -> Result<bool, Error> {
    if namespace != "mutations" {
        return Ok(false);
    }
    crate::commands::remote::repository_scoped_issue_creation_receipt(
        remote,
        path,
        key.repository.as_str(),
        key.issue,
    )
    .map_err(|_| Error::RecoveryRequired)
}

fn remote_residue(remote: &Path, key: &IssueKey) -> Result<bool, Error> {
    for namespace in ["intents", "mutations", "recoveries", "merges"] {
        let directory = remote.join(namespace);
        reject_symlinks(&directory)?;
        if !directory.try_exists().map_err(io)? {
            continue;
        }
        for entry in fs::read_dir(&directory).map_err(io)? {
            let path = entry.map_err(io)?.path();
            reject_symlinks(&path)?;
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or(Error::RecoveryRequired)?;
            if name.ends_with(".lock")
                && path.is_file()
                && fs::metadata(&path).map_err(io)?.len() == 0
            {
                continue;
            }
            let value = read_remote_json(&path)?;
            let identity = if namespace == "merges"
                && [
                    ".target.json",
                    ".input.json",
                    ".response.json",
                    ".dispatch-prestate.json",
                ]
                .iter()
                .any(|suffix| name.ends_with(suffix))
            {
                let digest = if name.ends_with(".target.json") {
                    if value["schema"] != "csdlc.v3.merge_target.v1" {
                        return Err(Error::RecoveryRequired);
                    }
                    value["operation_digest"]
                        .as_str()
                        .ok_or(Error::RecoveryRequired)?
                } else {
                    name.split('.').next().ok_or(Error::RecoveryRequired)?
                };
                if digest.is_empty()
                    || !digest
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
                {
                    return Err(Error::RecoveryRequired);
                }
                let identity = remote_identity(&read_remote_json(
                    &directory.join(format!("{digest}.intent.json")),
                )?)?;
                if name.ends_with(".target.json")
                    && value["repository"].as_str() != Some(identity.0.as_str())
                {
                    return Err(Error::RecoveryRequired);
                }
                identity
            } else {
                remote_identity(&value)?
            };
            // Creation before a positive issue exists is repository-scoped. Never
            // assign that effect to whichever issue happens to be prepared next.
            // Its reconciled receipt names the GitHub-assigned issue, but remains
            // part of that repository-scoped creation transaction rather than
            // legacy lifecycle state for the newly created issue.
            if namespace != "intents"
                && repository_scoped_issue_creation_result(remote, namespace, &path, key)?
            {
                continue;
            }
            if identity.1 > 0 && identity.1 == key.issue && identity.0 == key.repository {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

// These associated methods deliberately do not construct the historical session,
// whose retained construction API holds a lock for its lifetime.
impl DurableTransactionStore {
    /// Durably materialize the current read-only projection. Semantic activation
    /// remains authoritative; acknowledgement is a separate CAS mutation.
    pub fn write_issue_projection(
        root: &SemanticRoot,
        snapshot: &Snapshot,
    ) -> Result<ProjectionWriteProof, Error> {
        let directory = root.directory(snapshot.key())?;
        let _lock = acquire(&directory, false)?;
        let current = read_current(&directory, snapshot.key())?;
        if current.version() != snapshot.version() {
            return Err(Error::StaleVersion);
        }
        write_issue_projection_unlocked(root, &current)
    }

    /// Materialize a complete deterministic card bundle. A durable pending
    /// marker precedes every card write and the manifest is committed last, so
    /// readers can distinguish an interrupted rebuild from a healthy bundle.
    /// Replaying the same bundle repairs a partial write without changing the
    /// semantic snapshot.
    pub fn write_card_projection(
        root: &SemanticRoot,
        snapshot: &Snapshot,
        bundle: CardProjectionBundle,
    ) -> Result<ProjectionWriteProof, Error> {
        bundle.validate(snapshot)?;
        let directory = root.directory(snapshot.key())?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(&directory, snapshot.key())?;
        if current.version() != snapshot.version() {
            return Err(Error::StaleVersion);
        }
        bundle.validate(&current)?;
        let card_root = root.card_projection_directory(&current)?;
        reject_symlinks(&card_root)?;
        let projection_root = projection_root(root, &current, &card_root)?;
        let suffix = bundle
            .projection_digest()
            .as_str()
            .rsplit(':')
            .next()
            .ok_or(Error::InvalidDigest)?;
        // Validate every stale residue before the first mutation. A corrupt
        // old marker must not be erased or hidden by a new pending marker.
        let stale_paths = authenticated_stale_projection_paths(
            &card_root,
            suffix,
            current.acknowledged_card_projection(),
        )?;

        // Keep version admission and every projection write under one lock.
        write_issue_projection_unlocked(root, &current)?;
        create_directories(&card_root, &projection_root)?;
        let pending = card_root.join(format!(".projection-{suffix}.pending"));
        let manifest = bundle.manifest_bytes()?;
        write_exact_create_or_verify(&pending, &manifest, &projection_root)?;
        rebarrier([pending.clone()], &projection_root)?;

        #[cfg(debug_assertions)]
        let removing_stale_residue = !stale_paths.is_empty();
        for residue in &stale_paths {
            for path in &residue.staged {
                fs::remove_file(path).map_err(io)?;
            }
        }
        sync_chain(&card_root, &projection_root)?;
        #[cfg(debug_assertions)]
        if removing_stale_residue
            && std::env::var_os("CSDLC_TEST_INTERRUPT_AFTER_STALE_PROJECTION_DATA_SYNC").is_some()
        {
            return Err(Error::Io(
                "injected interruption after stale projection data sync".into(),
            ));
        }
        for residue in stale_paths {
            fs::remove_file(residue.pending).map_err(io)?;
        }
        sync_chain(&card_root, &projection_root)?;

        for kind in SEMANTIC_CARD_KINDS {
            let artifact = bundle.cards().get(kind).ok_or_else(|| {
                Error::InvalidInput("semantic card projection is incomplete".into())
            })?;
            replace_projection_file(
                &card_root,
                &projection_root,
                suffix,
                &format!("{kind}.values.json"),
                &artifact.values,
            )?;
            replace_projection_file(
                &card_root,
                &projection_root,
                suffix,
                &format!("{kind}.md"),
                &artifact.rendered,
            )?;
        }
        replace_projection_file(
            &card_root,
            &projection_root,
            suffix,
            "manifest.json",
            &manifest,
        )?;
        fs::remove_file(&pending).map_err(io)?;
        sync_chain(&card_root, &projection_root)?;
        let proof = ProjectionWriteProof {
            version: current.version().clone(),
            card_projection: Some(bundle),
        };
        proof.verify_current(root, &current)?;
        Ok(proof)
    }

    /// Inspect exact expected projection bytes without creating locks, files,
    /// journals or repairs. The caller supplies the deterministic bundle derived
    /// from the semantic snapshot and active registry.
    pub fn observe_card_projection(
        root: &SemanticRoot,
        snapshot: &Snapshot,
        bundle: &CardProjectionBundle,
    ) -> Result<CardProjectionObservation, Error> {
        bundle.validate(snapshot)?;
        let directory = root.directory(snapshot.key())?;
        if !directory.exists() {
            return Err(Error::InvalidInput("issue absent".into()));
        }
        let _lock = acquire(&directory, false)?;
        let current = read_current(&directory, snapshot.key())?;
        if current.version() != snapshot.version() {
            return Err(Error::StaleVersion);
        }
        observe_card_projection_unlocked(root, &current, bundle)
    }

    pub fn observe_issue(root: &SemanticRoot, key: &IssueKey) -> Result<Observation, Error> {
        Self::observe_issue_inner(root, key, None)
    }

    pub fn observe_issue_under_native_writer_fence(
        root: &SemanticRoot,
        key: &IssueKey,
        fence: &NativeWriterFenceGuard,
    ) -> Result<Observation, Error> {
        if !fence.authenticates(root, key.issue) {
            return Err(Error::InvalidInput(
                "native writer fence does not authenticate observed issue".into(),
            ));
        }
        Self::observe_issue_inner(root, key, Some(fence))
    }

    fn observe_issue_inner(
        root: &SemanticRoot,
        key: &IssueKey,
        fence: Option<&NativeWriterFenceGuard>,
    ) -> Result<Observation, Error> {
        let directory = root.directory(key)?;
        if !directory.exists() {
            return Ok(if root.legacy_with_fence(key, fence)? {
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
        let result = match result {
            Err(Error::RecoveryRequired) => Ok(Observation::RecoveryRequired),
            other => other.map(|s| {
                if !s.projection_required() && ProjectionWriteProof::verify(root, &s).is_err() {
                    Observation::ProjectionRepairRequired(Box::new(s))
                } else {
                    Observation::Current(Box::new(s))
                }
            }),
        };
        drop(lock);
        result
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
            // Read under the existing issue lock without creating missing lock
            // state: partial preparation must remain an explicit recovery case.
            let _lock = acquire(&directory, false)?;
            let current = read_current(&directory, &key)?;
            if current.phase() != LifecycleState::Ready
                || current.inputs() != &inputs
                || current.pending().is_some()
                || !current.completed().is_empty()
            {
                return Err(Error::AlreadyExists);
            }
            if !current.projection_required() {
                ProjectionWriteProof::verify(root, &current)?;
            }
            return Ok(CommitOutcome::Unchanged(Box::new(current)));
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
            causal_invalidations: vec![],
            acknowledged_card_projection: None,
            projection_required: true,
            pending: None,
            completed: Vec::new(),
        };
        let next = make_snapshot(payload, None, SemanticCommand::Prepare)?;
        activate(&directory, &root.common, &next)?;
        Ok(CommitOutcome::Committed(Box::new(next)))
    }

    /// Activate one copied record through the semantic store's durable
    /// transaction path. This is intentionally absent from the ordinary CLI:
    /// only the isolated conversion owner admits it after fencing and census
    /// validation. Existing or legacy operational state always fails closed.
    pub fn convert_copied_issue(
        root: &SemanticRoot,
        conversion: CopiedRecordConversion,
    ) -> Result<CommitOutcome, Error> {
        Self::convert_copied_issue_inner(root, conversion, None)
    }

    pub fn convert_copied_issue_under_native_writer_fence(
        root: &SemanticRoot,
        conversion: CopiedRecordConversion,
        fence: &NativeWriterFenceGuard,
    ) -> Result<CommitOutcome, Error> {
        if !fence.authenticates(root, conversion.key.issue) {
            return Err(Error::InvalidInput(
                "native writer fence does not authenticate converted issue".into(),
            ));
        }
        Self::convert_copied_issue_inner(root, conversion, Some(fence))
    }

    fn convert_copied_issue_inner(
        root: &SemanticRoot,
        conversion: CopiedRecordConversion,
        fence: Option<&NativeWriterFenceGuard>,
    ) -> Result<CommitOutcome, Error> {
        if conversion.source_generation == 0
            || !conversion
                .source_digest
                .as_str()
                .starts_with("semantic-projection-v1:")
        {
            return Err(Error::InvalidInput("invalid copied-record identity".into()));
        }
        let directory = root.directory(&conversion.key)?;
        if directory.exists() {
            return Err(Error::AlreadyExists);
        }
        if root.legacy_with_fence(&conversion.key, fence)? {
            return Err(Error::LegacyMigrationRequired);
        }
        let parent = directory.parent().ok_or(Error::UnsafePath)?;
        create_directories(parent, &root.common)?;
        let _parent = acquire(parent, true)?;
        fs::create_dir(&directory).map_err(io)?;
        sync_chain(&directory, &root.common)?;
        let _lock = acquire(&directory, true)?;
        let input_version = EvidenceInputVersion {
            revision: 1,
            digest: hash("semantic-input-v1", &conversion.inputs)?,
        };
        let payload = Payload {
            key: conversion.key,
            generation: 1,
            phase: conversion.phase,
            inputs: conversion.inputs,
            input_version,
            invalidations: vec![],
            causal_invalidations: vec![],
            acknowledged_card_projection: None,
            projection_required: true,
            pending: None,
            completed: Vec::new(),
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
        if current.payload.pending.is_some() {
            return Err(Error::PendingOperation);
        }
        if current.version() != &admission.expected {
            return Err(Error::StaleVersion);
        }
        if current.inputs().authority() != &admission.authority {
            return Err(Error::InvalidInput("authority changed".into()));
        }
        let mut payload = current.payload.clone();
        let mut facts = policy::Facts::default();
        let mut amendment_class = None;
        let command = match change {
            LocalChange::AmendCards(cards) => {
                payload.inputs.intent_plan.cards = cards;
                amendment_class = Some(AmendmentClass::ScopeAcceptance);
                SemanticCommand::AmendCards
            }
            LocalChange::AmendPlan(plan) => {
                payload.inputs.plan = plan;
                amendment_class = Some(AmendmentClass::Plan);
                SemanticCommand::AmendPlan
            }
            LocalChange::AmendValidation(validation) => {
                payload.inputs.intent_plan.validators = validation;
                amendment_class = Some(AmendmentClass::ProofValidator);
                SemanticCommand::AmendValidation
            }
            LocalChange::AmendBinding(verified) => {
                payload.inputs.binding = Some(verified.binding);
                facts.topology = true;
                amendment_class = Some(AmendmentClass::Binding);
                SemanticCommand::AmendBinding
            }
            LocalChange::AcknowledgeProjection(proof) => {
                proof.verify_current(root, &current)?;
                if let Some(bundle) = &proof.card_projection {
                    payload.acknowledged_card_projection = Some(bundle.projection_digest().clone());
                }
                payload.projection_required = false;
                SemanticCommand::AcknowledgeProjection
            }
        };
        payload.inputs.validate()?;
        if payload == current.payload {
            return Ok(CommitOutcome::Unchanged(Box::new(current)));
        }
        let (phase, invalidations, causal_invalidations) = if let Some(class) = amendment_class {
            let phase = current.phase();
            let amendment_facts = AmendmentFacts {
                source_version_current: true,
                issue_checkout_match: true,
                evidence_integrity: true,
                transition_approved: true,
                topology: payload.inputs.binding().is_some(),
                implementation_revision: false,
                current_proof: matches!(
                    phase,
                    LifecycleState::Implemented
                        | LifecycleState::Reviewed
                        | LifecycleState::Published
                        | LifecycleState::MergeReady
                ),
                independent_review: matches!(
                    phase,
                    LifecycleState::Reviewed
                        | LifecycleState::Published
                        | LifecycleState::MergeReady
                ),
                projection_change: payload.inputs != current.payload.inputs,
                new_commit: false,
            };
            match policy::decide_amendment(phase, class, &amendment_facts) {
                AmendmentOutcome::Admitted {
                    phase,
                    invalidations,
                    ..
                } => (
                    phase,
                    invalidations.iter().map(|item| item.evidence).collect(),
                    invalidations,
                ),
                _ => return Err(Error::InvalidInput("local amendment rejected".into())),
            }
        } else {
            let decision = policy::decide(
                Some(payload.phase),
                command,
                policy::Outcome::Success,
                &facts,
            )
            .map_err(|_| Error::InvalidInput("local transition rejected".into()))?;
            (decision.phase, decision.invalidations, Vec::new())
        };
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
        payload.phase = phase;
        payload.invalidations.extend(invalidations);
        payload.invalidations.sort();
        payload.invalidations.dedup();
        payload.causal_invalidations.extend(causal_invalidations);
        payload.causal_invalidations.sort();
        payload.causal_invalidations.dedup();
        let next = make_snapshot(payload, Some(&current), command)?;
        activate(&directory, &root.common, &next)?;
        Ok(CommitOutcome::Committed(Box::new(next)))
    }
}

fn projection_root(
    root: &SemanticRoot,
    snapshot: &Snapshot,
    path: &Path,
) -> Result<PathBuf, Error> {
    if path.starts_with(&root.common) {
        Ok(root.common.clone())
    } else {
        snapshot
            .inputs()
            .binding()
            .map(|binding| binding.worktree.join(".csdlc"))
            .ok_or(Error::UnsafePath)
    }
}

fn write_exact_create_or_verify(path: &Path, bytes: &[u8], root: &Path) -> Result<(), Error> {
    reject_symlinks(path)?;
    if path.try_exists().map_err(io)? {
        if fs::read(path).map_err(io)? != bytes {
            return Err(Error::EvidenceMismatch);
        }
        Ok(())
    } else {
        create_only(path, bytes, root)
    }
}

fn write_issue_projection_unlocked(
    root: &SemanticRoot,
    snapshot: &Snapshot,
) -> Result<ProjectionWriteProof, Error> {
    let path = root.projection_path(snapshot)?;
    reject_symlinks(&path)?;
    let parent = path.parent().ok_or(Error::UnsafePath)?;
    let projection_root = projection_root(root, snapshot, &path)?;
    create_directories(parent, &projection_root)?;
    let bytes = snapshot.projection_bytes()?;
    let suffix = snapshot
        .version()
        .digest()
        .as_str()
        .rsplit(':')
        .next()
        .ok_or(Error::InvalidDigest)?;
    let staged = parent.join(format!(".state-{suffix}.next"));
    reject_symlinks(&staged)?;
    if staged.try_exists().map_err(io)? {
        if fs::read(&staged).map_err(io)? != bytes {
            return Err(Error::EvidenceMismatch);
        }
        rebarrier([staged.clone()], &projection_root)?;
    } else {
        create_only(&staged, &bytes, &projection_root)?;
    }
    fs::rename(&staged, &path).map_err(io)?;
    rebarrier([path], &projection_root)?;
    ProjectionWriteProof::verify(root, snapshot)
}

fn projection_staging_paths(directory: &Path) -> Result<Vec<(PathBuf, String)>, Error> {
    if !directory.try_exists().map_err(io)? {
        return Ok(Vec::new());
    }
    let targets = SEMANTIC_CARD_KINDS
        .into_iter()
        .flat_map(|kind| [format!("{kind}.md"), format!("{kind}.values.json")])
        .chain(["manifest.json".into()])
        .collect::<Vec<String>>();
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory).map_err(io)? {
        let entry = entry.map_err(io)?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| Error::EvidenceMismatch)?;
        let suspicious = name.starts_with(".projection-") || name.ends_with(".next");
        if !suspicious {
            continue;
        }
        let suffix = if let Some(value) = name
            .strip_prefix(".projection-")
            .and_then(|value| value.strip_suffix(".pending"))
        {
            value
        } else {
            let Some(value) = name
                .strip_prefix('.')
                .and_then(|value| value.strip_suffix(".next"))
            else {
                return Err(Error::EvidenceMismatch);
            };
            let Some((target, suffix)) = value.rsplit_once('-') else {
                return Err(Error::EvidenceMismatch);
            };
            if !targets.iter().any(|candidate| candidate == target) {
                return Err(Error::EvidenceMismatch);
            }
            suffix
        };
        if suffix.len() != 64
            || !suffix
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(Error::EvidenceMismatch);
        }
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(io)?;
        if !metadata.file_type().is_file() {
            return Err(Error::EvidenceMismatch);
        }
        paths.push((path, suffix.to_owned()));
    }
    paths.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(paths)
}

/// Authenticate residue from an older, durably committed projection before
/// allowing the current rebuild to remove it. The old pending manifest must
/// match the last committed manifest byte-for-byte, and every staged file must
/// match the digest retained by that manifest. Validation completes before the
/// caller removes any residue.
struct AuthenticatedStaleProjectionResidue {
    staged: Vec<PathBuf>,
    pending: PathBuf,
}

fn authenticated_stale_projection_paths(
    directory: &Path,
    current_suffix: &str,
    acknowledged: Option<&Digest>,
) -> Result<Vec<AuthenticatedStaleProjectionResidue>, Error> {
    let staging = projection_staging_paths(directory)?;
    let mut by_suffix = BTreeMap::<String, Vec<PathBuf>>::new();
    for (path, suffix) in staging {
        if suffix != current_suffix {
            by_suffix.entry(suffix).or_default().push(path);
        }
    }
    if by_suffix.is_empty() {
        return Ok(Vec::new());
    }

    let committed_manifest = fs::read(directory.join("manifest.json")).map_err(io)?;
    let mut authenticated = Vec::new();
    for (suffix, paths) in by_suffix {
        let retained_digest = acknowledged.ok_or(Error::EvidenceMismatch)?;
        if retained_digest.as_str() != format!("card-projection-v1:{suffix}") {
            return Err(Error::EvidenceMismatch);
        }
        let pending_name = format!(".projection-{suffix}.pending");
        let pending = paths
            .iter()
            .find(|path| path.file_name().and_then(|name| name.to_str()) == Some(&pending_name))
            .ok_or(Error::EvidenceMismatch)?;
        let pending_bytes = fs::read(pending).map_err(io)?;
        if pending_bytes != committed_manifest {
            return Err(Error::EvidenceMismatch);
        }
        let manifest: serde_json::Value =
            serde_json::from_slice(&pending_bytes).map_err(|_| Error::EvidenceMismatch)?;
        if manifest["schema"] != "csdlc.v3.semantic_card_projection_manifest.v1"
            || manifest["projection_digest"].as_str() != Some(retained_digest.as_str())
        {
            return Err(Error::EvidenceMismatch);
        }
        let semantic_digest: Digest = serde_json::from_value(manifest["semantic_digest"].clone())
            .map_err(|_| Error::EvidenceMismatch)?;
        let registry_version = manifest["registry_version"]
            .as_str()
            .ok_or(Error::EvidenceMismatch)?;
        let cards = manifest["cards"]
            .as_object()
            .ok_or(Error::EvidenceMismatch)?
            .iter()
            .map(|(kind, card)| {
                let template_ref = card["template_ref"]
                    .as_str()
                    .ok_or(Error::EvidenceMismatch)?
                    .to_owned();
                let values: Digest = serde_json::from_value(card["values_digest"].clone())
                    .map_err(|_| Error::EvidenceMismatch)?;
                let rendered: Digest = serde_json::from_value(card["rendered_digest"].clone())
                    .map_err(|_| Error::EvidenceMismatch)?;
                Ok((kind.clone(), (template_ref, values, rendered)))
            })
            .collect::<Result<BTreeMap<_, _>, Error>>()?;
        if cards.len() != SEMANTIC_CARD_KINDS.len()
            || SEMANTIC_CARD_KINDS
                .iter()
                .any(|kind| !cards.contains_key(*kind))
            || hash(
                "card-projection-v1",
                &(
                    "csdlc.v3.semantic_card_projection.v1",
                    semantic_digest,
                    registry_version,
                    cards,
                ),
            )? != *retained_digest
        {
            return Err(Error::EvidenceMismatch);
        }

        for path in &paths {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or(Error::EvidenceMismatch)?;
            if name == pending_name {
                continue;
            }
            let body = name
                .strip_prefix('.')
                .and_then(|name| name.strip_suffix(".next"))
                .and_then(|name| name.strip_suffix(format!("-{suffix}").as_str()))
                .ok_or(Error::EvidenceMismatch)?;
            let bytes = fs::read(path).map_err(io)?;
            if body == "manifest.json" {
                if bytes != pending_bytes {
                    return Err(Error::EvidenceMismatch);
                }
                continue;
            }
            let (kind, digest_field) = if let Some(kind) = body.strip_suffix(".values.json") {
                (kind, "values_digest")
            } else if let Some(kind) = body.strip_suffix(".md") {
                (kind, "rendered_digest")
            } else {
                return Err(Error::EvidenceMismatch);
            };
            let expected = manifest["cards"][kind][digest_field]
                .as_str()
                .ok_or(Error::EvidenceMismatch)?;
            if Digest::projection(&bytes).as_str() != expected {
                return Err(Error::EvidenceMismatch);
            }
        }
        authenticated.push(AuthenticatedStaleProjectionResidue {
            staged: paths
                .iter()
                .filter(|path| *path != pending)
                .cloned()
                .collect(),
            pending: pending.clone(),
        });
    }
    Ok(authenticated)
}

fn replace_projection_file(
    directory: &Path,
    root: &Path,
    suffix: &str,
    name: &str,
    bytes: &[u8],
) -> Result<(), Error> {
    let target = directory.join(name);
    let staged = directory.join(format!(".{name}-{suffix}.next"));
    reject_symlinks(&target)?;
    write_exact_create_or_verify(&staged, bytes, root)?;
    rebarrier([staged.clone()], root)?;
    fs::rename(&staged, &target).map_err(io)?;
    rebarrier([target], root)
}

fn observe_card_projection_unlocked(
    root: &SemanticRoot,
    snapshot: &Snapshot,
    bundle: &CardProjectionBundle,
) -> Result<CardProjectionObservation, Error> {
    let card_root = root.card_projection_directory(snapshot)?;
    reject_symlinks(&card_root)?;
    if card_root.try_exists().map_err(io)? {
        let mut staged_paths = projection_staging_paths(&card_root)?
            .into_iter()
            .filter_map(|(path, _)| path.file_name()?.to_str().map(str::to_owned))
            .collect::<Vec<_>>();
        staged_paths.sort();
        if !staged_paths.is_empty() {
            return Ok(CardProjectionObservation::Interrupted { staged_paths });
        }
    }
    let mut expected = BTreeMap::new();
    for kind in SEMANTIC_CARD_KINDS {
        let artifact = bundle
            .cards()
            .get(kind)
            .ok_or_else(|| Error::InvalidInput("semantic card projection is incomplete".into()))?;
        expected.insert(format!("{kind}.values.json"), artifact.values.as_slice());
        expected.insert(format!("{kind}.md"), artifact.rendered.as_slice());
    }
    let manifest = bundle.manifest_bytes()?;
    expected.insert("manifest.json".into(), manifest.as_slice());
    let mut missing = Vec::new();
    let mut altered = Vec::new();
    for (name, bytes) in expected {
        let path = card_root.join(&name);
        reject_symlinks(&path)?;
        match fs::read(path) {
            Ok(actual) if actual == bytes => {}
            Ok(_) => altered.push(name),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => missing.push(name),
            Err(error) => return Err(io(error)),
        }
    }
    if !altered.is_empty() {
        Ok(CardProjectionObservation::Altered { paths: altered })
    } else if !missing.is_empty() {
        Ok(CardProjectionObservation::Missing { paths: missing })
    } else {
        Ok(CardProjectionObservation::Healthy)
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
        operation: None,
    };
    let audit_digest = hash("semantic-audit-v1", &audit)?;
    let next = Snapshot {
        schema: Schema::CommitV3,
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
#[cfg(test)]
thread_local! {
    static FAIL_REBARRIER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
// Re-establish durability of validated retained files before pointer publication.
// Readers and previews intentionally never call this mutation-side barrier.
fn rebarrier(paths: impl IntoIterator<Item = PathBuf>, root: &Path) -> Result<(), Error> {
    for path in paths {
        #[cfg(test)]
        if FAIL_REBARRIER.with(|flag| flag.replace(false)) {
            return Err(Error::Io("injected retained-file sync failure".into()));
        }
        reject_symlinks(&path)?;
        let file = File::open(&path).map_err(io)?;
        if !file.metadata().map_err(io)?.is_file() {
            return Err(Error::UnsafePath);
        }
        file.sync_all().map_err(io)?;
        sync_chain(path.parent().ok_or(Error::UnsafePath)?, root)?;
    }
    Ok(())
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    read_current_with_pending(directory, key, false)
}
fn read_current_with_pending(
    directory: &Path,
    key: &IssueKey,
    allow_unactivated: bool,
) -> Result<Snapshot, Error> {
    reject_symlinks(directory)?;
    if (!allow_unactivated && directory.join("current.next").exists())
        || !directory.join("current.json").exists()
    {
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
        if generation == 0 || (!allow_unactivated && generation > newest.version().generation) {
            return Err(Error::RecoveryRequired);
        }
    }
    protocol::validate_objects(directory, &newest)?;
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

#[cfg(test)]
mod prepare_retry_tests {
    // PVF: deterministic tooling/unit, isolated disk, required semantic replay guard.
    use super::protocol::tests::Fixture;
    use super::*;

    fn current(f: &Fixture) -> Snapshot {
        read_current(&f.root.directory(&f.key).unwrap(), &f.key).unwrap()
    }

    #[test]
    fn prepare_retry_identical_snapshot_is_unchanged() {
        let f = Fixture::new();
        let before = current(&f);
        let result =
            DurableTransactionStore::prepare_issue(&f.root, f.key.clone(), before.inputs().clone())
                .unwrap();
        let CommitOutcome::Unchanged(after) = result else {
            panic!("retry committed");
        };
        assert_eq!(
            after.canonical_bytes().unwrap(),
            before.canonical_bytes().unwrap()
        );
        assert_eq!(
            current(&f).canonical_bytes().unwrap(),
            before.canonical_bytes().unwrap()
        );
    }

    #[test]
    fn prepare_retry_mismatched_inputs_are_rejected() {
        let f = Fixture::new();
        let before = current(&f);
        let mut changed = before.inputs().clone();
        changed.intent_plan.publication.title = "different title".into();
        assert!(matches!(
            DurableTransactionStore::prepare_issue(&f.root, f.key.clone(), changed),
            Err(Error::AlreadyExists)
        ));
        assert_eq!(
            current(&f).canonical_bytes().unwrap(),
            before.canonical_bytes().unwrap()
        );
    }

    #[test]
    fn prepare_retry_partial_activation_requires_recovery() {
        let f = Fixture::new();
        let before = current(&f);
        let directory = f.root.directory(&f.key).unwrap();
        fs::write(directory.join("current.next"), b"interrupted pointer").unwrap();
        assert!(matches!(
            DurableTransactionStore::prepare_issue(&f.root, f.key.clone(), before.inputs().clone()),
            Err(Error::RecoveryRequired)
        ));
        assert_eq!(
            fs::read(directory.join("current.next")).unwrap(),
            b"interrupted pointer"
        );
    }

    #[test]
    fn prepare_retry_damaged_snapshot_is_rejected() {
        let f = Fixture::new();
        let before = current(&f);
        let path = f
            .root
            .directory(&f.key)
            .unwrap()
            .join("commits")
            .join(name(before.version()));
        fs::write(&path, b"damaged snapshot").unwrap();
        assert!(DurableTransactionStore::prepare_issue(
            &f.root,
            f.key.clone(),
            before.inputs().clone()
        )
        .is_err());
        assert_eq!(fs::read(path).unwrap(), b"damaged snapshot");
    }

    #[test]
    fn prepare_retry_busy_store_is_not_observed_unlocked() {
        let f = Fixture::new();
        let before = current(&f);
        let _lock = acquire(&f.root.directory(&f.key).unwrap(), true).unwrap();
        assert!(matches!(
            DurableTransactionStore::prepare_issue(&f.root, f.key.clone(), before.inputs().clone()),
            Err(Error::Busy)
        ));
    }

    #[test]
    fn v2_remote_intent_residue_is_scoped_to_its_exact_issue() {
        let f = Fixture::new();
        let directory = f.root.common.join("csdlc-v3/remote/intents");
        fs::create_dir_all(&directory).unwrap();
        fs::write(
            directory.join("retained-v2.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema":"csdlc.v3.github_mutation_intent.v2",
                "request":{"repository":"example/repo","issue":99}
            }))
            .unwrap(),
        )
        .unwrap();
        let unrelated = IssueKey::new("example/repo", 98).unwrap();
        let retained = IssueKey::new("example/repo", 99).unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&f.root, &unrelated).unwrap(),
            Observation::Absent
        );
        assert_eq!(
            DurableTransactionStore::observe_issue(&f.root, &retained).unwrap(),
            Observation::LegacyMigrationRequired
        );
    }
}
