#!/usr/bin/env python3
"""Registered paired access-policy confirmation with independent work counts."""
from pathlib import Path
import json,hashlib,itertools,subprocess,shutil,random,sys,math,statistics
import recursive_allocation_gate as gate
ROOT,pilot,analysis=gate.ROOT,gate.pilot,gate.analysis
OUT=ROOT/'docs/experiments/results/s06-recursive-access-confirmation'
BIN=Path('/tmp/chr-recursive-access-2f4920dc')
BUILDS=gate.BUILDS+[(b,m+'-scan') for b,m in gate.BUILDS]
CONFIGS=list(itertools.product(gate.FAMILIES,[0,64],[1,4],[False,True]))
CONTRASTS=[('on/contracted-scan','on/sealed-scan'),('on/contracted','on/sealed'),('off/sealed-scan','off/sealed'),('on/contracted-scan','on/contracted'),('on/contracted-scan','off/sealed-scan'),('on/sealed','off/sealed'),('on/sealed-scan','off/sealed-scan'),('off/original-scan','off/original'),('on/contracted-scan','off/original-scan')]

def cmd(build,mode,c,binary):
    return [str(BIN/build/binary),mode]+[str(int(x)) if isinstance(x,bool) else str(x) for x in c]

def main():
    OUT.mkdir(exist_ok=False);pilot.OUT=OUT
    old=ROOT/'docs/experiments/results/s06-recursive-lifecycle-sizing/freeze.json';previous=json.loads(old.read_text())
    changed=['research/chr-compiled/examples/recursive_cost.rs','research/chr-compiled/examples/recursive_work.rs']
    for p,h in previous['sources'].items():
        if p not in changed:assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    paths=list(previous['sources'])+['research/chr-compiled/experiments/recursive_access_confirmation.py','docs/experiments/registrations/S06-recursive-access-confirmation.md']
    freeze=dict(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),cpu=pilot.CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True),configs=CONFIGS,builds=BUILDS,sources={},binaries={})
    for p in paths:
        path=ROOT/p;freeze['sources'][p]=hashlib.sha256(path.read_bytes()).hexdigest()
        dest=OUT/'source'/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(path,dest)
    for build in ['on','off']:
        for name in ['ordinary','meter','work']:
            p=BIN/build/name;freeze['binaries'][str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
        p=subprocess.run([str(BIN/build/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
        (OUT/f'meter-{build}.log').write_text((p.stdout+p.stderr).rstrip()+'\n');assert p.returncode==0
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    memories=[]
    for rep in range(2):
        for i,c in enumerate(CONFIGS):
            for j,(b,m) in enumerate(BUILDS):
                pilot.BIN=BIN/b;v=pilot.run('allocation',rep*800+i*10+j,m,*c)
                readings=[p['memory'] for p in analysis.phases(v)]
                if rep==0:memories.append(readings)
                else:assert readings==memories[i*10+j]
        print(f'allocation gate {rep+1}/2',flush=True)
    for i,((b,m),f) in enumerate(itertools.product(BUILDS,gate.FAMILIES)):
        pilot.BIN=BIN/b;pilot.run('cancel-meter',i,m,f,64,2,True,32)
    rng=random.Random(7611)
    for rep in range(7):
        indices=list(range(80));rng.shuffle(indices)
        for i in indices:
            builds=list(enumerate(BUILDS));rng.shuffle(builds)
            for j,(b,m) in builds:
                pilot.BIN=BIN/b;pilot.run('ordinary',(rep*80+i)*10+j,m,*CONFIGS[i])
        print(f'ordinary block {rep+1}/7',flush=True)
    for i,((b,m),f) in enumerate(itertools.product(BUILDS,gate.FAMILIES)):
        pilot.BIN=BIN/b;pilot.run('cancel',i,m,f,64,2,True,32)
    for i,c in enumerate(CONFIGS):
        for j,(b,m) in enumerate(BUILDS):
            command=cmd(b,m,c,'work')
            try:
                p=subprocess.run(command,capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
                raw=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
            except subprocess.TimeoutExpired as e:
                (OUT/f'work-{i*10+j:03}.json').write_text(json.dumps(dict(command=command,timeout=True,stdout=str(e.stdout),stderr=str(e.stderr)),indent=2)+'\n');raise
            (OUT/f'work-{i*10+j:03}.json').write_text(json.dumps(raw,indent=2)+'\n');assert p.returncode==0

def summarize():
    def read(kind,index,b,m,c):
        path=OUT/f'{kind}-{index:03}.json';raw=json.loads(path.read_text())
        expected=cmd(b,m,c,'meter' if kind in ['allocation','cancel-meter'] else 'ordinary')
        if kind.startswith('cancel'):expected.append('32')
        assert raw['command']==expected
        _,v,_=analysis.load(path);assert v['meter']==(kind in ['allocation','cancel-meter'])
        f,n,q,r=c;assert len(v['samples'])==q and v['mode']==m and v['family']==f and v['resource']==r
        for j,s in enumerate(v['samples']):
            assert s['depth']==n+j%2 and s['complete']==(not kind.startswith('cancel') or j%2==1)
            if s['complete']:assert s['answers']==(0 if f=='fail' else 4 if f=='multi-choice' else 2 if f=='choice' else 1)
            if v['meter']:assert s['input_build']['memory']['live_start']==s['answer_drop']['memory']['live_end']
        if v['meter']:assert v['source_build']['memory']['live_start']==v['prepared_drop']['memory']['live_end']
        return v
    results=[]
    for i,c in enumerate(CONFIGS):
        row=dict(config=c,modes={},contrasts={});values={};signature=None
        for j,(b,m) in enumerate(BUILDS):
            key=b+'/'+m
            v=[read('ordinary',(rep*80+i)*10+j,b,m,c) for rep in range(7)]
            a=read('allocation',i*10+j,b,m,c);replay=read('allocation',800+i*10+j,b,m,c)
            assert [p['memory'] for p in analysis.phases(a)]==[p['memory'] for p in analysis.phases(replay)]
            raw=json.loads((OUT/f'work-{i*10+j:03}.json').read_text());assert raw['returncode']==0 and raw['command']==cmd(b,m,c,'work')
            w=json.loads(raw['stdout']);assert w['metrics'] and w['mode']==m and w['family']==c[0] and len(w['samples'])==c[2]
            current=[{k:s[k] for k in ['applications','forks','failed','answers']} for s in w['samples']]
            if signature is None:signature=current
            else:assert signature==current,(c,key)
            values[key]=[analysis.timing(x)['engine_ns'] for x in v]
            first=[s['first_answer_ns'] for x in v for s in x['samples'] if s['first_answer_ns'] is not None]
            row['modes'][key]=dict(primary_median_ms=statistics.median(values[key])/1e6,inclusive_median_ms=statistics.median(analysis.timing(x)['including_source_input_ns'] for x in v)/1e6,preparation_median_ms=statistics.median(x['preparation']['ns'] for x in v)/1e6,first_answer_median_ms=statistics.median(first)/1e6 if first else None,query_phase_median_ms={p:statistics.median(sum(s[p]['ns'] for s in x['samples']) for x in v)/1e6 for p in analysis.PHASES},primary_requested_bytes=sum(p['memory']['requested_bytes'] for p in analysis.phases(a))-a['source_build']['memory']['requested_bytes']-sum(s['input_build']['memory']['requested_bytes'] for s in a['samples']),allocation_inclusive=analysis.allocations(a),work={k:sum(s[k] for s in w['samples']) for k in w['samples'][0]})
        for j,(a,b) in enumerate(CONTRASTS):
            logs=[math.log(x/y) for x,y in zip(values[a],values[b])];rng=random.Random(7612+9*i+j)
            draws=sorted(math.exp(sum(rng.choices(logs,k=7))/7) for _ in range(10000));lo,hi=draws[249],draws[9749]
            row['contrasts'][a+' / '+b]=dict(ratio=math.exp(statistics.mean(logs)),interval95=[lo,hi],classification='gain' if hi<.9 else 'loss' if lo>1.1 else 'unresolved')
        results.append(row)
    for kind in ['cancel','cancel-meter']:
        for i,((b,m),f) in enumerate(itertools.product(BUILDS,gate.FAMILIES)):read(kind,i,b,m,(f,64,2,True))
    counts={a+' / '+b:{v:sum(r['contrasts'][a+' / '+b]['classification']==v for r in results) for v in ['gain','loss','unresolved']} for a,b in CONTRASTS}
    (OUT/'summary.json').write_text(json.dumps(dict(counts=counts,results=results),indent=2)+'\n');print(json.dumps(counts,indent=2))
if __name__=='__main__':
    if '--analyze' not in sys.argv:main()
    summarize()
