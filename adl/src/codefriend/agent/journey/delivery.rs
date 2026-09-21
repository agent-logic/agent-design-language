//! Bounded delivery state; effect tombstones survive payload expiration.
use super::*;
use std::os::unix::fs::DirBuilderExt;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reservation {
    job_digest: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Poll {
    job: Option<Job>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Ack {
    binding: Binding,
    agent_candidate_revision: String,
    checkpoint_sequence: usize,
    digest: String,
    superseded: bool,
}

fn directory(path: &Path) -> Result<()> {
    crate::codefriend::publication::manifest::reject_symlink_components(path)?;
    if !path.exists() {
        fs::DirBuilder::new().mode(0o700).create(path)?;
    }
    let metadata = fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_dir() && metadata.permissions().mode() & 0o077 == 0,
        "agent_journey_directory_permissions"
    );
    Ok(())
}

fn reserve(path: &Path, job: &Job) -> Result<bool> {
    let digest = hash(job)?;
    if path.exists() {
        let saved: Reservation = publication::read(path, 1024)?;
        ensure!(saved.job_digest == digest, "agent_journey_job_changed");
        return Ok(false);
    }
    save_private(path, &Reservation { job_digest: digest })?;
    Ok(true)
}

fn observation_slot(request: &Request) -> Option<&'static str> {
    match request {
        Request::Status => Some("status"),
        Request::Graph => Some("graph"),
        Request::Artifact { artifact } => Some(artifact.key()),
        _ => None,
    }
}

fn check_result(result: &StageResult, job: &Job) -> Result<()> {
    ensure!(
        result.schema
            == if job.binding.schema == JOB_SCHEMA_V2 {
                RESULT_SCHEMA_V2
            } else {
                RESULT_SCHEMA
            }
            && result.binding == job.binding
            && result.agent_candidate_revision == job.permitted_agent_candidate,
        "agent_journey_cached_identity_changed"
    );
    let mut unsigned = result.clone();
    unsigned.digest.clear();
    ensure!(
        hash(&unsigned)? == result.digest,
        "agent_journey_cached_digest_changed"
    );
    ensure!(
        serde_json::to_vec(result)?.len() as u64 <= MAX_RESPONSE,
        "agent_journey_result_limit"
    );
    Ok(())
}

fn check_previous(previous: &StageResult, current: &StageResult) -> Result<()> {
    ensure!(
        current.checkpoint_sequence >= previous.checkpoint_sequence,
        "agent_journey_checkpoint_regressed"
    );
    if current.checkpoint_sequence == previous.checkpoint_sequence {
        ensure!(
            current.digest == previous.digest,
            "agent_journey_checkpoint_changed"
        );
    }
    for (name, stage) in &previous.manifest.stages {
        if stage.status != native::StageStatus::Pending {
            let next = current
                .manifest
                .stages
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("agent_journey_stage_missing"))?;
            ensure!(
                hash(stage)? == hash(next)?,
                "agent_journey_committed_stage_changed"
            );
        }
    }
    Ok(())
}

fn replace_private(path: &Path, value: &impl Serialize) -> Result<()> {
    let temporary = path.with_extension("pending");
    // A prior uncommitted replacement cannot supersede the durable old value.
    if temporary.exists() {
        let metadata = fs::symlink_metadata(&temporary)?;
        ensure!(metadata.is_file(), "agent_journey_observation_file");
        fs::remove_file(&temporary)?;
    }
    save_private(&temporary, value)?;
    fs::rename(&temporary, path)?;
    File::open(path.parent().expect("observation parent"))?.sync_all()?;
    Ok(())
}

fn acquire_effect(tombstones: &Path, job: &Job) -> Result<()> {
    let digest = hash(job)?;
    let acknowledged = tombstones.join(format!("ack-{digest}.json"));
    if acknowledged.exists() {
        let saved: Reservation = publication::read(&acknowledged, 1024)?;
        ensure!(
            saved.job_digest == digest,
            "agent_journey_ack_identity_changed"
        );
        let original: Reservation = publication::read(
            &tombstones.join(format!("effect-{}.json", job.binding.job_id)),
            1024,
        )?;
        ensure!(
            original.job_digest == digest,
            "agent_journey_reservation_changed"
        );
        return Ok(()); // Redelivery of an old result cannot acquire new effects.
    }
    let active = tombstones.join("active.json");
    if active.exists() {
        let previous: Reservation = publication::read(&active, 1024)?;
        if previous.job_digest == digest {
            return Ok(());
        }
        ensure!(
            valid_digest(&previous.job_digest),
            "agent_journey_active_identity"
        );
        let completed: Reservation = publication::read(
            &tombstones.join(format!("ack-{}.json", previous.job_digest)),
            1024,
        )
        .map_err(|_| anyhow::anyhow!("agent_journey_previous_effect_unacknowledged"))?;
        ensure!(
            completed.job_digest == previous.job_digest,
            "agent_journey_ack_identity_changed"
        );
        replace_private(&active, &Reservation { job_digest: digest })?;
    } else {
        save_private(&active, &Reservation { job_digest: digest })?;
    }
    Ok(())
}

fn acknowledge_effect(tombstones: &Path, job: &Job) -> Result<()> {
    let digest = hash(job)?;
    let path = tombstones.join(format!("ack-{digest}.json"));
    if path.exists() {
        let saved: Reservation = publication::read(&path, 1024)?;
        ensure!(
            saved.job_digest == digest,
            "agent_journey_ack_identity_changed"
        );
    } else {
        save_private(&path, &Reservation { job_digest: digest })?;
    }
    Ok(())
}

impl Transport {
    pub(crate) fn poll_journey(
        &self,
        journal: &Journal,
        consent_path: &Path,
    ) -> Result<Option<String>> {
        let pairing = journal.pairing((self.clock)())?;
        self.paired(&pairing)?;
        let poll: Poll = self.request(
            Method::GET,
            "/v1/agent/journeys",
            Some(&pairing.agent_token),
            None,
        )?;
        let Some(job) = poll.job else { return Ok(None) };
        let owner = self.journey_owner(journal, consent_path, &job)?;
        self.journey_remote_job(&pairing, &job)?;
        let tombstones = owner.root.join("relay-reservations");
        let payloads = owner.root.join("relay-delivery");
        directory(&tombstones)?;
        directory(&payloads)?;
        let slot = observation_slot(&job.request);
        let name = slot
            .map(|slot| format!("observation-{slot}"))
            .unwrap_or_else(|| format!("effect-{}", job.binding.job_id));
        let reservation = tombstones.join(format!("{name}.json"));
        if slot.is_none() && !reservation.exists() {
            let count = fs::read_dir(&tombstones)?.try_fold(0usize, |count, entry| {
                let entry = entry?;
                Ok::<_, std::io::Error>(
                    count + usize::from(entry.file_name().to_string_lossy().starts_with("effect-")),
                )
            })?;
            ensure!(count < MAX_JOBS_PER_RUN, "agent_journey_reservation_limit");
        }
        if slot.is_none() {
            acquire_effect(&tombstones, &job)?;
        }
        let first = reserve(&reservation, &job)?;
        let result_path = payloads.join(format!("{name}.json"));
        let current = self.execute_journey_job(
            journal,
            consent_path,
            &job,
            first && !job.request.is_observation(),
        )?;
        check_result(&current, &job)?;
        let result = if result_path.exists() {
            let previous: StageResult = publication::read(&result_path, MAX_RESPONSE as usize)?;
            check_result(&previous, &job)?;
            check_previous(&previous, &current)?;
            if slot.is_some() {
                replace_private(&result_path, &current)?;
                current
            } else {
                previous
            }
        } else {
            save_private(&result_path, &current)?;
            current
        };
        self.journey_remote_job(&pairing, &job)?;
        self.recheck_journey_owner(journal, consent_path, &job, &owner)?;
        let baseline = self.selected_journey_baseline(journal, &job, &owner)?;
        if let Some(baseline) = &baseline {
            self.recheck_journey_baseline(journal, baseline)?;
        }
        let ack: Ack = self.request(
            Method::PUT,
            &format!("/v1/agent/journeys/{}/result", job.binding.job_id),
            Some(&pairing.agent_token),
            Some(&serde_json::to_value(&result)?),
        )?;
        ensure!(
            ack.binding == result.binding
                && ack.agent_candidate_revision == result.agent_candidate_revision
                && ack.checkpoint_sequence == result.checkpoint_sequence
                && ack.digest == result.digest,
            "agent_journey_ack_changed"
        );
        // Superseded confirms only this exact receipt, never that it is the current UI view.
        let _ = ack.superseded;
        self.recheck_journey_owner(journal, consent_path, &job, &owner)?;
        if let Some(baseline) = &baseline {
            self.recheck_journey_baseline(journal, baseline)?;
        }
        if slot.is_none() {
            acknowledge_effect(&tombstones, &job)?;
        }
        Ok(Some(job.binding.run_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job() -> Job {
        let request = Request::Status;
        Job {
            binding: Binding {
                schema: JOB_SCHEMA.into(),
                job_id: "status1".into(),
                subject: "subject1".into(),
                agent_id: "agent1".into(),
                run_id: "run1".into(),
                consent_digest: "a".repeat(64),
                report_digest: "b".repeat(64),
                received_digest: "c".repeat(64),
                request_digest: request.digest().unwrap(),
                expires_at: 200,
            },
            request,
            permitted_agent_candidate: "d".repeat(40),
        }
    }

    #[test]
    fn different_effect_waits_for_exact_ack_and_old_ack_cannot_authorize_reexecution() {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let first = job();
        let mut second = first.clone();
        second.binding.job_id = "next".into();
        acquire_effect(temp.path(), &first).unwrap();
        acquire_effect(temp.path(), &first).unwrap();
        let reservation = temp
            .path()
            .join(format!("effect-{}.json", first.binding.job_id));
        reserve(&reservation, &first).unwrap();
        assert!(acquire_effect(temp.path(), &second).is_err());
        acknowledge_effect(temp.path(), &first).unwrap();
        acquire_effect(temp.path(), &second).unwrap();
        acquire_effect(temp.path(), &first).unwrap();
        fs::remove_file(reservation).unwrap();
        assert!(acquire_effect(temp.path(), &first).is_err());
    }

    #[test]
    fn durable_reservation_survives_payload_scrub_and_rejects_identity_replacement() {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let root = temp.path().join("run-run1");
        directory(&root).unwrap();
        let reservations = root.join("relay-reservations");
        directory(&reservations).unwrap();
        let path = reservations.join("effect-job1.json");
        let original = job();
        assert!(reserve(&path, &original).unwrap());
        let bytes = fs::read(&path).unwrap();
        let payloads = root.join("relay-delivery");
        directory(&payloads).unwrap();
        save_private(
            &payloads.join("result.json"),
            &serde_json::json!({"source":"retained"}),
        )
        .unwrap();
        scrub_run_payloads(&root).unwrap();
        assert!(!payloads.exists());
        assert_eq!(bytes, fs::read(&path).unwrap());
        assert!(!reserve(&path, &original).unwrap());
        for field in ["candidate", "expiry", "request", "receipt"] {
            let mut changed = original.clone();
            match field {
                "candidate" => changed.permitted_agent_candidate = "e".repeat(40),
                "expiry" => changed.binding.expires_at += 1,
                "request" => changed.request = Request::Graph,
                _ => changed.binding.received_digest = "f".repeat(64),
            }
            assert!(reserve(&path, &changed).is_err(), "{field}");
            assert_eq!(bytes, fs::read(&path).unwrap());
        }
    }

    #[test]
    fn observation_has_eight_fixed_slots_and_cannot_replace_its_request_identity() {
        let requests = [
            Request::Status,
            Request::Graph,
            Request::Artifact {
                artifact: Artifact::Structure,
            },
            Request::Artifact {
                artifact: Artifact::Fitness,
            },
            Request::Artifact {
                artifact: Artifact::Impact,
            },
            Request::Artifact {
                artifact: Artifact::Rationale,
            },
            Request::Artifact {
                artifact: Artifact::Drift,
            },
            Request::Artifact {
                artifact: Artifact::PalaceComparison,
            },
        ];
        let slots: std::collections::BTreeSet<_> = requests.iter().map(observation_slot).collect();
        assert_eq!(slots.len(), 8);
        assert!(!slots.contains(&None));
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let path = temp.path().join("observation-status.json");
        let original = job();
        assert!(reserve(&path, &original).unwrap());
        let mut replacement = original.clone();
        replacement.binding.job_id = "status2".into();
        assert!(reserve(&path, &replacement).is_err());
        assert!(!reserve(&path, &original).unwrap());
    }

    #[test]
    fn request_canonicalization_matches_javascript_utf16_key_order_and_rejects_lossy_numbers() {
        let value = serde_json::json!({"\u{e000}":1,"\u{10000}":2,"a":{"z":0,"a":3}});
        let expected = "{\"a\":{\"a\":3,\"z\":0},\"𐀀\":2,\"\":1}";
        assert_eq!(
            canonical_request_bytes(&value).unwrap(),
            expected.as_bytes()
        );
        for invalid in [
            serde_json::json!(-1),
            serde_json::json!(1.5),
            serde_json::json!(9007199254740992u64),
        ] {
            assert!(canonical_request_bytes(&invalid).is_err());
        }
    }
}
