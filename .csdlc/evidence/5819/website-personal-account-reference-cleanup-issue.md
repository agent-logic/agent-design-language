## Summary

After WP-02 has created and verified the five `agent-logic` destination copies,
audit the live Agent Logic website repository and deployed production/beta
sites for references to `danielbaustin` or other personal-account repository
URLs. Replace current operational links with the verified organization-owned
destinations while preserving legitimate historical attribution.

## Dependency Gate

- Blocked by #5819.
- Do not change a repository URL until the corresponding
  `agent-logic/<repository>` destination exists and has passed WP-02 copy and
  source-immutability verification.
- This issue does not authorize changing, transferring, deleting, renaming, or
  archiving any repository under `danielbaustin`.

## Scope

- `agent-logic/agent-logic.ai` source, including production and beta surfaces
- Website build, deployment, redirects, metadata, structured data, feeds,
  manifests, sitemaps, badges, documentation links, and generated assets
- Current references to the five WP-02 source repositories:
  - `cognitive-sdlc-paper`
  - `godel-hadamard-bayes-paper`
  - `general-intelligence-paper-private`
  - `universal-tool-schema`
  - `agent-design-language`
- Any other current personal-account references discovered during the audit,
  with an explicit keep/update/remove disposition

## Required Analysis

1. Search the website repository and deployed production/beta output for all
   `danielbaustin`, personal GitHub-profile, and personal repository URLs.
2. Classify each match as current operational, navigational, metadata,
   generated, historical attribution, or unrelated.
3. Map every current operational reference to a verified organization-owned
   destination.
4. Identify references that must remain because they are historical evidence,
   authorship attribution, or point to `asksifu`/`Horust`, which are not moving.
5. Record the exact changed-file and deployed-URL denominator before editing.

## Acceptance Criteria

- A complete source and deployed-site inventory lists every personal-account
  reference and its disposition.
- No current operational link points to a personal-account copy when a verified
  `agent-logic` destination is canonical.
- `asksifu` and `Horust` references are not redirected to nonexistent company
  copies and are changed only when independently justified.
- Historical attribution and evidence are preserved rather than rewritten
  mechanically.
- Production and beta pages, metadata, sitemap/feed outputs, and affected
  generated assets resolve to the intended destinations.
- Link validation reports no broken organization-owned destination.
- Deployment receipts prove the exact reviewed revision reached production and
  beta.
- No secret values or unrelated website redesign enters the change.

## Non-Goals

- Repository copying or migration
- Any mutation of repositories under `danielbaustin`
- Website redesign, content rewrite, or unrelated SEO work
- Rewriting historical evidence solely to remove the personal account name
- Creating organization copies of `asksifu` or `Horust`

## Validation

- Focused repository-wide URL/reference scan
- Static-site build or existing focused website validation
- Production and beta link inspection after deployment
- Exact changed-file review and diff hygiene

## Closeout

Retain the reference inventory, disposition table, exact revision, deployment
receipt, and post-deploy link proof. Use `Closes #<issue>` in the implementation
PR.
