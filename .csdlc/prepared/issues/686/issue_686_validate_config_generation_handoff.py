#!/usr/bin/env python3
"""Focused static denominator for issue #686 configuration-generation handoff."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]


def require(path: str, needles: tuple[str, ...]) -> None:
    text = (ROOT / path).read_text(encoding="utf-8")
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(f"{path}: missing required #686 contract tokens: {missing}")


def main() -> None:
    require(
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        ("config_generation", "config_receipt_digest"),
    )
    require(
        "adl/tests/csm_runtime_v3_generation.rs",
        (
            "pre_activation",
            "post_pointer",
            "candidate_ready",
            "prior_generation",
        ),
    )
    print("issue #686 configuration-generation handoff denominator: PASS")


if __name__ == "__main__":
    main()
