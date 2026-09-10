#!/usr/bin/env python3
"""Run the prospectively registered order lifecycle matrix, sequentially."""
import hashlib, itertools, json, random, resource, subprocess
from pathlib import Path
ROOT=Path('docs/experiments/results/s08-order-lifecycle')
BINROOT=Path('target/s08-order-lifecycle').resolve()
CONTROLS=[('ordinary','inferred'),('combined','inferred'),('general','inferred'),('reverse','inferred'),('ordinary','scan'),('ordinary','resumable')]
CASES=[]
for f,n in [('aliases',64),('distinct',64),('oldest-first',8),('newest-first',8)]:
    CASES.extend([(f,0,'0',0,1,0),(f,n,'0',0,4,0),(f,n,'all',0,4,0)])
CASES += [('aliases',64,'4',0,4,0),('aliases',64,'0',0,4,1),('aliases',64,'0',1,4,0),('aliases',64,'all',1,4,0),('oldest-first',8,'0',0,4,1),('newest-first',8,'0',0,4,1)]
CELLS=[(*c,*s) for c,s in itertools.product(CONTROLS,CASES)]
assert len(CELLS)==len(set(CELLS))==108

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def clean(x):
    if isinstance(x,dict): return {k:clean(v) for k,v in x.items() if k not in ['ns','first_answer_ns']}
    if isinstance(x,list): return [clean(v) for v in x]
    return x

def main():
    ROOT.mkdir(exist_ok=False); BINROOT.mkdir(exist_ok=True)
    files=[Path(__file__),Path('Cargo.lock'),Path('Cargo.toml'),Path('docs/experiments/registrations/S08-order-lifecycle.md')]
    for d in ['research/chr-direct-conditional','research/chr-reuse','research/chr-compiled','research/chr-direct-choice','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        files.extend(Path(d).rglob('*.rs')); files.append(Path(d)/'Cargo.toml')
    freeze={str(p):sha(p) for p in files}
    for policy in ['ordinary','combined','general','reverse']:
        extra='' if policy=='ordinary' else ',support-identities,support-result-cache'
        if policy=='general': extra+=',support-generic-histories'
        if policy=='reverse': extra+=',support-reverse-order'
        for kind in ['meter','time']:
            features='experiment,head-dispatch,serial-body-accounting,equality-invalidation'+extra+(',alloc-meter' if kind=='meter' else '')
            cmd=['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','order_lifecycle']
            run=subprocess.run(cmd,capture_output=True,text=True)
            (ROOT/f'{policy}-{kind}-build.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)))
            assert run.returncode==0
            binary=BINROOT/f'{policy}-{kind}';binary.write_bytes(Path('target/release/examples/order_lifecycle').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
            if kind=='meter':
                check=subprocess.run([str(binary),'meter-check'],capture_output=True,text=True)
                (ROOT/f'{policy}-meter-check.txt').write_text(check.stdout+check.stderr);assert check.returncode==0
            print('built',policy,kind,flush=True)
    (ROOT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    (ROOT/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    (ROOT/'host.txt').write_text(Path('/proc/cpuinfo').read_text()+Path('/proc/meminfo').read_text())
    for kind,reps,seed in [('meter',2,7890),('time',5,7891)]:
        (ROOT/kind).mkdir();jobs=[(rep,*c) for rep in range(reps) for c in CELLS];random.Random(seed).shuffle(jobs)
        (ROOT/f'{kind}-order.json').write_text(json.dumps(jobs));prior={}
        (ROOT/f'{kind}-load.txt').write_text(Path('/proc/loadavg').read_text())
        for i,(rep,policy,mode,f,n,k,c,r,failure) in enumerate(jobs):
            cmd=[str(BINROOT/f'{policy}-{kind}'),mode,f,str(n),'4','8','1',k,str(c),str(r),str(failure)]
            try: run=subprocess.run(cmd,capture_output=True,text=True,timeout=75,preexec_fn=limits)
            except subprocess.TimeoutExpired as e:
                (ROOT/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr))));raise
            (ROOT/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr)))
            assert run.returncode==0,(kind,i,cmd)
            data=json.loads(run.stdout.splitlines()[-1]);assert data['meter']==(kind=='meter') and not data['counters']
            assert (data['mode'],data['family'],data['depth'],data['keep'],data['cancel'],data['reuse'],data['fail_tail'])==(mode,f,n,k,bool(c),r,bool(failure))
            if kind=='meter':
                key=(policy,mode,f,n,k,c,r,failure);v=clean(data)
                if key in prior: assert prior[key]==v,key
                prior[key]=v
            if (i+1)%36==0: print(kind,i+1,'/',len(jobs),flush=True)
    assert all(sha(Path(p))==h for p,h in freeze.items())
    (ROOT/'completion.json').write_text(json.dumps(dict(cells=108,allocation_processes=216,timing_processes=540,exact_replays=108,timing_interpretation='exploratory'))+'\n')
if __name__=='__main__': main()
