//! PVF: public serialization boundary proof for governed cognitive authority.

use adl_runtime_kernel::CognitiveAuthorityContext;
use serde_json::json;

#[test]
fn public_authority_payload_rejects_unknown_fields() {
    let value = json!({
        "authority_id": "cognitive-board",
        "key_id": "cognitive-key-1",
        "epoch": 1,
        "context_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "verifying_key_hex": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "attacker_private_state": "gho_secret"
    });
    assert!(serde_json::from_value::<CognitiveAuthorityContext>(value).is_err());
}
