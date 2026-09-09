"""Registered complete runtime/compilation pilot; separated diagnostics."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s01-native-cost-pilot'
TARGET = ROOT / 'target/s01-native-cost-pilot'
MODES = ['generic','planned','specialized','native','native-generic-repair','native-planned','native-specialized']
RETAINED = ['retained-indexed','retained-eager','retained-subscribed']
PROFILES = [('chain',32,[1,16]),('payload',64,[1,16,256]),('dispatch64',0,[1,16]),('dispatch64',32,[1,16]),('subscription',16,[1,16])] + [(f,8,[1,8]) for f in ['low-stable','low-reopen','low-churn']]
CPU = min(os.sched_getaffinity(0))


def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def subscription(f): return f == 'subscription' or f.startswith('low-')
def modes(f): return MODES + (RETAINED if subscription(f) else [])
def artifact(f,mode): return ('subscription' if subscription(f) else f) if mode.startswith('native') else 'generic'


def run(name,cmd,limit,env=None,pin=False):
    def bound():
        resource.setrlimit(resource.RLIMIT_AS,(limit,limit))
        if pin: os.sched_setaffinity(0,{CPU})
    before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic()
    try:
        p=subprocess.run([str(x) for x in cmd],cwd=ROOT,env=env,capture_output=True,text=True,timeout=60,preexec_fn=bound)
        result=dict(exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        result=dict(cutoff=True,stdout=(e.stdout or b'').decode(errors='replace'),stderr=(e.stderr or b'').decode(errors='replace'))
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    result.update(command=[str(x) for x in cmd],wall_seconds=time.monotonic()-start,
                  child_cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,
                  address_space_bytes=limit,cpu=CPU if pin else None)
    (OUT/(name+'.json')).write_text(json.dumps(result,indent=2)+'\n')
    assert result.get('exit_code')==0,(name,result.get('stderr','')[-1500:])
    return result


def validate(result,f,m,n,q,metered=False):
    rows=[json.loads(l) for l in result['stdout'].splitlines()];assert len(rows)==q+1
    h=rows[-1];assert (h['family'],h['mode'],h['queries'],h['cancelled_queries'])==(f,m,q,0)
    assert h['counters'] is False and h['allocator']==('requested-meter' if metered else 'ordinary')
    for i,r in enumerate(rows[:-1]):
        assert r['validated'] is True and r['query']==i
        assert r['size']==(n if f.startswith('low-') else n+i%2)
        assert r['query_ns']==sum(r[k] for k in ['setup_ns','execute_ns','observation_ns','engine_drop_ns','answer_drop_ns'])
        if metered: assert r['query_restored'] is True
    assert h['lifecycle_ns']==sum(r['query_ns'] for r in rows[:-1])+sum(h[k] for k in ['source_ns','prepare_ns','prepared_drop_ns'])
    if metered: assert h['prepared_restored'] is True
    return rows


def main():
    assert not OUT.exists(),'Preserve previous pilot receipts'
    OUT.mkdir(parents=True);TARGET.mkdir(parents=True,exist_ok=True)
    artifacts={};owned=[];shared={}
    for build,feature in [('ordinary','experiment'),('metered','alloc-meter')]:
        env=os.environ.copy();env['CARGO_TARGET_DIR']=str(TARGET/build);env['RUSTFLAGS']='-Cembed-bitcode=yes'
        run('build-'+build,['cargo','build','--offline','--release','-p','chr-compiled','--no-default-features','--features',feature,'--lib','--bin','chr-access-emit'],4<<30,env)
        release=TARGET/build/'release';rlib=release/'libchr_compiled.rlib'
        shared.update({str(p.relative_to(ROOT)):digest(p) for p in (release/'deps').glob('*.rlib')});shared[str(rlib.relative_to(ROOT))]=digest(rlib)
        for opt in (['off','thin'] if build=='ordinary' else ['off']):
            for family in ['generic','chain','payload','subscription','dispatch64']:
                for rep in range(5 if build=='ordinary' else 1):
                    name=f'{build}-{opt}-{family}-{rep}';source=TARGET/(name+'.rs');binary=TARGET/name
                    run('emit-'+name,[release/'chr-access-emit',family,source],1<<30)
                    run('compile-'+name,['rustc','--edition','2024','-C','opt-level=3','-C','codegen-units=1','-C','lto='+opt,source,'--extern','chr_compiled='+str(rlib),'-L','dependency='+str(release/'deps'),'-o',binary],4<<30)
                    artifacts[name]=dict(source_sha256=digest(source),binary_sha256=digest(binary),binary_bytes=binary.stat().st_size)
                    owned.extend([source,binary])
                print('compiled',build,opt,family,flush=True)
    for opt in ['off','thin']:
        for f in ['low-stable','low-reopen','low-churn']:
            for m in modes(f):
                binary=TARGET/f'ordinary-{opt}-{artifact(f,m)}-0'
                validate(run(f'preflight-{opt}-{f}-{m}',[binary,m,f,8,2],1<<30,pin=True),f,m,8,2)
    print('all source preflights pass',flush=True)
    cells=[(f,n,q,m) for f,n,qs in PROFILES for q in qs for m in modes(f)];assert len(cells)==143
    measured=[(opt,*c) for opt in ['off','thin'] for c in cells]
    for block in range(8):
        order=measured.copy();random.Random(7100+block).shuffle(order)
        for opt,f,n,q,m in order:
            binary=TARGET/f'ordinary-{opt}-{artifact(f,m)}-0'
            validate(run(f'time-{block}-{opt}-{f}-{n}-{q}-{m}',[binary,m,f,n,q],1<<30,pin=True),f,m,n,q)
        print('timing block',block,'complete',flush=True)
    for index,(f,n,q,m) in enumerate(cells):
        memories=[]
        for rep in range(2):
            binary=TARGET/f'metered-off-{artifact(f,m)}-0'
            rows=validate(run(f'alloc-{rep}-{f}-{n}-{q}-{m}',[binary,m,f,n,q],1<<30,pin=True),f,m,n,q,True)
            memories.append([r['memory'] for r in rows])
        assert memories[0]==memories[1],('allocation replay',f,n,q,m)
        if (index+1)%20==0:print('diagnostic cells',index+1,flush=True)
    assert all(digest(ROOT/p)==h for p,h in shared.items())
    disposals=[]
    for path in owned:
        start=time.perf_counter_ns();path.unlink();ns=time.perf_counter_ns()-start
        assert not path.exists();disposals.append(dict(path=str(path.relative_to(ROOT)),unlink_ns=ns))
    (OUT/'disposal.json').write_text(json.dumps(disposals,indent=2)+'\n')
    summary=dict(parent_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),rustc=subprocess.check_output(['rustc','-Vv'],text=True),cpu=CPU,artifacts=artifacts,shared_rlibs=shared,preflight_processes=60,timing_processes=2288,measured_cells=286,diagnostic_processes=286,diagnostic_cells=143)
    (OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print('pilot complete',flush=True)


if __name__=='__main__':main()
