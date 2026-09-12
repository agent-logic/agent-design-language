//! Durable effect protocol. No method runs a subprocess or remote mutation.
//! Native owners must mint the crate-restricted admission/outcome witnesses only
//! after authenticating authority, topology and the actual retained native result.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(Digest);
impl OperationId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeIdentity {
    owner: String,
    identity: String,
}
impl NativeIdentity {
    pub(crate) fn new(owner: String, identity: String) -> Result<Self, Error> {
        if owner.trim().is_empty()
            || identity.trim().is_empty()
            || owner.contains('\0')
            || identity.contains('\0')
        {
            return Err(Error::InvalidInput("native identity missing".into()));
        }
        Ok(Self { owner, identity })
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn identity(&self) -> &str {
        &self.identity
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindSource {
    repository: String,
    common: PathBuf,
    checkout: PathBuf,
    head: String,
}
impl BindSource {
    pub(crate) fn from_native_owner(
        repository: String,
        common: PathBuf,
        checkout: PathBuf,
        head: String,
    ) -> Result<Self, Error> {
        IssueKey::new(repository.clone(), 1)?;
        if !common.is_absolute()
            || !checkout.is_absolute()
            || head.len() != 40
            || !head.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(Error::AdmissionChanged);
        }
        Ok(Self {
            repository,
            common,
            checkout,
            head,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CleanupIdentity {
    terminal_receipt: Vec<u8>,
    preview: Vec<u8>,
    archive: Vec<u8>,
}
impl CleanupIdentity {
    pub(crate) fn from_native_owner(
        terminal_receipt: Vec<u8>,
        preview: Vec<u8>,
        archive: Vec<u8>,
    ) -> Result<Self, Error> {
        if terminal_receipt.is_empty() || preview.is_empty() || archive.is_empty() {
            return Err(Error::EvidenceMismatch);
        }
        Ok(Self {
            terminal_receipt,
            preview,
            archive,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum OriginData {
    Prepared {
        source: BindSource,
    },
    Bound {
        binding: Binding,
    },
    Bind {
        source: BindSource,
        target: Binding,
    },
    Cleanup {
        binding: Binding,
        identity: CleanupIdentity,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EffectOrigin(OriginData);
impl EffectOrigin {
    pub(crate) fn prepared(source: BindSource) -> Self {
        Self(OriginData::Prepared { source })
    }
    pub(crate) fn bound(binding: Binding) -> Self {
        Self(OriginData::Bound { binding })
    }
    pub(crate) fn bind(source: BindSource, target: Binding) -> Self {
        Self(OriginData::Bind { source, target })
    }
    pub(crate) fn cleanup(binding: Binding, identity: CleanupIdentity) -> Self {
        Self(OriginData::Cleanup { binding, identity })
    }
    fn source_head(&self) -> &str {
        match &self.0 {
            OriginData::Bind { source, .. } | OriginData::Prepared { source } => &source.head,
            OriginData::Bound { binding } | OriginData::Cleanup { binding, .. } => &binding.head,
        }
    }
    fn matches(&self, root: &SemanticRoot, snapshot: &Snapshot, command: SemanticCommand) -> bool {
        match &self.0 {
            OriginData::Prepared { source } => {
                snapshot.inputs().binding().is_none()
                    && snapshot.phase() == LifecycleState::Ready
                    && source.repository == snapshot.key().repository
                    && source.common == root.common
                    && matches!(
                        command,
                        SemanticCommand::FinishWithoutPr
                            | SemanticCommand::RecordIssueMutation
                            | SemanticCommand::RecordInstall
                            | SemanticCommand::RecordCutover
                            | SemanticCommand::RecordRollback
                    )
            }
            OriginData::Bind { source, target } => {
                command == SemanticCommand::Bind
                    && snapshot.inputs().binding().is_none()
                    && source.repository == snapshot.key().repository
                    && source.common == root.common
                    && source.checkout.is_absolute()
                    && target.worktree.is_absolute()
                    && target.branch.starts_with("codex/")
                    && target.head.len() == 40
            }
            OriginData::Bound { binding } => {
                command != SemanticCommand::Bind
                    && command != SemanticCommand::RecordCleanup
                    && snapshot.inputs().binding() == Some(binding)
            }
            OriginData::Cleanup { binding, identity } => {
                command == SemanticCommand::RecordCleanup
                    && snapshot.inputs().binding() == Some(binding)
                    && !identity.terminal_receipt.is_empty()
                    && !identity.preview.is_empty()
                    && !identity.archive.is_empty()
            }
        }
    }
}

/// Syntax/normalization is not authority. Request construction stays in the native
/// owner bridge; the retained canonical request is written before reservation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectRequest {
    command: SemanticCommand,
    native: NativeIdentity,
    origin: EffectOrigin,
    content: serde_json::Value,
}
impl EffectRequest {
    pub(crate) fn new(
        command: SemanticCommand,
        native: NativeIdentity,
        origin: EffectOrigin,
        bytes: &[u8],
    ) -> Result<Self, Error> {
        let content = codec::decode(bytes).map_err(encoding)?;
        Ok(Self {
            command,
            native,
            origin,
            content,
        })
    }
    pub fn command(&self) -> SemanticCommand {
        self.command
    }
    pub fn native_identity(&self) -> &NativeIdentity {
        &self.native
    }
    pub fn origin(&self) -> &EffectOrigin {
        &self.origin
    }
    pub fn canonical_content(&self) -> Result<Vec<u8>, Error> {
        codec::bytes(&self.content).map_err(encoding)
    }
}

/// Not Deserialize: parsing user data cannot grant native admission.
#[derive(Debug, Clone)]
pub struct EffectAdmission {
    admission: Admission,
    origin: EffectOrigin,
    facts: policy::Facts,
}
impl EffectAdmission {
    pub(crate) fn from_native_owner(
        admission: Admission,
        origin: EffectOrigin,
        facts: policy::Facts,
    ) -> Self {
        Self {
            admission,
            origin,
            facts,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Request,
    BindReceipt,
    ProofRun,
    GithubReadback,
    CleanupReceipt,
    AdministrativeReceipt,
}
fn evidence_kind(command: SemanticCommand) -> EvidenceKind {
    match command {
        SemanticCommand::Bind => EvidenceKind::BindReceipt,
        SemanticCommand::RecordProof => EvidenceKind::ProofRun,
        SemanticCommand::RecordCleanup => EvidenceKind::CleanupReceipt,
        SemanticCommand::RecordInstall
        | SemanticCommand::RecordCutover
        | SemanticCommand::RecordRollback => EvidenceKind::AdministrativeReceipt,
        _ => EvidenceKind::GithubReadback,
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRef {
    digest: Digest,
    object: String,
    kind: EvidenceKind,
    source_head: String,
    inputs: EvidenceInputVersion,
}
impl EvidenceRef {
    pub fn kind(&self) -> EvidenceKind {
        self.kind
    }
    pub fn source_head(&self) -> &str {
        &self.source_head
    }
    pub fn inputs_version(&self) -> &EvidenceInputVersion {
        &self.inputs
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectTruth {
    NotPerformed,
    Performed,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeKind {
    Success,
    Failure,
    Unresolved,
}

/// No Deserialize/public constructor: native authenticated readback and execution
/// owners alone attest outcome truth. Durable evidence is separately hash-bound.
#[derive(Debug, Clone)]
pub struct VerifiedOutcome {
    kind: OutcomeKind,
    truth: EffectTruth,
    evidence: Vec<u8>,
    facts: policy::Facts,
    native: NativeIdentity,
}
impl VerifiedOutcome {
    pub(crate) fn from_native_owner(
        kind: OutcomeKind,
        truth: EffectTruth,
        evidence: Vec<u8>,
        facts: policy::Facts,
        native: NativeIdentity,
    ) -> Result<Self, Error> {
        if evidence.is_empty() || (kind == OutcomeKind::Success && truth == EffectTruth::Unknown) {
            return Err(Error::EvidenceMismatch);
        }
        Ok(Self {
            kind,
            truth,
            evidence,
            facts,
            native,
        })
    }
}
#[derive(Debug, Clone)]
pub struct AttachmentAdmission {
    authority: Digest,
    origin: EffectOrigin,
}
impl AttachmentAdmission {
    pub(crate) fn from_native_owner(authority: Digest, origin: EffectOrigin) -> Self {
        Self { authority, origin }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RetainedOutcome {
    kind: OutcomeKind,
    truth: EffectTruth,
    native: NativeIdentity,
    evidence: EvidenceRef,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingOperation {
    id: OperationId,
    request: EvidenceRef,
    command: SemanticCommand,
    native: NativeIdentity,
    origin: EffectOrigin,
    admitted: SemanticVersion,
    reserved_generation: u64,
    inputs: EvidenceInputVersion,
    authority: Digest,
    observed: Option<RetainedOutcome>,
}
impl PendingOperation {
    pub fn id(&self) -> &OperationId {
        &self.id
    }
    pub fn command(&self) -> SemanticCommand {
        self.command
    }
    pub fn observed_truth(&self) -> Option<EffectTruth> {
        self.observed.as_ref().map(|o| o.truth)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedOperation {
    id: OperationId,
    request: EvidenceRef,
    native: NativeIdentity,
    outcome: RetainedOutcome,
    attachment_generation: u64,
}
impl CompletedOperation {
    pub fn id(&self) -> &OperationId {
        &self.id
    }
    pub fn truth(&self) -> EffectTruth {
        self.outcome.truth
    }
    pub fn outcome(&self) -> OutcomeKind {
        self.outcome.kind
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationTicket {
    key: IssueKey,
    id: OperationId,
    request: EvidenceRef,
    reserved: SemanticVersion,
}
impl OperationTicket {
    pub fn id(&self) -> &OperationId {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reservation {
    Reserved(OperationTicket),
    AlreadyPending(OperationTicket),
    AlreadyCompleted(Completion),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    operation: OperationId,
    original: SemanticVersion,
    current: SemanticVersion,
    kind: OutcomeKind,
    truth: EffectTruth,
}
impl Completion {
    pub fn operation_id(&self) -> &OperationId {
        &self.operation
    }
    pub fn outcome_kind(&self) -> OutcomeKind {
        self.kind
    }
    pub fn original_version(&self) -> &SemanticVersion {
        &self.original
    }
    pub fn current_version(&self) -> &SemanticVersion {
        &self.current
    }
    pub fn truth(&self) -> EffectTruth {
        self.truth
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attachment {
    Completed(Completion),
    AlreadyCompleted(Completion),
    RecoveryRequired(SemanticVersion),
}
/// Explicit native-owner approval of the exact changed-admission recovery. No
/// phase is supplied: normal policy still decides and stale evidence is invalidated.
#[derive(Debug, Clone)]
pub struct VerifiedRecoveryResolution {
    preview: Digest,
}
impl VerifiedRecoveryResolution {
    pub(crate) fn adopt_observed_after_native_reconciliation(preview: &RecoveryPreview) -> Self {
        Self {
            preview: preview.digest.clone(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EffectInspection {
    request: EffectRequest,
    ticket: Option<OperationTicket>,
    evidence: Option<Vec<u8>>,
    outcome: Option<OutcomeKind>,
    truth: Option<EffectTruth>,
}
impl EffectInspection {
    pub fn request(&self) -> &EffectRequest {
        &self.request
    }
    pub fn ticket(&self) -> Option<&OperationTicket> {
        self.ticket.as_ref()
    }
    pub fn evidence(&self) -> Option<&[u8]> {
        self.evidence.as_deref()
    }
    pub fn outcome_kind(&self) -> Option<OutcomeKind> {
        self.outcome
    }
    pub fn effect_truth(&self) -> Option<EffectTruth> {
        self.truth
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPreview {
    key: IssueKey,
    version: SemanticVersion,
    operation: OperationId,
    digest: Digest,
}
impl RecoveryPreview {
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

fn persist_blob(
    directory: &Path,
    common: &Path,
    bytes: &[u8],
    domain: &str,
    kind: EvidenceKind,
    source_head: String,
    inputs: EvidenceInputVersion,
) -> Result<EvidenceRef, Error> {
    let digest = hash_bytes(domain, bytes);
    let object = format!("{}.blob", digest.0.replace(':', "-"));
    let parent = directory.join("objects");
    create_directories(&parent, common)?;
    let path = parent.join(&object);
    reject_symlinks(&path)?;
    if path.try_exists().map_err(io)? {
        if fs::read(&path).map_err(io)? != bytes {
            return Err(Error::EvidenceMismatch);
        }
        File::open(&path).and_then(|f| f.sync_all()).map_err(io)?;
        sync_chain(&parent, common)?;
    } else {
        create_only(&path, bytes, common)?;
    }
    Ok(EvidenceRef {
        digest,
        object,
        kind,
        source_head,
        inputs,
    })
}
fn read_blob(directory: &Path, reference: &EvidenceRef) -> Result<Vec<u8>, Error> {
    let expected = format!("{}.blob", reference.digest.0.replace(':', "-"));
    if reference.object != expected {
        return Err(Error::EvidenceMismatch);
    }
    let path = directory.join("objects").join(&reference.object);
    reject_symlinks(&path)?;
    let bytes = fs::read(path).map_err(io)?;
    let domain = reference
        .digest
        .0
        .split(':')
        .next()
        .ok_or(Error::InvalidDigest)?;
    if !matches!(domain, "semantic-request-v1" | "semantic-evidence-v1")
        || hash_bytes(domain, &bytes) != reference.digest
    {
        return Err(Error::EvidenceMismatch);
    }
    Ok(bytes)
}
fn operation_id(key: &IssueKey, request: &EffectRequest) -> Result<OperationId, Error> {
    Ok(OperationId(hash("semantic-operation-v1", &(key, request))?))
}
fn version_at(
    directory: &Path,
    current: &Snapshot,
    generation: u64,
) -> Result<SemanticVersion, Error> {
    let mut snapshot = current.clone();
    while snapshot.version().generation > generation {
        snapshot = load_commit(
            directory,
            snapshot.audit.before.as_ref().ok_or(Error::InvalidDigest)?,
        )?;
    }
    if snapshot.version().generation != generation {
        return Err(Error::InvalidDigest);
    }
    Ok(snapshot.version().clone())
}
fn ticket(
    directory: &Path,
    snapshot: &Snapshot,
    pending: &PendingOperation,
) -> Result<OperationTicket, Error> {
    Ok(OperationTicket {
        key: snapshot.key().clone(),
        id: pending.id.clone(),
        request: pending.request.clone(),
        reserved: version_at(directory, snapshot, pending.reserved_generation)?,
    })
}
fn completion(
    directory: &Path,
    snapshot: &Snapshot,
    done: &CompletedOperation,
) -> Result<Completion, Error> {
    Ok(Completion {
        operation: done.id.clone(),
        original: version_at(directory, snapshot, done.attachment_generation)?,
        current: snapshot.version().clone(),
        kind: done.outcome.kind,
        truth: done.outcome.truth,
    })
}
fn next_snapshot(
    current: &Snapshot,
    mut payload: Payload,
    command: SemanticCommand,
    id: OperationId,
) -> Result<Snapshot, Error> {
    payload.generation = current
        .payload
        .generation
        .checked_add(1)
        .ok_or(Error::ExhaustedVersion)?;
    let mut snapshot = make_snapshot(payload, Some(current), command)?;
    snapshot.audit.operation = Some(id);
    snapshot.audit_digest = hash("semantic-audit-v1", &snapshot.audit)?;
    Ok(snapshot)
}
fn check_ticket(
    directory: &Path,
    current: &Snapshot,
    supplied: &OperationTicket,
    pending: &PendingOperation,
) -> Result<(), Error> {
    if &ticket(directory, current, pending)? != supplied {
        return Err(Error::ConflictingReplay);
    }
    Ok(())
}

impl DurableTransactionStore {
    pub fn reserve_effect(
        root: &SemanticRoot,
        admission: EffectAdmission,
        request: EffectRequest,
    ) -> Result<Reservation, Error> {
        let directory = root.directory(&admission.admission.key)?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(&directory, &admission.admission.key)?;
        let id = operation_id(current.key(), &request)?;
        // Resolve retained identity before ordinary CAS staleness. Replay performs no write.
        if current
            .completed()
            .iter()
            .any(|d| d.native == request.native && d.id != id)
            || current
                .pending()
                .is_some_and(|p| p.native == request.native && p.id != id)
        {
            return Err(Error::ConflictingReplay);
        }
        if let Some(done) = current.completed().iter().find(|d| d.id == id) {
            return Ok(Reservation::AlreadyCompleted(completion(
                &directory, &current, done,
            )?));
        }
        if let Some(pending) = current.pending() {
            if pending.id == id {
                return Ok(Reservation::AlreadyPending(ticket(
                    &directory, &current, pending,
                )?));
            }
            return Err(Error::PendingOperation);
        }
        if current.version() != &admission.admission.expected {
            return Err(Error::StaleVersion);
        }
        if current.inputs().authority() != &admission.admission.authority
            || request.origin != admission.origin
            || !request.origin.matches(root, &current, request.command)
        {
            return Err(Error::AdmissionChanged);
        }
        let mut facts = admission.facts;
        facts.original_command = Some(request.command);
        policy::decide(
            Some(current.phase()),
            SemanticCommand::Reserve,
            policy::Outcome::Success,
            &facts,
        )
        .map_err(|_| Error::AdmissionChanged)?;
        let reference = persist_blob(
            &directory,
            &root.common,
            &codec::bytes(&request).map_err(encoding)?,
            "semantic-request-v1",
            EvidenceKind::Request,
            request.origin.source_head().to_owned(),
            current.inputs_version().clone(),
        )?;
        let mut payload = current.payload.clone();
        payload.pending = Some(PendingOperation {
            id: id.clone(),
            request: reference,
            command: request.command,
            native: request.native,
            origin: request.origin,
            admitted: current.version().clone(),
            reserved_generation: current
                .version()
                .generation
                .checked_add(1)
                .ok_or(Error::ExhaustedVersion)?,
            inputs: current.inputs_version().clone(),
            authority: admission.admission.authority,
            observed: None,
        });
        let next = next_snapshot(&current, payload, SemanticCommand::Reserve, id)?;
        activate(&directory, &root.common, &next)?;
        Ok(Reservation::Reserved(ticket(
            &directory,
            &next,
            next.pending().expect("reserved"),
        )?))
    }
    pub fn attach_outcome(
        root: &SemanticRoot,
        ticket: OperationTicket,
        outcome: VerifiedOutcome,
        observed: AttachmentAdmission,
    ) -> Result<Attachment, Error> {
        let directory = root.directory(&ticket.key)?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(&directory, &ticket.key)?;
        attach_locked(root, &directory, current, ticket, outcome, observed, false)
    }
    pub fn describe_effect_recovery(
        root: &SemanticRoot,
        key: &IssueKey,
    ) -> Result<Option<RecoveryPreview>, Error> {
        let directory = root.directory(key)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        let current = read_current(&directory, key)?;
        current
            .pending()
            .map(|p| recovery_preview(&current, p))
            .transpose()
    }
    pub fn execute_effect_recovery(
        root: &SemanticRoot,
        preview: RecoveryPreview,
        outcome: VerifiedOutcome,
        observed: AttachmentAdmission,
    ) -> Result<Attachment, Error> {
        Self::execute_effect_recovery_with_resolution(root, preview, outcome, observed, None)
    }
    pub fn execute_effect_recovery_with_resolution(
        root: &SemanticRoot,
        preview: RecoveryPreview,
        outcome: VerifiedOutcome,
        observed: AttachmentAdmission,
        resolution: Option<VerifiedRecoveryResolution>,
    ) -> Result<Attachment, Error> {
        let directory = root.directory(&preview.key)?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(&directory, &preview.key)?;
        if current
            .completed()
            .iter()
            .any(|d| d.id == preview.operation)
        {
            let historical = load_commit(&directory, &preview.version)?;
            let pending = historical.pending().ok_or(Error::ConflictingReplay)?;
            if recovery_preview(&historical, pending)? != preview {
                return Err(Error::ConflictingReplay);
            }
            let ticket = ticket(&directory, &historical, pending)?;
            return attach_locked(root, &directory, current, ticket, outcome, observed, false);
        }
        let pending = current.pending().ok_or(Error::ConflictingReplay)?;
        if recovery_preview(&current, pending)? != preview {
            return Err(Error::StaleVersion);
        }
        let adopt = match resolution {
            Some(witness) if witness.preview == preview.digest => true,
            Some(_) => return Err(Error::StaleVersion),
            None => false,
        };
        let ticket = ticket(&directory, &current, pending)?;
        attach_locked(root, &directory, current, ticket, outcome, observed, adopt)
    }
    pub(crate) fn inspect_effect(
        root: &SemanticRoot,
        key: &IssueKey,
        id: &OperationId,
    ) -> Result<EffectInspection, Error> {
        let directory = root.directory(key)?;
        let _lock = acquire(&directory, false)?;
        let current = read_current(&directory, key)?;
        let (reference, outcome, ticket) =
            if let Some(p) = current.pending().filter(|p| &p.id == id) {
                (
                    &p.request,
                    p.observed.as_ref(),
                    Some(ticket(&directory, &current, p)?),
                )
            } else if let Some(done) = current.completed().iter().find(|d| &d.id == id) {
                (&done.request, Some(&done.outcome), None)
            } else {
                return Err(Error::ConflictingReplay);
            };
        Ok(EffectInspection {
            request: codec::decode(&read_blob(&directory, reference)?).map_err(encoding)?,
            ticket,
            evidence: outcome
                .map(|o| read_blob(&directory, &o.evidence))
                .transpose()?,
            outcome: outcome.map(|o| o.kind),
            truth: outcome.map(|o| o.truth),
        })
    }
}
fn recovery_preview(
    current: &Snapshot,
    pending: &PendingOperation,
) -> Result<RecoveryPreview, Error> {
    let digest = hash(
        "semantic-recovery-v1",
        &(
            current.key(),
            current.version(),
            &pending.id,
            &pending.observed,
        ),
    )?;
    Ok(RecoveryPreview {
        key: current.key().clone(),
        version: current.version().clone(),
        operation: pending.id.clone(),
        digest,
    })
}
fn attach_locked(
    root: &SemanticRoot,
    directory: &Path,
    current: Snapshot,
    ticket: OperationTicket,
    outcome: VerifiedOutcome,
    observed: AttachmentAdmission,
    adopt: bool,
) -> Result<Attachment, Error> {
    if let Some(done) = current.completed().iter().find(|d| d.id == ticket.id) {
        if done.request != ticket.request
            || done.native != outcome.native
            || done.outcome.kind != outcome.kind
            || done.outcome.truth != outcome.truth
            || read_blob(directory, &done.outcome.evidence)? != outcome.evidence
        {
            return Err(Error::ConflictingReplay);
        }
        return Ok(Attachment::AlreadyCompleted(completion(
            directory, &current, done,
        )?));
    }
    let pending = current.pending().ok_or(Error::ConflictingReplay)?.clone();
    check_ticket(directory, &current, &ticket, &pending)?;
    if outcome.native != pending.native {
        return Err(Error::ConflictingReplay);
    }
    let evidence = persist_blob(
        directory,
        &root.common,
        &outcome.evidence,
        "semantic-evidence-v1",
        evidence_kind(pending.command),
        pending.origin.source_head().to_owned(),
        pending.inputs.clone(),
    )?;
    let retained = RetainedOutcome {
        kind: outcome.kind,
        truth: outcome.truth,
        native: outcome.native,
        evidence,
    };
    let mut payload = current.payload.clone();
    let changed = observed.authority != pending.authority
        || observed.origin != pending.origin
        || current.inputs_version() != &pending.inputs;
    if (changed && !adopt)
        || outcome.truth == EffectTruth::Unknown
        || outcome.kind == OutcomeKind::Unresolved
    {
        if pending.observed.as_ref() == Some(&retained) {
            return Ok(Attachment::RecoveryRequired(current.version().clone()));
        }
        payload.pending.as_mut().expect("pending").observed = Some(retained);
        let next = next_snapshot(&current, payload, SemanticCommand::Reserve, pending.id)?;
        activate(directory, &root.common, &next)?;
        return Ok(Attachment::RecoveryRequired(next.version().clone()));
    }
    if changed && adopt {
        // Never adopt a different worktree/issue or substitute cleanup archive identity.
        let compatible = match (&pending.origin.0, &observed.origin.0) {
            (OriginData::Prepared { source: a }, OriginData::Prepared { source: b }) => {
                a.repository == b.repository && a.common == b.common && a.checkout == b.checkout
            }
            (
                OriginData::Bind {
                    source: a,
                    target: x,
                },
                OriginData::Bind {
                    source: b,
                    target: y,
                },
            ) => {
                a.repository == b.repository
                    && a.common == b.common
                    && a.checkout == b.checkout
                    && x == y
            }
            (OriginData::Bound { binding: a }, OriginData::Bound { binding: b }) => {
                a.branch == b.branch && a.worktree == b.worktree && a.registration == b.registration
            }
            (
                OriginData::Cleanup {
                    binding: a,
                    identity: x,
                },
                OriginData::Cleanup {
                    binding: b,
                    identity: y,
                },
            ) => a == b && x == y,
            _ => false,
        };
        if !compatible {
            return Err(Error::AdmissionChanged);
        }
        payload.inputs.authority = observed.authority;
        if let OriginData::Bound { binding } = &observed.origin.0 {
            payload.inputs.binding = Some(binding.clone());
        }
        payload.inputs.validate()?;
        payload.invalidations.extend([
            Invalidation::Proof,
            Invalidation::Readiness,
            Invalidation::Review,
            Invalidation::Publication,
            Invalidation::Terminal,
            Invalidation::Cleanup,
        ]);
    }
    let disposition = if outcome.kind == OutcomeKind::Success {
        policy::Outcome::Success
    } else {
        policy::Outcome::Failure
    };
    let decision = policy::decide(
        Some(current.phase()),
        pending.command,
        disposition,
        &outcome.facts,
    )
    .map_err(|_| Error::AdmissionChanged)?;
    if outcome.kind == OutcomeKind::Success && pending.command == SemanticCommand::Bind {
        let OriginData::Bind { target, .. } = &pending.origin.0 else {
            return Err(Error::AdmissionChanged);
        };
        payload.inputs.binding = Some(target.clone());
        payload.inputs.validate()?;
    }
    if payload.inputs != current.payload.inputs {
        payload.input_version = EvidenceInputVersion {
            revision: current
                .payload
                .input_version
                .revision
                .checked_add(1)
                .ok_or(Error::ExhaustedVersion)?,
            digest: hash("semantic-input-v1", &payload.inputs)?,
        };
    }
    payload.phase = decision.phase;
    payload.invalidations.extend(decision.invalidations);
    payload.invalidations.sort();
    payload.invalidations.dedup();
    payload.projection_required = true;
    payload.pending = None;
    payload.completed.push(CompletedOperation {
        id: pending.id.clone(),
        request: pending.request,
        native: pending.native,
        outcome: retained,
        attachment_generation: current
            .version()
            .generation
            .checked_add(1)
            .ok_or(Error::ExhaustedVersion)?,
    });
    let next = next_snapshot(&current, payload, pending.command, pending.id)?;
    activate(directory, &root.common, &next)?;
    Ok(Attachment::Completed(completion(
        directory,
        &next,
        next.completed().last().expect("completion"),
    )?))
}

pub(super) fn retained_object_paths(directory: &Path, snapshot: &Snapshot) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut add =
        |reference: &EvidenceRef| paths.push(directory.join("objects").join(&reference.object));
    for done in snapshot.completed() {
        add(&done.request);
        add(&done.outcome.evidence);
    }
    if let Some(pending) = snapshot.pending() {
        add(&pending.request);
        if let Some(outcome) = &pending.observed {
            add(&outcome.evidence);
        }
    }
    paths
}

pub(super) fn validate_objects(directory: &Path, snapshot: &Snapshot) -> Result<(), Error> {
    let mut ids = std::collections::BTreeSet::new();
    for done in snapshot.completed() {
        if !ids.insert(done.id.as_str())
            || done.attachment_generation > snapshot.version().generation
        {
            return Err(Error::InvalidDigest);
        }
        let request: EffectRequest =
            codec::decode(&read_blob(directory, &done.request)?).map_err(encoding)?;
        if operation_id(snapshot.key(), &request)? != done.id
            || request.native != done.native
            || done.outcome.native != done.native
        {
            return Err(Error::EvidenceMismatch);
        }
        if done.request.kind != EvidenceKind::Request
            || done.outcome.evidence.kind != evidence_kind(request.command)
            || done.outcome.evidence.inputs != done.request.inputs
            || done.outcome.evidence.source_head != done.request.source_head
        {
            return Err(Error::EvidenceMismatch);
        }
        read_blob(directory, &done.outcome.evidence)?;
    }
    if let Some(pending) = snapshot.pending() {
        if !ids.insert(pending.id.as_str())
            || pending.reserved_generation > snapshot.version().generation
            || pending.admitted.generation.checked_add(1) != Some(pending.reserved_generation)
        {
            return Err(Error::InvalidDigest);
        }
        let request: EffectRequest =
            codec::decode(&read_blob(directory, &pending.request)?).map_err(encoding)?;
        if operation_id(snapshot.key(), &request)? != pending.id
            || request.command != pending.command
            || request.native != pending.native
            || request.origin != pending.origin
        {
            return Err(Error::EvidenceMismatch);
        }
        if pending.request.kind != EvidenceKind::Request
            || pending.request.inputs != pending.inputs
            || pending.request.source_head != pending.origin.source_head()
        {
            return Err(Error::EvidenceMismatch);
        }
        if let Some(outcome) = &pending.observed {
            if outcome.evidence.inputs != pending.inputs
                || outcome.evidence.source_head != pending.origin.source_head()
                || outcome.evidence.kind != evidence_kind(pending.command)
            {
                return Err(Error::EvidenceMismatch);
            }
            read_blob(directory, &outcome.evidence)?;
        }
    }
    Ok(())
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    pub(in crate::storage::semantic) struct Fixture {
        pub(in crate::storage::semantic) path: PathBuf,
        pub(in crate::storage::semantic) root: SemanticRoot,
        pub(in crate::storage::semantic) key: IssueKey,
    }
    impl Fixture {
        pub(in crate::storage::semantic) fn new() -> Self {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join(format!(
                    "effect-protocol-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(path.join("repo/.git/objects")).unwrap();
            fs::write(path.join("repo/.git/HEAD"), "ref: refs/heads/main").unwrap();
            fs::write(path.join("repo/.git/config"), "[core]").unwrap();
            let root =
                SemanticRoot::from_git_common(path.join("repo/.git"), "example/repo").unwrap();
            let key = IssueKey::new("example/repo", 870).unwrap();
            let plan = AcceptedIntentPlan {
                schema: "csdlc.v3.intent_plan.v1".into(),
                slug: "test".into(),
                cards: ["sip", "stp", "spp", "vpp", "srp", "sor"]
                    .into_iter()
                    .map(|k| (k.into(), serde_json::json!({})))
                    .collect(),
                validators: vec![],
                publication: Publication {
                    base: "main".into(),
                    title: "title".into(),
                    body: "Closes #870".into(),
                    draft: true,
                },
            };
            let inputs = IssueInputs::new(
                "intent".into(),
                plan,
                vec![PlanStep {
                    id: "step".into(),
                    acceptance: "done".into(),
                }],
                None,
                Digest::authority(b"authority"),
            )
            .unwrap();
            DurableTransactionStore::prepare_issue(&root, key.clone(), inputs).unwrap();
            Self { path, root, key }
        }
        fn snapshot(&self) -> Snapshot {
            read_current(&self.root.directory(&self.key).unwrap(), &self.key).unwrap()
        }
        fn bind_request(&self) -> EffectRequest {
            let target = Binding {
                branch: "codex/870-test".into(),
                head: "a".repeat(40),
                worktree: self.path.join("target-checkout"),
                registration: "registration".into(),
            };
            EffectRequest::new(
                SemanticCommand::Bind,
                NativeIdentity::new("bind".into(), "bind-native-id".into()).unwrap(),
                EffectOrigin::bind(
                    BindSource::from_native_owner(
                        "example/repo".into(),
                        self.root.common.clone(),
                        self.path.join("repo"),
                        "b".repeat(40),
                    )
                    .unwrap(),
                    target,
                ),
                br#"{"target":"declared"}"#,
            )
            .unwrap()
        }
        fn admission(&self, request: &EffectRequest) -> EffectAdmission {
            let s = self.snapshot();
            EffectAdmission::from_native_owner(
                Admission::new(
                    self.key.clone(),
                    s.version().clone(),
                    s.inputs().authority().clone(),
                ),
                request.origin.clone(),
                policy::Facts {
                    bind_target: true,
                    topology: true,
                    current_proof: true,
                    terminal_receipt: true,
                    ..Default::default()
                },
            )
        }
        fn outcome(
            &self,
            request: &EffectRequest,
            kind: OutcomeKind,
            truth: EffectTruth,
            bytes: &[u8],
        ) -> VerifiedOutcome {
            VerifiedOutcome::from_native_owner(
                kind,
                truth,
                bytes.to_vec(),
                policy::Facts {
                    bind_target: true,
                    topology: true,
                    current_proof: true,
                    terminal_receipt: true,
                    cleanup: true,
                    ..Default::default()
                },
                request.native.clone(),
            )
            .unwrap()
        }
        fn observed(&self, request: &EffectRequest) -> AttachmentAdmission {
            AttachmentAdmission::from_native_owner(
                self.snapshot().inputs().authority().clone(),
                request.origin.clone(),
            )
        }
        fn reserve(&self, request: &EffectRequest) -> OperationTicket {
            match DurableTransactionStore::reserve_effect(
                &self.root,
                self.admission(request),
                request.clone(),
            )
            .unwrap()
            {
                Reservation::Reserved(t) => t,
                other => panic!("{other:?}"),
            }
        }
        fn bind(&self) {
            let request = self.bind_request();
            let ticket = self.reserve(&request);
            DurableTransactionStore::attach_outcome(
                &self.root,
                ticket,
                self.outcome(
                    &request,
                    OutcomeKind::Success,
                    EffectTruth::Performed,
                    b"native bind receipt",
                ),
                self.observed(&request),
            )
            .unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
    #[test]
    fn pending_restart_and_completed_replay_survive_later_commits() {
        let f = Fixture::new();
        let request = f.bind_request();
        let admission = f.admission(&request);
        let ticket = f.reserve(&request);
        assert!(matches!(
            DurableTransactionStore::reserve_effect(&f.root, admission.clone(), request.clone())
                .unwrap(),
            Reservation::AlreadyPending(_)
        ));
        let bound_version = match DurableTransactionStore::attach_outcome(
            &f.root,
            ticket.clone(),
            f.outcome(
                &request,
                OutcomeKind::Success,
                EffectTruth::Performed,
                b"receipt",
            ),
            f.observed(&request),
        )
        .unwrap()
        {
            Attachment::Completed(c) => c.original,
            _ => panic!(),
        };
        assert_eq!(f.snapshot().phase(), LifecycleState::Bound);
        let snapshot = f.snapshot();
        let mut cards = snapshot.inputs().cards().clone();
        cards.insert("stp".into(), serde_json::json!({"changed":true}));
        DurableTransactionStore::commit_issue_local(
            &f.root,
            Admission::new(
                f.key.clone(),
                snapshot.version().clone(),
                snapshot.inputs().authority().clone(),
            ),
            LocalChange::AmendCards(cards),
        )
        .unwrap();
        let Reservation::AlreadyCompleted(done) =
            DurableTransactionStore::reserve_effect(&f.root, admission, request.clone()).unwrap()
        else {
            panic!()
        };
        assert_eq!(done.original, bound_version);
        assert!(done.current.generation > done.original.generation);
        assert!(matches!(
            DurableTransactionStore::attach_outcome(
                &f.root,
                ticket,
                f.outcome(
                    &request,
                    OutcomeKind::Success,
                    EffectTruth::Performed,
                    b"receipt"
                ),
                f.observed(&request)
            )
            .unwrap(),
            Attachment::AlreadyCompleted(_)
        ));
    }
    #[test]
    fn unknown_outcome_recovery_is_exact_and_observational() {
        let f = Fixture::new();
        let request = f.bind_request();
        let ticket = f.reserve(&request);
        let preview = DurableTransactionStore::describe_effect_recovery(&f.root, &f.key)
            .unwrap()
            .unwrap();
        assert!(matches!(
            DurableTransactionStore::attach_outcome(
                &f.root,
                ticket,
                f.outcome(
                    &request,
                    OutcomeKind::Unresolved,
                    EffectTruth::Unknown,
                    b"interrupted"
                ),
                f.observed(&request)
            )
            .unwrap(),
            Attachment::RecoveryRequired(_)
        ));
        assert_eq!(
            f.snapshot().pending().unwrap().observed_truth(),
            Some(EffectTruth::Unknown)
        );
        assert_eq!(
            DurableTransactionStore::execute_effect_recovery(
                &f.root,
                preview,
                f.outcome(
                    &request,
                    OutcomeKind::Success,
                    EffectTruth::Performed,
                    b"readback"
                ),
                f.observed(&request)
            ),
            Err(Error::StaleVersion)
        );
        let current = f.snapshot();
        let preview = DurableTransactionStore::describe_effect_recovery(&f.root, &f.key)
            .unwrap()
            .unwrap();
        assert_eq!(f.snapshot(), current);
        assert!(matches!(
            DurableTransactionStore::execute_effect_recovery(
                &f.root,
                preview,
                f.outcome(
                    &request,
                    OutcomeKind::Success,
                    EffectTruth::Performed,
                    b"readback"
                ),
                f.observed(&request)
            )
            .unwrap(),
            Attachment::Completed(_)
        ));
    }
    #[test]
    fn evidence_tampering_and_changed_authority_fail_closed() {
        let f = Fixture::new();
        let request = f.bind_request();
        let ticket = f.reserve(&request);
        let observed = AttachmentAdmission::from_native_owner(
            Digest::authority(b"other"),
            request.origin.clone(),
        );
        assert!(matches!(
            DurableTransactionStore::attach_outcome(
                &f.root,
                ticket,
                f.outcome(
                    &request,
                    OutcomeKind::Success,
                    EffectTruth::Performed,
                    b"actual success"
                ),
                observed
            )
            .unwrap(),
            Attachment::RecoveryRequired(_)
        ));
        assert_eq!(f.snapshot().phase(), LifecycleState::Ready);
        let s = f.snapshot();
        let evidence = &s.pending().unwrap().observed.as_ref().unwrap().evidence;
        fs::write(
            f.root
                .directory(&f.key)
                .unwrap()
                .join("objects")
                .join(&evidence.object),
            b"tamper",
        )
        .unwrap();
        assert_eq!(
            DurableTransactionStore::observe_issue(&f.root, &f.key),
            Err(Error::EvidenceMismatch)
        );
    }
    #[test]
    fn cleanup_attachment_uses_retained_origin_after_checkout_removal() {
        let f = Fixture::new();
        f.bind();
        let current = f.snapshot();
        // Seed a verified terminal state through the same commit machinery; this
        // bounded protocol test is not proof of native finish integration.
        let mut payload = current.payload.clone();
        payload.phase = LifecycleState::ClosedOut;
        payload.generation += 1;
        let closed =
            make_snapshot(payload, Some(&current), SemanticCommand::FinishWithoutPr).unwrap();
        activate(&f.root.directory(&f.key).unwrap(), &f.root.common, &closed).unwrap();
        let binding = closed.inputs().binding().unwrap().clone();
        fs::create_dir_all(&binding.worktree).unwrap();
        let request = EffectRequest::new(
            SemanticCommand::RecordCleanup,
            NativeIdentity::new("cleanup".into(), "native-cleanup".into()).unwrap(),
            EffectOrigin::cleanup(
                binding.clone(),
                CleanupIdentity::from_native_owner(
                    b"terminal".to_vec(),
                    b"preview".to_vec(),
                    b"archive".to_vec(),
                )
                .unwrap(),
            ),
            br#"{"preview":"exact"}"#,
        )
        .unwrap();
        let ticket = f.reserve(&request);
        fs::remove_dir(&binding.worktree).unwrap();
        DurableTransactionStore::attach_outcome(
            &f.root,
            ticket,
            f.outcome(
                &request,
                OutcomeKind::Success,
                EffectTruth::Performed,
                b"verified archive and absence",
            ),
            f.observed(&request),
        )
        .unwrap();
        assert_eq!(f.snapshot().phase(), LifecycleState::ClosedOut);
        assert!(f.snapshot().pending().is_none());
    }
    #[test]
    fn prepared_noop_success_and_restart_inspection_are_truthful() {
        let f = Fixture::new();
        let source = BindSource::from_native_owner(
            "example/repo".into(),
            f.root.common.clone(),
            f.path.join("repo"),
            "a".repeat(40),
        )
        .unwrap();
        let origin = EffectOrigin::prepared(source);
        let request = EffectRequest::new(
            SemanticCommand::RecordIssueMutation,
            NativeIdentity::new("github".into(), "prepared-metadata".into()).unwrap(),
            origin.clone(),
            br#"{"labels":["done"]}"#,
        )
        .unwrap();
        let ticket = f.reserve(&request);
        let inspection =
            DurableTransactionStore::inspect_effect(&f.root, &f.key, ticket.id()).unwrap();
        assert_eq!(inspection.request(), &request);
        assert_eq!(inspection.ticket(), Some(&ticket));
        assert!(inspection.evidence().is_none());
        let outcome = f.outcome(
            &request,
            OutcomeKind::Success,
            EffectTruth::NotPerformed,
            b"authenticated labels already match",
        );
        let Attachment::Completed(done) = DurableTransactionStore::attach_outcome(
            &f.root,
            ticket.clone(),
            outcome,
            f.observed(&request),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(done.truth(), EffectTruth::NotPerformed);
        assert_eq!(done.outcome_kind(), OutcomeKind::Success);
        assert_eq!(f.snapshot().phase(), LifecycleState::Ready);
        let inspection =
            DurableTransactionStore::inspect_effect(&f.root, &f.key, ticket.id()).unwrap();
        assert_eq!(
            inspection.evidence(),
            Some(b"authenticated labels already match".as_slice())
        );
        assert_eq!(inspection.effect_truth(), Some(EffectTruth::NotPerformed));
    }
    #[test]
    fn changed_authority_has_finite_explicit_recovery_and_completed_replay() {
        let f = Fixture::new();
        let request = f.bind_request();
        let ticket = f.reserve(&request);
        let changed = AttachmentAdmission::from_native_owner(
            Digest::authority(b"new authority"),
            request.origin.clone(),
        );
        let outcome = f.outcome(
            &request,
            OutcomeKind::Success,
            EffectTruth::Performed,
            b"original successful native effect",
        );
        assert!(matches!(
            DurableTransactionStore::attach_outcome(
                &f.root,
                ticket,
                outcome.clone(),
                changed.clone()
            )
            .unwrap(),
            Attachment::RecoveryRequired(_)
        ));
        let preview = DurableTransactionStore::describe_effect_recovery(&f.root, &f.key)
            .unwrap()
            .unwrap();
        let approval =
            VerifiedRecoveryResolution::adopt_observed_after_native_reconciliation(&preview);
        assert!(matches!(
            DurableTransactionStore::execute_effect_recovery_with_resolution(
                &f.root,
                preview.clone(),
                outcome.clone(),
                changed.clone(),
                Some(approval)
            )
            .unwrap(),
            Attachment::Completed(_)
        ));
        assert!(f.snapshot().pending().is_none());
        assert_eq!(
            f.snapshot().inputs().authority(),
            &Digest::authority(b"new authority")
        );
        assert!(f.snapshot().invalidations().contains(&Invalidation::Proof));
        let generation = f.snapshot().version().generation();
        assert!(matches!(
            DurableTransactionStore::execute_effect_recovery(&f.root, preview, outcome, changed)
                .unwrap(),
            Attachment::AlreadyCompleted(_)
        ));
        assert_eq!(f.snapshot().version().generation(), generation);
    }

    #[test]
    fn failed_effect_truth_is_retained_without_phase_success() {
        for truth in [EffectTruth::NotPerformed, EffectTruth::Performed] {
            let f = Fixture::new();
            f.bind();
            let snapshot = f.snapshot();
            let request = EffectRequest::new(
                SemanticCommand::RecordProof,
                NativeIdentity::new("proof".into(), "failed-proof".into()).unwrap(),
                EffectOrigin::bound(snapshot.inputs().binding().unwrap().clone()),
                b"{}",
            )
            .unwrap();
            let ticket = f.reserve(&request);
            let Attachment::Completed(result) = DurableTransactionStore::attach_outcome(
                &f.root,
                ticket,
                f.outcome(
                    &request,
                    OutcomeKind::Failure,
                    truth,
                    b"actual failed process evidence",
                ),
                f.observed(&request),
            )
            .unwrap() else {
                panic!()
            };
            assert_eq!(result.truth(), truth);
            assert_eq!(result.outcome_kind(), OutcomeKind::Failure);
            assert_eq!(f.snapshot().phase(), LifecycleState::Bound);
        }
    }

    #[test]
    fn concurrent_reservations_have_one_winner_and_no_lifetime_lock() {
        let f = Fixture::new();
        let request = f.bind_request();
        let admission = f.admission(&request);
        let results = std::thread::scope(|scope| {
            let a = scope.spawn(|| {
                DurableTransactionStore::reserve_effect(&f.root, admission.clone(), request.clone())
            });
            let b = scope.spawn(|| {
                DurableTransactionStore::reserve_effect(&f.root, admission.clone(), request.clone())
            });
            vec![a.join().unwrap(), b.join().unwrap()]
        });
        assert_eq!(
            results
                .iter()
                .filter(|r| matches!(r, Ok(Reservation::Reserved(_))))
                .count(),
            1
        );
        assert!(results.iter().all(|r| matches!(
            r,
            Ok(Reservation::Reserved(_) | Reservation::AlreadyPending(_)) | Err(Error::Busy)
        )));
        assert!(DurableTransactionStore::observe_issue(&f.root, &f.key).is_ok());
    }
}

pub mod creation;
