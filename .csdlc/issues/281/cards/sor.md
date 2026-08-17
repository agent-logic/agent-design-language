# Structured Output Record

Template: 1.0.0

Issue: 281

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic HTML Observatory security/privacy/adversarial proof without changing Runtime authority or reading credentials, private cognition, raw provider payloads, cloud, or Unity state.

## Artifacts

- demos/html-observatory/tests/security_privacy_adversarial.test.mjs
- .csdlc/evidence/281/security_privacy_adversarial.json
- .csdlc/evidence/281/281-preparation-contract.log
- .csdlc/evidence/281/281-observatory-security-privacy-adversarial.log
- .csdlc/evidence/281/281-observatory-conversation-regression.log
- .csdlc/evidence/281/281-observatory-operator-attention-regression.log
- .csdlc/evidence/281/281-typed-validate.log
- .csdlc/evidence/281/281-diff-hygiene.log
- .csdlc/prepared/issues/281/validate_preparation_bundle.py
- .csdlc/issues/281

## Execution

- Added demos/html-observatory/tests/security_privacy_adversarial.test.mjs to prove XSS-safe text/rendering boundaries, credential/token exclusion, trusted HTTPS origin handling, replay/confused-deputy rejection, stale history behavior, denial/recovery state, operator-attention non-authority, and public-safe redaction evidence.
- Added .csdlc/evidence/281/security_privacy_adversarial.json as machine-readable public-safe proof metadata with no secrets, private cognition, or raw provider payloads.
- Added and ran the #281 preparation validator to prove live issue identity, dependency terminal-cache and ancestry gates, security/privacy scope, and forbidden sibling/parent ownership.
- Kept #279 accessibility/responsive, #280 performance/recovery, #282 qualification assembly, #117 parent, #110 umbrella, Runtime authority, cloud/public deployment, Unity, credentials, and provider payloads out of #281 scope.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/281/validate_preparation_bundle.py"
    ],
    "purpose": "Validate #281 issue identity, dependency terminal-cache and ancestry gates, security/privacy/adversarial proof scope, and forbidden sibling/parent ownership.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-preparation-contract.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/security_privacy_adversarial.test.mjs"
    ],
    "purpose": "Prove deterministic #281 XSS/content rendering, credential/token, origin/TLS metadata, replay/confused-deputy/stale/denial, redaction, and no-browser-authority boundaries.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-observatory-security-privacy-adversarial.log and .csdlc/evidence/281/security_privacy_adversarial.json"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "purpose": "Prove existing Runtime conversation acceptance/delivery/reconnect behavior still passes after the #281 proof addition.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-observatory-conversation-regression.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "purpose": "Prove operator-attention Runtime-authority boundaries still pass after the #281 proof addition.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-observatory-operator-attention-regression.log"
  },
  {
    "command": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "281"
    ],
    "purpose": "Run typed issue validation at the bound #281 proof state.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-typed-validate.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace or patch artifacts before review.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/281/281-diff-hygiene.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
