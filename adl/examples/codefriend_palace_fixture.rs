//! Deterministic proof fixture ONLY. Embedded signing seeds are public test data,
//! never production trust. All tokens are admitted through the production API.
use adl::codefriend::memory::palace_authority::{self, Evidence, Trust};
use adl_runtime_kernel::*;
use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";
fn reference_with_digest(id: &str, sha256: &str) -> IdentityReference {
    IdentityReference {
        id: id.to_owned(),
        path: format!("evidence/identity/{id}.json"),
        sha256: sha256.to_owned(),
    }
}
fn main() -> Result<()> {
    let output = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("fixture output directory required"))?;
    generate(Path::new(&output))
}
pub fn generate(root: &Path) -> Result<()> {
    generate_with_successor(root, false)
}
pub fn verify_successor(root: &Path) -> Result<()> {
    generate_with_successor(root, true)
}
fn generate_with_successor(root: &Path, successor: bool) -> Result<()> {
    fs::create_dir_all(root)?;
    let identity_authority = IdentityAuthority::from_bytes("identity-birthday-key", &[11_u8; 32]);
    let identity_keys = BTreeMap::from([(
        "identity-birthday-key".to_owned(),
        identity_authority.verifying_key(),
    )]);
    let binding = identity_authority
        .bind(
            "citizen-aster",
            "runtime-v3",
            "continuity-aster",
            7,
            BTreeSet::from(["birthday.identity".to_owned()]),
        )
        .expect("signed identity binding");
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &identity_keys,
            MemoryClass::Identity,
            BTreeMap::from([
                (
                    "birthday.origin_event".to_owned(),
                    "origin-event-001".to_owned(),
                ),
                ("birthday.stable_name".to_owned(), "Aster".to_owned()),
                (
                    "birthday.alias.alias-one".to_owned(),
                    "Aster One".to_owned(),
                ),
                (
                    "birthday.alias.alias-north".to_owned(),
                    "North Star".to_owned(),
                ),
            ]),
            None,
        )
        .expect("accepted identity event");
    let checkpoint = ledger
        .checkpoint(&binding, &identity_keys, &identity_authority)
        .expect("signed memory checkpoint");

    let private_authority = PrivateStateAuthority::from_bytes("private-birthday-key", &[7_u8; 32]);
    let available_projection = BTreeMap::from([
        (
            "identity_summary".to_owned(),
            "Aster continuity accepted".to_owned(),
        ),
        ("witness_status".to_owned(), "governed".to_owned()),
    ]);
    let mut private_record = private_authority
        .issue_record(PrivateStateSealRequest {
            subject_id: binding.citizen_id.clone(),
            lineage_id: binding.continuity_id.clone(),
            sequence: 1,
            predecessor_hash: GENESIS.to_owned(),
            private_payload: b"raw private birthday state that must never be projected".to_vec(),
            projection: available_projection.clone(),
            sanctuary_level: 1,
        })
        .expect("signed private-state record");
    let private_keys = BTreeMap::from([(
        "private-birthday-key".to_owned(),
        private_authority.verifying_key(),
    )]);
    let mut lineage = PrivateStateLineage::default();
    if successor {
        let predecessor = lineage
            .append(&private_record, &private_keys)
            .map_err(|_| anyhow!("fixture predecessor admission failed"))?;
        private_record = private_authority
            .issue_record(PrivateStateSealRequest {
                subject_id: binding.citizen_id.clone(),
                lineage_id: binding.continuity_id.clone(),
                sequence: 2,
                predecessor_hash: predecessor,
                private_payload: b"successor private fixture state".to_vec(),
                projection: available_projection.clone(),
                sanctuary_level: 1,
            })
            .map_err(|_| anyhow!("fixture successor signing failed"))?;
    }
    let original_head = lineage.head(&binding.continuity_id).map(str::to_owned);
    let checkpoint_authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let trust = Trust {
        schema: "codefriend.palace.trust.v1".to_owned(),
        identity_key_id: "identity-birthday-key".to_owned(),
        identity_public_key: hex::encode(identity_authority.verifying_key().as_bytes()),
        private_key_id: "private-birthday-key".to_owned(),
        private_public_key: hex::encode(private_authority.verifying_key().as_bytes()),
        continuity_key_id: "runtime-continuity".to_owned(),
        continuity_public_key: hex::encode(checkpoint_authority.verifying_key().as_bytes()),
        identity_generation: 7,
        continuity_generation: 1,
        projection_generation: if successor { 2 } else { 1 },
        runtime_state_dir: "runtime-authority-state".to_owned(),
    };
    let live = palace_authority::assembly(&trust, root)?;
    let evidence = live
        .verify_memory_palace_identity_evidence(MemoryPalaceIdentityEvidence {
            identity_binding: &binding,
            identity_checkpoint: &checkpoint,
            private_record: &private_record,
            private_lineage: &mut lineage.clone(),
            available_projection: &available_projection,
        })
        .map_err(|_| anyhow!("fixture identity admission failed"))?;
    let mut candidate = BirthdayIdentityCandidate {
        schema: BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA.to_owned(),
        basis: IdentityBasis::OriginEvidence,
        stable_name: "Aster".to_owned(),
        identity_root: "0".repeat(64),
        aliases: vec![
            AliasBinding {
                name: "North Star".to_owned(),
                provenance_id: "alias-north".to_owned(),
            },
            AliasBinding {
                name: "Aster One".to_owned(),
                provenance_id: "alias-one".to_owned(),
            },
        ],
        origin: adl_runtime_kernel::OriginBinding {
            event_id: "origin-event-001".to_owned(),
            provenance_id: "origin-prov".to_owned(),
            reference: reference_with_digest("origin-binding", evidence.binding_sha256()),
        },
        continuity: adl_runtime_kernel::ContinuityBinding {
            identity_root: "0".repeat(64),
            head_sha256: evidence.checkpoint_head().to_owned(),
            reference: reference_with_digest("continuity-checkpoint", evidence.checkpoint_sha256()),
        },
        provenance: vec![
            reference_with_digest("origin-prov", evidence.binding_sha256()),
            reference_with_digest("alias-one", evidence.checkpoint_sha256()),
            reference_with_digest("alias-north", evidence.checkpoint_sha256()),
        ],
        witnesses: vec![
            reference_with_digest("private-record", evidence.private_record_sha256()),
            reference_with_digest(
                "governed-projection-witness",
                &evidence.projection_receipt().projection_sha256,
            ),
        ],
        governed_projection: reference_with_digest(
            "governed-projection",
            &evidence.projection_receipt().projection_sha256,
        ),
    };
    candidate.identity_root = derive_identity_root(&candidate, &evidence).expect("identity root");
    candidate.continuity.identity_root = candidate.identity_root.clone();
    let mut manifest = CheckpointManifest {
        schema: CHECKPOINT_SCHEMA.to_owned(),
        generation: 1,
        previous_integrity: None,
        accepted_through: 1,
        provenance: "runtime-v3-live-shutdown".to_owned(),
        topology_hash: live.topology_hash.clone(),
        config_hash: live.config_hash.clone(),
        migration: MigrationPolicy::Exact,
        snapshots: vec![SnapshotEntry {
            service: "live_kernel".to_owned(),
            service_schema: LIVE_KERNEL_CHECKPOINT_SCHEMA.to_owned(),
            file: "0000-live_kernel.bin".to_owned(),
            bytes: 4,
            checksum: "c".repeat(64),
        }],
        integrity: String::new(),
        signing_algorithm: String::new(),
        signing_key_id: String::new(),
        signature: String::new(),
    };
    checkpoint_authority
        .sign_manifest(&mut manifest)
        .map_err(|_| anyhow!("fixture signing failed"))?;
    let mut second = manifest.clone();
    second.generation = 2;
    second.accepted_through = 2;
    second.previous_integrity = Some(manifest.integrity.clone());
    checkpoint_authority
        .sign_manifest(&mut second)
        .map_err(|_| anyhow!("fixture signing failed"))?;
    let manifests = vec![manifest, second];
    let mut rejected_manifests = manifests.clone();
    rejected_manifests[0].signature = "00".repeat(64);
    let rejected = live.prepare_memory_palace_authority(
        &candidate,
        MemoryPalaceIdentityEvidence {
            identity_binding: &binding,
            identity_checkpoint: &checkpoint,
            private_record: &private_record,
            private_lineage: &mut lineage,
            available_projection: &available_projection,
        },
        &rejected_manifests,
    );
    anyhow::ensure!(
        matches!(
            rejected,
            Err(MemoryPalaceAuthorityError::ContinuityCycles(_))
        ),
        "counterfeited manifest must fail continuity verification after identity admission"
    );
    anyhow::ensure!(
        lineage.head(&binding.continuity_id) == original_head.as_deref(),
        "failed preparation changed caller lineage"
    );
    let prepared = live
        .prepare_memory_palace_authority(
            &candidate,
            MemoryPalaceIdentityEvidence {
                identity_binding: &binding,
                identity_checkpoint: &checkpoint,
                private_record: &private_record,
                private_lineage: &mut lineage,
                available_projection: &available_projection,
            },
            &manifests,
        )
        .map_err(|_| anyhow!("fixture authority preparation failed"))?;
    let accepted_head = lineage.head(&binding.continuity_id).map(str::to_owned);
    anyhow::ensure!(
        accepted_head.as_deref()
            == Some(
                prepared
                    .identity_evidence()
                    .projection_receipt()
                    .accepted_record_hash
                    .as_str()
            ),
        "successful preparation did not advance caller lineage"
    );
    let replay = lineage
        .append(&private_record, &private_keys)
        .map_err(|_| anyhow!("accepted successor replay failed"))?;
    anyhow::ensure!(
        accepted_head.as_deref() == Some(replay.as_str()),
        "replay changed accepted head"
    );
    if successor {
        return Ok(());
    }
    let identity = prepared.identity().clone();
    let continuity_record = prepared.continuity().record().clone();
    let input = Evidence {
        schema: "codefriend.palace.authority-evidence.v1".to_owned(),
        identity_record: identity,
        identity_binding: binding,
        identity_checkpoint: checkpoint,
        private_record,
        available_projection,
        continuity_record,
        continuity_manifests: manifests,
    };
    write_new(&root.join("trust.json"), &trust)?;
    write_new(&root.join("authority-evidence.json"), &input)?;
    drop(live);
    let admitted = palace_authority::provision(
        &root.join("trust.json"),
        &root.join("authority-evidence.json"),
    )?;
    write_new(
        &root.join("authority-summary.json"),
        &serde_json::json!({
            "schema": "codefriend.palace.fixture-summary.v1",
            "identity_root": admitted.identity().identity_root,
            "continuity_head": admitted.continuity().record().continuity_head,
            "identity_record_sha256": admitted.identity().record_sha256,
            "continuity_record_sha256": admitted.continuity().record().record_sha256,
            "authority_route": "LiveAssembly::provision_memory_palace_authority",
            "fixture_only": true
        }),
    )?;
    Ok(())
}
fn write_new(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(&serde_json::to_vec_pretty(value)?)?;
    Ok(())
}
