#!/usr/bin/env python3
"""Registered allocation/cancellation gate; no timing interpretation."""
from pathlib import Path
import sys,itertools,json,hashlib,subprocess,shutil
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-conditional/experiments'))
import demand_sizing as pilot
import demand_sizing_analysis as analysis
OUT=ROOT/'docs/experiments/results/s06-recursive-allocation-gate'
BIN=Path('/tmp/chr-recursive-meter-ede3d7f3')
BUILDS=[('off','original'),('off','sealed'),('on','original'),('on','sealed'),('on','contracted')]
FAMILIES=['pass','unary','nested','open','late','malformed','choice','multi','multi-choice','fail']
CONFIGS=[(build,mode,f,n,q,r) for build,mode in BUILDS for f,n,q,r in itertools.product(FAMILIES,[0,1,16],[1,4],[False,True])]

def main():
    OUT.mkdir(exist_ok=False);pilot.OUT=OUT
    files=[p for folder in ['research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax'] for p in (ROOT/folder).rglob('*') if p.is_file() and p.suffix in ['.rs','.toml']]
    files += [ROOT/p for p in ['Cargo.toml','Cargo.lock','research/chr-direct-conditional/tests/runtime_support/mod.rs','research/chr-direct-conditional/experiments/demand_sizing.py','research/chr-direct-conditional/experiments/demand_sizing_analysis.py','research/chr-compiled/experiments/recursive_allocation_gate.py','docs/experiments/registrations/S06-recursive-allocation-gate.md']]
    freeze=dict(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu=pilot.CPU,configs=CONFIGS,sources={},binaries={})
    for p in files:
        rel=p.relative_to(ROOT);freeze['sources'][str(rel)]=hashlib.sha256(p.read_bytes()).hexdigest()
        dest=OUT/'source'/rel;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dest)
    for build in ['off','on']:
        p=BIN/build/'meter';freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
        check=subprocess.run([str(p),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
        (OUT/f'meter-{build}.log').write_text((check.stdout+check.stderr).rstrip()+'\n');assert check.returncode==0
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    for rep in range(2):
        for i,(build,*config) in enumerate(CONFIGS):
            pilot.BIN=BIN/build;pilot.run('allocation',rep*600+i,*config)
        print(f'allocation repetition {rep+1}/2',flush=True)
    for i,((build,mode),family,cancel) in enumerate(itertools.product(BUILDS,FAMILIES,[0,1])):
        pilot.BIN=BIN/build;pilot.run('cancel-meter',i,mode,family,16,2,True,cancel)

def summarize():
    def read(kind,i,config,cancel=None):
        raw=json.loads((OUT/f'{kind}-{i:03}.json').read_text());build,mode,f,n,q,r=config
        command=[str(BIN/build/'meter'),mode,f,str(n),str(q),str(int(r))]
        if cancel is not None:command.append(str(cancel))
        assert raw['command']==command
        _,v,_=analysis.load(OUT/f'{kind}-{i:03}.json');assert v['meter'] and len(v['samples'])==q
        for j,s in enumerate(v['samples']):
            assert s['depth']==n+j%2 and s['complete']==(cancel is None or j%2==1)
            if s['complete']:assert s['answers']==(0 if f=='fail' else 4 if f=='multi-choice' else 2 if f=='choice' else 1)
            assert s['input_build']['memory']['live_start']==s['answer_drop']['memory']['live_end']
        assert v['source_build']['memory']['live_start']==v['prepared_drop']['memory']['live_end']
        return v
    rows=[]
    for i,c in enumerate(CONFIGS):
        a=read('allocation',i,c);b=read('allocation',600+i,c)
        assert [p['memory'] for p in analysis.phases(a)]==[p['memory'] for p in analysis.phases(b)]
        rows.append(dict(config=c,allocation_inclusive=analysis.allocations(a),primary_requested_bytes=sum(p['memory']['requested_bytes'] for p in analysis.phases(a))-a['source_build']['memory']['requested_bytes']-sum(s['input_build']['memory']['requested_bytes'] for s in a['samples'])))
    for i,((build,mode),f,cancel) in enumerate(itertools.product(BUILDS,FAMILIES,[0,1])):read('cancel-meter',i,(build,mode,f,16,2,True),cancel)
    (OUT/'summary.json').write_text(json.dumps(dict(allocation_processes=1200,cancellation_processes=100,exact_replays=600,all_requested_live_restoration=True,comparative_timing=False,rows=rows),indent=2)+'\n')
    print('Validated 1300 processes; 600 exact allocation replays; all query/prepared restoration.',flush=True)
if __name__=='__main__':
    if '--analyze' not in sys.argv:main()
    summarize()
