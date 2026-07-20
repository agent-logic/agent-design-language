Closes #5594

## Summary

- make #5594 the active WP-01 readiness authority and create the single
  v0.91.8 sprint umbrella #5595
- route all 65 live v0.91.8 issues, including Runtime v3 parity
  #5591 -> (#5592, #5589, #5590), acceptance, cutover, and release-tail work
- pin and classify all 123 canonical feature rows so Runtime v3 cutover cannot
  silently drop retained behavior
- repair stale live issue parents, dependencies, labels, and malformed bodies
- retain a four-writer opening wave and one serialized integration queue

## Validation

- `ruby .csdlc/prepared/issues/5594/validate_structured_planning.rb`
- `ruby .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb`
- `ruby .csdlc/prepared/issues/5594/validate_live_routing.rb`
- `ruby .csdlc/prepared/issues/5594/validate_links.rb`
- `git diff --check`
- current `csdlc-doctor` reports no findings at implemented phase

## Boundaries

- no downstream product implementation
- no AWS or raw `gh`
- Runtime v2 remains retained pending reviewed parity, cutover, rollback, and
  deletion evidence
- external model agents remain read-only review evidence, never lifecycle or
  merge authority
