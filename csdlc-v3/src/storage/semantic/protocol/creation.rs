//! Repository-scoped remote issue creation. No IssueKey is made until verified
//! readback supplies a positive number; attachment never initializes issue cards.
use super::*;

#[derive(Debug, Clone)]
pub struct RepositoryAdmission {
    repository: String,
    authority: Digest,
    common: PathBuf,
    head: String,
}
impl RepositoryAdmission {
    pub(crate) fn from_native_owner(
        repository: String,
        authority: Digest,
        common: PathBuf,
        head: String,
    ) -> Result<Self, Error> {
        IssueKey::new(repository.clone(), 1)?;
        if !common.is_absolute() || head.len() != 40 || !head.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(Error::AdmissionChanged);
        }
        Ok(Self {
            repository,
            authority,
            common,
            head,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    repository: String,
    native: NativeIdentity,
    content: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreationTicket {
    repository: String,
    id: OperationId,
    request: EvidenceRef,
    reservation: Digest,
}
impl CreationTicket {
    pub fn id(&self) -> &OperationId {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreationOutcome {
    kind: OutcomeKind,
    truth: EffectTruth,
    issue: Option<IssueKey>,
    evidence: EvidenceRef,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    schema: String,
    repository: String,
    id: OperationId,
    generation: u64,
    previous: Option<Digest>,
    request: EvidenceRef,
    native: NativeIdentity,
    authority: Digest,
    head: String,
    observed: Option<CreationOutcome>,
    completed: Option<CreationOutcome>,
    recovery_adoption: Option<(Digest, String, Digest)>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreationSnapshot {
    payload: Payload,
    digest: Digest,
}
impl CreationSnapshot {
    pub fn operation_id(&self) -> &OperationId {
        &self.payload.id
    }
    pub fn generation(&self) -> u64 {
        self.payload.generation
    }
    pub fn created_issue(&self) -> Option<&IssueKey> {
        self.payload
            .completed
            .as_ref()
            .and_then(|o| o.issue.as_ref())
    }
    pub fn completed_kind(&self) -> Option<OutcomeKind> {
        self.payload.completed.as_ref().map(|o| o.kind)
    }
    pub fn completed_truth(&self) -> Option<EffectTruth> {
        self.payload.completed.as_ref().map(|o| o.truth)
    }
    pub fn observed_truth(&self) -> Option<EffectTruth> {
        self.payload.observed.as_ref().map(|o| o.truth)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(
    clippy::large_enum_variant,
    reason = "reservation outcomes retain complete typed snapshots for exact replay"
)]
pub enum CreationReservation {
    Reserved(CreationTicket),
    AlreadyPending(CreationTicket),
    AlreadyCompleted(CreationSnapshot),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreationAttachment {
    Completed(CreationSnapshot),
    AlreadyCompleted(CreationSnapshot),
    RecoveryRequired(CreationSnapshot),
}
#[derive(Debug, Clone)]
pub struct VerifiedCreationOutcome {
    kind: OutcomeKind,
    truth: EffectTruth,
    issue: Option<IssueKey>,
    evidence: Vec<u8>,
    native: NativeIdentity,
}
impl VerifiedCreationOutcome {
    pub(crate) fn from_native_owner(
        kind: OutcomeKind,
        truth: EffectTruth,
        issue: Option<IssueKey>,
        evidence: Vec<u8>,
        native: NativeIdentity,
    ) -> Result<Self, Error> {
        if evidence.is_empty()
            || (kind == OutcomeKind::Success && (issue.is_none() || truth == EffectTruth::Unknown))
            || (kind != OutcomeKind::Success && issue.is_some())
        {
            return Err(Error::EvidenceMismatch);
        }
        Ok(Self {
            kind,
            truth,
            issue,
            evidence,
            native,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreationRecoveryPreview {
    repository: String,
    action: String,
    id: OperationId,
    generation: u64,
    digest: Digest,
}
impl CreationRecoveryPreview {
    pub fn repository(&self) -> &str {
        &self.repository
    }
    pub fn operation_id(&self) -> &OperationId {
        &self.id
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn action(&self) -> &str {
        &self.action
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}
#[derive(Debug, Clone)]
pub struct VerifiedCreationRecovery {
    preview: CreationRecoveryPreview,
}
impl VerifiedCreationRecovery {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "reserved for authenticated native creation reconciliation"
        )
    )]
    pub(crate) fn adopt_after_native_reconciliation(preview: &CreationRecoveryPreview) -> Self {
        Self {
            preview: preview.clone(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CreationInspection {
    native: NativeIdentity,
    request: Vec<u8>,
    ticket: CreationTicket,
    evidence: Option<Vec<u8>>,
}
impl CreationInspection {
    pub fn native_identity(&self) -> &NativeIdentity {
        &self.native
    }
    pub fn request_bytes(&self) -> &[u8] {
        &self.request
    }
    pub fn ticket(&self) -> &CreationTicket {
        &self.ticket
    }
    pub fn evidence(&self) -> Option<&[u8]> {
        self.evidence.as_deref()
    }
}
fn validate_objects(
    root: &SemanticRoot,
    directory: &Path,
    current: &CreationSnapshot,
    id: &OperationId,
) -> Result<(), Error> {
    let request: Request =
        codec::decode(&read_blob(directory, &current.payload.request)?).map_err(encoding)?;
    if request.repository != root.repository
        || request.native != current.payload.native
        || OperationId(hash(
            "semantic-operation-v1",
            &(&request.repository, &request.native),
        )?) != *id
    {
        return Err(Error::EvidenceMismatch);
    }
    if current.payload.request.kind != EvidenceKind::Request
        || current.payload.request.source_head != current.payload.head
    {
        return Err(Error::EvidenceMismatch);
    }
    for outcome in [&current.payload.observed, &current.payload.completed]
        .into_iter()
        .flatten()
    {
        if outcome
            .issue
            .as_ref()
            .is_some_and(|key| key.repository() != root.repository)
            || outcome.evidence.kind != EvidenceKind::GithubReadback
            || outcome.evidence.inputs != current.payload.request.inputs
            || outcome.evidence.source_head != current.payload.head
        {
            return Err(Error::EvidenceMismatch);
        }
        read_blob(directory, &outcome.evidence)?;
    }
    let expected_inputs = EvidenceInputVersion {
        revision: 1,
        digest: hash(
            "semantic-input-v1",
            &(&request, &current.payload.authority, &current.payload.head),
        )?,
    };
    if current.payload.request.inputs != expected_inputs {
        return Err(Error::EvidenceMismatch);
    }
    if let Some(done) = &current.payload.completed {
        if done.kind == OutcomeKind::Unresolved
            || done.truth == EffectTruth::Unknown
            || (done.kind == OutcomeKind::Success) != done.issue.is_some()
        {
            return Err(Error::EvidenceMismatch);
        }
    }
    Ok(())
}
fn directory(root: &SemanticRoot, id: &OperationId) -> Result<PathBuf, Error> {
    if !id.0 .0.starts_with("semantic-operation-v1:") {
        return Err(Error::InvalidDigest);
    }
    let path = root
        .common
        .join("csdlc-v3/semantic/repository-operations")
        .join(id.0 .0.rsplit(':').next().ok_or(Error::InvalidDigest)?);
    reject_symlinks(&path)?;
    Ok(path)
}
fn validate_admission(root: &SemanticRoot, admission: &RepositoryAdmission) -> Result<(), Error> {
    if root.repository != admission.repository || root.common != admission.common {
        return Err(Error::WrongRepository);
    }
    Ok(())
}
fn record(payload: Payload) -> Result<CreationSnapshot, Error> {
    let digest = hash("semantic-state-v1", &payload)?;
    Ok(CreationSnapshot { payload, digest })
}
fn activate(root: &SemanticRoot, directory: &Path, next: &CreationSnapshot) -> Result<(), Error> {
    for leaf in ["commits", "intents"] {
        create_directories(&directory.join(leaf), &root.common)?;
    }
    let pointer = codec::bytes(&(next.payload.generation, &next.digest)).map_err(encoding)?;
    create_only(
        &directory
            .join("intents")
            .join(format!("{}.json", next.payload.generation)),
        &pointer,
        &root.common,
    )?;
    create_only(
        &directory
            .join("commits")
            .join(format!("{}.json", next.payload.generation)),
        &codec::bytes(next).map_err(encoding)?,
        &root.common,
    )?;
    create_only(&directory.join("current.next"), &pointer, &root.common)?;
    fs::rename(
        directory.join("current.next"),
        directory.join("current.json"),
    )
    .map_err(io)?;
    sync_chain(directory, &root.common)
}
fn load_record(directory: &Path, generation: u64) -> Result<CreationSnapshot, Error> {
    let path = directory.join("commits").join(format!("{generation}.json"));
    reject_symlinks(&path)?;
    let record: CreationSnapshot = codec::decode(&fs::read(path).map_err(io)?).map_err(encoding)?;
    if record.payload.schema != "csdlc.v3.repository_creation.v1"
        || record.payload.generation != generation
        || generation == 0
        || hash("semantic-state-v1", &record.payload)? != record.digest
    {
        return Err(Error::InvalidDigest);
    }
    Ok(record)
}
fn read_current(root: &SemanticRoot, id: &OperationId) -> Result<CreationSnapshot, Error> {
    read_current_with_pending(root, id, false)
}
fn read_current_with_pending(
    root: &SemanticRoot,
    id: &OperationId,
    allow_unactivated: bool,
) -> Result<CreationSnapshot, Error> {
    let directory = directory(root, id)?;
    if (!allow_unactivated && directory.join("current.next").try_exists().map_err(io)?)
        || !directory.join("current.json").try_exists().map_err(io)?
    {
        return Err(Error::RecoveryRequired);
    }
    let pointer_path = directory.join("current.json");
    reject_symlinks(&pointer_path)?;
    let (generation, digest): (u64, Digest) =
        codec::decode(&fs::read(pointer_path).map_err(io)?).map_err(encoding)?;
    let current = load_record(&directory, generation)?;
    if current.digest != digest
        || current.payload.repository != root.repository
        || current.operation_id() != id
    {
        return Err(Error::InvalidDigest);
    }
    validate_objects(root, &directory, &current, id)?;
    let mut cursor = current.clone();
    loop {
        let intent = directory
            .join("intents")
            .join(format!("{}.json", cursor.generation()));
        reject_symlinks(&intent)?;
        let pointer: (u64, Digest) =
            codec::decode(&fs::read(intent).map_err(io)?).map_err(encoding)?;
        if pointer != (cursor.generation(), cursor.digest.clone()) {
            return Err(Error::InvalidDigest);
        }
        if cursor.generation() == 1 {
            if cursor.payload.previous.is_some() {
                return Err(Error::InvalidDigest);
            }
            break;
        }
        let previous = load_record(&directory, cursor.generation() - 1)?;
        if cursor.payload.previous.as_ref() != Some(&previous.digest)
            || previous.payload.request != current.payload.request
            || previous.operation_id() != id
        {
            return Err(Error::InvalidDigest);
        }
        cursor = previous;
    }
    for entry in fs::read_dir(directory.join("intents")).map_err(io)? {
        let entry = entry.map_err(io)?;
        reject_symlinks(&entry.path())?;
        let generation = entry
            .file_name()
            .to_str()
            .and_then(|n| n.strip_suffix(".json"))
            .and_then(|n| n.parse::<u64>().ok())
            .ok_or(Error::RecoveryRequired)?;
        if generation == 0 || (!allow_unactivated && generation > current.generation()) {
            return Err(Error::RecoveryRequired);
        }
    }
    Ok(current)
}
fn ticket(root: &SemanticRoot, current: &CreationSnapshot) -> Result<CreationTicket, Error> {
    let initial = load_record(&directory(root, current.operation_id())?, 1)?;
    Ok(CreationTicket {
        repository: root.repository.clone(),
        id: current.payload.id.clone(),
        request: current.payload.request.clone(),
        reservation: initial.digest,
    })
}
impl DurableTransactionStore {
    pub fn reserve_issue_creation(
        root: &SemanticRoot,
        admission: RepositoryAdmission,
        native: NativeIdentity,
        request_bytes: &[u8],
    ) -> Result<CreationReservation, Error> {
        validate_admission(root, &admission)?;
        let request = Request {
            repository: root.repository.clone(),
            native,
            content: codec::decode(request_bytes).map_err(encoding)?,
        };
        let id = OperationId(hash(
            "semantic-operation-v1",
            &(&request.repository, &request.native),
        )?);
        let directory = directory(root, &id)?;
        let parent = directory.parent().ok_or(Error::UnsafePath)?;
        create_directories(parent, &root.common)?;
        let _parent = acquire(parent, true)?;
        let bytes = codec::bytes(&request).map_err(encoding)?;
        if directory.exists() {
            let _lock = acquire(&directory, true)?;
            let current = read_current(root, &id)?;
            if read_blob(&directory, &current.payload.request)? != bytes {
                return Err(Error::ConflictingReplay);
            }
            return Ok(if current.payload.completed.is_some() {
                CreationReservation::AlreadyCompleted(current)
            } else {
                CreationReservation::AlreadyPending(ticket(root, &current)?)
            });
        }
        fs::create_dir(&directory).map_err(io)?;
        sync_chain(&directory, &root.common)?;
        let _lock = acquire(&directory, true)?;
        let inputs = EvidenceInputVersion {
            revision: 1,
            digest: hash(
                "semantic-input-v1",
                &(&request, &admission.authority, &admission.head),
            )?,
        };
        let reference = persist_blob(
            &directory,
            &root.common,
            &bytes,
            "semantic-request-v1",
            EvidenceKind::Request,
            admission.head.clone(),
            inputs,
        )?;
        let next = record(Payload {
            schema: "csdlc.v3.repository_creation.v1".into(),
            repository: root.repository.clone(),
            id,
            generation: 1,
            previous: None,
            request: reference,
            native: request.native,
            authority: admission.authority,
            head: admission.head,
            observed: None,
            completed: None,
            recovery_adoption: None,
        })?;
        activate(root, &directory, &next)?;
        Ok(CreationReservation::Reserved(ticket(root, &next)?))
    }
    pub fn observe_issue_creation(
        root: &SemanticRoot,
        id: &OperationId,
    ) -> Result<Option<CreationSnapshot>, Error> {
        let directory = directory(root, id)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        read_current(root, id).map(Some)
    }
    pub fn attach_issue_creation(
        root: &SemanticRoot,
        supplied: CreationTicket,
        outcome: VerifiedCreationOutcome,
        admission: RepositoryAdmission,
    ) -> Result<CreationAttachment, Error> {
        validate_admission(root, &admission)?;
        let directory = directory(root, &supplied.id)?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(root, &supplied.id)?;
        attach_locked(
            root, &directory, current, supplied, outcome, admission, false,
        )
    }
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "reserved for the explicit native creation recovery route"
        )
    )]
    pub(crate) fn inspect_issue_creation_by_native(
        root: &SemanticRoot,
        native: &NativeIdentity,
    ) -> Result<Option<CreationInspection>, Error> {
        let id = OperationId(hash("semantic-operation-v1", &(&root.repository, native))?);
        let directory = directory(root, &id)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        let current = read_current(root, &id)?;
        let request: Request =
            codec::decode(&read_blob(&directory, &current.payload.request)?).map_err(encoding)?;
        let outcome = current
            .payload
            .completed
            .as_ref()
            .or(current.payload.observed.as_ref());
        Ok(Some(CreationInspection {
            native: request.native,
            request: codec::bytes(&request.content).map_err(encoding)?,
            ticket: ticket(root, &current)?,
            evidence: outcome
                .map(|o| read_blob(&directory, &o.evidence))
                .transpose()?,
        }))
    }
    pub fn describe_issue_creation_recovery(
        root: &SemanticRoot,
        id: &OperationId,
    ) -> Result<Option<CreationRecoveryPreview>, Error> {
        let directory = directory(root, id)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        let current = read_current(root, id)?;
        Ok(if current.payload.completed.is_none() {
            Some(CreationRecoveryPreview {
                repository: root.repository.clone(),
                action: "reconcile_retained_issue_creation".into(),
                id: id.clone(),
                generation: current.generation(),
                digest: current.digest,
            })
        } else {
            None
        })
    }
    pub fn execute_issue_creation_recovery(
        root: &SemanticRoot,
        preview: CreationRecoveryPreview,
        outcome: VerifiedCreationOutcome,
        admission: RepositoryAdmission,
        approval: Option<VerifiedCreationRecovery>,
    ) -> Result<CreationAttachment, Error> {
        validate_admission(root, &admission)?;
        let directory = directory(root, &preview.id)?;
        let _lock = acquire(&directory, true)?;
        let current = read_current(root, &preview.id)?;
        let retained = load_record(&directory, preview.generation)?;
        if retained.digest != preview.digest || retained.operation_id() != &preview.id {
            return Err(Error::ConflictingReplay);
        }
        if current.payload.completed.is_none() && current.digest != preview.digest {
            return Err(Error::StaleVersion);
        }
        let adopt = match approval {
            Some(witness) if witness.preview == preview => true,
            Some(_) => return Err(Error::StaleVersion),
            None => false,
        };
        let ticket = ticket(root, &current)?;
        attach_locked(root, &directory, current, ticket, outcome, admission, adopt)
    }
}
fn attach_locked(
    root: &SemanticRoot,
    directory: &Path,
    current: CreationSnapshot,
    supplied: CreationTicket,
    outcome: VerifiedCreationOutcome,
    admission: RepositoryAdmission,
    adopt: bool,
) -> Result<CreationAttachment, Error> {
    if ticket(root, &current)? != supplied
        || outcome.native != current.payload.native
        || outcome
            .issue
            .as_ref()
            .is_some_and(|key| key.repository() != root.repository)
    {
        return Err(Error::ConflictingReplay);
    }
    if let Some(done) = &current.payload.completed {
        if done.kind != outcome.kind
            || done.truth != outcome.truth
            || done.issue != outcome.issue
            || read_blob(directory, &done.evidence)? != outcome.evidence
        {
            return Err(Error::ConflictingReplay);
        }
        return Ok(CreationAttachment::AlreadyCompleted(current));
    }
    let evidence = persist_blob(
        directory,
        &root.common,
        &outcome.evidence,
        "semantic-evidence-v1",
        EvidenceKind::GithubReadback,
        current.payload.head.clone(),
        current.payload.request.inputs.clone(),
    )?;
    let retained = CreationOutcome {
        kind: outcome.kind,
        truth: outcome.truth,
        issue: outcome.issue,
        evidence,
    };
    let unresolved = (!adopt
        && (admission.authority != current.payload.authority
            || admission.head != current.payload.head))
        || retained.kind == OutcomeKind::Unresolved
        || retained.truth == EffectTruth::Unknown;
    if unresolved && current.payload.observed.as_ref() == Some(&retained) {
        return Ok(CreationAttachment::RecoveryRequired(current));
    }
    let mut payload = current.payload.clone();
    payload.generation = payload
        .generation
        .checked_add(1)
        .ok_or(Error::ExhaustedVersion)?;
    payload.previous = Some(current.digest.clone());
    if adopt {
        payload.recovery_adoption = Some((admission.authority, admission.head, current.digest));
    }
    if unresolved {
        payload.observed = Some(retained);
    } else {
        payload.completed = Some(retained);
    }
    let next = record(payload)?;
    activate(root, directory, &next)?;
    Ok(if unresolved {
        CreationAttachment::RecoveryRequired(next)
    } else {
        CreationAttachment::Completed(next)
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreationJournalPreview {
    repository: String,
    action: String,
    digest: Digest,
    id: OperationId,
    before: Option<Digest>,
    generation: u64,
    target: Digest,
    next_present: bool,
}
impl CreationJournalPreview {
    pub fn repository(&self) -> &str {
        &self.repository
    }
    pub fn operation_id(&self) -> &OperationId {
        &self.id
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn target(&self) -> &Digest {
        &self.target
    }
    pub fn before(&self) -> Option<&Digest> {
        self.before.as_ref()
    }
    pub fn action(&self) -> &str {
        &self.action
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}
#[derive(Debug, Clone)]
pub struct CreationJournalApproval {
    preview: CreationJournalPreview,
}
impl CreationJournalApproval {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "reserved for the explicit native creation journal recovery route"
        )
    )]
    pub(crate) fn from_native_owner(preview: &CreationJournalPreview) -> Self {
        Self {
            preview: preview.clone(),
        }
    }
}
fn creation_barrier(
    directory: &Path,
    root: &SemanticRoot,
    target: &CreationSnapshot,
    pointer: &str,
) -> Result<(), Error> {
    validate_objects(root, directory, target, target.operation_id())?;
    let mut paths = vec![directory
        .join("objects")
        .join(&target.payload.request.object)];
    for outcome in target
        .payload
        .observed
        .iter()
        .chain(target.payload.completed.iter())
    {
        paths.push(directory.join("objects").join(&outcome.evidence.object));
    }
    paths.push(
        directory
            .join("commits")
            .join(format!("{}.json", target.generation())),
    );
    paths.push(
        directory
            .join("intents")
            .join(format!("{}.json", target.generation())),
    );
    paths.push(directory.join(pointer));
    rebarrier(paths, &root.common)
}
fn describe_creation_journal(
    root: &SemanticRoot,
    id: &OperationId,
) -> Result<Option<CreationJournalPreview>, Error> {
    let directory = directory(root, id)?;
    let previous = if directory.join("current.json").try_exists().map_err(io)? {
        Some(read_current_with_pending(root, id, true)?)
    } else {
        None
    };
    let generation = previous.as_ref().map_or(Ok(1), |s| {
        s.generation().checked_add(1).ok_or(Error::ExhaustedVersion)
    })?;
    let path = directory.join("intents").join(format!("{generation}.json"));
    reject_symlinks(&path)?;
    if !path.try_exists().map_err(io)? {
        return if previous.is_none() || directory.join("current.next").try_exists().map_err(io)? {
            Err(Error::RecoveryRequired)
        } else {
            Ok(None)
        };
    }
    let pointer: (u64, Digest) = codec::decode(&fs::read(path).map_err(io)?).map_err(encoding)?;
    let target = load_record(&directory, generation)?;
    if pointer != (generation, target.digest.clone())
        || target.operation_id() != id
        || target.payload.repository != root.repository
        || target.payload.previous.as_ref() != previous.as_ref().map(|s| &s.digest)
    {
        return Err(Error::InvalidDigest);
    }
    validate_objects(root, &directory, &target, id)?;
    if previous
        .as_ref()
        .is_some_and(|p| p.payload.request != target.payload.request)
    {
        return Err(Error::EvidenceMismatch);
    }
    for entry in fs::read_dir(directory.join("intents")).map_err(io)? {
        let entry = entry.map_err(io)?;
        reject_symlinks(&entry.path())?;
        let n = entry
            .file_name()
            .to_str()
            .and_then(|n| n.strip_suffix(".json"))
            .and_then(|n| n.parse::<u64>().ok())
            .ok_or(Error::RecoveryRequired)?;
        if n == 0 || n > generation {
            return Err(Error::RecoveryRequired);
        }
    }
    let next = directory.join("current.next");
    reject_symlinks(&next)?;
    let next_present = next.try_exists().map_err(io)?;
    if next_present {
        let retained: (u64, Digest) =
            codec::decode(&fs::read(next).map_err(io)?).map_err(encoding)?;
        if retained != pointer {
            return Err(Error::InvalidDigest);
        }
    }
    let digest = hash(
        "semantic-recovery-v1",
        &(
            &root.repository,
            id,
            previous.as_ref().map(|s| &s.digest),
            generation,
            &target.digest,
            next_present,
        ),
    )?;
    Ok(Some(CreationJournalPreview {
        repository: root.repository.clone(),
        action: "activate_retained_creation_commit".into(),
        digest,
        id: id.clone(),
        before: previous.map(|s| s.digest),
        generation,
        target: target.digest,
        next_present,
    }))
}
impl DurableTransactionStore {
    /// Derive the retained repository operation identity even before current exists.
    /// This is read-only and grants no reservation or recovery authority.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "reserved for exact native creation recovery lookup"
        )
    )]
    pub(crate) fn issue_creation_id_from_native(
        root: &SemanticRoot,
        native: &NativeIdentity,
    ) -> Result<OperationId, Error> {
        Ok(OperationId(hash(
            "semantic-operation-v1",
            &(&root.repository, native),
        )?))
    }
    pub fn describe_creation_journal_recovery(
        root: &SemanticRoot,
        id: &OperationId,
    ) -> Result<Option<CreationJournalPreview>, Error> {
        let directory = directory(root, id)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        describe_creation_journal(root, id)
    }
    pub fn execute_creation_journal_recovery(
        root: &SemanticRoot,
        preview: CreationJournalPreview,
        approval: CreationJournalApproval,
    ) -> Result<CreationSnapshot, Error> {
        if approval.preview != preview {
            return Err(Error::StaleVersion);
        }
        let directory = directory(root, &preview.id)?;
        let _lock = acquire(&directory, true)?;
        if let Ok(current) = read_current(root, &preview.id) {
            if current.digest == preview.target {
                creation_barrier(&directory, root, &current, "current.json")?;
                return Ok(current);
            }
        }
        if describe_creation_journal(root, &preview.id)?.as_ref() != Some(&preview) {
            return Err(Error::StaleVersion);
        }
        let next = directory.join("current.next");
        if !preview.next_present {
            create_only(
                &next,
                &codec::bytes(&(preview.generation, &preview.target)).map_err(encoding)?,
                &root.common,
            )?;
        }
        let target = load_record(&directory, preview.generation)?;
        creation_barrier(&directory, root, &target, "current.next")?;
        fs::rename(next, directory.join("current.json")).map_err(io)?;
        sync_chain(&directory, &root.common)?;
        read_current(root, &preview.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn admission(f: &super::super::tests::Fixture) -> RepositoryAdmission {
        RepositoryAdmission::from_native_owner(
            "example/repo".into(),
            Digest::authority(b"authority"),
            f.root.common.clone(),
            "a".repeat(40),
        )
        .unwrap()
    }
    #[test]
    fn repository_creation_replay_never_creates_issue_zero_or_prepares_cards() {
        let f = super::super::tests::Fixture::new();
        let native = NativeIdentity::new("github".into(), "create-one".into()).unwrap();
        let CreationReservation::Reserved(ticket) =
            DurableTransactionStore::reserve_issue_creation(
                &f.root,
                admission(&f),
                native.clone(),
                br#"{"title":"create"}"#,
            )
            .unwrap()
        else {
            panic!()
        };
        assert!(matches!(
            DurableTransactionStore::reserve_issue_creation(
                &f.root,
                admission(&f),
                native.clone(),
                br#"{"title":"create"}"#
            )
            .unwrap(),
            CreationReservation::AlreadyPending(_)
        ));
        assert_eq!(
            DurableTransactionStore::reserve_issue_creation(
                &f.root,
                admission(&f),
                native.clone(),
                br#"{"title":"different"}"#
            ),
            Err(Error::ConflictingReplay)
        );
        let key = IssueKey::new("example/repo", 999).unwrap();
        let outcome = VerifiedCreationOutcome::from_native_owner(
            OutcomeKind::Success,
            EffectTruth::Performed,
            Some(key.clone()),
            b"authenticated creation receipt".to_vec(),
            native.clone(),
        )
        .unwrap();
        let CreationAttachment::Completed(done) = DurableTransactionStore::attach_issue_creation(
            &f.root,
            ticket.clone(),
            outcome.clone(),
            admission(&f),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(done.created_issue(), Some(&key));
        assert!(matches!(
            DurableTransactionStore::attach_issue_creation(&f.root, ticket, outcome, admission(&f))
                .unwrap(),
            CreationAttachment::AlreadyCompleted(_)
        ));
        assert_eq!(
            DurableTransactionStore::observe_issue(&f.root, &key).unwrap(),
            Observation::Absent
        );
        assert!(!f.root.common.join("csdlc-v3/semantic/issues/0").exists());
    }
    #[test]
    fn lost_creation_ticket_and_changed_authority_have_explicit_recovery() {
        let f = super::super::tests::Fixture::new();
        let native = NativeIdentity::new("github".into(), "restart-create".into()).unwrap();
        DurableTransactionStore::reserve_issue_creation(
            &f.root,
            admission(&f),
            native.clone(),
            br#"{"title":"retained"}"#,
        )
        .unwrap();
        // A restarted native owner has its retained native identity, not a serialized ticket.
        let inspection =
            DurableTransactionStore::inspect_issue_creation_by_native(&f.root, &native)
                .unwrap()
                .unwrap();
        assert_eq!(inspection.request_bytes(), br#"{"title":"retained"}"#);
        assert_eq!(inspection.native_identity(), &native);
        let outcome = VerifiedCreationOutcome::from_native_owner(
            OutcomeKind::Success,
            EffectTruth::NotPerformed,
            Some(IssueKey::new("example/repo", 999).unwrap()),
            b"authenticated existing created issue".to_vec(),
            native,
        )
        .unwrap();
        let changed = RepositoryAdmission::from_native_owner(
            "example/repo".into(),
            Digest::authority(b"new authority"),
            f.root.common.clone(),
            "b".repeat(40),
        )
        .unwrap();
        assert!(matches!(
            DurableTransactionStore::attach_issue_creation(
                &f.root,
                inspection.ticket().clone(),
                outcome.clone(),
                changed.clone()
            )
            .unwrap(),
            CreationAttachment::RecoveryRequired(_)
        ));
        let preview = DurableTransactionStore::describe_issue_creation_recovery(
            &f.root,
            inspection.ticket().id(),
        )
        .unwrap()
        .unwrap();
        assert_eq!(preview.repository(), "example/repo");
        assert_eq!(preview.operation_id(), inspection.ticket().id());
        assert!(preview.generation() > 0);
        assert_eq!(preview.action(), "reconcile_retained_issue_creation");
        let serialized = serde_json::to_value(&preview).unwrap();
        assert_eq!(
            serialized["digest"],
            serde_json::to_value(preview.digest()).unwrap()
        );
        assert_eq!(serialized["repository"], preview.repository());
        let approval = VerifiedCreationRecovery::adopt_after_native_reconciliation(&preview);
        let result = DurableTransactionStore::execute_issue_creation_recovery(
            &f.root,
            preview.clone(),
            outcome.clone(),
            changed.clone(),
            Some(approval),
        )
        .unwrap();
        let CreationAttachment::Completed(done) = result else {
            panic!()
        };
        assert_eq!(done.created_issue().unwrap().issue(), 999);
        assert_eq!(done.completed_truth(), Some(EffectTruth::NotPerformed));
        assert!(matches!(
            DurableTransactionStore::execute_issue_creation_recovery(
                &f.root, preview, outcome, changed, None
            )
            .unwrap(),
            CreationAttachment::AlreadyCompleted(_)
        ));
        assert_eq!(
            DurableTransactionStore::observe_issue_creation(&f.root, inspection.ticket().id())
                .unwrap()
                .unwrap()
                .generation(),
            done.generation()
        );
    }

    #[test]
    fn creation_retained_commit_pointer_recovery_is_exact() {
        let f = super::super::tests::Fixture::new();
        let native = NativeIdentity::new("github".into(), "journal-create".into()).unwrap();
        let CreationReservation::Reserved(ticket) =
            DurableTransactionStore::reserve_issue_creation(&f.root, admission(&f), native, b"{}")
                .unwrap()
        else {
            panic!()
        };
        let directory = directory(&f.root, ticket.id()).unwrap();
        let pointer = fs::read(directory.join("current.json")).unwrap();
        fs::rename(
            directory.join("current.json"),
            directory.join("current.next"),
        )
        .unwrap();
        assert_eq!(fs::read(directory.join("current.next")).unwrap(), pointer);
        let restarted_id = DurableTransactionStore::issue_creation_id_from_native(
            &f.root,
            &NativeIdentity::new("github".into(), "journal-create".into()).unwrap(),
        )
        .unwrap();
        assert_eq!(&restarted_id, ticket.id());
        let preview =
            DurableTransactionStore::describe_creation_journal_recovery(&f.root, &restarted_id)
                .unwrap()
                .unwrap();
        assert_eq!(preview.repository(), "example/repo");
        assert_eq!(preview.operation_id(), ticket.id());
        assert_eq!(preview.generation(), 1);
        assert!(preview.before().is_none());
        assert_eq!(preview.action(), "activate_retained_creation_commit");
        let serialized = serde_json::to_value(&preview).unwrap();
        assert_eq!(
            serialized["digest"],
            serde_json::to_value(preview.digest()).unwrap()
        );
        assert_eq!(
            serialized["target"],
            serde_json::to_value(preview.target()).unwrap()
        );
        let approval = CreationJournalApproval::from_native_owner(&preview);
        FAIL_REBARRIER.with(|flag| flag.set(true));
        assert!(matches!(
            DurableTransactionStore::execute_creation_journal_recovery(
                &f.root,
                preview.clone(),
                approval.clone()
            ),
            Err(Error::Io(_))
        ));
        assert!(!directory.join("current.json").exists());
        assert_eq!(fs::read(directory.join("current.next")).unwrap(), pointer);
        let recovered = DurableTransactionStore::execute_creation_journal_recovery(
            &f.root,
            preview.clone(),
            approval.clone(),
        )
        .unwrap();
        assert_eq!(recovered.generation(), 1);
        FAIL_REBARRIER.with(|flag| flag.set(true));
        assert!(matches!(
            DurableTransactionStore::execute_creation_journal_recovery(
                &f.root,
                preview.clone(),
                approval.clone()
            ),
            Err(Error::Io(_))
        ));
        assert_eq!(
            DurableTransactionStore::execute_creation_journal_recovery(&f.root, preview, approval)
                .unwrap(),
            recovered
        );
    }

    #[test]
    fn unresolved_creation_retains_identity_and_evidence_for_exact_readback() {
        let f = super::super::tests::Fixture::new();
        let native = NativeIdentity::new("github".into(), "uncertain-create".into()).unwrap();
        let CreationReservation::Reserved(ticket) =
            DurableTransactionStore::reserve_issue_creation(
                &f.root,
                admission(&f),
                native.clone(),
                b"{}",
            )
            .unwrap()
        else {
            panic!()
        };
        let unknown = VerifiedCreationOutcome::from_native_owner(
            OutcomeKind::Unresolved,
            EffectTruth::Unknown,
            None,
            b"connection lost".to_vec(),
            native.clone(),
        )
        .unwrap();
        assert!(matches!(
            DurableTransactionStore::attach_issue_creation(
                &f.root,
                ticket.clone(),
                unknown,
                admission(&f)
            )
            .unwrap(),
            CreationAttachment::RecoveryRequired(_)
        ));
        let observed = DurableTransactionStore::observe_issue_creation(&f.root, ticket.id())
            .unwrap()
            .unwrap();
        assert_eq!(observed.observed_truth(), Some(EffectTruth::Unknown));
        assert!(observed.created_issue().is_none());
        let known = VerifiedCreationOutcome::from_native_owner(
            OutcomeKind::Success,
            EffectTruth::Performed,
            Some(IssueKey::new("example/repo", 999).unwrap()),
            b"readback existing issue".to_vec(),
            native,
        )
        .unwrap();
        assert!(matches!(
            DurableTransactionStore::attach_issue_creation(&f.root, ticket, known, admission(&f))
                .unwrap(),
            CreationAttachment::Completed(_)
        ));
    }
}
