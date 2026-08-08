# Issue 22 Design

Add Ruby 3.3.6 to the immutable ADL builder image from the official ruby-lang source archive, verify the pinned archive SHA-256 before compilation, and record the installed runtime in the builder provenance file.

Extend the existing builder preflight to execute `ruby --version`, a minimal Ruby expression, and one repository Ruby validator self-test before any requested validation command runs. Preserve every existing immutable-image, architecture, Rust toolchain, cache, and digest check.
