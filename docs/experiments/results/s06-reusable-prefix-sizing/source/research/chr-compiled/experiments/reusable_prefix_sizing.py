#!/usr/bin/env python3
"""Registered reusable-prefix exploratory lifecycle sizing and exact replay."""
from pathlib import Path
import sys, json, hashlib, random, subprocess, shutil, itertools
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'research/chr-direct-conditional/experiments'))
import demand_sizing as pilot
import demand_sizing_analysis as analysis
pilot.BIN = Path('/tmp/chr-reusable-prefix-8cedb32d')
pilot.OUT = ROOT / 'docs/experiments/results/s06-reusable-prefix-sizing'
MODES = ['original', 'sealed', 'lowered', 'lowered-sealed', 'reusable', 'rebuilt']
FAMILIES = ['plain1', 'plain4', 'plain16', 'choice1', 'choice4', 'fail1', 'fail4']
CONFIGS = list(itertools.product(MODES, FAMILIES, [1,8,32], [1,8], [False,True]))


def main():
    pilot.OUT.mkdir(exist_ok=False)
    files = [p for folder in ['research/chr-compiled', 'research/chr-persistent', 'research/chr-observe', 'crates/chr-syntax'] for p in (ROOT / folder).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml']]
    files += [ROOT / p for p in ['Cargo.toml','Cargo.lock','research/chr-direct-conditional/tests/runtime_support/mod.rs','research/chr-direct-conditional/experiments/demand_sizing.py','research/chr-direct-conditional/experiments/demand_sizing_analysis.py','research/chr-compiled/experiments/reusable_prefix_sizing.py','docs/experiments/registrations/S06-reusable-prefix-sizing.md']]
    freeze = dict(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(), cpu=pilot.CPU, rustc=subprocess.check_output(['rustc','-Vv'],text=True), sources={}, binaries={}, configs=CONFIGS)
    for p in files:
        rel = p.relative_to(ROOT)
        freeze['sources'][str(rel)] = hashlib.sha256(p.read_bytes()).hexdigest()
        target = pilot.OUT / 'source' / rel
        target.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(p,target)
    for name in ['ordinary','meter']:
        p=pilot.BIN/name;freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
    (pilot.OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    check=subprocess.run([str(pilot.BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
    (pilot.OUT/'meter-check.log').write_text((check.stdout+check.stderr).rstrip()+'\n');assert check.returncode==0
    indices=list(range(len(CONFIGS)));random.Random(7401).shuffle(indices)
    for j,i in enumerate(indices):
        pilot.run('ordinary',i,*CONFIGS[i])
        if (j+1)%84==0:print(f'ordinary {j+1}/504',flush=True)
    for rep in range(2):
        for i,c in enumerate(CONFIGS):pilot.run('allocation',rep*len(CONFIGS)+i,*c)
        print(f'allocation repetition {rep+1}/2',flush=True)
    for i,m in enumerate(MODES):
        for kind in ['cancel','cancel-meter']:pilot.run(kind,i,m,'choice4',8,2,True,1)


def summarize():
    freeze=json.loads((pilot.OUT/'freeze.json').read_text())
    for p,h in freeze['sources'].items():assert hashlib.sha256((pilot.OUT/'source'/p).read_bytes()).hexdigest()==h
    rows=[]
    def read(kind,i,config):
        p=pilot.OUT/f'{kind}-{i:03}.json';k,r,wall=analysis.load(p)
        assert tuple(k)==tuple(config)
        assert r['meter']==(kind in ['allocation','cancel-meter'])
        for j,s in enumerate(r['samples']):
            assert s['depth']==config[2]+j%2
            assert s['complete']==(not kind.startswith('cancel') or j%2==1)
            if s['complete']:assert s['answers']==(0 if config[1].startswith('fail') else 2**(int(config[1][-1]) if config[1].startswith('choice') else 0))
        if r['meter']:
            assert r['source_build']['memory']['live_start']==r['prepared_drop']['memory']['live_end']
            for s in r['samples']:assert s['input_build']['memory']['live_start']==s['answer_drop']['memory']['live_end']
        return r
    for i,c in enumerate(CONFIGS):
        r=read('ordinary',i,c);a=read('allocation',i,c);b=read('allocation',len(CONFIGS)+i,c)
        assert [p['memory'] for p in analysis.phases(a)]==[p['memory'] for p in analysis.phases(b)]
        rows.append(dict(config=c,**analysis.timing(r),allocation_inclusive=analysis.allocations(a),query_phase_ns={p:sum(s[p]['ns'] for s in r['samples']) for p in analysis.PHASES},primary_requested_bytes=sum(p['memory']['requested_bytes'] for p in analysis.phases(a))-a['source_build']['memory']['requested_bytes']-sum(s['input_build']['memory']['requested_bytes'] for s in a['samples'])))
    for i,m in enumerate(MODES):
        for kind in ['cancel','cancel-meter']:read(kind,i,(m,'choice4',8,2,True))
    summary=dict(status='exploratory single ordinary timing per cell; no inferential ranking',ordinary=504,allocation=1008,cancellation=12,exact_allocation_replays=504,rows=rows)
    (pilot.OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    for r in rows:
        if r['config'][2:]==[32,8,True] or tuple(r['config'][2:])==(32,8,True):print(r['config'][:2],round(r['engine_ns']/1e6,3),r['primary_requested_bytes'],flush=True)

if __name__=='__main__':
    if '--analyze' not in sys.argv:main()
    summarize()
