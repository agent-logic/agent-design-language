#![cfg(feature = "internal-test-fixtures")]

use adl_runtime::distributed::{
    authority_protocol::test_observatory_published_authority,
    serving_authority::{
        verify_observatory_authority_projection, ObservatoryBindingFixture,
        VerifiedServingAuthorityCut,
    },
};

#[test]
fn exact_pair_projects_and_cross_pairs_fail_closed() {
    let a = ObservatoryBindingFixture::new("a");
    let b = ObservatoryBindingFixture::new("b");
    let authority_a = test_observatory_published_authority(a.artifact_bytes());
    let authority_b = test_observatory_published_authority(b.artifact_bytes());
    let cut_a = VerifiedServingAuthorityCut::fixture_from_observatory(&a);
    let cut_b = VerifiedServingAuthorityCut::fixture_from_observatory(&b);

    assert!(verify_observatory_authority_projection(&authority_a, &cut_a).is_ok());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_b).is_ok());
    assert!(verify_observatory_authority_projection(&authority_a, &cut_b).is_err());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_a).is_err());
}

#[test]
fn identifiers_and_noncanonical_artifacts_fail_closed() {
    for suffix in ["bad space", "bad☃", "bad/char", "bad\nline"] {
        let fixture = ObservatoryBindingFixture::new(suffix);
        let authority = test_observatory_published_authority(fixture.artifact_bytes());
        let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
        assert!(verify_observatory_authority_projection(&authority, &cut).is_err());
    }

    let fixture = ObservatoryBindingFixture::new("canonical");
    let mut bytes = fixture.artifact_bytes();
    bytes.insert(1, b' ');
    let authority = test_observatory_published_authority(bytes);
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
    assert!(verify_observatory_authority_projection(&authority, &cut).is_err());
}

#[test]
fn projection_is_redacted_and_deadline_is_committed() {
    let fixture = ObservatoryBindingFixture::new("redacted");
    let authority = test_observatory_published_authority(fixture.artifact_bytes());
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
    let projection = verify_observatory_authority_projection(&authority, &cut).unwrap();
    let json = serde_json::to_string(&projection).unwrap();
    assert!(json.contains("observatory-authority-projection"));
    assert!(!json.contains("guardian"));
    assert!(!json.contains("owner-commit"));
    assert!(projection.inclusive_deadline_unix_seconds() > 0);
}
