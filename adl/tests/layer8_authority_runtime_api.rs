use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use adl::csm_runtime_api::{authorize_layer8_runtime_delivery, Layer8RuntimeDeliveryRequest};
use adl_runtime::layer8_authority::{
    ConversationAuthorityProfile, Layer8Action, Layer8AuthorityStore, Layer8ConversationAuthority,
    RefusalReason,
};

struct TestRoot(PathBuf);

impl TestRoot {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "adl-layer8-runtime-api-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn authority(root: &Path) -> Layer8ConversationAuthority {
    Layer8ConversationAuthority::new(
        Layer8AuthorityStore::open(root.join("authority.jsonl")).unwrap(),
        ConversationAuthorityProfile {
            principal_id: "layer8-operator".to_owned(),
            polis_id: "polis-test".to_owned(),
            policy_epoch: 7,
            allowed_actions: BTreeSet::from([Layer8Action::Contact]),
            allowed_recipients: BTreeSet::from(["shepherd".to_owned()]),
        },
    )
    .unwrap()
}

fn request(recipient_id: &str, replay_id: &str) -> Layer8RuntimeDeliveryRequest {
    Layer8RuntimeDeliveryRequest {
        action: Layer8Action::Contact,
        conversation_id: "conversation-1".to_owned(),
        recipient_id: recipient_id.to_owned(),
        replay_id: replay_id.to_owned(),
        correlation_id: "correlation-1".to_owned(),
        credential_generation: 3,
        now_epoch_secs: 1_700_000_000,
    }
}

#[test]
fn runtime_api_delivers_only_after_authority_grants() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let deliveries = AtomicUsize::new(0);

    let delivered =
        authorize_layer8_runtime_delivery(&authority, request("shepherd", "request-1"), || {
            deliveries.fetch_add(1, Ordering::SeqCst)
        })
        .unwrap();

    assert_eq!(delivered, 0);
    assert_eq!(deliveries.load(Ordering::SeqCst), 1);
}

#[test]
fn runtime_api_refusal_cannot_invoke_delivery() {
    let root = TestRoot::new();
    let authority = authority(root.path());
    let deliveries = AtomicUsize::new(0);

    let refusal = authorize_layer8_runtime_delivery(
        &authority,
        request("agent-not-authorized", "request-2"),
        || deliveries.fetch_add(1, Ordering::SeqCst),
    )
    .unwrap_err();

    assert_eq!(refusal.reason, RefusalReason::ScopeDenied);
    assert_eq!(deliveries.load(Ordering::SeqCst), 0);
}
