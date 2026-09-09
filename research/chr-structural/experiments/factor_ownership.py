#!/usr/bin/env python3
"""Run the prospectively registered overlap attribution with unchanged ownership endpoints."""
import hashlib
from pathlib import Path
import finite_ownership as gate

gate.OUT=gate.ROOT/'docs/experiments/results/s06-overlap-factor-attribution'
gate.BIN=Path('/tmp/s06-overlap-factor-12f1a409e')
if __name__=='__main__':
    gate.OUT.mkdir(exist_ok=True)
    assert not (gate.OUT/'runs.jsonl').exists()
    paths=[Path(__file__).resolve(),gate.ROOT/'docs/experiments/registrations/S06-overlap-factor-attribution.md']
    (gate.OUT/'driver.sha256').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p}\n' for p in paths))
    gate.run()
    gate.audit()
