"""Freeze and run isolated guarded-choice ownership confirmations."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s02-choice-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(mode):
    rows=[json.loads(s) for s in (RAW/f'build-{mode}.jsonl').read_text().splitlines()]
    return Path(next(r['executable'] for r in rows if r.get('executable') and r.get('target',{}).get('name')=='choice_ownership'))
def limits():
    resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(path,args,label):
    p=subprocess.run([str(path),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    data=dict(args=args,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    with (RAW/f'{label}.json').open('x') as f:json.dump(data,f,indent=2)
    assert p.returncode==0,(label,p.stderr)
    return p.stdout
if __name__=='__main__':
    bins={m:binary(m) for m in ['meter','ordinary']}
    files=[Path(__file__),ROOT/'Cargo.lock',ROOT/'research/chr-relational/Cargo.toml',ROOT/'docs/experiments/registrations/S02-choice-ownership.md',ROOT/'research/chr-relational/tests/choice_ownership.rs',ROOT/'research/chr-compiled/experiments/meter.rs']
    for directory in ['research/chr-relational/tests/support','research/chr-relational/src','research/chr-compiled/src','research/chr-persistent/src','research/chr-observe/src','research/chr-direct-conditional/tests/runtime_support','crates/chr-syntax/src']:
        files.extend((ROOT/directory).rglob('*.rs'))
    with (RAW/'freeze.json').open('x') as f:json.dump(dict(inputs={str(p.relative_to(ROOT)):sha(p) for p in files},binaries={m:dict(path=str(p),sha256=sha(p)) for m,p in bins.items()}),f,indent=2)
    count=0
    for args in itertools.product(['local','scan','indexed','context'],['early','late','early-fail','late-fail'],[0,3],[0,8],[1,8],[0,2,1000000]):
        a=run(bins['meter'],args,f'meter-{count}-0');b=run(bins['meter'],args,f'meter-{count}-1')
        assert a==b,('allocation mismatch',args)
        count+=1
        if count%48==0:print('paired configurations',count,flush=True)
    ordinary=0
    for args in itertools.product(['local','scan','indexed','context'],['early','late-fail'],[0,3],[0,8],[1,8],[0,2,1000000]):
        run(bins['ordinary'],args,f'ordinary-{ordinary}');ordinary+=1
    for mode,path in bins.items():
        for engine in ['local','scan','indexed','context']:
            for rep in range(2):run(path,[engine,'cancel'],f'cancel-{mode}-{engine}-{rep}')
    audit=dict(paired_configurations=count,allocation_processes=count*2,ordinary_processes=ordinary,cancellation_processes=16,source_inputs=len(files),allocation_repeats_identical=True)
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(audit)
