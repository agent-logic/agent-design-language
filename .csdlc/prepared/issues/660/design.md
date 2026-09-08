# Issue #660 emergency design

The public website origin accidentally exposed the current podcast route under
`podcast/`. The emergency design is intentionally small:

1. Hide only the exact unintended public `podcast/` keys by creating S3 delete
   markers in the public website bucket.
2. Invalidate only the public podcast route and hidden preview route in
   CloudFront.
3. Keep the current The Cognitive Stack landing page under
   `/_preview/podcast/`, with `noindex,nofollow`.
4. Remove preview links that fetch from the public podcast feed or public audio
   path.
5. Retain exact, redacted, non-secret evidence under the issue packet.

No provider submission, private archive deletion, S3 version purge, or
credential-bearing evidence is part of this issue.
