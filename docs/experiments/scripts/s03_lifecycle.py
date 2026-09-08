#!/usr/bin/env python3
"""Freeze and run the prospectively registered S03 pilot; never restart a run implicitly."""
import hashlib,json,os,platform,random,resource,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s03-lifecycle'
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{min(os.sched_getaffinity(0))})
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
def invoke(command):
    p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True,timeout=60,preexec_fn=limits)
    return dict(command=list(map(str,command)),exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr)
def binary(kind):return ROOT/f'target/s03-{kind}/release/examples/s03_lifecycle'
def prepare():
    assert not (OUT/'raw.jsonl').exists(),'existing comparative run requires explicit disposition'
    cells=[]
    for family in ['binary','nested','duplicate','plain','shared','discriminate']:
        modes=['global-scan','global-index','active-index','conditional','graph','words'] if family in ['binary','nested','duplicate'] else ['global-scan','global-index','conditional','graph']
        cells.extend(dict(mode=m,family=family) for m in modes)
    order=[]
    for kind,seed,repetitions in [('time',30301,5),('memory',30302,2)]:
        rng=random.Random(seed)
        for repetition in range(repetitions):
            block=cells.copy();rng.shuffle(block)
            order.extend(dict(kind=kind,repetition=repetition,**cell) for cell in block)
    (OUT/'order.json').write_text(json.dumps(order,indent=2)+'\n')
    checks=[]
    for kind in ['time','memory']:
        for command in ['validate','calibrate']+(['meter-check'] if kind=='memory' else []):
            r=invoke([str(binary(kind)),command]);checks.append(r)
            (OUT/f'{kind}-{command}.log').write_text((r['stdout']+r['stderr']).rstrip()+'\n')
            assert r['exit_code']==0,r
    features={}
    for kind in ['time','memory']:
        cmd=['cargo','tree','-p','chr-direct-conditional','--no-default-features','-e','features']
        if kind=='memory':cmd+=['--features','alloc-meter']
        p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,check=True)
        (OUT/f'features-{kind}.log').write_text(p.stdout)
        features[kind]=cmd
    paths=subprocess.check_output(['git','ls-files','-z'],cwd=ROOT).decode().split('\0')
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock')) and not p.startswith(('.agents/','.codex/'))]
    paths+=['research/chr-direct-conditional/examples/s03_lifecycle.rs','docs/experiments/scripts/s03_lifecycle.py','docs/experiments/registrations/S03-lifecycle-pilot.md']
    freeze=dict(head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),source_sha256={p:digest(ROOT/p) for p in sorted(set(paths))},binary_sha256={k:digest(binary(k)) for k in ['time','memory']},order_sha256=digest(OUT/'order.json'),compiler=subprocess.check_output(['rustc','-Vv'],cwd=ROOT,text=True),machine=platform.uname()._asdict(),affinity=sorted(os.sched_getaffinity(0)),cpu=min(os.sched_getaffinity(0)),features=features,checks=checks)
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    print('frozen 30 cells; 150 primary and 60 memory processes; runner/oracle validation and meter checks passed',flush=True)
def run():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for p,h in freeze['source_sha256'].items():assert digest(ROOT/p)==h,p
    for k,h in freeze['binary_sha256'].items():assert digest(binary(k))==h,k
    assert digest(OUT/'order.json')==freeze['order_sha256']
    assert min(os.sched_getaffinity(0))==freeze['cpu']
    order=json.loads((OUT/'order.json').read_text())
    with (OUT/'raw.jsonl').open('x') as f:
        for index,cell in enumerate(order):
            command=[str(binary(cell['kind'])),cell['mode'],cell['family']]
            try:r=invoke(command)
            except subprocess.TimeoutExpired as e:
                r=dict(command=command,exit_code=None,timeout=True,stdout=str(e.stdout or ''),stderr=str(e.stderr or ''))
            record=dict(index=index,**cell,**r)
            if r['exit_code']==0:record['measurement']=json.loads(r['stdout'])
            f.write(json.dumps(record)+'\n');f.flush()
            assert r['exit_code']==0,record
            assert len(record['measurement']['queries'])==16
            assert record['measurement']['metered']==(cell['kind']=='memory')
            if (index+1)%30==0:print(f'{index+1}/{len(order)} complete',flush=True)
    print('all registered processes complete',flush=True)
if __name__=='__main__':
    {'prepare':prepare,'run':run}[sys.argv[1]]()
