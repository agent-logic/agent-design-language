# Monitoring, Correction, Rollback, And Link Activation Rules

No provider submission has been performed. Explicit future operator authorization is still required.

These rules apply only after the operator authorizes a provider action using `operator-authorization-template.md`.

## Monitoring

For each authorized provider, retain only:

- provider id: `apple_podcasts`, `spotify_for_creators`, `amazon_music_for_podcasters`, or `youtube_rss_ingestion`;
- non-secret submission timestamp;
- redacted status summary;
- canonical provider URL or ID after it exists;
- follow-up owner and next check date.

Do not retain credentials, verification codes, recovery codes, mailbox contents, cookies, tokens, or private screenshots.

## Corrections

Before correcting metadata, confirm whether the correction belongs in:

1. the public RSS feed or website;
2. the provider account UI;
3. a provider support request;
4. a future issue outside #264.

Stop if the provider asks for payment, advertising, monetization, paid subscription, ambiguous legal/rights attestations, personal-account recovery, duplicate show merge/removal, or channel visibility choices not already authorized.

## Rollback

Rollback preserves history. Do not delete episode source packages or rewrite the ledger to hide a submission attempt. If withdrawal, hiding, rescheduling, or provider support is needed, append a redacted ledger update with the prior status and the new state.

## Destination-link activation

Do not activate destination links until the provider listing is live and verified. A destination link is live only when the canonical provider URL resolves publicly or under the explicitly authorized visibility state and the listing title/feed identity matches `The Cognitive Stack`.

Website updates remain blocked until live verification exists and the operator authorizes link activation.
