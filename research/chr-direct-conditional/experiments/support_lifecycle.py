#!/usr/bin/env python3
import hashlib,itertools,json,random,resource,subprocess
from pathlib import Path
ROOT=Path('docs/experiments/results/s08-support-lifecycle')
BINROOT=Path('target/s08-support-lifecycle').resolve()
CONTROLS=[('ordinary','inferred'),('identities','inferred'),('cache','inferred'),('combined','inferred'),('ordinary','scan'),('ordinary','resumable')]
CELLS=[(p,m,f,n,k,0,r,0) for (p,m),f,n,k,r in itertools.product(CONTROLS,['aliases','distinct'],[0,64],['0','all'],[1,4])]
CELLS += [(p,m,f,64,'4',0,4,0) for (p,m),f in itertools.product(CONTROLS,['aliases','distinct'])]
CELLS += [(p,m,'aliases',64,k,0,4,1) for (p,m),k in itertools.product(CONTROLS,['0','4'])]
CELLS += [(p,m,'aliases',64,k,1,4,0) for (p,m),k in itertools.product(CONTROLS,['0','4','all'])]
assert len(CELLS)==len(set(CELLS))==138

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
def memory_only(value):
    if isinstance(value,dict):return {k:memory_only(v) for k,v in value.items() if k not in ['ns','first_answer_ns']}
    if isinstance(value,list):return [memory_only(v) for v in value]
    return value

def main():
    ROOT.mkdir(exist_ok=False);BINROOT.mkdir(exist_ok=True)
    files=[Path(__file__),Path('Cargo.lock'),Path('Cargo.toml'),Path('docs/experiments/registrations/S08-support-lifecycle.md')]
    for d in ['research/chr-direct-conditional','research/chr-reuse','research/chr-compiled','research/chr-relational','research/chr-persistent','research/chr-observe','crates/chr-syntax']:
        files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
    freeze={str(p):sha(p) for p in files}
    for policy,extra in [('ordinary',''),('identities',',support-identities'),('cache',',support-result-cache'),('combined',',support-identities,support-result-cache')]:
        for kind in ['meter','time']:
            features='experiment,head-dispatch,serial-body-accounting,equality-invalidation'+extra+(',alloc-meter' if kind=='meter' else '')
            run=subprocess.run(['cargo','build','--release','-p','chr-direct-conditional','--no-default-features','--features',features,'--example','support_lifecycle'],capture_output=True,text=True)
            (ROOT/f'{policy}-{kind}-build.log').write_text(run.stdout+run.stderr);assert run.returncode==0
            binary=BINROOT/f'{policy}-{kind}';binary.write_bytes(Path('target/release/examples/support_lifecycle').read_bytes());binary.chmod(0o755);freeze[str(binary)]=sha(binary)
    (ROOT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n');(ROOT/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    for kind,reps,seed in [('meter',2,7870),('time',5,7871)]:
        (ROOT/kind).mkdir();(ROOT/f'{kind}-load.txt').write_text(Path('/proc/loadavg').read_text())
        jobs=[(rep,*c) for rep in range(reps) for c in CELLS];random.Random(seed).shuffle(jobs);(ROOT/f'{kind}-order.json').write_text(json.dumps(jobs)+'\n');prior={}
        for i,(rep,policy,mode,family,n,keep,cancel,reuse,fail) in enumerate(jobs):
            cmd=[str(BINROOT/f'{policy}-{kind}'),mode,family,str(n),'4','8','1',keep,str(cancel),str(reuse),str(fail)]
            run=subprocess.run(cmd,capture_output=True,text=True,timeout=150,preexec_fn=limits)
            (ROOT/kind/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr),indent=2)+'\n');assert run.returncode==0,(kind,i)
            data=json.loads(run.stdout.splitlines()[-1]);assert (data['policy'],data['mode'],data['family'],data['depth'],data['keep'],data['cancel'],data['reuse'],data['fail_tail'])==(policy,mode,family,n,keep,bool(cancel),reuse,bool(fail))
            assert data['meter']==(kind=='meter') and not data['counters']
            if kind=='meter':
                key=(policy,mode,family,n,keep,cancel,reuse,fail);clean=memory_only(data)
                if key in prior:assert prior[key]==clean,key
                prior[key]=clean
            if (i+1)%48==0:print(kind,i+1,'/',len(jobs),'pass',flush=True)
        print(kind,'completed',len(jobs),flush=True)
    assert all(sha(Path(p))==h for p,h in freeze.items())
    (ROOT/'completion.json').write_text(json.dumps(dict(cells=138,allocation_processes=276,timing_processes=690,exact_replays=138,timing_interpretation='exploratory'))+'\n')
if __name__=='__main__':main()
