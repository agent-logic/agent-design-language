//! PVF: deterministic public serialization boundary for WP-15.

use adl_runtime_kernel::BirthWitnessPacket;

#[test]
fn public_packet_rejects_unknown_fields() {
    let value = serde_json::json!({
        "schema": "adl.birth_witness.packet.v1",
        "witness_set": {},
        "receipt": {},
        "packet_sha256": "0".repeat(64),
        "raw_private_state": true
    });
    assert!(serde_json::from_value::<BirthWitnessPacket>(value).is_err());
}
