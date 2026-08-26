# Observatory Redesign — v0.92.1

## Outcome

Create a coherent Observatory product that explains authentic Runtime authority and agent activity without invented data.

## Design boundary

Design and information architecture can begin immediately. Implementation consumes only stable Runtime authority APIs. Runtime v4 changes trigger explicit rebaseline rather than silent adapter drift.

Existing `#251` TLS 1.2 and `#122` Route53/ACM exposure may execute in parallel. Existing `#84` Unity Observatory preparation may proceed concurrently, but its final proving lane must consume reviewed merged #251 and #122 evidence plus terminal #340 (PR #430, merge `aa36a828793366f92d0d9e16247bd3fb1cce7878`) and terminal #256 (PR #427, merge `fb4c853bdb9cb140059d2a28af02d70bd36a27a4`) as exact ancestral inputs. OBS-A may proceed in parallel; OBS-B waits for reviewed merged OBS-A and Unity/public-exposure authority and preserves #340/#256 evidence, not closeout.

## Experience

The redesign covers navigation, hierarchy, progressive disclosure, accessibility, keyboard use, screen readers, reduced motion, contrast, responsive layouts, and explicit empty, loading, degraded, refused, revoked, and recovery states.

## Proof

Every displayed claim binds to a real projection or is visibly classified as unavailable. Accessibility checks and source-grounded browser scenarios form the release denominator.
