#!/usr/bin/env python3
"""Issue771 approved authority-only template/schema versioning; no old-set edits."""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/templates/prompts/1.0.4'
DEST = ROOT / 'docs/templates/prompts/1.0.5'
NOTICE = ('Authority notice: C-SDLC v3 is operational after V3-F/#505 and merged PR #591.\n'
          'Authority requires the authenticated canonical native selector and reconciliation\n'
          'receipt; missing or stale proof suspends authority. Retained typed v2 requires\n'
          'explicit issue-scoped rollback or remediation approval.')
REPLACEMENTS = {
    '1.0.4': '1.0.5',
    '`csdlc-bind`': 'native v3 `csdlc bind`',
    'Use the typed C-SDLC v2 operator skills and Rust binaries for lifecycle routing.':
        'Use authenticated native C-SDLC v3 for lifecycle routing. Retained typed v2 requires explicit issue-scoped rollback or remediation approval.',
    'typed `csdlc-finish`': 'native v3 `csdlc finish`',
    'Authority notice: V3-F/#505 is the pending tooling changeover decision; until\nthat operator-reviewed cutover is approved, merged, and terminally reconciled,\nC-SDLC v2 remains live authority.': NOTICE,
}
def transform(value):
    if isinstance(value, str):
        for old, new in REPLACEMENTS.items(): value = value.replace(old, new)
        return value
    if isinstance(value, list): return [transform(v) for v in value]
    if isinstance(value, dict): return {k:transform(v) for k,v in value.items()}
    return value
for source in sorted(BASE.rglob('*')):
    if not source.is_file(): continue
    target = DEST / source.relative_to(BASE)
    if source.suffix == '.json':
        value = transform(json.loads(source.read_text()))
        if source.name == 'sor.structure.json':
            value['authority_notice'] = NOTICE.removeprefix('Authority notice: ').replace('\n', ' ')
            value['locked_lines'][:0] = [dict(heading_path=['<dynamic-heading>'], text=line) for line in NOTICE.splitlines()]
        text = json.dumps(value, indent=2) + '\n'
    else: text = transform(source.read_text())
    if target.exists() and target.read_text() != text: raise SystemExit(f'refuse conflicting target: {target.relative_to(ROOT)}')
    target.parent.mkdir(parents=True, exist_ok=True); target.write_text(text)
print('Generated1.0.5 from immutable1.0.4 with explicit authority replacements; inherited Markdown structure unchanged.')
