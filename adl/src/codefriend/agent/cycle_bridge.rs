//! Cycle continuation imports exact gateway producer bytes into a distinct owner.
use super::*;
use crate::codefriend::cycle_bridge::{Capsule, ImportBinding};

impl RunReport {
    /// The original report remains unchanged; nested gateway run identity is retained.
    pub(crate) fn completed_review(
        &self,
    ) -> Result<&super::super::review::runner::FourPerspectiveReviewRun> {
        ensure!(self.status == "complete", "agent_cycle_review_unavailable");
        match (&self.result, &self.cycle_result) {
            (Some(run), None) => {
                run.successful_execution()?;
                Ok(run)
            }
            (None, Some(cycle)) => crate::codefriend::cycle_bridge::review(cycle),
            _ => anyhow::bail!("agent_cycle_review_unavailable"),
        }
    }
    pub(crate) fn review_store(&self, root: &Path) -> PathBuf {
        if self.cycle_result.is_some() {
            root.join("imported-cycle/evidence")
        } else {
            root.join("evidence")
        }
    }
    pub(crate) fn review_root(&self, root: &Path) -> PathBuf {
        if self.cycle_result.is_some() {
            root.join("imported-cycle/review")
        } else {
            root.join("work/review")
        }
    }
}

impl Transport {
    /// Each continuation observes the live gateway again. Offline capsule bytes
    /// never substitute for remote revocation/retention checks.
    pub(super) fn ensure_cycle_owner(
        &self,
        journal: &Journal,
        consent_path: &Path,
        pairing: &Pairing,
        command: &Command,
        report: &RunReport,
        root: &Path,
    ) -> Result<()> {
        let Some(cycle) = &report.cycle_result else {
            return Ok(());
        };
        let imported = root.join("imported-cycle");
        let result = (|| -> Result<()> {
            report.validate((self.clock)())?;
            report.completed_review()?;
            command.validate(
                pairing,
                &read_consent(consent_path, (self.clock)())?,
                (self.clock)(),
            )?;
            ensure!(
                command.cycle.as_ref() == Some(&cycle.plan),
                "agent_cycle_plan_changed"
            );
            let original: super::super::evidence::Admission =
                publication::read(&root.join("admission.json"), MAX_RESPONSE as usize)?;
            original.validate()?;
            let clock = self.clock.clone();
            let local =
                super::super::evidence::store::Store::open(&root.join("evidence"), move || {
                    clock()
                })?;
            ensure!(
                local.get(&original.packet.packet_id)? == original
                    && original.packet == cycle.admission.packet
                    && report.expires_at <= original.expires_at,
                "agent_cycle_local_admission_changed"
            );
            drop(local);
            let path = format!("/v1/operations/{}/review-evidence", cycle.run_id);
            let capsule: Capsule =
                self.request(Method::GET, &path, Some(&pairing.model_token), None)?;
            capsule.validate(cycle, (self.clock)())?;
            ensure!(
                report.expires_at <= capsule.expires_at,
                "agent_cycle_import_expiry"
            );
            // This binding is an import receipt, not a fabricated producer receipt.
            let binding = ImportBinding {
                schema: "codefriend.imported_cycle_review.v1".into(),
                website_run_id: command.run_id.clone(),
                agent_id: pairing.agent_id.clone(),
                consent_digest: command.consent_digest.clone(),
                local_admission: original.clone(),
                capsule,
            };
            let marker = imported.join("import.json");
            if marker.exists() {
                let prior: ImportBinding = publication::read(&marker, MAX_RESPONSE as usize)?;
                ensure!(
                    hash(&prior)? == hash(&binding)?,
                    "agent_cycle_import_changed"
                );
            } else {
                // An interrupted import has no authority and may be reconstructed
                // solely from the same authenticated GET; no provider dispatch.
                if imported.exists() {
                    ensure!(
                        !fs::symlink_metadata(&imported)?.file_type().is_symlink(),
                        "agent_cycle_import_symlink"
                    );
                    fs::remove_dir_all(&imported)?;
                }
                publication::private_dirs(&imported)?;
                let clock = self.clock.clone();
                let store = super::super::evidence::store::Store::open(
                    &imported.join("evidence"),
                    move || clock(),
                )?;
                store.import_cycle_admission(&cycle.admission)?;
                drop(store);
                for (path, bytes) in &binding.capsule.files {
                    let target = imported.join("review").join(path);
                    publication::private_dirs(target.parent().unwrap())?;
                    let mut file = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .mode(0o600)
                        .open(target)?;
                    use std::io::Write;
                    file.write_all(bytes.as_bytes())?;
                    file.sync_all()?;
                }
                save_private(&marker, &binding)?;
            }
            for (path, bytes) in &binding.capsule.files {
                let target = imported.join("review").join(path);
                crate::codefriend::publication::manifest::reject_symlink_components(&target)?;
                ensure!(
                    !fs::symlink_metadata(&target)?.file_type().is_symlink(),
                    "agent_cycle_import_symlink"
                );
                ensure!(
                    fs::metadata(&target)?.len() == bytes.len() as u64
                        && fs::read(&target)? == bytes.as_bytes(),
                    "agent_cycle_import_bytes_changed"
                );
            }
            super::super::evidence::store::Store::verify_cycle_import(
                &imported.join("evidence"),
                &cycle.admission,
                (self.clock)(),
            )?;
            // Writes/reads/fsync can outlive any owner. Observe remote authority
            // again, then recheck every original local owner before returning.
            let final_capsule: Capsule =
                self.request(Method::GET, &path, Some(&pairing.model_token), None)?;
            final_capsule.validate(cycle, (self.clock)())?;
            ensure!(
                final_capsule == binding.capsule,
                "agent_cycle_import_changed"
            );
            ensure!(!self.control(pairing, command)?, "agent_cancelled");
            let now = (self.clock)();
            let current = journal.pairing(now)?;
            ensure!(
                hash(&current)? == hash(pairing)?,
                "agent_cycle_pairing_changed"
            );
            command.validate(&current, &read_consent(consent_path, now)?, now)?;
            let after_command: Command = publication::read(&root.join("command.json"), 8192)?;
            let after_report: RunReport =
                publication::read(&root.join("report.json"), MAX_RESPONSE as usize)?;
            ensure!(
                hash(&after_command)? == hash(command)? && hash(&after_report)? == hash(report)?,
                "agent_cycle_owner_changed"
            );
            after_report.validate(now)?;
            let clock = self.clock.clone();
            let local =
                super::super::evidence::store::Store::open(&root.join("evidence"), move || {
                    clock()
                })?;
            ensure!(
                local.get(&original.packet.packet_id)? == original,
                "agent_cycle_local_admission_changed"
            );
            Ok(())
        })();
        if result.is_err() && imported.exists() {
            // Imported source bytes are owned by this exact run. Failure never
            // modifies original report/receipt/admission or retries a provider.
            if fs::symlink_metadata(&imported)?.file_type().is_symlink() {
                anyhow::bail!("agent_cycle_import_symlink");
            }
            fs::remove_dir_all(&imported)?;
        }
        result
    }
}
