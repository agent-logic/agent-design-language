# Repository Authority Without Local-Only Dependencies — v0.92.1

Issue #432 is the opening prerequisite. Tracked code, plans, tests, and operational contracts must not depend on local untracked directories or machine-specific files.

Required sources are promoted into tracked repository paths. Generated operational binaries are resolved through the typed generation selector and installation contract, not referenced as tracked artifacts.

The gate scans every changed planning artifact and the complete milestone package before any execution lane begins.
