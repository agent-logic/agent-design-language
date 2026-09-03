# Issue #660 — Podcast public exposure emergency rollback

This packet records the emergency correction for issue #660.

## What happened

`https://agent-logic.ai/podcast/` and related RSS/media/site assets were
reachable before the operator-approved public launch gate. The intended state is
that the current show page for **The Cognitive Stack** remains hidden under
`https://agent-logic.ai/_preview/podcast/` until explicit launch approval.

## What changed

- The current public S3 objects under `podcast/` received delete markers only;
  previous object versions were not purged.
- CloudFront invalidated `/podcast/`, `/podcast/*`, and `/podcast`.
- The hidden preview page was updated and deployed under `/_preview/podcast/`
  with `noindex,nofollow`.
- The hidden preview no longer links the public RSS feed or public MP3 path.
- The preview logo is served from `/_preview/podcast/assets/`.
- The preview player uses `/_preview/podcast/audio/meet-the-ai-coworkers.mp3`;
  the public `/podcast/audio/` route remains withheld.

## Evidence

- Machine-readable evidence:
  `.csdlc/evidence/660/emergency-exposure-rollback.json`
- Exact public-delete manifest:
  `.csdlc/prepared/issues/660/delete-public-podcast-prefix.json`
- Local validator:
  `.csdlc/prepared/issues/660/validate-emergency-rollback.rb`

## Boundaries

No provider directories were submitted or mutated. No private archive bucket
content was deleted or purged. No credentials, verification receipts, recovery
codes, cookies, or private provider screenshots are included in this packet.

Issue #51 should remain open until #660 is closed or otherwise dispositioned
and the operator separately accepts the still-blocked external provider/public
launch gate.
