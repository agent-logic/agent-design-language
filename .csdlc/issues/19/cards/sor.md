# Structured Output Record

Template: 1.0.0

Issue: 19

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Deployed the Synthetic Minds podcast preview through existing Agent Logic S3 and CloudFront infrastructure and replaced the blocked design runtime with a local-only static page.

## Artifacts

- demos/_preview/podcast/index.html
- .csdlc/evidence/19/deployment-manifest.json
- .csdlc/evidence/19/live-verification.json
- .csdlc/evidence/19/browser/podcast-preview-live-desktop.png
- .csdlc/evidence/19/browser/podcast-preview-live-mobile.png
- .csdlc/evidence/19/browser/failures/initial-design-runtime-blank.png
- .csdlc/prepared/issues/19/validate-deployment-evidence.rb

## Execution

- Published both the directory and explicit index preview routes with byte-for-byte source parity.
- Published the existing feed, smoke WAV, and Agent Logic logo with explicit content types, cache policy, checksums, and server-side encryption.
- Removed React, Babel, external fonts, and the generic design runtime from the preview page; retained native audio and native details FAQ controls.
- Verified desktop and mobile rendering, zero external asset requests, zero scripts, no console errors, and no horizontal overflow.
- Preserved the production /podcast/ route at HTTP 403 and invoked only STS, S3, and CloudFront with no EC2 or remote compute operation.

## Validation

[
  {
    "command": [
      "python3",
      "adl/tools/validate_podcast_launch_packet.py",
      "demos/podcast",
      "docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json",
      "--preview-root",
      "demos/_preview/podcast"
    ],
    "purpose": "Validate the local podcast page, preview references, feed, artwork, and smoke-audio source packet.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/19/README.md"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/19/validate-deployment-evidence.rb"
    ],
    "purpose": "Authenticate exact source and screenshot digests, local-only static browser behavior, production-route non-mutation, and the no-EC2 AWS boundary.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/19/live-verification.json"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
