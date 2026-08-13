// PVF: lane=exact-child-tests; proof=issue-260 production caller adapter boundary;
// deterministic=true; resource_profile=small; release_gate=true; nonzero selection required.

const CAPABILITY: &str = include_str!("../src/distributed/capability_advertisement.rs");
const SNAPSHOT: &str = include_str!("../src/distributed/snapshot_catalog.rs");
const WEATHER: &str = include_str!("../src/distributed/resource_weather.rs");
const PLACEMENT: &str = include_str!("../src/distributed/placement.rs");
const PROJECTION: &str = include_str!("../src/distributed/projection.rs");
const MIGRATION: &str = include_str!("../src/distributed/migration.rs");
const RECOVERY: &str = include_str!("../src/distributed/recovery.rs");

#[test]
fn production_verifiers_require_governed_certificate_adapters() {
    assert!(CAPABILITY.contains("certificate_store: AuthorityBoundCertificateStore"));
    assert!(CAPABILITY
        .contains("certificate_store: CapabilityCertificateAuthority::Bound(certificate_store)"));
    assert!(SNAPSHOT.contains("certificate_store: AuthorityBoundCertificateStore"));
    assert!(SNAPSHOT
        .contains("certificate_store: SnapshotCertificateAuthority::Bound(certificate_store)"));
}

#[test]
fn remaining_read_callers_are_governed_in_production() {
    assert!(WEATHER
        .contains("impl ResourceWeatherCertificateAuthority for AuthorityBoundCertificateStore"));
    assert!(WEATHER.contains(
        "#[cfg(test)]\nimpl ResourceWeatherCertificateAuthority for DistributedCertificateStore"
    ));
    assert!(PLACEMENT.contains("pub fn capture<C: ResourceWeatherCertificateAuthority>"));
    assert!(PLACEMENT.contains("type PlacementLeaseAuthority = AuthorityBoundLeaseLedger"));
    assert!(PLACEMENT.contains("type PlacementFencingAuthority = AuthorityBoundFencingStore"));
    assert!(PLACEMENT.contains("placement_lease_snapshot(ledger)?"));
    assert!(PLACEMENT.contains("placement_fencing_floor(store, lineage.lineage_id.as_bytes())"));
    assert!(
        PROJECTION.contains("type ProjectionCertificateSource = AuthorityBoundCertificateStore")
    );
    assert!(PROJECTION.contains("type ProjectionLeaseSource = AuthorityBoundLeaseLedger"));
    assert!(PROJECTION.contains("type ProjectionFencingSource = AuthorityBoundFencingStore"));
    assert!(PROJECTION
        .contains("#[cfg(test)]\ntype ProjectionCertificateSource = DistributedCertificateStore"));
}

#[test]
fn migration_and_recovery_transitions_are_governed_in_production() {
    for source in [MIGRATION, RECOVERY] {
        assert!(source.contains("#[cfg(not(test))]"));
        assert!(source.contains("AuthorityBoundLeaseLedger"));
        assert!(source.contains("AuthorityBoundFencingStore"));
        assert!(source.contains("#[cfg(test)]"));
        assert!(source.contains("AUTHORITY_BOUND_LEASE_ACCESS"));
        assert!(source.contains("AUTHORITY_BOUND_FENCING_ACCESS"));
    }
    assert!(
        MIGRATION.contains("migration_apply(ledger, certificate_bytes, membership, application)")
    );
    assert!(RECOVERY.contains("recovery_apply(ledger, certificate_bytes, membership, application)"));
}

#[test]
fn raw_certificate_store_seams_are_test_only() {
    for source in [CAPABILITY, SNAPSHOT] {
        let test_import = source
            .find("#[cfg(test)]\nuse super::certificates::{DistributedCertificateStore")
            .expect("raw certificate store import must be test-only");
        let raw_variant = source
            .find("#[cfg(test)]\n    TestRaw(Arc<DistributedCertificateStore>)")
            .expect("raw variant must be test-only");
        let raw_constructor = source
            .find("#[cfg(test)]\n    pub fn open_for_test(")
            .expect("raw constructor must be test-only");
        assert!(test_import < raw_variant && raw_variant < raw_constructor);
    }
}

#[test]
fn production_authorization_does_not_pass_raw_access_tokens() {
    let capability_production = CAPABILITY
        .split("#[cfg(test)]\n    pub fn open_for_test(")
        .next()
        .unwrap();
    let snapshot_production = SNAPSHOT
        .split("#[cfg(test)]\n    pub fn open_for_test(")
        .next()
        .unwrap();
    assert!(!capability_production.contains("certificate_store: Arc<DistributedCertificateStore>"));
    assert!(!snapshot_production.contains("certificate_store: Arc<DistributedCertificateStore>"));
}
