"""Isolated paired allocation qualification for matched finite-name observations."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-joint-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(binary,args,label):
    p=subprocess.run([binary,*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    with (RAW/f'{label}.json').open('x') as f:json.dump(dict(args=args,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr),f)
    assert p.returncode==0,(args,p.stderr)
    return [json.loads(x) for x in p.stdout.splitlines()]
if __name__=='__main__':
    build=[json.loads(x) for x in (RAW/'build.jsonl').read_text().splitlines()]
    binary=next(x['executable'] for x in build if x.get('executable') and x['target']['name']=='joint_ownership')
    files=[Path(__file__),ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S06-joint-ownership.md',ROOT/'research/chr-structural/Cargo.toml',ROOT/'research/chr-structural/tests/joint_ownership.rs',ROOT/'research/chr-compiled/experiments/meter.rs']
    for directory in ['research/chr-structural/src','crates/chr-syntax/src']:
        files.extend((ROOT/directory).rglob('*.rs'))
    freeze=dict(inputs={str(p.relative_to(ROOT)):sha(p) for p in files},binary=dict(path=binary,sha256=sha(Path(binary))))
    with (RAW/'freeze.json').open('x') as f:json.dump(freeze,f,indent=2)
    stats=[]
    configs=itertools.product(['symbolic','projected','names','explicit'],['free','star','clique'],[3,6],[2,3],[1,16,128],['member','full'],['immediate','all'])
    for i,args in enumerate(configs):
        a=run(binary,args,f'case-{i}-0');b=run(binary,args,f'case-{i}-1');assert a==b,(i,args)
        phases=a[1:];identity=next(x['allocation'] for x in phases if x['phase']=='identity-owner-dispose');transports=[x['allocation'] for x in phases if x['phase']=='transport']
        stats.append(dict(args=args,identity_records=a[0]['identity_records'],identity_bytes=identity['live_start']-identity['live_end'],requested_bytes=sum(x['allocation']['requested_bytes'] for x in phases),first_transport_bytes=transports[0]['requested_bytes'],last_transport_bytes=transports[-1]['requested_bytes']))
        if (i+1)%72==0:print('paired configurations',i+1,flush=True)
    for p,h in freeze['inputs'].items():assert sha(ROOT/p)==h,p
    (RAW/'summary.json').write_text(json.dumps(stats,indent=2)+'\n')
    audit=dict(configurations=len(stats),processes=2*len(stats),exact_allocation_repeats=True,source_inputs=len(files),final_owners_restored=True)
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(audit)
