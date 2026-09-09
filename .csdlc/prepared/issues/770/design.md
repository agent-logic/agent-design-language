# Issue 770 design

Add plan-time recovery guards to public module; preserve isolated ingress and hardening; prove mocked negative/positive cases; prepare exact cloud plan, obtain approval, prove SSH/isolation, dispose, independently review and publish.

AC-1: Public null or blank key is rejected before apply.
AC-2: Public empty SSH CIDRs are rejected before apply.
AC-3: One existing approved key and explicit /32 SSH documented and proved.
AC-4: Runtime application ingress, IMDSv2 and encrypted EBS preserved.
AC-5: Separate private-only deployment is unchanged.
AC-6: Authorized bounded AWS reachability/isolation proof and disposal evidence retained.

Local mocked plan proof precedes operator-approved live proof. No cloud mutation is authorized by this design.
