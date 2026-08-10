# Structured Task Prompt

Template: 1.0.0

Issue: 100

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Recover, reconcile, upload, verify, review, and publish issue #100 evidence; stop before merge without explicit authority.

## Deliverables

- Ten canonical Medium launch article drafts in the approved Drive folder
- Preserved inventory of every distinct substantive draft
- Recovery manifest with title, Drive URL, source path or revision, status, and digest
- Focused verifier that fails on an empty or incomplete destination
- Explicit unrecoverable-item report if any article cannot be found

## Acceptance

1. AC-1: Recovery searches approved Drive, repository content and history, and registered FastWork worktrees.
2. AC-2: Every distinct substantive draft is preserved and attributed; no source is silently replaced or deleted.
3. AC-3: Exactly one evidence-backed canonical revision is selected for each of the ten required titles.
4. AC-4: All ten canonical drafts have stable names and readable content in the approved destination folder.
5. AC-5: The retained manifest records title, Drive URL, source path or revision, recovery status, and content digest for every article.
6. AC-6: Any unrecoverable article is explicitly reported and no synthetic replacement is created without approval.
7. AC-7: No publication, sharing, deletion, or destructive overwrite occurs.
8. AC-8: A focused verifier fails closed when the destination or manifest is empty, incomplete, duplicated, unreadable, or missing provenance.

## Dependencies

- Canonical GitHub issue agent-logic/agent-design-language#100
- Approved company Google Drive credential
- Destination folder 1hacu6zwCUlIYXYtvpMW0IFtk506LUb8Q
- Retained repository and FastWork history

## Inputs

- .adl/docs/TBD/publication
- demos
- Git history
- registered FastWork ADL worktrees

## Non Goals

- Writing replacement articles
- Publishing to Medium or podcast channels
- Changing Drive access or public visibility
- Deleting or rewriting historical drafts
- Lifecycle tooling changes
- Any change to PR #98
