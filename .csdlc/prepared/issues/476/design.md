# Issue #476 Design

Apply the preserved post-merge review-truth repair from `ed454a2461daccf95f75191ccea69d7df9ae06df` as one narrow follow-on to #315. The change updates only typed #315 planning/review projections, narrows the local validator claim to evidence it actually proves, removes its unused GitHub helper, and corrects the remediation README payload type. Runtime behavior is unchanged.

The follow-on must preserve typed C-SDLC authority: exact-head review and merge/terminal truth remain owned by `csdlc-review` and `csdlc-finish`, not by the local Ruby validator. It must not inspect or execute #269.
