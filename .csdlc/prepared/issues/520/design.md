# Issue #520: exact-candidate internal review

## Goal

Produce one findings-first internal review register for the final v0.92.1
candidate, with a deterministic denominator that cannot silently omit changed
code, tests, documentation, issue/PR truth, or release evidence.

## Design

TAIL-03/#519 supplies the immutable candidate commit. The review records the
v0.92.1 base and candidate as full SHAs, derives the complete changed-file
inventory from that range, and reconciles it with the complete live v0.92.1
issue/PR roster. Every changed production file and every changed test receives
code/test review; canonical documentation, lifecycle evidence, demos,
providers, cloud boundaries, security, architecture, and release claims are
routed to explicit specialist lanes. Sampling may prioritize reading order but
may not reduce the denominator.

The canonical register retains every raw finding or an explicit, evidenced
no-finding result for each assigned surface. Green CI and successful commands
are evidence inputs, never substitutes for semantic review.

## Boundary

#520 reviews and reports. It does not repair product findings, conduct the
independent external review, approve release, merge, deploy, or spend money.
