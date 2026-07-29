# Structured Output Record

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Updated the integrated Synthetic Minds Podcast studio copy, corrected the logo to the supplied SVG, replaced fake historical episodes with proposed launch topics numbered 1-10, fixed guest/contact/video/FAQ/footer truth, added a podcast email contact button, removed the bare footer email link, and preserved generated studio/audio/RSS behavior.

## Artifacts

- .csdlc/evidence/5717
- /Volumes/FastWork/adl-podcast-launch-5717/audio-render/audio_manifest.json
- http://127.0.0.1:8915/studio/podcast-studio.html

## Execution

- .csdlc/issues/5715/index.json
- .csdlc/issues/5715/audit.jsonl
- .csdlc/issues/5717
- .csdlc/prepared/issues/5717
- demos/podcast/studio-reference/REFERENCE_DIGESTS.txt
- demos/podcast/studio-reference/podcast-studio.html
- demos/podcast/studio-reference/uploads/agent-logic-logo.svg
- demos/podcast/studio/REFERENCE_DIGESTS.txt
- demos/podcast/studio/podcast-studio.html
- demos/podcast/studio/reference.sha256
- demos/podcast/studio/uploads/agent-logic-logo.svg

## Validation

[
  {
    "command": [
      "python3",
      "-c",
      "from pathlib import Path\npaths=[Path('demos/podcast/studio-reference/podcast-studio.html'),Path('demos/podcast/studio/podcast-studio.html')]\nrequired=['<title>Synthetic Minds Podcast</title>','Synthetic <span style=\"color:oklch(55% 0.2 265); font-weight:600;\">Minds</span> Podcast','Special guests join us occasionally.','href=\"mailto:podcast@agent-logic.ai\"','Contact the studio','href=\"../feed.xml\"','Frequently Asked Questions','No. Synthetic Minds Podcast is audio-first for launch.','agent-logic-logo.svg','height:56px','height:34px','num: 1','num: 10','DeepSeek Drops By','© 2026 Agent Logic, Inc.</div>']\nfor path in paths:\n    text=path.read_text()\n    footer=text.split('<!-- FOOTER -->',1)[1]\n    missing=[s for s in required if s not in text]\n    forbidden=[s for s in ['href=\"#\"','<svg width=\"18\"','translate(120, 190)','New guests drop in most weeks','num: 42','num: 41','num: 40','num: 39','num: 38','Most episodes ship as audio-first; select episodes get a full video cut.','Frequently asked questions','YouTube'] if s in text]\n    if 'podcast@agent-logic.ai</a>' in footer:\n        forbidden.append('bare footer email link')\n    if missing or forbidden:\n        raise SystemExit(f'{path}: missing={missing} forbidden={forbidden}')\nfor path in [Path('demos/podcast/studio-reference/uploads/agent-logic-logo.svg'),Path('demos/podcast/studio/uploads/agent-logic-logo.svg')]:\n    text=path.read_text()\n    forbidden=[s for s in ['translate(120, 190)'] if s in text]\n    if forbidden:\n        raise SystemExit(f'{path}: forbidden={forbidden}')\nfor path in [Path('demos/podcast/feed.xml'),Path('demos/podcast/audio/meet-the-ai-coworkers.wav')]:\n    if not path.exists():\n        raise SystemExit(f'missing linked asset: {path}')\nprint('studio_copy_contract: PASS')"
    ],
    "purpose": "Prove the reference and served studio HTML contain the operator-requested copy/logo/episode/video/footer/contact fixes, prove the subscribe/contact links are wired, and prove no stale fake episode, video-platform copy, footer email link, or old inline logo artifact remains.",
    "outcome": "passed",
    "evidence_ref": "podcast-studio-copy-contract.log"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
