# The Cognitive Stack S3 And CloudFront Runbook

This runbook governs durable podcast storage and public media delivery. It
separates retained production evidence from approved public artifacts so that
storing a candidate never publishes it.

## Authority And Resources

- AWS profile: `agent-logic-admin`
- Region: `us-west-2`
- Bucket: `agent-logic-podcast-archive-agentlogic`
- CloudFront OAC: `E1K62WYR1DVOXQ`
- CloudFront distribution: `E34IBPFTBM0242`
- Initial CloudFront domain: `dqyy7yagafimc.cloudfront.net`
- CloudFront origin path: `/public`
- Custom podcast hostname: not configured

Before any mutation, verify that `agent-logic-admin` resolves to the approved
Agent Logic business AWS account. Never fall back to the default or a personal
AWS profile.

## Security Contract

The bucket is private and uses:

- S3 Block Public Access with all four controls enabled
- bucket-owner-enforced object ownership
- AES-256 server-side encryption
- versioning
- no public ACLs or public bucket policy
- incomplete multipart uploads automatically aborted after seven days
- `Project=TheCognitiveStack`, `Purpose`, and `ManagedBy=WP-24A` resource tags

The bucket policy grants the dedicated CloudFront distribution `s3:GetObject`
only for `public/*`. CloudFront cannot read `archive/*`.

## Prefix Contract

```text
archive/the-cognitive-stack/episodes/<number>/package/
archive/the-cognitive-stack/episodes/<number>/media/
archive/the-cognitive-stack/episodes/<number>/production/dialogue-source/
archive/the-cognitive-stack/episodes/<number>/production/rendered-segments/
public/index.html
public/feed.xml
public/artwork.png
public/audio/<episode>.mp3
public/episodes/<slug>/index.html
```

`archive/` holds scripts, transcripts, manifests, source artwork, WAV masters,
distribution candidates, retained model dialogue, and rendered segments.
Objects there use `publication-status=held-for-human-review` metadata until a
human publication decision is recorded.

Episode artwork retains the operator-selected source PNG unchanged. The
3000 x 3000 RGB podcast artwork is a proportional technical derivative for
Apple compatibility; do not redraw, restyle, recolor, or replace the selected
source during promotion.

`public/` contains only the approved web page, RSS feed, artwork, and MP3
files. No WAV master, raw dialogue source, provider response, credential, or
intermediate render belongs under `public/`.

## Archive A Candidate

Archive from the issue worktree using the approved business profile. Preserve
the episode number and package layout shown above. Upload with AES-256 and an
SHA-256 checksum, then retain the resulting object inventory in the episode
package.

Archive completion requires:

1. every expected object is present
2. critical MP3, WAV, artwork, script, transcript, and manifest sizes and
   checksums match local retained evidence
3. versioning, encryption, ownership, and Block Public Access remain enabled
4. the CloudFront distribution cannot retrieve an `archive/*` object
5. `public/` remains empty unless publication was separately approved

## Promote An Approved Episode

Promotion requires explicit human approval of audio, metadata, artwork, and
publication. It is a separate operation from archival storage.

1. Copy the approved show page, feed, artwork, episode page, and MP3 into their
   exact `public/` keys.
2. Set the correct `Content-Type` for HTML, XML, PNG, and MP3 objects.
3. Use a short cache lifetime for `feed.xml`; use longer cache lifetimes for
   immutable episode media.
4. Verify HTTPS retrieval through CloudFront, including range requests for the
   MP3.
5. Recompute the live enclosure byte count and compare it with the feed.
6. Invalidate only changed mutable keys such as `/feed.xml`, `/index.html`, or
   `/artwork.png`.
7. Submit the feed to directories only after the public URL and mailbox checks
   pass.

The initial CloudFront hostname is suitable for infrastructure verification.
A stable branded hostname and matching ACM certificate must be configured
before podcast-directory registration. Do not publish the CloudFront-generated
hostname as the permanent RSS identity.

## Verification

Verify the resource controls directly from AWS:

```bash
AWS_PROFILE=agent-logic-admin aws sts get-caller-identity
AWS_PROFILE=agent-logic-admin aws s3api get-public-access-block --bucket agent-logic-podcast-archive-agentlogic
AWS_PROFILE=agent-logic-admin aws s3api get-bucket-versioning --bucket agent-logic-podcast-archive-agentlogic
AWS_PROFILE=agent-logic-admin aws s3api get-bucket-encryption --bucket agent-logic-podcast-archive-agentlogic
AWS_PROFILE=agent-logic-admin aws s3api get-bucket-ownership-controls --bucket agent-logic-podcast-archive-agentlogic
AWS_PROFILE=agent-logic-admin aws cloudfront get-distribution --id E34IBPFTBM0242
```

The direct S3 object URL must remain inaccessible. Before promotion, the
CloudFront root may return a missing-object response because `public/` is
intentionally empty; that is expected and does not mean the archive failed.

## Rollback And Recovery

- To stop public delivery, disable the CloudFront distribution or remove its
  bucket-policy statement. Do not delete the archive.
- To roll back one public artifact, restore the prior S3 object version and
  invalidate only that key.
- To correct a candidate, upload a new version under the same archive key and
  refresh the retained storage inventory.
- Never suspend versioning, open bucket-wide reads, make `archive/` readable by
  CloudFront, or delete retained production evidence as a publication rollback.

## Cost And Operations

The dominant variable cost is public audio transfer, not storage. Keep archive
masters private, publish MP3 rather than WAV, use CloudFront caching and range
requests, and monitor S3 storage, CloudFront transfer, requests, and invalidation
usage in the Agent Logic business account. Add billing alerts before directory
promotion; do not weaken cache or security controls merely to reduce cost.
