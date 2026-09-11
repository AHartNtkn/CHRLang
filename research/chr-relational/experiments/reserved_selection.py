"""Registered source-trace, work, heap and ordinary lifecycle pilot."""
import gzip, hashlib, itertools, json, os, random, resource, subprocess, time, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s02-reserved-selection'
MODES=['local','local-filtered','scan','indexed','sealed','chr-scan','chr-indexed','chr-sealed','reserved-scan','reserved-indexed','reserved-sealed']
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(command):
    try:
        r=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=limits)
        return dict(command=command,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
    except subprocess.TimeoutExpired as e:
        dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
        return dict(command=command,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
def main():
    assert not (BASE/'freeze.json').exists()
    assert 0 in os.sched_getaffinity(0)
    bins={}
    for kind,features in [('ordinary',[]),('meter',['--features','alloc-meter']),('work',['--features','local-work,compiled-work']),('trace',[])]:
        target=['--example','selection_trace'] if kind=='trace' else ['--test','local_lifecycle']
        cmd=['cargo','build' if kind=='trace' else 'test','-p','chr-relational',*target,'--release','--no-default-features',*features,'--target-dir',f'target/s02-reserved-selection-{kind}','--message-format=json']
        if kind!='trace':cmd+=['--no-run']
        r=subprocess.run(cmd,capture_output=True,text=True,timeout=300)
        (BASE/f'build-{kind}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr)))
        assert r.returncode==0,r.stderr
        artifacts=[json.loads(x) for x in r.stdout.splitlines()]
        binary=Path(next(x['executable'] for x in artifacts if x.get('executable')))
        bins[kind]=dict(path=str(binary),sha256=sha(binary))
        if kind in ['ordinary','meter','trace']:
            for a in artifacts:
                if a.get('reason')=='compiler-artifact':
                    assert not set(a['features'])&{'metrics','kernel-metrics','local-work','compiled-work'},a
                    if kind!='meter':assert 'alloc-meter' not in a['features'],a
    cells=list(itertools.product(['flat','sparse','chain','competition'],[2,6],MODES,['immediate','all'],['complete','cancel']))
    assert len(cells)==352
    rng=random.Random(720611);jobs=[]
    for kind,rep,chosen,warmup in [('meter',0,cells,False),('meter',1,cells,False),('work',0,[c for c in cells if c[3:] == ('all','complete')],False),('work',1,[c for c in cells if c[3:] == ('all','complete')],False),('ordinary',0,[c for c in cells if c[4]=='cancel'],False)]:
        order=chosen.copy();rng.shuffle(order)
        jobs.extend(dict(kind=kind,rep=rep,cell=c,warmup=warmup) for c in order)
    for rep in [-1,0,1,2,3,4]:
        scenarios=list(itertools.product(['flat','sparse','chain','competition'],[2,6],['immediate','all']))
        rng.shuffle(scenarios)
        for family,n,retention in scenarios:
            modes=MODES.copy();rng.shuffle(modes)
            jobs.extend(dict(kind='ordinary',rep=rep,cell=[family,n,m,retention,'complete'],warmup=rep==-1) for m in modes)
    assert len(jobs)==2112
    (BASE/'jobs.json').write_text(json.dumps(jobs))
    paths=subprocess.check_output(['git','ls-files','crates','research','Cargo.toml','Cargo.lock'],text=True).splitlines()
    paths=[p for p in paths if p.endswith(('.rs','.toml','.lock'))]+['research/chr-relational/experiments/reserved_selection.py','research/chr-relational/experiments/audit_reserved_selection.py','docs/experiments/registrations/S02-reserved-selection.md']
    hashes={p:sha(ROOT/p) for p in sorted(set(paths))}
    with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,binaries=bins,archive_sha256=sha(BASE/'sources.zip'),jobs_sha256=sha(BASE/'jobs.json'),cpu=0,affinity=sorted(os.sched_getaffinity(0)),toolchain=subprocess.check_output(['rustc','-Vv'],text=True)),indent=2))
    for name,kind,args in [('meter-check','meter',['selection','meter-check'])]+[(f'clock-{i}','ordinary',['selection','clock-check']) for i in range(3)]+[(f'trace-{i}','trace',[]) for i in range(2)]:
        raw=invoke([bins[kind]['path'],*args]);(BASE/f'{name}.json').write_text(json.dumps(raw));assert raw['exit_code']==0,raw
    start=time.monotonic();count=0
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<1800,'campaign budget'
            family,n,mode,retention,stop=j['cell']
            raw=invoke([bins[j['kind']]['path'],'selection',mode,family,str(n),retention,stop])
            out.write(json.dumps(dict(index=i,**raw))+'\n');out.flush();count+=1
            assert raw['exit_code']==0 and not raw['stderr'],(i,raw)
            if count%176==0:print(count,'/',len(jobs),'seconds',round(time.monotonic()-start),flush=True)
    assert all(sha(ROOT/p)==h for p,h in hashes.items())
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=count,seconds=time.monotonic()-start)))
if __name__=='__main__':main()
