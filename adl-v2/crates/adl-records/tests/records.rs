use std::collections::{BTreeMap, BTreeSet};

use adl_records::{
    canonical_bytes, decode_envelope, encode_envelope, sign_record, verify_envelope,
    ArtifactDescriptor, ErrorCode, ErrorRecord, EventRecord, ExecutionResult, InMemoryReplayGuard,
    Limits, Record, RecordHeader, RecordKind, TraceRecord, TrustEntry, TrustPolicy,
    CONTRACT_VERSION,
};
use ed25519_dalek::SigningKey;

fn header(sequence: u64) -> RecordHeader {
    RecordHeader {
        contract_version: CONTRACT_VERSION.into(),
        record_id: format!("record-{sequence}"),
        subject_id: "agent-1".into(),
        sequence,
        logical_timestamp: 100 + sequence,
        metadata: BTreeMap::from([("region".into(), "local".into())]),
    }
}

fn event(sequence: u64) -> Record {
    Record::Event(EventRecord {
        header: header(sequence),
        name: "task.completed".into(),
        detail: "ok".into(),
    })
}

fn policy(key: &SigningKey, revoked: bool, kinds: &[RecordKind]) -> TrustPolicy {
    TrustPolicy::new(
        BTreeMap::from([(
            "key-1".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: kinds.iter().copied().collect::<BTreeSet<_>>(),
                not_before: 10,
                not_after: 1000,
                revoked,
            },
        )]),
        &Limits::default(),
    )
    .unwrap()
}

#[test]
fn all_record_contracts_validate_and_schema_is_versioned() {
    let digest = "ab".repeat(32);
    let records = vec![
        Record::Error(ErrorRecord {
            header: header(1),
            code: "E1".into(),
            message: "failed".into(),
            retryable: false,
        }),
        event(2),
        Record::Trace(TraceRecord {
            header: header(3),
            trace_id: "trace".into(),
            span_id: "span".into(),
            parent_span_id: None,
            operation: "execute".into(),
            attributes: BTreeMap::from([("component".into(), "engine".into())]),
        }),
        Record::ExecutionResult(ExecutionResult {
            header: header(4),
            status: "succeeded".into(),
            output_digest: Some(digest.clone()),
            diagnostic: None,
        }),
        Record::Artifact(ArtifactDescriptor {
            header: header(5),
            media_type: "application/json".into(),
            content_digest: digest,
            byte_length: 42,
        }),
    ];
    for record in records {
        record.validate(&Limits::default()).unwrap();
        assert!(!canonical_bytes(&record, &Limits::default())
            .unwrap()
            .is_empty());
    }
    let schema = schemars::schema_for!(Record);
    let text = serde_json::to_string(&schema).unwrap();
    assert!(text.contains("execution_result"));
    assert!(text.contains("artifact"));
}

#[test]
fn canonical_bytes_are_deterministic_and_order_independent_for_maps() {
    let mut first = event(1);
    let mut second = event(1);
    first.header_mut().metadata =
        BTreeMap::from([("b".into(), "2".into()), ("a".into(), "1".into())]);
    second.header_mut().metadata =
        BTreeMap::from([("a".into(), "1".into()), ("b".into(), "2".into())]);
    assert_eq!(
        canonical_bytes(&first, &Limits::default()).unwrap(),
        canonical_bytes(&second, &Limits::default()).unwrap()
    );
}

#[test]
fn sign_encode_decode_verify_round_trip_and_replay_fails() {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[7; 32]);
    let envelope = sign_record(event(1), "key-1", &key, &limits).unwrap();
    let bytes = encode_envelope(&envelope, &limits).unwrap();
    let decoded = decode_envelope(&bytes, &limits).unwrap();
    let mut replay = InMemoryReplayGuard::new(&limits);
    assert_eq!(
        verify_envelope(
            &decoded,
            &policy(&key, false, &[RecordKind::Event]),
            &mut replay,
            100,
            &limits
        )
        .unwrap(),
        event(1)
    );
    assert_eq!(
        verify_envelope(
            &decoded,
            &policy(&key, false, &[RecordKind::Event]),
            &mut replay,
            100,
            &limits
        )
        .unwrap_err()
        .code,
        ErrorCode::Replay
    );
}

#[test]
fn trust_policy_rejects_unknown_wrong_kind_expiry_and_revocation() {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[8; 32]);
    let envelope = sign_record(event(1), "key-1", &key, &limits).unwrap();
    for (candidate, now) in [
        (
            TrustPolicy::new(
                BTreeMap::from([(
                    "other".into(),
                    TrustEntry {
                        verifying_key: key.verifying_key(),
                        profile_version: 1,
                        allowed_kinds: BTreeSet::from([RecordKind::Event]),
                        not_before: 0,
                        not_after: 1000,
                        revoked: false,
                    },
                )]),
                &limits,
            )
            .unwrap(),
            100,
        ),
        (policy(&key, false, &[RecordKind::Error]), 100),
        (policy(&key, false, &[RecordKind::Event]), 1001),
        (policy(&key, true, &[RecordKind::Event]), 100),
    ] {
        let error = verify_envelope(
            &envelope,
            &candidate,
            &mut InMemoryReplayGuard::new(&limits),
            now,
            &limits,
        )
        .unwrap_err();
        assert_eq!(error.code, ErrorCode::Trust);
    }
}

#[test]
fn explicit_bounds_fail_closed() {
    let limits = Limits {
        max_string_bytes: 4,
        ..Limits::default()
    };
    assert_eq!(
        event(1).validate(&limits).unwrap_err().code,
        ErrorCode::Bounds
    );
    let limits = Limits {
        max_envelope_bytes: 8,
        ..Limits::default()
    };
    assert_eq!(
        decode_envelope(br#"{\"too\":\"large\"}"#, &limits)
            .unwrap_err()
            .code,
        ErrorCode::Bounds
    );
}

trait HeaderMut {
    fn header_mut(&mut self) -> &mut RecordHeader;
}

impl HeaderMut for Record {
    fn header_mut(&mut self) -> &mut RecordHeader {
        match self {
            Record::Error(value) => &mut value.header,
            Record::Event(value) => &mut value.header,
            Record::Trace(value) => &mut value.header,
            Record::ExecutionResult(value) => &mut value.header,
            Record::Artifact(value) => &mut value.header,
        }
    }
}
