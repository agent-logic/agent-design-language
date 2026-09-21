# Issue #1109: native 4+1 architecture evidence

The installed native CodeFriend journey generated and retrieved evidence-bound
architecture packages on ADL and a bounded external CodeFriend.ai repository.
Both packages populated logical, development, process and deployment views and
included two scenarios. Both correctly remained `complete: false`: limited source
scope and missing execution evidence do not establish verified running topology.

This is implementation qualification for #1109. Independent Beta 1 qualification
remains #915, with its own prescribed external repository, provider, platforms,
fresh users and both website modes. Hosted and paired-local activity integration
remains #1101 and agent-logic/codefriend.ai#6; those paths must consume the 4+1
package and existing approval contract before advertising website support.

## Installed executions

[qualification.json](qualification.json) records exact source revisions, compiled
candidates, installer source hashes, admitted scope, view/scenario counts and
retained evidence hashes. These are earlier clean candidates on this branch;
subsequent export presentation changes do not imply those binaries were rerun.

| Repository | Candidate | Entities | Relationships: logical/development/process/deployment | Scenarios |
| --- | --- | --- | --- | --- |
| ADL | `ed929a498257acb8b2fdd919e3a1a961a8a092e7` | 12 | 4 / 3 / 4 / 3 | 2 |
| CodeFriend.ai | `69afb828f31fdbb64acc6483d7f52878f9ab4e4d` | 10 | 4 / 1 / 4 / 2 | 2 |

Each execution used an isolated stable installation of `adl`, the native
`journey local` preparation, then `journey resume` with the `four_plus_one`
continuation, followed by a separate `status` resume. Reconstruction checked the
retained response, graph, live admission and deterministic rendered bytes.
Repository acquisition was pinned and read-only. Source and model response packets
remain local under the evidence roots in the summary; they are not published here.
After execution, the retained evidence directory moved from `.adl/issue-1109/`
to `.csdlc/issue-1109-evidence/` for native proof admission. Original request/session
paths and installer provenance remain immutable historical execution evidence;
relocation is not a claim that a relocated session was resumed.

Commands, with each path selected outside the analyzed checkout:

```sh
adl codefriend journey local --request prepare.json
adl codefriend journey resume --output journey --request generate.json
adl codefriend journey resume --output journey --request status.json
```

`generate.json` contains `{"stage":"four_plus_one","provider_request":"provider.json"}`
with the actual operator-owned provider-request path. `status.json` contains
`{"stage":"status"}`. Preparation follows the existing native journey request
contract and the exact scope/revision in `qualification.json`; it selects V2
Rust analysis for ADL and JavaScript analysis for CodeFriend.ai. Each provider
request used the documented Anthropic route, model `claude-fable-5`, 16,384 output
tokens and one attempt. Credentials were scoped to the subprocess environment.

Five provider attempts were made across explicitly started fresh journeys:
ADL and external initial attempts retained incomplete responses and failed;
ADL's second attempt passed; external's second attempt exposed missing redundant
view membership and failed; external's third attempt passed after the guarded
membership repair. Failed journeys were not erased or automatically replayed.
No observed runtime or deployment-state evidence was supplied. ADL also reports
privacy-omitted inputs; both outputs retain unresolved-module and scope gaps.

## Contract and export proof

The focused targets are `codefriend_four_plus_one` (14 tests) and
`codefriend_journey` (21 tests). The clean `69afb828f3` run passed all 35, including
complete/incomplete/conflicting evidence, invalid references, deletion/expiry,
response reconstruction, duplicate keys, fenced JSON, create-only output, artifact
tampering and actual native generation/resume/approved Markdown/HTML/PDF exports.
The native journey regression retains legacy 18-stage compatibility and exercises
the optional 19th architecture stage through the provider adapter on loopback.

A deterministic sufficient-source fixture populates all views and both success
and failure scenarios. Synthetic review/approval records exercise the existing
publication contract; they are not claimed as real approval of the live packages.
Exports preserve Mermaid sources, SVG diagrams, citations, shared entity IDs and
scenario cross-references. The PDF draws corresponding native vector diagrams.

The fixture's eight-page PDF was inspected on narrative page 3 and diagram page 8:
text and diagram labels were readable without clipping. Static HTML inspection
verified all 17 links and five image targets. It also exposed pipe tables rendered
as prose; the repair enables GFM tables, bounded table/image styling and a focused
assertion for real table headers. Browser visual inspection could not run because
the supported browser runtime rejected its trusted RPC dependency path. This
packet does not claim browser-rendered inspection. PDF evidence does not substitute
for website-mode or live-package approval proof.

Focused clippy passed at `69afb828f3`. The final publication requires native
exact-head proof and independent review after the presentation repair and records
are committed. CI integration and merge readiness remain separate claims.

## Review dispositions and boundaries

Bounded independent review identified missing actionable gaps, omitted isolated
nodes, response decoding, attachment link rewriting and fenced-response export
packaging. All were repaired with focused coverage. The latest source repair
review at `69afb828f3` reported no actionable findings; the final presentation and
evidence delta receives a separate bounded review before native publication.

The package validates citation identity and exact source spans, not semantic
entailment. Human review still judges whether a declaration supports the associated
architectural interpretation. Generated completeness never certifies running
behavior. Existing retention, redaction and approval owners remain authoritative;
no additional review pipeline, automatic source mutation or external publication
was introduced.
