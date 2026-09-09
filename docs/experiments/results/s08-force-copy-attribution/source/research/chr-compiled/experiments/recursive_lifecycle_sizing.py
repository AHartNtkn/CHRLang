#!/usr/bin/env python3
"""Registered timing/allocation/work sizing; no inferential ranking."""
from pathlib import Path
import json, hashlib, itertools, subprocess, shutil, random, time, sys
import recursive_allocation_gate as gate
ROOT, pilot, analysis = gate.ROOT, gate.pilot, gate.analysis
OUT = ROOT/'docs/experiments/results/s06-recursive-lifecycle-sizing'
BIN = Path('/tmp/chr-recursive-cost-0c8ccc88')
CONFIGS = [(b,m,f,n,q,r) for b,m in gate.BUILDS for f,n,q,r in itertools.product(gate.FAMILIES,[0,16,64],[1,4],[False,True])]
CANCEL = [(b,m,f,64,2,True) for b,m in gate.BUILDS for f in gate.FAMILIES]

def command(c, binary):
    b,*args=c
    return [str(BIN/b/binary)]+[str(int(x)) if isinstance(x,bool) else str(x) for x in args]

def main():
    OUT.mkdir(exist_ok=False); pilot.OUT=OUT
    old=ROOT/'docs/experiments/results/s06-recursive-allocation-gate/freeze.json'
    previous=json.loads(old.read_text())
    for p,h in previous['sources'].items():
        assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    paths=list(previous['sources'])+['research/chr-compiled/examples/recursive_work.rs','research/chr-compiled/experiments/recursive_lifecycle_sizing.py','docs/experiments/registrations/S06-recursive-lifecycle-sizing.md']
    freeze=dict(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),cpu=pilot.CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True),configs=CONFIGS,sources={},binaries={},preceding_freeze_sha256=hashlib.sha256(old.read_bytes()).hexdigest())
    for p in paths:
        source=ROOT/p;freeze['sources'][p]=hashlib.sha256(source.read_bytes()).hexdigest()
        dest=OUT/'source'/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,dest)
    for build in ['off','on']:
        oldbin=gate.BIN/build/'meter'
        assert hashlib.sha256(oldbin.read_bytes()).hexdigest()==previous['binaries'][str(oldbin)]
        shutil.copy2(oldbin,BIN/build/'meter')
        for name in ['ordinary','meter','work']:
            p=BIN/build/name;freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
        p=subprocess.run([str(BIN/build/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
        (OUT/f'meter-{build}.log').write_text((p.stdout+p.stderr).rstrip()+'\n');assert p.returncode==0
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    memories=[]
    for rep in range(2):
        for i,(b,*c) in enumerate(CONFIGS):
            pilot.BIN=BIN/b;v=pilot.run('allocation',rep*600+i,*c)
            readings=[p['memory'] for p in analysis.phases(v)]
            if rep==0:memories.append(readings)
            else:assert readings==memories[i]
        print(f'allocation gate {rep+1}/2',flush=True)
    for i,(b,*c) in enumerate(CANCEL):
        pilot.BIN=BIN/b;pilot.run('cancel-meter',i,*c,32)
    order=list(range(600));random.Random(7511).shuffle(order)
    for j,i in enumerate(order):
        b,*c=CONFIGS[i];pilot.BIN=BIN/b;pilot.run('ordinary',i,*c)
        if (j+1)%100==0:print(f'ordinary {j+1}/600',flush=True)
    for i,(b,*c) in enumerate(CANCEL):
        pilot.BIN=BIN/b;pilot.run('cancel',i,*c,32)
    for i,c in enumerate(CONFIGS):
        cmd=command(c,'work');start=time.monotonic()
        try:
            p=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
            raw=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,wall_seconds=time.monotonic()-start)
        except subprocess.TimeoutExpired as e:
            (OUT/f'work-{i:03}.json').write_text(json.dumps(dict(command=cmd,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr)),indent=2)+'\n')
            raise
        (OUT/f'work-{i:03}.json').write_text(json.dumps(raw,indent=2)+'\n');assert p.returncode==0

def summarize():
    rows=[];semantics={}
    def read(kind,i,c):
        path=OUT/f'{kind}-{i:03}.json';raw=json.loads(path.read_text())
        expected=command(c,'meter' if kind in ['allocation','cancel-meter'] else 'ordinary')
        if kind.startswith('cancel'):expected.append('32')
        assert raw['command']==expected
        _,v,_=analysis.load(path);assert v['meter']==(kind in ['allocation','cancel-meter'])
        _,_,f,n,q,_=c;assert len(v['samples'])==q
        for j,s in enumerate(v['samples']):
            assert s['depth']==n+j%2 and s['complete']==(not kind.startswith('cancel') or j%2==1)
            if s['complete']:assert s['answers']==(0 if f=='fail' else 4 if f=='multi-choice' else 2 if f=='choice' else 1)
            if v['meter']:assert s['input_build']['memory']['live_start']==s['answer_drop']['memory']['live_end']
        if v['meter']:assert v['source_build']['memory']['live_start']==v['prepared_drop']['memory']['live_end']
        return v
    for i,c in enumerate(CONFIGS):
        v=read('ordinary',i,c);a=read('allocation',i,c);b=read('allocation',600+i,c)
        assert [p['memory'] for p in analysis.phases(a)]==[p['memory'] for p in analysis.phases(b)]
        raw=json.loads((OUT/f'work-{i:03}.json').read_text());assert raw['returncode']==0 and raw['command']==command(c,'work')
        w=json.loads(raw['stdout']);assert w['metrics'] and w['mode']==c[1] and w['family']==c[2] and len(w['samples'])==c[4]
        signature=[{k:s[k] for k in ['applications','forks','failed','answers']} for s in w['samples']]
        key=tuple(c[2:])
        if key in semantics:assert semantics[key]==signature,(c,signature,semantics[key])
        else:semantics[key]=signature
        rows.append(dict(config=c,**analysis.timing(v),query_phase_ns={p:sum(s[p]['ns'] for s in v['samples']) for p in analysis.PHASES},allocation_inclusive=analysis.allocations(a),primary_requested_bytes=sum(p['memory']['requested_bytes'] for p in analysis.phases(a))-a['source_build']['memory']['requested_bytes']-sum(s['input_build']['memory']['requested_bytes'] for s in a['samples']),work={k:sum(s[k] for s in w['samples']) for k in w['samples'][0]}))
    for kind in ['cancel','cancel-meter']:
        for i,c in enumerate(CANCEL):read(kind,i,c)
    (OUT/'summary.json').write_text(json.dumps(dict(status='exploratory single ordinary timings; no inferential ranking',processes=2500,exact_allocation_replays=600,source_work_signatures_equal=True,rows=rows),indent=2)+'\n')
    for r in rows:
        if tuple(r['config'][3:])==(64,4,True):
            print(r['config'][:3],round(r['engine_ns']/1e6,3),r['work']['candidates'],r['work']['carrier_steps'],flush=True)

if __name__=='__main__':
    if '--analyze' not in sys.argv:main()
    summarize()
