"""Registered isolated allocation repeats and ordinary-allocator endpoint confirmations."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess,time
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-diagram-ownership'
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(binary,args,path):
    p=subprocess.run([str(binary),*map(str,args)],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    with path.open('x') as f:json.dump({'args':args,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr},f)
    p.check_returncode()
    return [json.loads(line) for line in p.stdout.splitlines()]
def main():
    assert not (RAW/'freeze.json').exists(),'results already frozen'
    binaries={}
    for mode in ['meter','ordinary']:
        command=['cargo','build','-p','chr-structural','--release','--no-default-features','--example','diagram_ownership','--message-format=json','--target-dir',str(ROOT/f'target/s06-diagram-ownership-{mode}')]
        if mode=='meter':command+=['--features','alloc-meter']
        p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=300)
        (RAW/f'build-{mode}.jsonl').write_text(p.stdout);(RAW/f'build-{mode}.log').write_text(p.stderr);p.check_returncode()
        binaries[mode]=Path(next(json.loads(x)['executable'] for x in p.stdout.splitlines() if json.loads(x).get('executable') and json.loads(x)['target']['name']=='diagram_ownership'))
    files=[Path(__file__),ROOT/'research/chr-structural/examples/diagram_ownership.rs',ROOT/'research/chr-structural/Cargo.toml',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S06-diagram-ownership.md',ROOT/'Cargo.toml',ROOT/'Cargo.lock']
    for directory in ['research/chr-structural/src','crates/chr-syntax/src']:files+=list((ROOT/directory).rglob('*.rs'))
    freeze={'sources':{str(p.relative_to(ROOT)):sha(p) for p in files},'binaries':{m:{'path':str(p.relative_to(ROOT)),'sha256':sha(p)} for m,p in binaries.items()},'rustc':subprocess.check_output(['rustc','-Vv'],text=True)}
    (RAW/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    p=subprocess.run([str(binaries['meter']),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    (RAW/'meter-check.log').write_text(p.stdout+p.stderr);p.check_returncode();assert 'meter self-check passed' in p.stdout
    configs=list(itertools.product(['diagram','names','projected','symbolic','explicit'],['free','star','clique','duplicate','overlap','union'],[3,6],[2,3],[1,16,128],['member','full'],['immediate','window','all']))
    (RAW/'configurations.json').write_text(json.dumps(configs,indent=2)+'\n')
    (RAW/'runs').mkdir();start=time.monotonic();summary=[]
    for index,args in enumerate(configs):
        assert time.monotonic()-start<1800,'campaign wall bound'
        a=run(binaries['meter'],args,RAW/f'runs/{index}-0.json')
        b=run(binaries['meter'],args,RAW/f'runs/{index}-1.json');assert a==b,(index,args)
        c=run(binaries['ordinary'],args,RAW/f'runs/{index}-ordinary.json')
        assert {k:v for k,v in a[0].items() if k!='meter'}=={k:v for k,v in c[0].items() if k!='meter'}
        assert [r['phase'] for r in a[1:]]==[r['phase'] for r in c[1:]]
        assert all(r['memory'] is None for r in c[1:])
        phases=a[1:];baseline=phases[0]['memory']['live_start'];assert phases[-1]['memory']['live_end']==baseline
        summary.append({'args':args,'identity_records':a[0]['identity_records'],'requested_bytes':sum(r['memory']['requested_bytes'] for r in phases),'peak_excess':max(r['memory']['peak_live'] for r in phases)-baseline,'prepare_bytes':phases[0]['memory']['requested_bytes'],'prepare_live':phases[0]['memory']['live_end']-baseline,'observe_bytes':sum(r['memory']['requested_bytes'] for r in phases if r['phase']=='observe'),'transport_bytes':sum(r['memory']['requested_bytes'] for r in phases if r['phase']=='transport')})
        if (index+1)%120==0:print(f'completed {index+1}/{len(configs)} configurations',flush=True)
    for p,h in freeze['sources'].items():assert sha(ROOT/p)==h,p
    (RAW/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    audit={'configurations':len(configs),'metered_processes':2*len(configs),'ordinary_processes':len(configs),'exact_allocation_repeats':True,'final_owners_restored':True,'elapsed_seconds':time.monotonic()-start}
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n');print(json.dumps(audit))
if __name__=='__main__':main()
