#!/usr/bin/env node

const args = process.argv.slice(2);
const artifactsOnly = args.length === 1 && args[0] === "--artifacts-only";
const normalMode = args.length === 0;

if (!normalMode && !artifactsOnly) {
  console.error("IMPLEMENTATION REQUIRED: #117 Observatory hardening validator supports only normal or --artifacts-only mode");
  process.exit(2);
}

const selectedAssertions = artifactsOnly
  ? [
      "operator-runbook",
      "exact-artifact-index",
      "candidate-revision",
      "desktop-mobile-screenshots",
      "machine-readable-results",
      "recovery-rollback-evidence",
      "redaction-inventory",
      "independent-review-receipts",
    ]
  : [
      "runtime-backed-roster",
      "one-to-one-chat",
      "durable-history",
      "governed-rooms",
      "attention-inbox",
      "accessibility-responsive-browser",
      "adversarial-security",
      "resilience-resource-bounds",
    ];

const mode = artifactsOnly ? "artifacts-review-receipts" : "integrated-browser-hardening";
console.error(`#117 Observatory hardening sentinel: mode=${mode} selected_assertions=${selectedAssertions.length}`);
console.error(`Selected assertions: ${selectedAssertions.join(", ")}`);
console.error("IMPLEMENTATION REQUIRED: #117 Observatory hardening proof is not implemented; this sentinel cannot produce acceptance evidence");
process.exit(1);
