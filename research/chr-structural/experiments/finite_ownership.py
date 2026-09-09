#!/usr/bin/env python3
"""Execute and audit the registered finite-cost ownership gate; never use meter time as speed evidence."""
import hashlib
import itertools
import json
import random
import resource
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s06-finite-cost-ownership-gate'
BIN=Path('/tmp/s06-finite-ownership-corrected-6539a109c')
MODES=['lazy','reduced','enumerate','enum-reuse','lowered']
FAMILIES=['all','equal','selective','empty','redundant','overlap']
CELLS=list(itertools.product(MODES,FAMILIES,[0,4,6],[0,1],[0,1]))

def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))

def run():
    OUT.mkdir(exist_ok=True)
    assert not (OUT/'runs.jsonl').exists(),'Existing evidence must be audited, not overwritten'
    paths=[ROOT/p for p in ['research/chr-structural/src/finite.rs','research/chr-structural/Cargo.toml','research/chr-structural/examples/finite_cost.rs','research/chr-structural/examples/support/finite_cost.rs','research/chr-structural/examples/support/finite_oracle.rs','research/chr-structural/tests/finite_cost.rs','research/chr-structural/experiments/finite_ownership.py','research/chr-compiled/experiments/meter.rs','Cargo.lock']]+[BIN]
    (OUT/'freeze.sha256').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p}\n' for p in paths))
    (OUT/'source-head.txt').write_text(subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True))
    (OUT/'source.patch').write_bytes(subprocess.check_output(['git','diff','--binary','--','research/chr-structural'],cwd=ROOT))
    (OUT/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    count=0
    with (OUT/'runs.jsonl').open('w') as log:
        for rep in range(2):
            cells=CELLS.copy();random.Random(760611+rep).shuffle(cells)
            for cell in cells:
                try:
                    result=subprocess.run([str(BIN),*map(str,cell)],capture_output=True,text=True,cwd=ROOT,timeout=60,preexec_fn=bounds)
                except subprocess.TimeoutExpired:
                    (OUT/'failure.json').write_text(json.dumps(dict(rep=rep,cell=cell,error='60-second process cutoff')))
                    raise
                if result.returncode:
                    (OUT/'failure.json').write_text(json.dumps(dict(rep=rep,cell=cell,returncode=result.returncode,stderr=result.stderr)))
                    raise RuntimeError(f'Failed {rep} {cell}: {result.stderr[-1000:]}')
                row=json.loads(result.stdout);row['rep']=rep
                assert tuple(row[k] for k in ['mode','family','width','retain','cancel'])==cell
                log.write(json.dumps(row,separators=(',',':'))+'\n');log.flush();count+=1
                if count%60==0:print(f'{count}/720 processes completed',flush=True)
    assert count==720

def audit():
    groups={}
    for line in (OUT/'runs.jsonl').open():
        r=json.loads(line);key=tuple(r[k] for k in ['mode','family','width','retain','cancel'])
        assert key in CELLS
        groups.setdefault(key,[]).append(r)
        phases=[('prepare',0)]+[(phase,q) for q in range(4) for phase in ['input','setup','execute_observe','query_drop','consumer_drop']]+[('prepared_drop',0)]
        assert [(x['phase'],x['query']) for x in r['rows']]==phases
        width=r['width'];family=r['family']
        expected=1 if width==0 else {'all':2**width,'equal':2,'selective':2,'empty':0,'redundant':1,'overlap':2**width}[family]
        assert r['answers']==[min(expected,1) if r['cancel'] else expected]*4
        ms=[x['memory'] for x in r['rows']]
        assert ms[0]['live_start']==ms[-1]['live_end']
        assert all(a['live_end']==b['live_start'] for a,b in zip(ms,ms[1:]))
        assert all(m['peak_live']>=max(m['live_start'],m['live_end']) for m in ms)
    assert set(groups)==set(CELLS)
    summary=[]
    for key,rs in sorted(groups.items()):
        rs.sort(key=lambda r:r['rep']);assert [r['rep'] for r in rs]==[0,1]
        signature=lambda r:[(x['phase'],x['query'],x['memory']) for x in r['rows']]
        assert signature(rs[0])==signature(rs[1]),key
        r=rs[0];baseline=r['rows'][0]['memory']['live_start']
        item={k:r[k] for k in ['mode','family','width','retain','cancel']}
        item.update(requested_bytes=sum(x['memory']['requested_bytes'] for x in r['rows']),peak_growth=max(x['memory']['peak_live'] for x in r['rows'])-baseline,
                    phases=[dict(phase=x['phase'],query=x['query'],**x['memory']) for x in r['rows']])
        summary.append(item)
    (OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    receipt=dict(processes=720,cells=360,exact_replays=360,owner_restorations=720,interpretation='Requested allocation and ownership only; no speed claims')
    (OUT/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps(receipt),flush=True)

if __name__=='__main__':
    import sys
    if len(sys.argv)==1:run()
    else:assert sys.argv[1:]==['--audit']
    audit()
