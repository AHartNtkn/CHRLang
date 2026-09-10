"""Prospectively sized lifecycle pilot; primary and diagnostics run separately."""
from pathlib import Path
import hashlib,itertools,json,math,os,random,resource,subprocess,sys
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s02-choice-cost'
ENGINES=['local','scan','indexed','context']
CASES=list(itertools.product(['early','late','early-fail','late-fail'],[0,3],[0,8],[1,8],[0,1000000],[0,16]))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(mode):
    rows=[json.loads(s) for s in (RAW/f'build-{mode}.jsonl').read_text().splitlines()]
    return Path(next(r['executable'] for r in rows if r.get('executable') and r.get('target',{}).get('name')=='choice_cost'))
def limits():
    resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(mode,args,cpu,label):
    cmd=['taskset','-c',str(cpu),str(binary(mode)),*map(str,args)]
    p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    data=dict(command=cmd,args=args,cpu=cpu,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    with (RAW/f'{label}.json').open('x') as f:json.dump(data,f)
    assert p.returncode==0,(label,p.stderr)
    return [json.loads(x) for x in p.stdout.splitlines()]
def verify():
    f=json.loads((RAW/'freeze.json').read_text())
    for p,h in f['inputs'].items():assert sha(ROOT/p)==h,p
    for x in f['binaries'].values():assert sha(Path(x['path']))==x['sha256']
if __name__=='__main__':
    stage=sys.argv[1]
    if stage=='qualify':
        files=[Path(__file__),ROOT/'Cargo.lock',ROOT/'research/chr-relational/Cargo.toml',ROOT/'docs/experiments/registrations/S02-choice-cost.md',ROOT/'research/chr-relational/tests/choice_cost.rs',ROOT/'research/chr-compiled/experiments/meter.rs']
        for directory in ['research/chr-relational/tests/support','research/chr-relational/src','research/chr-compiled/src','research/chr-persistent/src','research/chr-observe/src','research/chr-direct-conditional/tests/runtime_support','crates/chr-syntax/src']:
            files.extend((ROOT/directory).rglob('*.rs'))
        with (RAW/'freeze.json').open('x') as f:json.dump(dict(inputs={str(p.relative_to(ROOT)):sha(p) for p in files},binaries={m:dict(path=str(binary(m)),sha256=sha(binary(m))) for m in ['primary','meter']},affinity=sorted(os.sched_getaffinity(0))),f,indent=2)
        for i,case in enumerate(CASES):
            for e in ENGINES:
                a=run('meter',[e,*case,1],0,f'diagnostic-{i}-{e}-0');b=run('meter',[e,*case,1],0,f'diagnostic-{i}-{e}-1');assert a==b,(i,e)
            if i%16==15:print('qualified',i+1,flush=True)
    elif stage=='size':
        verify();sizes={}
        for i,case in enumerate(CASES):
            for e in ENGINES:
                row=run('primary',[e,*case,1],0,f'size-{i}-{e}')[0]
                sizes[f'{i}-{e}']=min(2048,max(1,math.ceil(10_000_000/row['total_ns'])))
            if i%16==15:print('sized',i+1,flush=True)
        with (RAW/'sizes.json').open('x') as f:json.dump(sizes,f,indent=2)
    elif stage=='confirm':
        verify();sizes=json.loads((RAW/'sizes.json').read_text());rng=random.Random(7204)
        jobs=[]
        for cpu in [0,8]:
            for rep in range(5):
                block=list(itertools.product(range(len(CASES)),ENGINES));rng.shuffle(block)
                jobs.extend((cpu,rep,i,e) for i,e in block)
        with (RAW/'schedule.json').open('x') as f:json.dump(dict(sizes_sha256=sha(RAW/'sizes.json'),jobs=jobs),f)
        for n,(cpu,rep,i,e) in enumerate(jobs):
            run('primary',[e,*CASES[i],sizes[f'{i}-{e}']],cpu,f'confirm-{cpu}-{rep}-{i}-{e}')
            if (n+1)%512==0:print('confirmed',n+1,flush=True)
        for cpu,rep,e in itertools.product([0,8],range(5),ENGINES):run('primary',[e,'cancel'],cpu,f'cancel-{cpu}-{rep}-{e}')
        verify();print('complete',len(jobs),'primary and 40 cancellation processes',flush=True)
    else:raise ValueError(stage)
