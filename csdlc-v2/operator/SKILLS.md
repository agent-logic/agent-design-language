# C-SDLC v2 operator skills

The nine skills in `skills.json` are thin typed routes. Skills select a binary/subcommand, collect typed input, and display typed output. They never edit Markdown, mutate canonical state directly, invoke shell/Python lifecycle logic, or infer success from prose.

V2 is explicit opt-in during Gate 10A. V1 remains the repository default, installed, and runnable. Install only into `.adl/bin/csdlc-v2/`, never shared `.adl/bin/`. `csdlc-install verify` fails unless the v1 coexistence inventory and regular executable v2 binary set are complete.
