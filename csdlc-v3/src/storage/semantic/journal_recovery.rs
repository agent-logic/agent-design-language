//! Explicit completion of a fully retained immutable commit after pointer failure.
//! Incomplete/torn immutable objects are refused; this path does not delete or
//! fabricate evidence, rewrite a commit, or rerun an external operation.
use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JournalRecoveryPreview {
    action: String,
    key: IssueKey,
    before: Option<SemanticVersion>,
    target: SemanticVersion,
    audit: Digest,
    next_present: bool,
    digest: Digest,
}
impl JournalRecoveryPreview {
    pub fn key(&self) -> &IssueKey {
        &self.key
    }
    pub fn before(&self) -> Option<&SemanticVersion> {
        self.before.as_ref()
    }
    pub fn target(&self) -> &SemanticVersion {
        &self.target
    }
    pub fn action(&self) -> &str {
        &self.action
    }
    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}
#[derive(Debug, Clone)]
pub struct JournalRecoveryApproval {
    digest: Digest,
}
impl JournalRecoveryApproval {
    pub(crate) fn from_native_owner(preview: &JournalRecoveryPreview) -> Self {
        Self {
            digest: preview.digest.clone(),
        }
    }
}
fn describe(directory: &Path, key: &IssueKey) -> Result<Option<JournalRecoveryPreview>, Error> {
    let prior = if directory.join("current.json").try_exists().map_err(io)? {
        Some(read_current_with_pending(directory, key, true)?)
    } else {
        None
    };
    let generation = prior.as_ref().map_or(Ok(1), |s| {
        s.version()
            .generation
            .checked_add(1)
            .ok_or(Error::ExhaustedVersion)
    })?;
    let intent = directory.join("intents").join(format!("{generation}.json"));
    reject_symlinks(&intent)?;
    if !intent.try_exists().map_err(io)? {
        return if directory.join("current.next").try_exists().map_err(io)? || prior.is_none() {
            Err(Error::RecoveryRequired)
        } else {
            Ok(None)
        };
    }
    let pointer: Pointer = codec::decode(&fs::read(&intent).map_err(io)?).map_err(encoding)?;
    if pointer.generation != generation {
        return Err(Error::InvalidDigest);
    }
    let target = SemanticVersion {
        generation,
        digest: pointer.digest,
    };
    let snapshot = load_commit(directory, &target)?;
    if snapshot.key() != key
        || snapshot.audit.before.as_ref() != prior.as_ref().map(Snapshot::version)
        || snapshot.audit.previous.as_ref() != prior.as_ref().map(|s| &s.audit_digest)
        || snapshot.audit_digest != pointer.audit
    {
        return Err(Error::InvalidDigest);
    }
    protocol::validate_objects(directory, &snapshot)?;
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
        let next_pointer: Pointer =
            codec::decode(&fs::read(next).map_err(io)?).map_err(encoding)?;
        if next_pointer.generation != generation
            || next_pointer.digest != target.digest
            || next_pointer.audit != snapshot.audit_digest
        {
            return Err(Error::InvalidDigest);
        }
    }
    let before = prior.as_ref().map(|s| s.version().clone());
    let digest = hash(
        "semantic-recovery-v1",
        &(key, &before, &target, &snapshot.audit_digest, next_present),
    )?;
    Ok(Some(JournalRecoveryPreview {
        action: "activate_retained_issue_commit".into(),
        key: key.clone(),
        before,
        target,
        audit: snapshot.audit_digest,
        next_present,
        digest,
    }))
}
fn barrier(
    directory: &Path,
    root: &SemanticRoot,
    snapshot: &Snapshot,
    pointer: &str,
) -> Result<(), Error> {
    protocol::validate_objects(directory, snapshot)?;
    let mut paths = protocol::retained_object_paths(directory, snapshot);
    paths.push(directory.join("commits").join(name(snapshot.version())));
    paths.push(
        directory
            .join("intents")
            .join(format!("{}.json", snapshot.version().generation)),
    );
    paths.push(directory.join(pointer));
    rebarrier(paths, &root.common)
}
impl DurableTransactionStore {
    pub fn describe_journal_recovery(
        root: &SemanticRoot,
        key: &IssueKey,
    ) -> Result<Option<JournalRecoveryPreview>, Error> {
        let directory = root.directory(key)?;
        if !directory.exists() {
            return Ok(None);
        }
        let _lock = acquire(&directory, false)?;
        describe(&directory, key)
    }
    /// Read the exact retained target for native authority and topology admission.
    pub(crate) fn journal_recovery_target(
        root: &SemanticRoot,
        preview: &JournalRecoveryPreview,
    ) -> Result<Snapshot, Error> {
        let directory = root.directory(&preview.key)?;
        let _lock = acquire(&directory, false)?;
        if describe(&directory, &preview.key)?.as_ref() != Some(preview) {
            return Err(Error::StaleVersion);
        }
        load_commit(&directory, &preview.target)
    }
    pub fn execute_journal_recovery(
        root: &SemanticRoot,
        preview: JournalRecoveryPreview,
        approval: JournalRecoveryApproval,
    ) -> Result<CommitOutcome, Error> {
        if approval.digest != preview.digest {
            return Err(Error::StaleVersion);
        }
        let directory = root.directory(&preview.key)?;
        let _lock = acquire(&directory, true)?;
        if let Ok(current) = read_current(&directory, &preview.key) {
            if current.version() == &preview.target && current.audit_digest == preview.audit {
                barrier(&directory, root, &current, "current.json")?;
                return Ok(CommitOutcome::Unchanged(Box::new(current)));
            }
        }
        if describe(&directory, &preview.key)?.as_ref() != Some(&preview) {
            return Err(Error::StaleVersion);
        }
        let pointer = Pointer {
            generation: preview.target.generation,
            digest: preview.target.digest.clone(),
            audit: preview.audit.clone(),
        };
        let next = directory.join("current.next");
        if !preview.next_present {
            create_only(
                &next,
                &codec::bytes(&pointer).map_err(encoding)?,
                &root.common,
            )?;
        }
        let target = load_commit(&directory, &preview.target)?;
        barrier(&directory, root, &target, "current.next")?;
        fs::rename(next, directory.join("current.json")).map_err(io)?;
        sync_chain(&directory, &root.common)?;
        Ok(CommitOutcome::Committed(Box::new(read_current(
            &directory,
            &preview.key,
        )?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retained_commit_activation_is_exact_observational_and_replayable() {
        let f = super::super::protocol::tests::Fixture::new();
        let directory = f.root.directory(&f.key).unwrap();
        let prior = read_current(&directory, &f.key).unwrap();
        let mut payload = prior.payload.clone();
        payload.generation += 1;
        payload.projection_required = false;
        let staged = make_snapshot(
            payload,
            Some(&prior),
            SemanticCommand::AcknowledgeProjection,
        )
        .unwrap();
        let pointer = Pointer {
            generation: staged.version().generation,
            digest: staged.version().digest.clone(),
            audit: staged.audit_digest.clone(),
        };
        create_only(
            &directory.join("intents/2.json"),
            &codec::bytes(&pointer).unwrap(),
            &f.root.common,
        )
        .unwrap();
        create_only(
            &directory.join("commits").join(name(staged.version())),
            &staged.canonical_bytes().unwrap(),
            &f.root.common,
        )
        .unwrap();
        let preview = DurableTransactionStore::describe_journal_recovery(&f.root, &f.key)
            .unwrap()
            .unwrap();
        assert_eq!(
            fs::read(directory.join("current.json")).unwrap(),
            codec::bytes(&Pointer {
                generation: prior.version().generation,
                digest: prior.version().digest.clone(),
                audit: prior.audit_digest.clone()
            })
            .unwrap()
        );
        assert_eq!(preview.key(), &f.key);
        assert_eq!(preview.before(), Some(prior.version()));
        assert_eq!(preview.target(), staged.version());
        assert_eq!(preview.action(), "activate_retained_issue_commit");
        assert_eq!(
            serde_json::to_value(&preview).unwrap()["digest"],
            serde_json::to_value(preview.digest()).unwrap()
        );
        let approval = JournalRecoveryApproval::from_native_owner(&preview);
        FAIL_REBARRIER.with(|flag| flag.set(true));
        assert!(matches!(
            DurableTransactionStore::execute_journal_recovery(
                &f.root,
                preview.clone(),
                approval.clone()
            ),
            Err(Error::Io(_))
        ));
        let retained: Pointer =
            codec::decode(&fs::read(directory.join("current.json")).unwrap()).unwrap();
        assert_eq!(retained.generation, prior.version().generation);
        // The failed barrier retained current.next; obtain its exact updated preview.
        let preview = DurableTransactionStore::describe_journal_recovery(&f.root, &f.key)
            .unwrap()
            .unwrap();
        assert_eq!(preview.key(), &f.key);
        assert_eq!(preview.before(), Some(prior.version()));
        assert_eq!(preview.target(), staged.version());
        assert_eq!(preview.action(), "activate_retained_issue_commit");
        assert_eq!(
            serde_json::to_value(&preview).unwrap()["digest"],
            serde_json::to_value(preview.digest()).unwrap()
        );
        let approval = JournalRecoveryApproval::from_native_owner(&preview);
        assert!(matches!(
            DurableTransactionStore::execute_journal_recovery(
                &f.root,
                preview.clone(),
                approval.clone()
            )
            .unwrap(),
            CommitOutcome::Committed(_)
        ));
        FAIL_REBARRIER.with(|flag| flag.set(true));
        assert!(matches!(
            DurableTransactionStore::execute_journal_recovery(
                &f.root,
                preview.clone(),
                approval.clone()
            ),
            Err(Error::Io(_))
        ));
        assert!(matches!(
            DurableTransactionStore::execute_journal_recovery(&f.root, preview, approval).unwrap(),
            CommitOutcome::Unchanged(_)
        ));
        assert_eq!(
            read_current(&directory, &f.key).unwrap().version(),
            staged.version()
        );
    }
}
