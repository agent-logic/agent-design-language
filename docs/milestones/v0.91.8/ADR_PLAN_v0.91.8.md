# v0.91.8 ADR Plan

Issue `#5728` records the minimum durable ADR set for architecture decisions
already accepted by merged v0.91.8 work. Issue `#5007` adds the Memory Palace
decision from the ready #4760 proof PR while preserving the unmerged boundary.

| Topic | Owning evidence | ADR | Disposition |
| --- | --- | ---: | --- |
| ADL v2 modular clean-room architecture | #5336, #5339, #5338, #5340, #5384 | 0052 | accepted |
| Portable records signing and external trust | #5342 | 0053 | accepted |
| Runtime v3 guardian-owned kernel and API boundary | #5341, #5361, #5590 | 0054 | accepted |
| Runtime v3 unified durable state | #5663, #5698 | 0055 | accepted |
| C-SDLC v2 final authority and v1 sunset | #5358, #5541 | 0056 | accepted |
| ADL v2 generation selector and rollback | #5350, #5344, #5343 | 0057 | accepted |
| Memory Palace context handoff architecture | #4760 PR #5740, #5007, WP-21 #5362 | 0058 | accepted from ready proof PR; not merged/closed yet |

OpenAPI, WSS, TLS, Observatory, and OTel details remain consequences or
interfaces of ADR 0054 and existing ADR 0048 unless a future incompatible
authority change requires a separate decision. They are not duplicated here.

ADR 0051 remains retained as the deferred Chronosense/Memory Palace disposition
record. ADR 0058 consumes its Memory Palace obligation using the ready #4760
proof surface at PR #5740 head `94156d55d0a1f4bfda7ce32ac136437520325906`;
that is not a claim that #5740 has merged or #4760 has closed.
