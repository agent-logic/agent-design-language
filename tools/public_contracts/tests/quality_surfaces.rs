// PVF: deterministic local retained-document contract. Synthetic positive
// fixture does not establish current route availability or lifecycle acceptance.
use adl_public_contracts::{quality_surfaces as q, root};
use std::{fs, process::Command};
#[test]
fn original_two_passes_and_existing_map_refusal_are_preserved() {
    q::check(&root(), "quality_gate_doc").unwrap();
    q::check(&root(), "quality_gate_packet").unwrap();
    assert!(q::check(&root(), "demo_coverage")
        .unwrap_err()
        .to_string()
        .contains("Card Lifecycle Integration"));
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .args([
            "quality-review-surfaces",
            root().to_str().unwrap(),
            "quality_gate_doc",
        ])
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(q::check(&root(), "unknown").is_err());
}
#[test]
fn synthetic_surface_markers_links_routes_and_verdict_are_required() {
    let temp = tempfile::tempdir().unwrap();
    let docs = temp.path().join("docs/milestones/v0.91.3");
    for (_, path, linked, markers) in q::SURFACES {
        for p in [path, linked] {
            fs::create_dir_all(docs.join(p).parent().unwrap()).unwrap();
            if !docs.join(p).exists() {
                fs::write(docs.join(p), "").unwrap();
            }
        }
        fs::write(docs.join(path), markers.join("\n")).unwrap();
    }
    let map = docs.join(q::SURFACES[2].2);
    let content = q::ROUTES
        .iter()
        .map(|(a, b)| format!("{a}\n{b}"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\nMissing feature with no truthful demo/proof route: none found";
    fs::write(&map, &content).unwrap();
    for (surface, path, linked, markers) in q::SURFACES {
        q::check(temp.path(), surface).unwrap();
        let original = fs::read_to_string(docs.join(path)).unwrap();
        for marker in *markers {
            fs::write(docs.join(path), original.replace(marker, "REMOVED")).unwrap();
            assert!(q::check(temp.path(), surface).is_err());
        }
        fs::write(docs.join(path), original).unwrap();
        let original = fs::read(docs.join(linked)).unwrap();
        fs::remove_file(docs.join(linked)).unwrap();
        assert!(q::check(temp.path(), surface).is_err());
        fs::write(docs.join(linked), original).unwrap();
    }
    for marker in q::ROUTES
        .iter()
        .flat_map(|(a, b)| [*a, *b])
        .chain(["Missing feature with no truthful demo/proof route: none found"])
    {
        fs::write(&map, content.replace(marker, "REMOVED")).unwrap();
        assert!(q::check(temp.path(), "demo_coverage").is_err());
    }
}
