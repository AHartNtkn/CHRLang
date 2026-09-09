#!/usr/bin/env python3
"""Frozen allocation-only reunion gate; stop on failed receipts, resume exact successes."""
import hashlib,json,os,random,resource,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s04-reunion-ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def sources():
    paths=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S04-reunion-ownership-gate.md',Path(__file__).resolve(),ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs']
    for name in ['crates/chr-syntax','research/chr-restoration','research/chr-factors','research/chr-compiled','research/chr-persistent','research/chr-observe']:
        paths.extend(p for p in (ROOT/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
    return {str(p.relative_to(ROOT)):sha(p) for p in sorted(set(paths))}
def call(cmd,path,cpu=None,seconds=60):
    def bound():
        resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));os.sched_setaffinity(0,{cpu})
    try:
        p=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True,timeout=seconds,preexec_fn=bound if cpu is not None else None)
        r={'command':cmd,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
    except subprocess.TimeoutExpired as e:
        r={'command':cmd,'exit_code':None,'cutoff':'wall','stdout':(e.stdout or b'').decode(),'stderr':(e.stderr or b'').decode()}
    path.write_text(json.dumps(r,indent=2)+'\n');assert r['exit_code']==0,path
    return r
if sys.argv[1]=='build':
    assert not (OUT/'freeze.json').exists()
    bins={}
    for name,features in [('meter','alloc-meter'),('cow','alloc-meter,arena-cow')]:
        target=ROOT/'target/s04-reunion-ownership'/('measured-'+name)
        call(['cargo','build','-p','chr-restoration','--release','--example','reunion_cost','--features',features,'--target-dir',str(target)],OUT/f'build-{name}.json',seconds=120)
        binary=target/'release/examples/reunion_cost';bins[name]={'path':str(binary),'sha256':sha(binary)}
        feature=subprocess.check_output(['cargo','tree','-p','chr-restoration','-e','features','--edges','normal,build,dev','--features',features],cwd=ROOT,text=True)
        assert not any('feature "'+x+'"' in feature for x in ['metrics','kernel-metrics','replay-diagnostic'])
        (OUT/f'features-{name}.txt').write_text(feature)
        print('built',name,flush=True)
    (OUT/'freeze.json').write_text(json.dumps({'sources':sources(),'binaries':bins,'cpu':min(os.sched_getaffinity(0)),'affinity':sorted(os.sched_getaffinity(0)),'toolchain':subprocess.check_output(['rustc','--version'],text=True),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True)},indent=2)+'\n')
else:
    freeze=json.loads((OUT/'freeze.json').read_text());assert freeze['sources']==sources()
    for b in freeze['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    rng=random.Random(20260910);order=[]
    for rep in range(2):
        cells=[dict(rep=rep,family=f,owners=o,depth=d,reuse=r,mode=m) for f in ['plain','equal','late','payload'] for o in [2,4] for d in [0,12,48] for r in [1,4] for m in ['copy','reunion','indexed','indexed-cow','factored']]
        rng.shuffle(cells);order.extend(cells)
    (OUT/'order.json').write_text(json.dumps(order,indent=2)+'\n');groups={};rows=[]
    for i,c in enumerate(order):
        path=OUT/('-'.join(str(c[k]) for k in ['rep','family','owners','depth','reuse','mode'])+'.json')
        cow=c['mode']=='indexed-cow';mode='indexed' if cow else c['mode']
        cmd=[freeze['binaries']['cow' if cow else 'meter']['path'],mode,c['family'],str(c['owners']),str(c['depth']),str(c['reuse'])]
        if path.exists():r=json.loads(path.read_text());assert r['command']==cmd and r['exit_code']==0,path
        else:r=call(cmd,path,freeze['cpu'])
        v=json.loads(r['stdout']);assert v['mode']==mode and v['cow']==cow
        assert all(v[k]==c[k] for k in ['family','owners','depth','reuse'])
        ps=v['phases'];expected=['prepare']+['input','setup','execute','engine-drop','answers-drop','input-drop']*c['reuse']+['cancel-input','cancel-setup','cancel-first','cancel-engine-drop','cancel-answers-drop','cancel-input-drop','prepared-drop']
        assert [x['phase'] for x in ps]==expected
        assert all(a['memory']['live_end']==b['memory']['live_start'] for a,b in zip(ps,ps[1:]))
        assert ps[-1]['memory']['live_end']==ps[0]['memory']['live_start']
        assert all(x['memory']['peak_live']>=max(x['memory']['live_start'],x['memory']['live_end']) for x in ps)
        assert all(x['memory']['live_end']==ps[0]['memory']['live_end'] for x in ps if x['phase'] in ['input-drop','cancel-input-drop'])
        key=tuple(c[k] for k in ['family','owners','depth','reuse','mode'])
        if key in groups:assert groups[key]==ps,(key,'non-reproducible allocation')
        else:groups[key]=ps
        primary=[x for x in ps if not x['phase'].startswith('cancel-')]
        rows.append({**c,'requested_bytes':sum(x['memory']['requested_bytes'] for x in primary),'peak_growth':max(x['memory']['peak_live'] for x in primary)-ps[0]['memory']['live_start'],'prepared_live':ps[0]['memory']['live_end']-ps[0]['memory']['live_start'],'cancel_bytes':sum(x['memory']['requested_bytes'] for x in ps if x['phase'].startswith('cancel-'))})
        if (i+1)%20==0:print(i+1,'/',len(order),'validated',flush=True)
    assert len(rows)==480 and len(groups)==240 and freeze['sources']==sources()
    for b in freeze['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    (OUT/'summary.json').write_text(json.dumps(rows,indent=2)+'\n')
    (OUT/'audit.json').write_text(json.dumps({'processes':480,'exact_allocation_pairs':240,'source_and_binary_freeze':True,'phase_continuity':True,'query_and_prepared_restoration':True,'independent_complete_answers':True},indent=2)+'\n')
