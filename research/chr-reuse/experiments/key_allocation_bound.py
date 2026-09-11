"""Optimistic allocation subtraction; never a runtime or implemented-repair claim."""
import json
from pathlib import Path
BASE=Path(__file__).resolve().parents[3]/'docs/experiments/results/s05-key-allocation'
a=json.loads((BASE/'audit.json').read_text());lookup={(r['family'],r['depth'],r['distinct'],r['offset'],r['mode']):r for r in a['summary']};rows=[]
for r in a['summary']:
 if r['mode']=='memo':
  direct=lookup[r['family'],r['depth'],r['distinct'],r['offset'],'direct'];remaining=r['requested']-r['stage_bytes'][0]
  rows.append(dict(family=r['family'],depth=r['depth'],distinct=r['distinct'],offset=r['offset'],memo_requested=r['requested'],key_bytes=r['stage_bytes'][0],zero_key_remainder=remaining,direct_requested=direct['requested'],still_exceeds_direct=remaining>direct['requested']))
print(json.dumps(dict(interpretation='Optimistic requested-byte subtraction, not an implemented repair or elapsed-time bound.',comparisons=len(rows),still_exceeds_direct=sum(r['still_exceeds_direct'] for r in rows),rows=rows),indent=2))
