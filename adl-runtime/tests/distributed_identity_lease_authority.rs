use std::{
    collections::BTreeMap,
    fs,
    process::Command,
    sync::{Arc, Mutex},
};

use adl_runtime::distributed::{
    authority_reconciliation::{AuthorityReconciliationBarrier, AuthorityReconciliationIdentity},
    authority_store_adapters::{AuthorityStoreAdapterError, AuthorityStoreAdapterRegistry},
    certificates::{
        CertificateError, CertificatePolicy, CertificateStoreAccess, DistributedCertificateStore,
    },
    fencing::{
        FencingCheckpoint, FencingCheckpointAuthority, FencingError, FencingPolicy, FencingStore,
        FencingStoreAccess,
    },
    lease::{AuthorityError, AuthorityLedger, LeasePolicy, LeaseStoreAccess},
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
};
use ed25519_dalek::SigningKey;

// PVF: lane=identity-lease-fencing-authority-boundary; proof=#258 authority-store-boundary
// security-boundary guardrail; deterministic=true; resource_profile=small;
// release_gate=false.

#[derive(Default)]
struct MemoryCheckpoint {
    values: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
}

impl ConsensusCheckpointAuthority for MemoryCheckpoint {
    fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
        Ok(self.values.lock().unwrap().get(object).cloned())
    }

    fn compare_and_swap(
        &self,
        expected: Option<&ConsensusCheckpoint>,
        candidate: &ConsensusCheckpoint,
    ) -> Result<(), PolisRuntimeError> {
        let mut values = self.values.lock().unwrap();
        if values.get(&candidate.object) != expected {
            return Err(PolisRuntimeError::StateRegression);
        }
        values.insert(candidate.object.clone(), candidate.clone());
        Ok(())
    }
}

#[derive(Debug, Default)]
struct RejectUnusedCheckpoint;

impl FencingCheckpointAuthority for RejectUnusedCheckpoint {
    fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
        Ok(None)
    }

    fn compare_and_swap(
        &self,
        _expected: Option<FencingCheckpoint>,
        _next: FencingCheckpoint,
    ) -> Result<(), FencingError> {
        panic!("forged fencing access must be rejected before checkpoint mutation")
    }
}

fn identity() -> AuthorityReconciliationIdentity {
    AuthorityReconciliationIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-a".to_owned(),
        guardian_id: "guardian-a".to_owned(),
        boot_generation: 7,
        protocol_instance: "adl.authority-reconciliation.v1".to_owned(),
    }
}

fn repo_local_root() -> tempfile::TempDir {
    let root = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .expect("current test directory must have a canonical symlink-free path");
    tempfile::TempDir::new_in(root).expect("portable repository-local test root")
}

fn newest_file_modification(root: &std::path::Path) -> std::time::SystemTime {
    fs::read_dir(root)
        .expect("read current crate source tree")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .map(|path| {
            if path.is_dir() {
                newest_file_modification(&path)
            } else {
                path.metadata()
                    .expect("read current crate input metadata")
                    .modified()
                    .expect("read current crate input modification time")
            }
        })
        .max()
        .expect("current crate source tree must not be empty")
}

fn run_external_no_feature_fixture(
    fixture: &tempfile::TempDir,
    name: &str,
    source: String,
) -> std::process::Output {
    let current_test_binary =
        std::env::current_exe().expect("resolve current test binary for external fixture");
    let deps_dir = current_test_binary
        .parent()
        .expect("current test binary must live under target deps");
    let fingerprint_root = deps_dir
        .parent()
        .expect("target debug directory must exist")
        .join(".fingerprint");
    let no_feature_rlib = fs::read_dir(&fingerprint_root)
        .expect("read Cargo fingerprints for external compile-fail fixture")
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let metadata_path = entry.path().join("lib-adl_runtime.json");
            let metadata = fs::read_to_string(metadata_path).ok()?;
            if !metadata.contains("\"features\":\"[]\"")
                || !metadata.contains("internal-test-fixtures")
            {
                return None;
            }
            let fingerprint = entry.file_name();
            let fingerprint = fingerprint.to_str()?.strip_prefix("adl-runtime-")?;
            let rlib = deps_dir.join(format!("libadl_runtime-{fingerprint}.rlib"));
            let modified = rlib.metadata().ok()?.modified().ok()?;
            Some((modified, rlib))
        })
        .max_by_key(|(modified, _)| *modified)
        .map(|(_, rlib)| rlib)
        .expect("a current no-feature adl_runtime rlib must exist");
    let rlib_modified = no_feature_rlib
        .metadata()
        .expect("read no-feature rlib metadata")
        .modified()
        .expect("read no-feature rlib modification time");
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let newest_input = manifest_dir
        .join("Cargo.toml")
        .metadata()
        .expect("read current manifest metadata")
        .modified()
        .expect("read current manifest modification time")
        .max(newest_file_modification(&manifest_dir.join("src")));
    assert!(
        rlib_modified >= newest_input,
        "selected no-feature adl_runtime rlib predates current crate inputs"
    );
    let source_path = fixture.path().join(format!("{name}.rs"));
    fs::write(&source_path, source).expect("write external fixture source");
    Command::new("rustc")
        .arg("--edition=2021")
        .arg("--crate-type=lib")
        .arg(&source_path)
        .arg("--extern")
        .arg(format!("adl_runtime={}", no_feature_rlib.display()))
        .arg("-L")
        .arg(format!("dependency={}", deps_dir.display()))
        .arg("--out-dir")
        .arg(fixture.path())
        .output()
        .expect("run external no-feature rustc fixture")
}

fn assert_contains(source: &str, needle: &str) {
    assert!(
        source.contains(needle),
        "source guard missing expected snippet: {needle}"
    );
}

fn assert_not_contains(source: &str, needle: &str) {
    assert!(
        !source.contains(needle),
        "source guard found forbidden snippet: {needle}"
    );
}

#[test]
fn issue_258_authority_store_boundary_guardrails_are_bound() {
    let adapters = include_str!("../src/distributed/authority_store_adapters.rs");
    let certificates = include_str!("../src/distributed/certificates.rs");
    let lease = include_str!("../src/distributed/lease.rs");
    let fencing = include_str!("../src/distributed/fencing.rs");

    assert_contains(adapters, "pub struct AuthorityStoreAdapterRegistry");
    assert_contains(adapters, "pub struct PublishedStoreAuthorityReceiptView");
    assert_contains(adapters, "pub struct AuthorityBoundCertificateStore");
    assert_contains(adapters, "pub struct AuthorityBoundLeaseLedger");
    assert_contains(adapters, "pub struct AuthorityBoundFencingStore");
    assert_contains(adapters, "action_class: String");
    assert_contains(adapters, "adapter_kind: String");
    assert_contains(adapters, "adapter_version: u32");
    assert_contains(adapters, "receipt_sha256: [u8; 32]");
    assert_contains(
        adapters,
        "published_view_action_class(result.mutation_kind())",
    );
    assert_contains(adapters, "PUBLISHED_VIEW_ACTION_OWNER_COMMIT");
    assert_contains(adapters, "receipt_sha256: result.receipts_sha256()");
    assert_contains(
        adapters,
        "validate_permit(&permit, &AuthorityPermitAction::Read)",
    );
    assert_contains(
        adapters,
        "AuthorityPermitAction::Mutation(mutation_kind.to_owned())",
    );

    assert_contains(certificates, "pub struct CertificateStoreAccess");
    assert_contains(certificates, "struct CertificateStoreAccessSeal");
    assert_contains(certificates, "static AUTHORITY_BOUND_SEAL");
    assert_contains(certificates, "seal: &'static CertificateStoreAccessSeal");
    assert_contains(
        certificates,
        "std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)",
    );
    assert_contains(
        certificates,
        "fn validate_raw_access(access: &CertificateStoreAccess)",
    );
    assert_contains(
        certificates,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_CERTIFICATE_ACCESS",
    );
    assert_contains(
        certificates,
        "#[cfg(test)]\n#[allow(unused_imports)]\npub(crate) use raw_access::TEST_FIXTURE as TEST_CERTIFICATE_STORE_ACCESS;",
    );
    assert_contains(
        certificates,
        "#[cfg(all(not(test), feature = \"internal-test-fixtures\"))]\n#[doc(hidden)]\n#[allow(unused_imports)]\npub use raw_access::TEST_FIXTURE as TEST_CERTIFICATE_STORE_ACCESS;",
    );
    assert_contains(
        certificates,
        "pub fn open(\n        access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn activate(\n        &self,\n        access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn authorize(\n        &self,\n        access: &CertificateStoreAccess,",
    );
    assert_contains(
        certificates,
        "pub fn revoke(\n        &self,\n        access: &CertificateStoreAccess,",
    );

    assert_contains(lease, "pub struct LeaseState");
    assert_contains(lease, "pub struct LeaseStoreAccess");
    assert_contains(lease, "struct LeaseStoreAccessSeal");
    assert_contains(lease, "static AUTHORITY_BOUND_SEAL");
    assert_contains(lease, "seal: &'static LeaseStoreAccessSeal");
    assert_contains(lease, "std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)");
    assert_contains(lease, "fn validate_raw_access(access: &LeaseStoreAccess)");
    assert_contains(
        lease,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_LEASE_ACCESS",
    );
    assert_contains(
        lease,
        "#[cfg(test)]\n#[allow(unused_imports)]\npub(crate) use raw_access::TEST_FIXTURE as TEST_LEASE_STORE_ACCESS;",
    );
    assert_contains(
        lease,
        "#[cfg(all(not(test), feature = \"internal-test-fixtures\"))]\n#[doc(hidden)]\n#[allow(unused_imports)]\npub use raw_access::TEST_FIXTURE as TEST_LEASE_STORE_ACCESS;",
    );
    assert_contains(
        lease,
        "pub fn new(access: &LeaseStoreAccess, policy: LeasePolicy)",
    );
    assert_contains(
        lease,
        "pub fn apply(\n        &mut self,\n        access: &LeaseStoreAccess,",
    );
    assert_contains(
        lease,
        "pub fn authorize_mutation(\n        &mut self,\n        access: &LeaseStoreAccess,",
    );
    assert_contains(fencing, "pub safety_deadline_unix_millis: u64");
    assert_contains(fencing, "pub struct FencingStoreAccess");
    assert_contains(fencing, "struct FencingStoreAccessSeal");
    assert_contains(fencing, "static AUTHORITY_BOUND_SEAL");
    assert_contains(fencing, "seal: &'static FencingStoreAccessSeal");
    assert_contains(fencing, "std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)");
    assert_contains(
        fencing,
        "fn validate_raw_access(access: &FencingStoreAccess)",
    );
    assert_contains(
        fencing,
        "pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_FENCING_ACCESS",
    );
    assert_contains(
        fencing,
        "pub fn create(\n        access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn open(\n        access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn commit(\n        &mut self,\n        access: &FencingStoreAccess,",
    );
    assert_contains(
        fencing,
        "pub fn authorize_active_lease(\n        &self,\n        access: &FencingStoreAccess,",
    );
    assert_not_contains(
        fencing,
        "pub fn authorize_active_lease(&self, check: ActiveLeaseCheck",
    );
    assert_not_contains(
        fencing,
        "#[cfg(debug_assertions)]\n    #[doc(hidden)]\n    pub const TEST_FIXTURE",
    );
    assert_not_contains(
        fencing,
        "#[cfg(debug_assertions)]\n#[allow(unused_imports)]\npub use raw_access::TEST_FIXTURE as TEST_FENCING_STORE_ACCESS;",
    );
    assert_contains(
        fencing,
        "#[cfg(test)]\n#[allow(unused_imports)]\npub(crate) use raw_access::TEST_FIXTURE as TEST_FENCING_STORE_ACCESS;",
    );
    assert_contains(
        fencing,
        "#[cfg(all(not(test), feature = \"internal-test-fixtures\"))]\n#[doc(hidden)]\n#[allow(unused_imports)]\npub use raw_access::TEST_FIXTURE as TEST_FENCING_STORE_ACCESS;",
    );
    assert_contains(
        adapters,
        ".authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)",
    );
}

#[test]
fn external_dev_profile_caller_cannot_import_authority_store_test_access() {
    let fixture = repo_local_root();
    for (module, token, access_type) in [
        (
            "certificates",
            "TEST_CERTIFICATE_STORE_ACCESS",
            "CertificateStoreAccess",
        ),
        ("lease", "TEST_LEASE_STORE_ACCESS", "LeaseStoreAccess"),
        ("fencing", "TEST_FENCING_STORE_ACCESS", "FencingStoreAccess"),
    ] {
        let output = run_external_no_feature_fixture(
            &fixture,
            &format!("{module}_token_import_denied"),
            format!(
                "use adl_runtime::distributed::{module}::{token};\n\
                 pub fn leaked_token() {{ let _ = {token}; }}\n"
            ),
        );
        assert!(
            !output.status.success(),
            "external fixture unexpectedly imported {module} test access token"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&format!("no `{token}`"))
                || stderr.contains(&format!(
                    "not found in `adl_runtime::distributed::{module}`"
                ))
                || stderr.contains("private"),
            "unexpected compile failure for {module} token import: {stderr}"
        );

        let output = run_external_no_feature_fixture(
            &fixture,
            &format!("{module}_external_construct_denied"),
            format!(
                "use adl_runtime::distributed::{module}::{access_type};\n\
                 pub unsafe fn constructed() -> {access_type} {{ {access_type} {{ seal: std::mem::zeroed() }} }}\n"
            ),
        );
        assert!(
            !output.status.success(),
            "external fixture unexpectedly constructed {module} access token"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("field `seal`")
                && (stderr.contains("private") || stderr.contains("is private")),
            "unexpected compile failure for {module} external construction: {stderr}"
        );

        let output = run_external_no_feature_fixture(
            &fixture,
            &format!("{module}_fat_trait_forge_denied"),
            format!(
                "use adl_runtime::distributed::{module}::{access_type};\n\
                 trait ForgedSeal {{ fn magic(&self) -> [u8; 32]; }}\n\
                 struct Forgery;\n\
                 impl ForgedSeal for Forgery {{ fn magic(&self) -> [u8; 32] {{ [0; 32] }} }}\n\
                 static FORGERY: Forgery = Forgery;\n\
                 pub unsafe fn forged_fat_pointer() -> {access_type} {{\n\
                     let forged: &'static dyn ForgedSeal = &FORGERY;\n\
                     std::mem::transmute(forged)\n\
                 }}\n"
            ),
        );
        assert!(
            !output.status.success(),
            "external fixture unexpectedly forged {module} access token with a fat trait pointer"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("different sizes") || stderr.contains("cannot transmute"),
            "unexpected compile failure for {module} fat-pointer forgery: {stderr}"
        );

        let output = run_external_no_feature_fixture(
            &fixture,
            &format!("{module}_unit_forge_denied"),
            format!(
                "use adl_runtime::distributed::{module}::{access_type};\n\
                 pub unsafe fn forged_unit() -> {access_type} {{ std::mem::transmute(()) }}\n"
            ),
        );
        assert!(
            !output.status.success(),
            "external fixture unexpectedly forged {module} access token with unit transmute"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("cannot transmute between types of different sizes"),
            "unexpected compile failure for {module} unit transmute: {stderr}"
        );

        let output = run_external_no_feature_fixture(
            &fixture,
            &format!("{module}_zeroed_forge_denied"),
            format!(
                "#![deny(invalid_value)]\n\
                 use adl_runtime::distributed::{module}::{access_type};\n\
                 pub unsafe fn forged_zeroed() -> {access_type} {{ std::mem::MaybeUninit::<{access_type}>::zeroed().assume_init() }}\n"
            ),
        );
        assert!(
            !output.status.success(),
            "external fixture unexpectedly forged {module} access token with zeroed MaybeUninit"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("does not permit zero-initialization")
                || stderr.contains("the type")
                    && stderr.contains("does not permit being left uninitialized"),
            "unexpected compile failure for {module} zeroed forge: {stderr}"
        );
    }
}

#[test]
fn copied_magic_thin_pointer_cannot_authorize_raw_store_entrypoints() {
    static CERTIFICATE_MAGIC_COPY: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x43, 0x45, 0x52, 0x54, 0x2d, 0x53, 0x54, 0x4f, 0x52, 0x45, 0x2d,
        0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x53, 0x45, 0x41, 0x4c, 0x2d, 0x56, 0x31, 0x21,
        0x02, 0x58,
    ];
    static LEASE_MAGIC_COPY: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x4c, 0x45, 0x41, 0x53, 0x45, 0x2d, 0x53, 0x54, 0x4f, 0x52, 0x45,
        0x2d, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x56, 0x31, 0x2d, 0x53, 0x45, 0x41, 0x4c,
        0x03, 0x59,
    ];
    static FENCING_MAGIC_COPY: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x46, 0x45, 0x4e, 0x43, 0x49, 0x4e, 0x47, 0x2d, 0x53, 0x54, 0x4f,
        0x52, 0x45, 0x2d, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x56, 0x31, 0x2d, 0x53, 0x45,
        0x04, 0x5a,
    ];
    let certificate_access = unsafe {
        std::mem::transmute::<&'static [u8; 32], CertificateStoreAccess>(&CERTIFICATE_MAGIC_COPY)
    };
    let lease_access =
        unsafe { std::mem::transmute::<&'static [u8; 32], LeaseStoreAccess>(&LEASE_MAGIC_COPY) };
    let fencing_access = unsafe {
        std::mem::transmute::<&'static [u8; 32], FencingStoreAccess>(&FENCING_MAGIC_COPY)
    };

    let directory = repo_local_root();
    let certificate_policy = CertificatePolicy::new(
        "runtime-prod",
        [SigningKey::from_bytes(&[29; 32]).verifying_key()],
    )
    .unwrap();
    assert!(matches!(
        DistributedCertificateStore::open(
            &certificate_access,
            directory.path().join("forged-certificates.redb"),
            certificate_policy,
        ),
        Err(CertificateError::IssuerNotApproved)
    ));
    assert!(matches!(
        AuthorityLedger::new(
            &lease_access,
            LeasePolicy {
                max_lease_duration_millis: 60_000,
                max_clock_uncertainty_millis: 100,
                message_delay_margin_millis: 100,
                max_lineages: 16,
                max_snapshot_bytes: 4096,
            },
        ),
        Err(AuthorityError::CertificateUnauthorized)
    ));
    assert!(matches!(
        FencingStore::create(
            &fencing_access,
            directory.path().join("forged-fencing"),
            FencingPolicy {
                max_lineages: 16,
                max_receipts: 32,
                max_state_bytes: 4096,
                max_clock_uncertainty_millis: 100,
                message_delay_margin_millis: 100,
            },
            Arc::new(RejectUnusedCheckpoint),
        ),
        Err(FencingError::UnauthorizedOperation)
    ));
}

#[test]
fn authority_store_adapter_denies_unpublished_lineage() {
    let root = repo_local_root();
    let barrier = AuthorityReconciliationBarrier::open(
        root.path(),
        identity(),
        Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    let registry = AuthorityStoreAdapterRegistry::new(Arc::new(barrier));

    assert!(matches!(
        registry.published_view("lineage-a"),
        Err(AuthorityStoreAdapterError::Reconciliation(
            adl_runtime::distributed::authority_reconciliation::AuthorityReconciliationError::ReconciliationRequired
        ))
    ));

    println!("ADL_ISSUE_258_ADAPTER_GUARD_V1 unpublished_lineage_denied");
}
