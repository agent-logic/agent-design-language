---
name: adl_pr_cycle
description: "Historical compatibility documentation for the retired adl_pr_cycle route. Do not use as a current ADL lifecycle entrypoint."
---

# adl_pr_cycle

This tracked file is retained as historical compatibility evidence for the
former local Codex skill at:

- `$CODEX_HOME/skills/adl_pr_cycle/SKILL.md`

Do not install, resync, invoke, or route current ADL work through
`adl_pr_cycle`. It is not a current workflow entrypoint.

Current C-SDLC issue work uses the independent Rust v2 binary set under
`.adl/bin/csdlc-v2/` and the typed owner skills in
`csdlc-v2/operator/skills/`. C-SDLC v2 remains the live lifecycle authority
until explicit V3-F/#505 cutover approval. C-SDLC v3 remains construction-only
and non-authoritative before that cutover.

## Historical prompt boundary

```text
You are running skill: adl_pr_cycle.

Purpose:
- Preserve historical compatibility text without making `adl_pr_cycle` a
  current lifecycle route.
- Refuse current issue execution and point operators at the typed C-SDLC v2
  lifecycle owner skills.

Current-work procedure:
1) Stop. This compatibility route is blocked by policy for current ADL work.
2) Use the typed C-SDLC v2 owner skill for the needed operation instead:
   `csdlc-v2-init`, `csdlc-v2-bind`, `csdlc-v2-card-editor`,
   `csdlc-v2-validate`, `csdlc-v2-review`, `csdlc-v2-publish`,
   `csdlc-v2-shepherd`, `csdlc-v2-finish`, or `csdlc-v2-clean`.
3) Keep v3 work construction-only until V3-F/#505 explicitly approves and
   proves any authority transition.

Stop boundaries:
- Any current-work request to use `adl_pr_cycle` is blocked by policy.
- Report the applicable typed C-SDLC v2 owner skill and stop before mutating
  lifecycle state through this compatibility surface.
- Do not present this file as a runnable current procedure.
```

## Truth boundaries

- This file is historical evidence, not executable guidance.
- The independent Rust v2 binaries and their typed operator skills are the only
  active lifecycle authority before V3-F/#505.
- Repository-declared validation may invoke bounded external tools, including a
  shell or Python program, but only as an explicit typed proof command. The
  C-SDLC control plane itself never depends on shell/Python lifecycle logic.
- Historical v1 records remain evidence only; they are not executable guidance.

## Failure policy

Fail closed on any request to use `adl_pr_cycle` for current lifecycle work.
Preserve the request context and route the operator to the applicable typed v2
owner skill instead of improvising through a compatibility surface.
