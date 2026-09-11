#!/usr/bin/env python3
"""Compatibility entrypoint for #835's current release projection validation.
PVF: deterministic local documentation contract, small, required for #835.
"""
from pathlib import Path
import runpy
import sys
ROOT = Path(__file__).resolve().parents[6]
sys.argv = [str(ROOT / '.csdlc/prepared/issues/835/project_release.py'), '--check', *sys.argv[1:]]
runpy.run_path(sys.argv[0], run_name='__main__')
