//! PVF: installed deterministic fixture attempts, local Git/CPU/disk, required
//! SIM baseline input. Subsequent remote/proof/terminal scenarios share recorder.
use super::attempt_corpus::{Corpus, FixtureClock};
use super::*;

fn corpus_file(name: &str) -> PathBuf {
    repo_root()
        .join("csdlc-v3/target/sim01-corpus")
        .join(std::process::id().to_string())
        .join(name)
}

// PVF: measurement reliability only; these cases do not prove CLI semantics.
#[test]
fn attempt_recorder_retains_spawn_parse_and_collector_failures() {
    use super::attempt_corpus::{Clock, ClockSample};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    let fixture = operational_fixture("attempt-retention");
    let output = corpus_file("retention-negative.json");
    let index = fixture.root.join("absent-index");
    let binary = Path::new(env!("CARGO_BIN_EXE_csdlc"));
    let mut corpus = Corpus::new(
        &fixture.root,
        binary,
        &repo_root(),
        FixtureClock(0),
        &output,
    );
    assert!(catch_unwind(AssertUnwindSafe(|| {
        corpus.run(
            ("spawn", "missing", "guard"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            &mut Command::new("/nonexistent-sim01-command"),
        );
    }))
    .is_err());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(saved["attempts"][0]["outcome"], "failed");
    assert_eq!(saved["denominators"]["attempted"], 1);
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let result = corpus.run(
            ("parse", "printf", "malformed_output"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            Command::new("/usr/bin/printf").arg("not-json"),
        );
        serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap();
    }))
    .is_err());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(saved["attempts"][1]["outcome"], "failed");
    assert_eq!(saved["attempts"][1]["stdout_text"], "not-json");
    struct InterruptedCollector(bool);
    impl Clock for InterruptedCollector {
        fn sample(&mut self) -> ClockSample {
            assert!(!self.0, "injected collection interruption");
            self.0 = true;
            ClockSample {
                wall_unix_millis: 100,
                monotonic_millis: 0,
            }
        }
    }
    let interrupted = output.with_file_name("retention-censored.json");
    let mut corpus = Corpus::new(
        &fixture.root,
        binary,
        &repo_root(),
        InterruptedCollector(false),
        &interrupted,
    );
    assert!(catch_unwind(AssertUnwindSafe(|| {
        corpus.run(
            ("collector", "printf", "collection_interrupted"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            Command::new("/usr/bin/printf").arg("{}"),
        );
    }))
    .is_err());
    let saved: serde_json::Value =
        serde_json::from_slice(&fs::read(&interrupted).unwrap()).unwrap();
    assert_eq!(saved["denominators"]["censored"], 1);
    assert_eq!(saved["attempts"][0]["outcome"], "censored");
}

// Current installed lifecycle journeys are covered by the semantic intent suites.
// This module retains only corpus-recorder failure accounting; the retired direct
// writer baseline must not be used as current execution evidence.
