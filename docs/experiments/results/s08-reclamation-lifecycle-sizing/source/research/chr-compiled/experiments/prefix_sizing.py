#!/usr/bin/env python3
"""Registered S02 sizing; uses the established bounded process launcher."""
from pathlib import Path
import sys,json,hashlib,random,subprocess,platform
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import demand_sizing as pilot
import demand_sizing_analysis as analysis
pilot.BIN=Path('/tmp/chr-prefix-sizing-17444e43')
pilot.OUT=ROOT/'docs/experiments/results/s06-prefix-sizing'
MODES=['original','sealed','lowered','lowered-sealed']
FAMILIES=['plain1','plain4','choice1','choice4']
def main():
    files=[ROOT/p for p in ['research/chr-compiled/examples/prefix_cost.rs','research/chr-compiled/examples/support/prefix_source.rs','research/chr-compiled/src/pure_prefix.rs','research/chr-compiled/src/regions.rs','research/chr-direct-conditional/experiments/demand_sizing.py','research/chr-compiled/experiments/prefix_sizing.py','docs/experiments/registrations/S06-prefix-lifecycle-sizing.md','Cargo.lock']]
    freeze=dict(base_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),cpu=pilot.CPU,host=platform.platform(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),sha256={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},binaries={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [pilot.BIN/'ordinary',pilot.BIN/'meter']})
    (pilot.OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    check=subprocess.run([str(pilot.BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
    (pilot.OUT/'meter-check.log').write_text(check.stdout+check.stderr);assert check.returncode==0
    for i,m in enumerate(MODES):
        pilot.run('cancel',i,m,'choice4',8,2,True,1)
        pilot.run('cancel-meter',i,m,'choice4',8,2,True,1)
    configs=[(m,f,n,q,r) for m in MODES for f in FAMILIES for n in [1,8,32] for q in [1,8] for r in [False,True]]
    assert len(configs)==192
    random.Random(7301).shuffle(configs)
    for i,c in enumerate(configs):
        pilot.run('ordinary',i,*c)
        if (i+1)%32==0:print(f'ordinary {i+1}/192',flush=True)
    configs=[(m,f,32,8,r) for m in MODES for f in FAMILIES for r in [False,True]]
    for repeat in range(2):
        for i,c in enumerate(configs):pilot.run('allocation',repeat*32+i,*c)
        print(f'allocation repeat {repeat+1}',flush=True)
def summarize():
    rows=[];mem={}
    for p in pilot.OUT.glob('allocation-*.json'):
        k,r,_=analysis.load(p);readings=[x['memory'] for x in analysis.phases(r)]
        if k in mem:assert mem[k][0]==readings
        else:mem[k]=(readings,analysis.allocations(r))
    assert len(mem)==32
    for p in pilot.OUT.glob('ordinary-*.json'):
        k,r,wall=analysis.load(p)
        rows.append(dict(mode=k[0],family=k[1],depth=k[2],queries=k[3],resource=k[4],**analysis.timing(r),wall_seconds=wall,allocation=mem[k][1] if k in mem else None))
    assert len(rows)==192
    (pilot.OUT/'analysis.json').write_text(json.dumps(dict(status='exploratory single samples, no confirmed rankings',rows=rows),indent=2)+'\n')
    for r in sorted(rows,key=lambda r:(r['family'],r['mode'])):
        if r['depth']==32 and r['queries']==8 and r['resource']:print(r['family'],r['mode'],round(r['engine_ns']/1e6,3),'ms',r['allocation']['requested_bytes'],'bytes')
if __name__=='__main__':
    if '--analyze' not in sys.argv:main()
    summarize()
