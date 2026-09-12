# Decision authority and conflicts

| Surface | Disposition | Accountable owner / required next evidence |
|---|---|---|
| Twelve issue-911 candidates | Complete Proposed records; no accepted ADR numbers assigned. | Named scope owner in each candidate; operator or designated decision owner must accept exact text. |
| Repository decomposition #848 | Reuse its decision process; no duplicate split plan. Live readback remains Open. CF-09 is a present local-product selection, not permanent repository allocation. | ARCH-SPLIT #848 owns the five-boundary plan and its required independent reviews; link its settled recommendation and actual decision when available. |
| Static Observatory #910 | Existing #679/#685 design is a source, not deployed acceptance. PR #685 is merged; #910 remains Open. | OBS-S3 #910 owns authorized apply, versioned asset readback, invalidation and browser/Runtime proof. ADR0054 preserves external-client authority. |
| Deferred Observatory ADR0069 | Remains Deferred; no implicit narrowing of its original HTML-and-Unity gate. | Original gate needs both-client proof and human review; #910 HTML-only deployment cannot promote it. |
| Provider ADR0075 versus v0.92.2 | Historical MLX deferral preserved; bounded PLAT-MLX admission recorded in ADR-PLAT-01 using milestone CF-D11. OCI and broad local-model claims remain excluded. | PLAT-PROVIDER / RT-PROVIDER / PLAT-MLX own their contracts and actual qualification. |
| Candidate0072 versus current authority | Proposed documentation and operational selector authority are distinct. C-SDLC01/02 refine v3; no generation change or writer activation occurs here. | SIM owners supply implementation/transition proof; operator explicitly controls acceptance and live writer pause. |
| Source alternatives and tradeoffs | The proposed rationale is curator assessment, not invented historical stakeholder testimony. | Scope owners and operator review it against their actual constraints before acceptance. |

## Existing Observatory design reference

The source package is `infra/aws/observatory/README.md` at the pinned source revision. It describes private versioned S3 for static content, CloudFront OAC, ACM/Route53, access logging and security headers. Runtime HTTPS/WSS are external and not created/proxied by this root. Browser-to-Runtime authentication is not stored in static assets; exact Runtime origins must be declared. Content rollback restores prior object versions and invalidates caches; it does not delete infrastructure. This packet changes none of that design and performs no deployment.

Live issue/PR state is retained in [external-state.json](external-state.json); it is observation evidence, not an authorization receipt. Local source hashes are in [source-manifest.json](source-manifest.json).

## Completion truth

The twelve proposed records and all69-task mapping are a delivered documentation packet. Formal decision acceptance remains unresolved, as do #848's owner decision and #910's deployment acceptance. Do not mark ARCH-ADR decision-set acceptance or TAIL-10 release readiness satisfied from this packet alone. No added early integration or quality-gate dependency is implied.
