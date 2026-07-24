README focused validation for issue #5653

Revision under test: 3a45acd5d

1. git diff --check
   PASS

2. README assertions
   PASS: homepage URL https://agent-logic.ai is present
   PASS: v0.91.8 status is present
   PASS: stale v0.91.7 closeout badge is absent
   PASS: CI and coverage badges target branch=main

3. Homepage reachability
   PASS: https://agent-logic.ai returned HTTP 200
