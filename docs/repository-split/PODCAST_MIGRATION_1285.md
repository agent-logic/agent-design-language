# The Cognitive Stack migration — ADL #1285

The podcast product now belongs to the private [agent-logic/the-cognitive-stack](https://github.com/agent-logic/the-cognitive-stack) repository. Source preservation was verified at commit **13f217d846b305f43943a50cbbdd6ee4292a1aef** using a fresh GitHub clone before removing the ADL originals.

The source snapshot is ADL `02cf7874596a45c5eb76e0e43762c28d77ddf723`. The [manifest](../../tools/podcast_migration/source-manifest.json) records 521 imported files, original paths, SHA256, size and mode. Original ADL-relative layout is retained in the destination. This bounded snapshot preserves exact source identity without copying unrelated ADL Git history.

## Relocated sources

| Original | Destination relative path |
| --- | --- |
| `demos/podcast/` | `demos/podcast/` |
| `demos/_preview/podcast/` | `demos/_preview/podcast/` |
| Fourteen podcast tools and the three-provider adapter | `adl/tools/` |
| Podcast pilot ADL example and studio episode card | Original relative paths |
| Three local `.adl/docs/TBD/` podcast notes | `planning/TBD/` |
| FastWork production package | `production/2026-10-16-ai-goes-to-hawkins/` |

182 tracked product files leave ADL. Supporting historical milestone documents and the shared logo are copied to the destination while their original ADL records remain unchanged. The source license and embedded notices remain present. Local production originals remain a recovery copy until their owner completes handoff; the new repository is the production source of truth.

## Publication and production boundaries

The public feed remains **https://agent-logic.ai/podcast/feed.xml**; no GUID, enclosure URL, artwork identity or subscription changes. No deployment occurred. This migration does not transfer hosting infrastructure or authorize directory resubmission.

Episode 1 is the published release. Ten historical source packages are not ten releases. **AI Goes to Hawkins** is the revised 29-turn draft and awaits script approval; no final audio or publication is established. Earlier planning recommendations to postpone migration are superseded by the October 15 operator instruction, while their original text is preserved.

Legacy scripts retain historical runtime and machine-path dependencies. The pilot's old Cargo invocation requires installed-runtime adaptation before independent live use. Four-speaker audio rendering and publication remain follow-up work. Do not deploy the private repository wholesale.

## Validation and retained references

Fresh remote clone verification covered all 521 imported files and executable modes, tracked membership, and Episode 1 feed identity. Offline launch/studio validators and the audio-policy synthetic-tone fixture passed. No paid providers or speech rendering were used. ADL's migration test covers provenance, removed source coverage, historical evidence and active reference routing; it is not media or release qualification.

Historical milestone and lifecycle records remain immutable and continue to describe their original paths. Resolve those paths against the pinned source commit or the destination manifest. The old `validation_lane_selector.v0.91.6.json` and its legacy test remain historical; the active `validation_lane_selector.public.v1.json` selects the relocation contract. `tools/sprint_contracts/owner_inputs.json` retains the exact historical conductor resource pin. The packet-builder changed-path fixture is historical test data. The account-switch handoff, #1269 inventory, and #1223 podcast-retention packet retain original claims.

The #1223 source-retirement guard is preserved through an exact, separately pinned #1285 transition: only listed before/after identities may change; unrelated removals and protected evidence still fail closed. Required CI and PR review remain separate from the local preservation checks above.
