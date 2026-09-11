# Existing issue launch review

Read-only authenticated `gh issue view` readbacks are saved in `readbacks.json`; no GitHub mutation was performed. Scope: nine existing identities against the current issue wave, execution specifications and atomic task contracts.

## Findings

### P2: eight existing tasks are absent from the live milestone

`readbacks.json` shows `milestone: null` for #720, #848, #849, #852, #854, #855, #861 and #862. Canonical planning binds each to v0.92.2. #720 also lacks `version:v0.92.2`; #848 has only the `v0.92.2` alias. Reconcile milestone 2 and the canonical version label through native issue actions, preserving other labels. #864 already has the correct milestone and version label. This is launch metadata drift, not missing implementation acceptance.

### P2: live issue dependencies have not received the new launch bindings

Canonical wave edges require #848, #849, #852, #854, #861 and #862 to depend on WP-01/#864, but their live bodies do not state that edge. #855 correctly requires PLAT-PROVIDER, but its existing numeric identity #876 is not linked. Reconcile the precise existing edges, without introducing batch-order gates. OBS-LIVE/#720 has no WP-01 dependency and must retain its independently admitted execution authority.

### P2: current scheduling prose retains the pre-opening/pre-creation state

#720 Scheduling and #848 Milestone routing say the v0.92.2 milestone does not yet exist. #852 says split tasks require separately authorized creation, and #852/#855/#862 carry a number-free-task split note. Preserve these as historical scope-change evidence, but add current launch mapping and authorization status so they cannot be mistaken for a current creation hold. Add concrete follow-on numeric identities once their native creation/readback is complete. Do not rewrite historical receipt markers or claim follow-on execution is complete.

## Per-issue semantic result

| Issue / task | Complete-task contract | Launch disposition |
|---|---|---|
| #720 OBS-LIVE | PASS: live-only controls, timer removal, obsolete assignments, live regression and historical evidence retention are one usable safety repair. | Metadata/scheduling reconciliation; no added gate or scope. |
| #848 ARCH-SPLIT | PASS: one complete reviewed five-repository decision with public/private boundaries, independent Claude/Gemini review and bounded first step; no extraction claimed. | Metadata, dependency and scheduling reconciliation. |
| #849 CSDLC-MERGE | PASS: one merge-linkage repair with same-head negatives, Closing/PartOf success, uncertainty/reconciliation and remote CAS limitation. | Metadata and dependency reconciliation. |
| #852 QUAL-RUNTIME | PASS: one real correlated ingress failure-event repair, authenticated WSS/redaction and success preservation; five-row criteria and 19/5/2 historical accounting explicitly retained in separate tasks. | Metadata/dependency and split follow-on current mapping reconciliation. |
| #854 RT-COST | PASS: one wasteful-probe repair with startup/steady-state/recovery behavior, reason/token counters and nonzero deterministic soak. | Metadata and dependency reconciliation. |
| #855 RT-PROVIDER | PASS: full registered-provider lifecycle, five routes, generated conversations/A2A, replace/checkpoint/rehydration, secrets/transport/capabilities/accounting/projection; schema-only closure rejected. | Metadata and #876 numeric binding reconciliation. |
| #861 CSDLC-MAN | PASS: complete installed command/option reference, examples, recovery, parity and macOS/Linux lookup; no outline-only acceptance. | Metadata and dependency reconciliation. |
| #862 CSDLC-DECOMPOSE | PASS: complete local owner decomposition with all production routes, preserved serialized/authority/digest/error/path behavior, recursive inventory and no replacement god module. Remote task retains #862/#849 prerequisites. | Metadata/dependency and follow-on mapping reconciliation. |
| #864 WP-01 | PASS for existing first-batch body; canonical complete planning outcome remains present. | Known planned final update to all batches/all 69 mappings; not an additional blocker in this interim review. |

No lost behavioral acceptance or unsupported delivery claim was found. The launch remains pending the listed metadata/body reconciliation and the already planned final #864 scope update. This report does not attest implementation, release approval, native lifecycle admission or terminal closeout.
