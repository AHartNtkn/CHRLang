#!/usr/bin/env python3
"""Execute the prospectively registered S05 complete-caller pilot."""
import hashlib
import itertools
import json
import random
import resource
import subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-caller-cost-pilot'
BIN = Path('/tmp/chr-call-pilot-48bb729f6')
MODES = ['direct', 'whole', 'scan', 'sealed', 'call-direct', 'call-memo', 'lowered']
CELLS = [(m, *c, 0) for c in itertools.product([0,1],[0,32],[0,8],[0,8],[1,8],[0,1]) for m in MODES]
CANCEL = [(m,b,32,8,8,8,d,1) for b,d in itertools.product([0,1],[0,1]) for m in MODES]
OUT.mkdir(exist_ok=True)
assert not (OUT/'runs.jsonl').exists(), 'Existing run must be audited, not overwritten'
sources = ['research/chr-reuse/examples/call_cost.rs','research/chr-reuse/examples/support/call_cost.rs','research/chr-reuse/experiments/call_cost_pilot.py','research/chr-reuse/src/calls.rs','research/chr-persistent/src/state.rs','research/chr-persistent/src/continuations.rs','Cargo.lock']
(OUT/'freeze.sha256').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p}\n' for p in [*(ROOT/s for s in sources), BIN/'ordinary',BIN/'meter']))
(OUT/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
(OUT/'host.txt').write_text(subprocess.check_output(['uname','-a'],text=True)+next((line+'\n' for line in Path('/proc/cpuinfo').read_text().splitlines() if line.startswith('model name')),''))
(OUT/'source-head.txt').write_text(subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True))
(OUT/'source.patch').write_bytes(subprocess.check_output(['git','diff','--binary','--','research/chr-reuse','research/chr-persistent'],cwd=ROOT))
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
count=0
with (OUT/'runs.jsonl').open('w') as log:
    for kind,reps,seed in [('ordinary',5,751903),('meter',2,751913)]:
        for rep in range(reps):
            cells=CELLS.copy()
            if kind=='meter' or rep==0:
                cells+=CANCEL
            random.Random(seed+rep).shuffle(cells)
            for cell in cells:
                result=subprocess.run([str(BIN/kind),*map(str,cell)],capture_output=True,text=True,timeout=60,preexec_fn=limits,cwd=ROOT)
                if result.returncode:
                    (OUT/'failure.json').write_text(json.dumps({'kind':kind,'rep':rep,'cell':cell,'returncode':result.returncode,'stdout':result.stdout,'stderr':result.stderr},indent=2))
                    raise RuntimeError(f'pilot failure {kind} {rep} {cell}: {result.stderr[-1500:]}')
                row=json.loads(result.stdout)
                row.update(kind=kind,rep=rep)
                log.write(json.dumps(row,separators=(',',':'))+'\n');log.flush();count+=1
                if count%100==0:print(f'{count}/3220 processes completed',flush=True)
assert count==3220,count
print(f'Completed {count} processes',flush=True)
