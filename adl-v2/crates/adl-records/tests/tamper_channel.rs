use std::collections::{BTreeMap, BTreeSet};

use adl_records::{
    decode_envelope, encode_envelope, sign_record, verify_envelope, ErrorCode, EventRecord,
    InMemoryReplayGuard, Limits, Record, RecordHeader, RecordKind, TrustEntry, TrustPolicy,
    CONTRACT_VERSION,
};
use ed25519_dalek::SigningKey;
use serde_json::Value;

fn fixture() -> (SigningKey, TrustPolicy, adl_records::SignedEnvelope, Limits) {
    let limits = Limits::default();
    let key = SigningKey::from_bytes(&[9; 32]);
    let record = Record::Event(EventRecord {
        header: RecordHeader {
            contract_version: CONTRACT_VERSION.into(),
            record_id: "record-1".into(),
            subject_id: "agent-1".into(),
            sequence: 1,
            logical_timestamp: 10,
            metadata: BTreeMap::from([("a".into(), "b".into())]),
        },
        name: "started".into(),
        detail: "bounded".into(),
    });
    let policy = TrustPolicy::new(
        BTreeMap::from([(
            "key-1".into(),
            TrustEntry {
                verifying_key: key.verifying_key(),
                profile_version: 1,
                allowed_kinds: BTreeSet::from([RecordKind::Event]),
                not_before: 0,
                not_after: 100,
                revoked: false,
            },
        )]),
        &limits,
    )
    .unwrap();
    let envelope = sign_record(record, "key-1", &key, &limits).unwrap();
    (key, policy, envelope, limits)
}

fn rejects(mutator: impl FnOnce(&mut Value)) -> ErrorCode {
    let (_, policy, envelope, limits) = fixture();
    let mut value = serde_json::to_value(envelope).unwrap();
    mutator(&mut value);
    let bytes = serde_json::to_vec(&value).unwrap();
    match decode_envelope(&bytes, &limits) {
        Err(error) => error.code,
        Ok(decoded) => {
            verify_envelope(
                &decoded,
                &policy,
                &mut InMemoryReplayGuard::new(&limits),
                20,
                &limits,
            )
            .unwrap_err()
            .code
        }
    }
}

#[test]
fn every_signed_field_class_rejects_tampering() {
    let paths: &[&[&str]] = &[
        &["profile_version"],
        &["record_kind"],
        &["contract_version"],
        &["key_id"],
        &["payload_digest"],
        &["signature"],
        &["payload", "record", "header", "record_id"],
        &["payload", "record", "header", "subject_id"],
        &["payload", "record", "header", "sequence"],
        &["payload", "record", "header", "logical_timestamp"],
        &["payload", "record", "name"],
        &["payload", "record", "detail"],
    ];
    for path in paths {
        let code = rejects(|value| {
            let mut current = value;
            for component in &path[..path.len() - 1] {
                current = &mut current[*component];
            }
            let leaf = path[path.len() - 1];
            current[leaf] = match current[leaf] {
                Value::Number(_) => Value::from(99),
                _ => Value::String("tampered".into()),
            };
        });
        assert!(matches!(
            code,
            ErrorCode::InvalidEnvelope
                | ErrorCode::InvalidRecord
                | ErrorCode::InvalidSignature
                | ErrorCode::Trust
                | ErrorCode::Bounds
        ));
    }
}

#[test]
fn duplicate_unknown_truncated_extended_utf8_float_and_oversize_fail() {
    let (_, _, envelope, limits) = fixture();
    let bytes = encode_envelope(&envelope, &limits).unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    let duplicate = text.replacen("{", "{\"profile_version\":1,", 1);
    assert_eq!(
        decode_envelope(duplicate.as_bytes(), &limits)
            .unwrap_err()
            .code,
        ErrorCode::DuplicateField
    );
    let mut unknown: Value = serde_json::from_slice(&bytes).unwrap();
    unknown["unknown"] = Value::Bool(true);
    assert_eq!(
        decode_envelope(&serde_json::to_vec(&unknown).unwrap(), &limits)
            .unwrap_err()
            .code,
        ErrorCode::InvalidEnvelope
    );
    assert!(decode_envelope(&bytes[..bytes.len() - 1], &limits).is_err());
    let mut extended = bytes.clone();
    extended.extend_from_slice(b"x");
    assert!(decode_envelope(&extended, &limits).is_err());
    assert!(decode_envelope(&[0xff], &limits).is_err());
    let float = text.replace("\"profile_version\":1", "\"profile_version\":1.5");
    assert!(decode_envelope(float.as_bytes(), &limits).is_err());
    let tiny = Limits {
        max_envelope_bytes: 4,
        ..limits
    };
    assert_eq!(
        decode_envelope(&bytes, &tiny).unwrap_err().code,
        ErrorCode::Bounds
    );
}

#[test]
fn sequence_rollback_and_tuple_collision_fail() {
    let (key, policy, first, limits) = fixture();
    let mut guard = InMemoryReplayGuard::new(&limits);
    verify_envelope(&first, &policy, &mut guard, 20, &limits).unwrap();
    let mut prior = first.payload.clone();
    if let Record::Event(value) = &mut prior {
        value.header.record_id = "record-0".into();
    }
    let prior = sign_record(prior, "key-1", &key, &limits).unwrap();
    assert_eq!(
        verify_envelope(&prior, &policy, &mut guard, 20, &limits)
            .unwrap_err()
            .code,
        ErrorCode::Replay
    );
}
