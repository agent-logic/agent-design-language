# CSM Runtime ALB origin

This root stack creates the replaceable Runtime origin ALB for issue #122. It is
separate from the permanent public edge stack so operators can destroy and
recreate the ALB without touching CloudFront, WAF, DNS delegation, or the static
Observatory edge.

Typical order:

1. Apply this stack once with `target_instance_id = null`.
2. Use the `alb_security_group_id` output in `infra/aws/csm-runtime-spot`.
3. Apply the Spot stack and copy its `instance_id`.
4. Re-apply this stack with `target_instance_id` set.
5. Point the public edge origin variables at `https://<origin_fqdn>`.

The ALB stack is intentionally configured to reuse one regional ACM certificate.
By default, `certificate_arn = null` and `reuse_existing_certificate = true`
make Terraform look up an existing ISSUED regional ACM certificate for
`origin_fqdn`. Set `certificate_lookup_domain` when the reusable certificate is
a wildcard, for example `*.wuji.dev.csm.agent-logic.ai`. Only set
`reuse_existing_certificate = false` and `create_certificate = true` when
intentionally creating the first reusable origin certificate; do not mint a new
certificate for each ALB run. If the lookup finds no certificate, Terraform
fails closed instead of silently creating another one.
