#!/usr/bin/env python3
"""Prospective original/repaired graph comparison with fresh general controls."""
import sys
sys.dont_write_bytecode=True
import hashlib,json,os,platform,random,subprocess
from pathlib import Path
import s03_lifecycle as base
ROOT=base.ROOT;OUT=ROOT/'docs/experiments/results/s03-context-repair'
ORIGINAL=ROOT/'docs/experiments/results/s03-lifecycle/freeze.json'
def binary(kind,mode='graph'):
    target=f's03-{kind}' if mode=='graph-original' else f's03-repair-{kind}'
    return ROOT/f'target/{target}/release/examples/s03_lifecycle'
def prepare():
    assert not (OUT/'raw.jsonl').exists()
    original=json.loads(ORIGINAL.read_text())
    assert min(os.sched_getaffinity(0))==original['cpu']
    for kind,digest in original['binary_sha256'].items():assert base.digest(binary(kind,'graph-original'))==digest
    runner='research/chr-direct-conditional/examples/s03_lifecycle.rs'
    assert base.digest(ROOT/runner)==original['source_sha256'][runner]
    cells=[]
    for family in ['binary','nested','duplicate','plain','shared','discriminate']:
        modes=['global-scan','global-index','active-index','conditional','graph','words'] if family in ['binary','nested','duplicate'] else ['global-scan','global-index','conditional','graph']
        cells.extend(dict(mode=m,family=family) for m in modes+['graph-original'])
    order=[]
    for kind,seed,repetitions in [('time',30311,5),('memory',30312,2)]:
        rng=random.Random(seed)
        for repetition in range(repetitions):
            block=cells.copy();rng.shuffle(block)
            order.extend(dict(kind=kind,repetition=repetition,**cell) for cell in block)
    (OUT/'order.json').write_text(json.dumps(order,indent=2)+'\n')
    checks=[]
    for kind in ['time','memory']:
        for command in ['validate','calibrate']+(['meter-check'] if kind=='memory' else []):
            r=base.invoke([str(binary(kind)),command]);checks.append(r)
            (OUT/f'{kind}-{command}.log').write_text((r['stdout']+r['stderr']).rstrip()+'\n');assert r['exit_code']==0,r
    paths=list(original['source_sha256'])+['docs/experiments/scripts/s03_context_repair.py','docs/experiments/registrations/S03-context-repair.md']
    freeze=dict(original_freeze_sha256=base.digest(ORIGINAL),source_sha256={p:base.digest(ROOT/p) for p in sorted(set(paths))},binary_sha256={f'{k}/{m}':base.digest(binary(k,m)) for k in ['time','memory'] for m in ['graph-original','graph']},order_sha256=base.digest(OUT/'order.json'),compiler=subprocess.check_output(['rustc','-Vv'],cwd=ROOT,text=True),machine=platform.uname()._asdict(),affinity=sorted(os.sched_getaffinity(0)),cpu=min(os.sched_getaffinity(0)),checks=checks)
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n');print('frozen 36 cells, 252 processes, unchanged runner and original binaries',flush=True)
def run():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for p,h in freeze['source_sha256'].items():assert base.digest(ROOT/p)==h,p
    for km,h in freeze['binary_sha256'].items():assert base.digest(binary(*km.split('/')))==h
    assert base.digest(OUT/'order.json')==freeze['order_sha256']
    assert min(os.sched_getaffinity(0))==freeze['cpu']
    order=json.loads((OUT/'order.json').read_text())
    with (OUT/'raw.jsonl').open('x') as f:
        for index,cell in enumerate(order):
            mode='graph' if cell['mode']=='graph-original' else cell['mode']
            command=[str(binary(cell['kind'],cell['mode'])),mode,cell['family']]
            try:r=base.invoke(command)
            except subprocess.TimeoutExpired as e:r=dict(command=command,exit_code=None,timeout=True,stdout=str(e.stdout or ''),stderr=str(e.stderr or ''))
            record=dict(index=index,**cell,**r)
            if r['exit_code']==0:record['measurement']=json.loads(r['stdout'])
            f.write(json.dumps(record)+'\n');f.flush();assert r['exit_code']==0,record
            assert len(record['measurement']['queries'])==16
            assert record['measurement']['metered']==(cell['kind']=='memory')
            if (index+1)%36==0:print(f'{index+1}/{len(order)} complete',flush=True)
if __name__=='__main__':{'prepare':prepare,'run':run}[sys.argv[1]]()
