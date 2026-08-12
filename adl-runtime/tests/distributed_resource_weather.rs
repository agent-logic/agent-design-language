// PVF: lane=exact-child-tests; proof=signed resource-weather positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[path = "../src/distributed/certificates.rs"]
#[allow(dead_code)]
mod certificates;
#[path = "../src/distributed/resource_weather.rs"]
mod resource_weather;

use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, RevocationReason,
    TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::SigningKey;
use redb::{Database, ReadableDatabase, TableDefinition};
use resource_weather::{
    MetricValue, NormalizedResourceMetrics, ObservationWindow, PlacementWeather,
    RawResourceMetrics, ResourceWeatherClaims, ResourceWeatherPolicy, ResourceWeatherStore,
    SignedResourceWeather, WeatherAction, WeatherAvailability, WeatherError, MAX_AVAILABLE_SLOTS,
    MAX_FUTURE_SKEW_SECS, MAX_HOLDERS, MAX_OBSERVATION_LIFETIME_SECS, MAX_PAYLOAD_BYTES,
    RESOURCE_WEATHER_SCHEMA,
};
use tempfile::TempDir;

const DOMAIN: &str = "polis.example";
const NOW: u64 = 1_000_000;

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn certificate_policy(root: &SigningKey) -> CertificatePolicy {
    CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3_600, 1, 10, 64, 64)
        .unwrap()
}

fn open_certificates(temp: &TempDir, root: &SigningKey) -> DistributedCertificateStore {
    DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        temp.path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        certificate_policy(root),
    )
    .unwrap()
}

fn issue_certificate(
    root: &SigningKey,
    signer: &SigningKey,
    holder: &str,
    purpose: CertificatePurpose,
    generation: u64,
) -> AuthorityCertificate {
    issue_certificate_with_validity(
        root,
        signer,
        holder,
        purpose,
        generation,
        NOW - 10,
        NOW + 300,
    )
}

fn issue_certificate_with_validity(
    root: &SigningKey,
    signer: &SigningKey,
    holder: &str,
    purpose: CertificatePurpose,
    generation: u64,
    issued_at_unix_secs: u64,
    expires_at_unix_secs: u64,
) -> AuthorityCertificate {
    AuthorityCertificate::issue(
        CertificateBody::new(
            DOMAIN,
            holder,
            purpose,
            generation,
            CertificateValidity {
                issued_at_unix_secs,
                expires_at_unix_secs,
            },
            signer.verifying_key(),
            &root.verifying_key(),
        ),
        root,
    )
    .unwrap()
}

fn weather_policy(max_holders: u64) -> ResourceWeatherPolicy {
    ResourceWeatherPolicy::new(DOMAIN)
        .unwrap()
        .with_bounds(60, 2, 4_096, max_holders, 64)
        .unwrap()
}

fn weather_path(temp: &TempDir) -> std::path::PathBuf {
    temp.path().canonicalize().unwrap().join("weather.redb")
}

fn open_weather(temp: &TempDir, max_holders: u64) -> ResourceWeatherStore {
    ResourceWeatherStore::open(weather_path(temp), weather_policy(max_holders)).unwrap()
}

fn assert_weather_open_error(path: impl AsRef<std::path::Path>, expected: WeatherError) {
    match ResourceWeatherStore::open(path, weather_policy(8)) {
        Ok(_) => panic!("unsafe resource-weather database path unexpectedly opened"),
        Err(error) => assert_eq!(error, expected),
    }
}

#[test]
fn database_path_rejects_relative_paths() {
    assert_weather_open_error("weather.redb", WeatherError::RelativeDatabasePath);
}

#[cfg(unix)]
#[test]
fn database_path_rejects_symlinked_ancestors_and_final_component() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let real_parent = root.join("real-parent");
    std::fs::create_dir(&real_parent).unwrap();

    let linked_parent = root.join("linked-parent");
    symlink(&real_parent, &linked_parent).unwrap();
    assert_weather_open_error(
        linked_parent.join("weather.redb"),
        WeatherError::DatabasePathIsSymlink,
    );

    let target = root.join("target.redb");
    let target_store = ResourceWeatherStore::open(&target, weather_policy(8)).unwrap();
    drop(target_store);
    let linked_database = root.join("linked-weather.redb");
    symlink(&target, &linked_database).unwrap();
    assert_weather_open_error(linked_database, WeatherError::DatabasePathIsSymlink);
}

fn mutate_durable_record(path: &std::path::Path, mutate: impl FnOnce(&mut serde_json::Value)) {
    let database = Database::create(path).unwrap();
    let bytes = {
        let read = database.begin_read().unwrap();
        let table = read
            .open_table(TableDefinition::<&str, &[u8]>::new(
                "distributed_resource_weather_v1",
            ))
            .unwrap();
        table.get("node-a").unwrap().unwrap().value().to_vec()
    };
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    mutate(&mut value);
    let encoded = serde_json::to_vec(&value).unwrap();
    let write = database.begin_write().unwrap();
    let mut table = write
        .open_table(TableDefinition::<&str, &[u8]>::new(
            "distributed_resource_weather_v1",
        ))
        .unwrap();
    table.insert("node-a", encoded.as_slice()).unwrap();
    drop(table);
    write.commit().unwrap();
}

fn normalized(base: u16, slots: Option<u16>) -> NormalizedResourceMetrics {
    NormalizedResourceMetrics::from_raw(
        RawResourceMetrics {
            cpu_utilization_percent: Some(f64::from(base) / 10.0),
            memory_total_bytes: Some(1_000),
            memory_used_bytes: Some(u64::from(base)),
            disk_total_bytes: Some(1_000),
            disk_used_bytes: Some(u64::from(base + 1)),
            gpu_utilization_percent: None,
            available_slots: slots,
        },
        64,
    )
    .unwrap()
}

fn signed(
    holder: &str,
    generation: u64,
    sequence: u64,
    action: WeatherAction,
    certificate: &AuthorityCertificate,
    signer: &SigningKey,
) -> SignedResourceWeather {
    let certificate_id = certificate.certificate_id().unwrap();
    SignedResourceWeather::sign(
        ResourceWeatherClaims::new(
            DOMAIN,
            holder,
            certificate_id,
            generation,
            sequence,
            ObservationWindow {
                sampled_at_unix_secs: NOW,
                expires_at_unix_secs: NOW + 30,
            },
            action,
        ),
        certificate.clone(),
        signer,
    )
    .unwrap()
}

#[test]
fn certificate_authorized_weather_is_durable_deterministic_and_advisory() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(1);
    let signer = key(2);
    let certificate = issue_certificate(
        &root,
        &signer,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW)
        .unwrap();
    let expected = {
        let weather = open_weather(&temp, 8);
        let advertisement = signed(
            "node-a",
            1,
            1,
            WeatherAction::Observe(normalized(400, Some(12))),
            &certificate,
            &signer,
        );
        assert!(advertisement
            .observation_id()
            .unwrap()
            .starts_with("weather_"));
        let projection = weather.admit(advertisement, &certificates, NOW).unwrap();
        assert_eq!(projection.availability, WeatherAvailability::Partial);
        assert_eq!(projection.pressure_permille, Some(401));
        assert_eq!(projection.available_slots, Some(12));
        assert!(projection.advisory_only);
        projection
    };
    let restarted = open_weather(&temp, 8);
    assert_eq!(restarted.database_path(), weather_path(&temp));
    assert_eq!(
        restarted
            .weather_for("node-a", &certificates, NOW + 1)
            .unwrap(),
        expected
    );
}

#[test]
fn domain_holder_certificate_and_signature_tampering_fail_closed() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(3);
    let signer = key(4);
    let certificate = issue_certificate(
        &root,
        &signer,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW)
        .unwrap();
    let weather = open_weather(&temp, 8);

    let mut wrong_domain = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &certificate,
        &signer,
    );
    wrong_domain.claims.trust_domain = "other.example".to_owned();
    assert_eq!(
        weather.admit(wrong_domain, &certificates, NOW).unwrap_err(),
        WeatherError::WrongTrustDomain
    );

    let mut wrong_holder = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &certificate,
        &signer,
    );
    wrong_holder.claims.holder_id = "node-b".to_owned();
    assert_eq!(
        weather.admit(wrong_holder, &certificates, NOW).unwrap_err(),
        WeatherError::CertificateMismatch
    );

    let mut tampered = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &certificate,
        &signer,
    );
    tampered.claims.sequence = 2;
    assert_eq!(
        weather.admit(tampered, &certificates, NOW).unwrap_err(),
        WeatherError::InvalidSignature
    );

    let mut malformed = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &certificate,
        &signer,
    );
    malformed.signature.truncate(63);
    assert_eq!(
        weather.admit(malformed, &certificates, NOW).unwrap_err(),
        WeatherError::MalformedSignature
    );
}

#[test]
fn wrong_purpose_revoked_and_unapproved_certificates_are_rejected() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(5);
    let signer = key(6);
    let advertisement_certificate = issue_certificate(
        &root,
        &signer,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let transport_certificate =
        issue_certificate(&root, &signer, "node-a", CertificatePurpose::Transport, 1);
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &advertisement_certificate,
            NOW,
        )
        .unwrap();
    let weather = open_weather(&temp, 8);

    let mut wrong_purpose = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &advertisement_certificate,
        &signer,
    );
    wrong_purpose.certificate = transport_certificate;
    assert_eq!(
        weather
            .admit(wrong_purpose, &certificates, NOW)
            .unwrap_err(),
        WeatherError::WrongCertificatePurpose
    );

    let revoked = signed(
        "node-a",
        1,
        1,
        WeatherAction::Observe(normalized(100, Some(1))),
        &advertisement_certificate,
        &signer,
    );
    certificates
        .revoke(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &advertisement_certificate.certificate_id().unwrap(),
            NOW,
            RevocationReason::OperatorRevoked,
        )
        .unwrap();
    assert_eq!(
        weather.admit(revoked, &certificates, NOW).unwrap_err(),
        WeatherError::CertificateRejected
    );
}

#[test]
fn freshness_lifetime_and_normalization_bounds_are_explicit() {
    let raw_unavailable = RawResourceMetrics {
        cpu_utilization_percent: None,
        memory_total_bytes: Some(0),
        memory_used_bytes: Some(0),
        disk_total_bytes: None,
        disk_used_bytes: None,
        gpu_utilization_percent: None,
        available_slots: None,
    };
    let unavailable = NormalizedResourceMetrics::from_raw(raw_unavailable, 64).unwrap();
    assert_eq!(
        unavailable.cpu_utilization_permille,
        MetricValue::Unavailable
    );
    assert_eq!(
        unavailable.memory_utilization_permille,
        MetricValue::Unavailable
    );
    assert_eq!(
        unavailable.disk_utilization_permille,
        MetricValue::Unavailable
    );
    assert_eq!(
        unavailable.gpu_utilization_permille,
        MetricValue::Unavailable
    );

    for invalid in [f64::NAN, f64::INFINITY, -1.0, 100.1] {
        assert_eq!(
            NormalizedResourceMetrics::from_raw(
                RawResourceMetrics {
                    cpu_utilization_percent: Some(invalid),
                    memory_total_bytes: None,
                    memory_used_bytes: None,
                    disk_total_bytes: None,
                    disk_used_bytes: None,
                    gpu_utilization_percent: None,
                    available_slots: None,
                },
                64,
            )
            .unwrap_err(),
            WeatherError::MetricOutOfBounds
        );
    }
    assert_eq!(
        NormalizedResourceMetrics::from_raw(
            RawResourceMetrics {
                cpu_utilization_percent: None,
                memory_total_bytes: Some(10),
                memory_used_bytes: Some(11),
                disk_total_bytes: None,
                disk_used_bytes: None,
                gpu_utilization_percent: None,
                available_slots: None,
            },
            64,
        )
        .unwrap_err(),
        WeatherError::MetricOutOfBounds
    );

    let temp = tempfile::tempdir().unwrap();
    let root = key(7);
    let signer = key(8);
    let certificate = issue_certificate(
        &root,
        &signer,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW)
        .unwrap();
    let weather = open_weather(&temp, 8);
    let base = ResourceWeatherClaims::new(
        DOMAIN,
        "node-a",
        certificate.certificate_id().unwrap(),
        1,
        1,
        ObservationWindow {
            sampled_at_unix_secs: NOW,
            expires_at_unix_secs: NOW + 30,
        },
        WeatherAction::Observe(normalized(100, Some(1))),
    );
    let mut future = base.clone();
    future.sampled_at_unix_secs = NOW + 3;
    future.expires_at_unix_secs = NOW + 30;
    let future = SignedResourceWeather::sign(future, certificate.clone(), &signer).unwrap();
    assert_eq!(
        weather.admit(future, &certificates, NOW).unwrap_err(),
        WeatherError::NotYetValid
    );
    let mut expired = base.clone();
    expired.sampled_at_unix_secs = NOW - 30;
    expired.expires_at_unix_secs = NOW;
    let expired = SignedResourceWeather::sign(expired, certificate.clone(), &signer).unwrap();
    assert_eq!(
        weather.admit(expired, &certificates, NOW).unwrap_err(),
        WeatherError::Expired
    );
    let mut overlong = base;
    overlong.expires_at_unix_secs = NOW + 61;
    let overlong = SignedResourceWeather::sign(overlong, certificate, &signer).unwrap();
    assert_eq!(
        weather.admit(overlong, &certificates, NOW).unwrap_err(),
        WeatherError::InvalidLifetime
    );
}

#[test]
fn certificate_window_revocation_rotation_and_refresh_bound_cached_weather() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(20);
    let signer_a = key(21);
    let signer_b = key(22);
    let certificate_a = issue_certificate(
        &root,
        &signer_a,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificate_b = issue_certificate(
        &root,
        &signer_b,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        2,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_a, NOW)
        .unwrap();
    let weather = open_weather(&temp, 8);
    weather
        .admit(
            signed(
                "node-a",
                1,
                1,
                WeatherAction::Observe(normalized(100, Some(1))),
                &certificate_a,
                &signer_a,
            ),
            &certificates,
            NOW,
        )
        .unwrap();
    assert_ne!(
        weather
            .weather_for("node-a", &certificates, NOW + 1)
            .unwrap(),
        PlacementWeather::no_data("node-a")
    );
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_b, NOW + 1)
        .unwrap();
    assert_eq!(
        weather
            .weather_for("node-a", &certificates, NOW + 2)
            .unwrap(),
        PlacementWeather::no_data("node-a")
    );

    let revoked_temp = tempfile::tempdir().unwrap();
    let revoked_certificates = open_certificates(&revoked_temp, &root);
    revoked_certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_a, NOW)
        .unwrap();
    let revoked_weather = open_weather(&revoked_temp, 8);
    revoked_weather
        .admit(
            signed(
                "node-a",
                1,
                1,
                WeatherAction::Observe(normalized(100, Some(1))),
                &certificate_a,
                &signer_a,
            ),
            &revoked_certificates,
            NOW,
        )
        .unwrap();
    revoked_certificates
        .revoke(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate_a.certificate_id().unwrap(),
            NOW + 1,
            RevocationReason::OperatorRevoked,
        )
        .unwrap();
    assert_eq!(
        revoked_weather
            .weather_for("node-a", &revoked_certificates, NOW + 1)
            .unwrap(),
        PlacementWeather::no_data("node-a")
    );

    let short_certificate = issue_certificate_with_validity(
        &root,
        &signer_a,
        "node-c",
        CertificatePurpose::AdvertisementSigning,
        1,
        NOW + 1,
        NOW + 20,
    );
    let short_temp = tempfile::tempdir().unwrap();
    let short_certificates = open_certificates(&short_temp, &root);
    short_certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &short_certificate, NOW + 1)
        .unwrap();
    let short_weather = open_weather(&short_temp, 8);
    let claims = ResourceWeatherClaims::new(
        DOMAIN,
        "node-c",
        short_certificate.certificate_id().unwrap(),
        1,
        1,
        ObservationWindow {
            sampled_at_unix_secs: NOW,
            expires_at_unix_secs: NOW + 21,
        },
        WeatherAction::Observe(normalized(100, Some(1))),
    );
    let outside_certificate =
        SignedResourceWeather::sign(claims, short_certificate, &signer_a).unwrap();
    assert_eq!(
        short_weather
            .admit(outside_certificate, &short_certificates, NOW + 1)
            .unwrap_err(),
        WeatherError::InvalidLifetime
    );
}

#[test]
fn resource_weather_policy_rejects_platform_maxima() {
    for bounds in [
        (MAX_OBSERVATION_LIFETIME_SECS + 1, 2, 4_096, 8, 64),
        (60, MAX_FUTURE_SKEW_SECS + 1, 4_096, 8, 64),
        (60, 2, MAX_PAYLOAD_BYTES + 1, 8, 64),
        (60, 2, 4_096, MAX_HOLDERS + 1, 64),
        (60, 2, 4_096, 8, MAX_AVAILABLE_SLOTS + 1),
    ] {
        assert!(matches!(
            ResourceWeatherPolicy::new(DOMAIN)
                .unwrap()
                .with_bounds(bounds.0, bounds.1, bounds.2, bounds.3, bounds.4),
            Err(WeatherError::ResourceExhausted)
        ));
    }
}

#[test]
fn replay_floor_survives_restart_rotation_and_invalid_high_sequence() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(9);
    let signer_a = key(10);
    let signer_b = key(11);
    let certificate_a = issue_certificate(
        &root,
        &signer_a,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificate_b = issue_certificate(
        &root,
        &signer_b,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        2,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_a, NOW)
        .unwrap();
    {
        let weather = open_weather(&temp, 8);
        weather
            .admit(
                signed(
                    "node-a",
                    1,
                    1,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate_a,
                    &signer_a,
                ),
                &certificates,
                NOW,
            )
            .unwrap();
        let mut invalid_high = signed(
            "node-a",
            1,
            99,
            WeatherAction::Observe(normalized(100, Some(1))),
            &certificate_a,
            &signer_a,
        );
        invalid_high.signature[0] ^= 1;
        assert_eq!(
            weather.admit(invalid_high, &certificates, NOW).unwrap_err(),
            WeatherError::InvalidSignature
        );
    }
    let weather = open_weather(&temp, 8);
    weather
        .admit(
            signed(
                "node-a",
                1,
                2,
                WeatherAction::Observe(normalized(200, Some(2))),
                &certificate_a,
                &signer_a,
            ),
            &certificates,
            NOW,
        )
        .unwrap();
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_b, NOW + 1)
        .unwrap();
    weather
        .admit(
            signed(
                "node-a",
                2,
                1,
                WeatherAction::Observe(normalized(300, Some(3))),
                &certificate_b,
                &signer_b,
            ),
            &certificates,
            NOW + 1,
        )
        .unwrap();
    assert_eq!(
        weather
            .admit(
                signed(
                    "node-a",
                    1,
                    3,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate_a,
                    &signer_a,
                ),
                &certificates,
                NOW + 1,
            )
            .unwrap_err(),
        WeatherError::ReplayRefused
    );
}

#[test]
fn signed_withdrawal_is_durable_no_data_and_cannot_be_replayed() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(12);
    let signer = key(13);
    let certificate = issue_certificate(
        &root,
        &signer,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW)
        .unwrap();
    {
        let weather = open_weather(&temp, 8);
        weather
            .admit(
                signed(
                    "node-a",
                    1,
                    1,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate,
                    &signer,
                ),
                &certificates,
                NOW,
            )
            .unwrap();
        let withdrawn = weather
            .admit(
                signed(
                    "node-a",
                    1,
                    2,
                    WeatherAction::Withdraw,
                    &certificate,
                    &signer,
                ),
                &certificates,
                NOW,
            )
            .unwrap();
        assert_eq!(withdrawn, PlacementWeather::no_data("node-a"));
    }
    let weather = open_weather(&temp, 8);
    assert_eq!(
        weather.weather_for("node-a", &certificates, NOW).unwrap(),
        PlacementWeather::no_data("node-a")
    );
    assert_eq!(
        weather
            .admit(
                signed(
                    "node-a",
                    1,
                    1,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate,
                    &signer,
                ),
                &certificates,
                NOW,
            )
            .unwrap_err(),
        WeatherError::ReplayRefused
    );
}

#[test]
fn holder_capacity_payload_and_durable_corruption_fail_closed() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(14);
    let signer_a = key(15);
    let signer_b = key(16);
    let certificate_a = issue_certificate(
        &root,
        &signer_a,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificate_b = issue_certificate(
        &root,
        &signer_b,
        "node-b",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_a, NOW)
        .unwrap();
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_b, NOW)
        .unwrap();
    {
        let weather = open_weather(&temp, 1);
        weather
            .admit(
                signed(
                    "node-a",
                    1,
                    1,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate_a,
                    &signer_a,
                ),
                &certificates,
                NOW,
            )
            .unwrap();
        assert_eq!(
            weather
                .admit(
                    signed(
                        "node-b",
                        1,
                        1,
                        WeatherAction::Observe(normalized(100, Some(1))),
                        &certificate_b,
                        &signer_b,
                    ),
                    &certificates,
                    NOW,
                )
                .unwrap_err(),
            WeatherError::ResourceExhausted
        );
    }

    let tiny = ResourceWeatherPolicy::new(DOMAIN)
        .unwrap()
        .with_bounds(60, 2, 32, 8, 64)
        .unwrap();
    let tiny_path = temp.path().canonicalize().unwrap().join("tiny.redb");
    let tiny_store = ResourceWeatherStore::open(&tiny_path, tiny).unwrap();
    assert_eq!(
        tiny_store
            .admit(
                signed(
                    "node-a",
                    1,
                    2,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    &certificate_a,
                    &signer_a,
                ),
                &certificates,
                NOW,
            )
            .unwrap_err(),
        WeatherError::PayloadTooLarge
    );

    let corrupt_path = temp.path().canonicalize().unwrap().join("corrupt.redb");
    {
        let database = Database::create(&corrupt_path).unwrap();
        let write = database.begin_write().unwrap();
        let mut table = write
            .open_table(TableDefinition::<&str, &[u8]>::new(
                "distributed_resource_weather_v1",
            ))
            .unwrap();
        table.insert("node-a", b"not-json".as_slice()).unwrap();
        drop(table);
        write.commit().unwrap();
    }
    match ResourceWeatherStore::open(&corrupt_path, weather_policy(8)) {
        Ok(_) => panic!("corrupt durable weather unexpectedly opened"),
        Err(error) => assert_eq!(error, WeatherError::DurableStateCorrupt),
    }

    for (name, mutation) in [
        ("claims", "claims"),
        ("signature", "signature"),
        ("digest", "digest"),
        ("null-withdrawal", "null-withdrawal"),
        ("forged-high-withdrawal", "forged-high-withdrawal"),
    ] {
        let path = temp
            .path()
            .canonicalize()
            .unwrap()
            .join(format!("semantic-{name}.redb"));
        {
            let weather = ResourceWeatherStore::open(&path, weather_policy(8)).unwrap();
            weather
                .admit(
                    signed(
                        "node-a",
                        1,
                        3,
                        WeatherAction::Observe(normalized(100, Some(1))),
                        &certificate_a,
                        &signer_a,
                    ),
                    &certificates,
                    NOW,
                )
                .unwrap();
        }
        mutate_durable_record(&path, |record| match mutation {
            "claims" => record["advertisement"]["claims"]["sequence"] = 999.into(),
            "signature" => record["advertisement"]["signature"][0] = 255.into(),
            "digest" => record["advertisement_digest"] = "weather_tampered".into(),
            "null-withdrawal" => record["advertisement"] = serde_json::Value::Null,
            "forged-high-withdrawal" => {
                record["sequence"] = 999.into();
                record["advertisement"]["claims"]["sequence"] = 999.into();
                record["advertisement"]["claims"]["action"] =
                    serde_json::json!({"action": "withdraw"});
            }
            _ => unreachable!(),
        });
        match ResourceWeatherStore::open(&path, weather_policy(8)) {
            Ok(_) => panic!("semantic durable corruption unexpectedly opened"),
            Err(error) => assert_eq!(error, WeatherError::DurableStateCorrupt),
        }
    }
}

#[test]
fn snapshots_are_sorted_and_serialized_surfaces_are_redacted() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(17);
    let signer_a = key(18);
    let signer_b = key(19);
    let certificate_a = issue_certificate(
        &root,
        &signer_a,
        "node-a",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificate_b = issue_certificate(
        &root,
        &signer_b,
        "node-b",
        CertificatePurpose::AdvertisementSigning,
        1,
    );
    let certificates = open_certificates(&temp, &root);
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_a, NOW)
        .unwrap();
    certificates
        .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate_b, NOW)
        .unwrap();
    let weather = open_weather(&temp, 8);
    for (holder, signer, certificate) in [
        ("node-b", &signer_b, &certificate_b),
        ("node-a", &signer_a, &certificate_a),
    ] {
        weather
            .admit(
                signed(
                    holder,
                    1,
                    1,
                    WeatherAction::Observe(normalized(100, Some(1))),
                    certificate,
                    signer,
                ),
                &certificates,
                NOW,
            )
            .unwrap();
    }
    let snapshot = weather.snapshot(&certificates, NOW).unwrap();
    assert_eq!(snapshot[0].holder_id, "node-a");
    assert_eq!(snapshot[1].holder_id, "node-b");
    let encoded = serde_json::to_string(&snapshot).unwrap();
    for forbidden in [
        "mount_point",
        "provider",
        "reason_code",
        "cloudwatch",
        "private_key",
    ] {
        assert!(!encoded.contains(forbidden));
    }
    assert_eq!(
        WeatherError::CertificateRejected.to_string(),
        "certificate_rejected"
    );
    assert_eq!(
        RESOURCE_WEATHER_SCHEMA,
        "adl.distributed.resource_weather.v1"
    );
}
