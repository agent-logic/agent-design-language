# Future Operator Authorization Template For #264

No provider submission has been performed. Explicit future operator authorization is still required.

Use this template only when the operator is ready to authorize account-side work. Until a completed authorization exists, every row in `submission-ledger.json` remains `not_authorized`.

## Authorization record

- Authorized by:
- Authorization timestamp UTC:
- Feed approved for submission: `https://agent-logic.ai/podcast/feed.xml`
- Show title approved for submission: `The Cognitive Stack`
- Contact mailbox approved for verification-code use: `podcast@agent-logic.ai`
- Rights/content attestation approver:
- Provider account owner:
- Public-launch announcement authorized: yes/no
- Website destination-link activation authorized after live verification: yes/no

## Provider actions

Mark each provider explicitly as `authorized`, `deferred`, or `operator-only`.

| Provider | Internal id | Submission | Mailbox verification-code use | Rights/terms attestation | Public visibility | Destination-link activation |
| --- | --- | --- | --- | --- | --- | --- |
| Apple Podcasts | `apple_podcasts` | deferred | deferred | operator-only | deferred | deferred |
| Spotify for Creators | `spotify_for_creators` | deferred | deferred | operator-only | deferred | deferred |
| Amazon Music for Podcasters | `amazon_music_for_podcasters` | deferred | deferred | operator-only | deferred | deferred |
| YouTube RSS ingestion | `youtube_rss_ingestion` | deferred | deferred | operator-only | deferred | deferred |

## Stop signs

Stop and return to the operator if any provider asks for payment, advertising, monetization, paid subscription, ambiguous rights/legal attestations, personal-account recovery, duplicate show merge/removal, or channel visibility choices not explicitly authorized here.

Do not retain credentials, verification codes, recovery codes, mailbox contents, cookies, tokens, or private screenshots.
