#!/usr/bin/env python3
"""Registered ordinary-allocator finite-structure timing pilot."""
import collections,hashlib,json,random,statistics,subprocess
from pathlib import Path
import finite_ownership as gate
ROOT=gate.ROOT
OUT=ROOT/'docs/experiments/results/s06-finite-timing-pilot'
BIN=Path('/tmp/s06-finite-ordinary-e5a6b1d72')
FIELDS=['mode','family','width','retain','cancel']

def run():
    OUT.mkdir(exist_ok=True)
    assert not (OUT/'runs.jsonl').exists()
    paths=[ROOT/p for p in ['research/chr-structural/src/finite.rs','research/chr-structural/Cargo.toml','research/chr-structural/examples/finite_cost.rs','research/chr-structural/examples/support/finite_cost.rs','research/chr-structural/examples/support/finite_oracle.rs','research/chr-structural/experiments/finite_timing.py','research/chr-structural/experiments/finite_ownership.py','docs/experiments/registrations/S06-finite-timing-pilot.md','Cargo.lock']]+[BIN]
    (OUT/'freeze.sha256').write_text(''.join(f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p}\n' for p in paths))
    for name,cmd in [('source-head.txt',['git','rev-parse','HEAD']),('toolchain.txt',['rustc','-Vv']),('host.txt',['uname','-a'])]:
        (OUT/name).write_text(subprocess.check_output(cmd,cwd=ROOT,text=True))
    count=0
    with (OUT/'runs.jsonl').open('w') as log:
        for rep in range(5):
            cells=gate.CELLS.copy();random.Random(760901+rep).shuffle(cells)
            for cell in cells:
                try:r=subprocess.run([str(BIN),*map(str,cell)],cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=gate.bounds)
                except subprocess.TimeoutExpired:
                    (OUT/'failure.json').write_text(json.dumps(dict(rep=rep,cell=cell,error='timeout')));raise
                if r.returncode:
                    (OUT/'failure.json').write_text(json.dumps(dict(rep=rep,cell=cell,error=r.stderr)));raise RuntimeError(r.stderr[-1000:])
                row=json.loads(r.stdout);assert tuple(row[k] for k in FIELDS)==cell
                row['rep']=rep;log.write(json.dumps(row,separators=(',',':'))+'\n');log.flush();count+=1
                if count%180==0:print(f'{count}/1800',flush=True)
    assert count==1800

def interval(xs):return dict(median=statistics.median(xs),minimum=min(xs),maximum=max(xs))
def audit():
    for line in (OUT/'freeze.sha256').read_text().splitlines():
        digest,name=line.split('  ',1);assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==digest,name
    groups=collections.defaultdict(list)
    for line in (OUT/'runs.jsonl').open():
        r=json.loads(line);key=tuple(r[k] for k in FIELDS);assert key in gate.CELLS;groups[key].append(r)
        phases=[('prepare',0)]+[(p,q) for q in range(4) for p in ['input','setup','execute_observe','query_drop','consumer_drop']]+[('prepared_drop',0)]
        assert [(x['phase'],x['query']) for x in r['rows']]==phases
        assert all(x['memory'] is None for x in r['rows'])
        n=1 if r['width']==0 else {'all':2**r['width'],'equal':2,'selective':2,'empty':0,'redundant':1,'overlap':2**r['width']}[r['family']]
        assert r['answers']==[min(n,1) if r['cancel'] else n]*4
        assert len(r['first'])==4
        for q,first in enumerate(r['first']):
            elapsed=next(x['ns'] for x in r['rows'] if x['phase']=='execute_observe' and x['query']==q)
            assert (0<first<=elapsed) if n else first==0
    assert set(groups)==set(gate.CELLS)
    summaries=[]
    for key,rs in sorted(groups.items()):
        assert sorted(r['rep'] for r in rs)==list(range(5))
        s=dict(zip(FIELDS,key))
        s['total_ns']=interval([sum(x['ns'] for x in r['rows']) for r in rs])
        s['without_input_ns']=interval([sum(x['ns'] for x in r['rows'] if x['phase']!='input') for r in rs])
        s['phase_ns']={p:interval([sum(x['ns'] for x in r['rows'] if x['phase']==p) for r in rs]) for p in ['prepare','input','setup','execute_observe','query_drop','consumer_drop','prepared_drop']}
        s['first_ns']=[interval([r['first'][q]+next(x['ns'] for x in r['rows'] if x['phase']=='setup' and x['query']==q) for r in rs]) if rs[0]['answers'][q] else None for q in range(4)]
        summaries.append(s)
    (OUT/'summary.json').write_text(json.dumps(summaries,indent=2,sort_keys=True)+'\n')
    receipt=dict(processes=1800,cells=360,repetitions=5,interpretation='exploratory ordinary-allocator medians and ranges; no gain/loss classification')
    (OUT/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n');print(receipt)
if __name__=='__main__':
    import sys
    if len(sys.argv)==1:run()
    else:assert sys.argv[1:]==['--audit']
    audit()
