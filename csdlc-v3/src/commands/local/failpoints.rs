//! Test-only crash injection for native local mutation boundaries.

pub(super) fn local_transaction_failpoint(name: &str) {
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref() == Ok(name) {
        std::process::exit(91);
    }
}
