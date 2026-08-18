# #407 design: implemented-phase SIP Goal recovery

## Problem

Implemented issues can be review-recovered and then repaired through narrow typed card operations, but SIP `goal` is not repairable after recovery. #286 now needs exactly that repair: the SIP Goal still carries WP-18C-level terminal framing after `required_outcome` and `declared_scope` were corrected.

## Design

Add one narrow semantic operation for recovered implemented SIP Goal truth repair, parallel to the existing `correct_required_outcome_after_recovery` path:

- operation name: `correct_goal_after_recovery`
- card owner: SIP only
- phase: implemented only
- precondition: current typed review recovery with cleared review assignment/review/publication/readiness/terminal truth
- input: non-empty replacement goal string
- behavior: update SIP values, render card projections, increment generation, append audit with previous/new goal and recovery provenance

Do not authorize generic implemented-phase SIP mutation, broad `set_field goal`, lifecycle reset, or publication bypass.

## Proof

Focused csdlc-v2 regressions should construct an implemented issue with review recovery, apply `correct_goal_after_recovery`, verify the goal changes, verify audit/projection generation changes, and verify fail-closed coverage for:

- no current recovery provenance;
- stale generation/digest;
- reviewed/published/terminal state after recovery is no longer clear;
- unrelated implemented-phase SIP mutations such as broad `set_field goal`, `set_field required_outcome`, and non-goal SIP collection edits.

The positive test should model the #286 failure shape without mutating #286 itself; the negative tests should prove the new operation is not a generic implemented-phase SIP rewrite path.
