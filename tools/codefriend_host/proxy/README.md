# Beta HTTPS reverse proxy preparation (#1077)

This standalone configuration targets Ubuntu's nginx 1.24 or newer. It is a
reviewable installation input, not proof of deployed TLS. Do not include it
inside another `http` block or merge it with an existing server configuration.
The host is dedicated to this beta. No package, certificate, service or DNS
changes were performed when preparing these files.

## Required deployment inputs

- An explicitly authorized deployed host with ports 80/443 reachable and only
  the website on `127.0.0.1:43156`; gateway stays on `127.0.0.1:43155`.
- Authorized `beta.codefriend.ai` A record targeting the host's reserved IPv4;
  no AAAA until IPv6 is separately configured. This does not require changing
  registrar delegation: use the currently authoritative zone until an approved
  DNS migration is complete.
- A reviewed certificate issuance/renewal mechanism providing a valid public
  certificate for exactly this beta host, with complete chain and matching key.
  Certificate issuance and DNS validation are separate authorized operations.
  This config neither invokes ACME nor automatically opens validation routes.
- Root-installed nginx including its HTTP proxy and SSL modules, `www-data`
  user, fixed TLS files below, and tested certificate renewal with nginx reload.
- Website private config `origin: "https://beta.codefriend.ai"`, `port: 43156`,
  registered callback `https://beta.codefriend.ai/auth/github/callback`, and
  gateway origin `http://127.0.0.1:43155`. Keep real secrets in private config.

## Authorized installation and activation

These commands are deployment instructions, not local validation commands.
Run only after host deployment, DNS and certificate work is authorized and
complete. From this directory in the reviewed release checkout, install the
certificate files from an approved private staging location; do not paste key
material into a terminal, source repository, or evidence log.

```sh
sudo install -d -o root -g root -m 0700 /etc/codefriend/tls
sudo install -o root -g root -m 0644 /approved/private/tls/fullchain.pem /etc/codefriend/tls/fullchain.pem
sudo install -o root -g root -m 0600 /approved/private/tls/privkey.pem /etc/codefriend/tls/privkey.pem
sudo install -o root -g root -m 0644 nginx.conf /etc/nginx/nginx.conf
sudo nginx -t -c /etc/nginx/nginx.conf
sudo systemctl enable --now nginx.service
```

Replace `/approved/private/tls/` with the operator-approved certificate staging
path. Preserve any pre-existing nginx configuration privately before replacement;
if the host serves anything else, stop and prepare a separate reviewed topology.
For updates, validate first then `sudo systemctl reload nginx.service`.
Certificate renewal must install verified files atomically and reload only after
`nginx -t` passes. Renewal scheduler, expiry alerting and live renewal proof are
outstanding deployment acceptance, not supplied by this config.

## Routing, confidentiality and resource boundaries

All paths enter website routing and its authentication policy; even agent `/v1/operations` routes never
proxy straight to the gateway. Unknown TLS names reject handshakes, other HTTP
hosts are denied, and plain HTTP for the exact beta name redirects to its fixed
HTTPS origin. Secure host-only session cookies, OAuth query parameters and CSRF
headers pass through without rewrite; the website owns authorization and errors.

The outer body ceiling is 4 MiB for agent result uploads. The website enforces
its tighter 2 MiB packet/ordinary JSON limit. Request/response buffering, cache
and upstream retry are disabled to avoid disk-retained source payloads and
unintended retry. HTTP/1.1 upstream transport avoids chunked-body buffering
fallback. The website's in-flight response accounting still governs drain while
nginx is consuming its response; delivery to a remote client is not a guaranteed
atomic transaction with host poweroff.

Access logging is off and request-bearing nginx errors go to `/dev/null` so
OAuth codes, tokens and payloads are not persisted in proxy logs. This sacrifices
nginx error diagnostics; systemd state and bounded redacted service events remain
available. Do not enable debug/access logs during acceptance with real credentials.
The proxy forwards client Origin rather than overwriting it, preserving CSRF
checks. No request-header-derived upstream address is accepted.

## Validation boundary

Local preparation verified the route against website HTTP handlers, 2/4 MiB
limits and fixed service listener ports. `git diff --check` checks formatting.
An isolated Linux fixture subsequently passed nginx syntax validation and seven
runtime checks after fixing the default TLS server's post-handshake Host rejection.
The tested source configuration SHA-256 was
`62677d6378a1f24826ab8c507c64654d789a4ca03b29c7da5862172b272cc763`.
The fixture substituted ports8080/8443, its PID path and a self-signed certificate;
it did not use production DNS, public certificate issuance or deployed beta TLS.
This is bounded proxy behavior proof, not production TLS acceptance.
On the prepared production host, first run `nginx -t`; then prove wrong-host rejection,
HTTPS chain/name validation, no direct public gateway access, unauthenticated
route denial, exact-limit uploads, CSRF preservation, OAuth callback, and absence
of retained request/proxy temporary files. This is required deployed acceptance.

Directive references: [proxy module](https://nginx.org/en/docs/http/ngx_http_proxy_module.html),
[HTTP core](https://nginx.org/en/docs/http/ngx_http_core_module.html),
[SSL module](https://nginx.org/en/docs/http/ngx_http_ssl_module.html).
