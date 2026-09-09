#!/usr/bin/env python3
"""Prospectively registered paired reusable-prefix confirmation."""
import hashlib, itertools, json, random, shutil, statistics, math, subprocess
from pathlib import Path
import reusable_prefix_sizing as sizing
pilot, analysis, ROOT = sizing.pilot, sizing.analysis, sizing.ROOT
OUT = ROOT / 'docs/experiments/results/s06-reusable-prefix-confirmation'
CONFIGS = list(itertools.product(sizing.FAMILIES, [1,32], [1,8], [False,True]))
MODES = sizing.MODES
CONTRASTS = [('reusable','original'),('reusable','sealed'),('reusable','lowered'),('reusable','lowered-sealed'),('reusable','rebuilt'),('rebuilt','lowered')]


def main():
    OUT.mkdir(exist_ok=False);pilot.OUT=OUT
    old=ROOT/'docs/experiments/results/s06-reusable-prefix-sizing-corrected/freeze.json'
    preceding=json.loads(old.read_text())
    for p,h in preceding['sources'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
    paths=list(preceding['sources'])+['research/chr-compiled/experiments/reusable_prefix_confirmation.py','docs/experiments/registrations/S06-reusable-prefix-confirmation.md']
    freeze=dict(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),cpu=pilot.CPU,rustc=subprocess.check_output(['rustc','-Vv'],text=True),sources={},binaries={},configs=CONFIGS,modes=MODES,preceding_freeze_sha256=hashlib.sha256(old.read_bytes()).hexdigest())
    for p in paths:
        source=ROOT/p;freeze['sources'][p]=hashlib.sha256(source.read_bytes()).hexdigest()
        dest=OUT/'source'/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,dest)
    for p,h in preceding['binaries'].items():
        assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h
        freeze['binaries'][p]=h
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    check=subprocess.run([str(pilot.BIN/'meter'),'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=pilot.limits)
    (OUT/'meter-check.log').write_text((check.stdout+check.stderr).rstrip()+'\n');assert check.returncode==0
    rng=random.Random(7411)
    for rep in range(7):
        indices=list(range(56));rng.shuffle(indices)
        for i in indices:
            modes=list(enumerate(MODES));rng.shuffle(modes)
            for j,m in modes:pilot.run('ordinary',(rep*56+i)*6+j,m,*CONFIGS[i])
        print(f'ordinary repetition {rep+1}/7',flush=True)
    for rep in range(2):
        for i,c in enumerate(CONFIGS):
            for j,m in enumerate(MODES):pilot.run('allocation',rep*336+i*6+j,m,*c)
        print(f'allocation repetition {rep+1}/2',flush=True)
    for i,m in enumerate(MODES):
        for kind in ['cancel','cancel-meter']:pilot.run(kind,i,m,'choice4',8,2,True,1)


def summarize():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for p,h in freeze['sources'].items():assert hashlib.sha256((OUT/'source'/p).read_bytes()).hexdigest()==h
    def read(kind,i,config):
        path=OUT/f'{kind}-{i:03}.json';raw=json.loads(path.read_text());k,r,_=analysis.load(path)
        assert tuple(k)==tuple(config)
        expected=[str(int(x)) if isinstance(x,bool) else str(x) for x in config]
        if kind.startswith('cancel'):expected.append('1')
        assert raw['command'][1:]==expected
        assert r['meter']==(kind in ['allocation','cancel-meter'])
        for j,s in enumerate(r['samples']):
            assert s['depth']==config[2] and s['complete']==(not kind.startswith('cancel') or j%2==1)
            if s['complete']:assert s['answers']==(0 if config[1].startswith('fail') else 2**(int(config[1][-1]) if config[1].startswith('choice') else 0))
        if r['meter']:
            assert r['source_build']['memory']['live_start']==r['prepared_drop']['memory']['live_end']
            for s in r['samples']:assert s['input_build']['memory']['live_start']==s['answer_drop']['memory']['live_end']
        return r
    results=[]
    for i,c in enumerate(CONFIGS):
        row=dict(config=c,modes={},contrasts={});values={}
        for j,m in enumerate(MODES):
            v=[read('ordinary',(rep*56+i)*6+j,(m,*c)) for rep in range(7)]
            a=read('allocation',i*6+j,(m,*c));b=read('allocation',336+i*6+j,(m,*c))
            assert [p['memory'] for p in analysis.phases(a)]==[p['memory'] for p in analysis.phases(b)]
            values[m]=[analysis.timing(r)['engine_ns'] for r in v]
            first=[s['first_answer_ns'] for r in v for s in r['samples'] if s['first_answer_ns'] is not None]
            row['modes'][m]=dict(primary_median_ms=statistics.median(values[m])/1e6,inclusive_median_ms=statistics.median(analysis.timing(r)['including_source_input_ns'] for r in v)/1e6,preparation_median_ms=statistics.median(r['preparation']['ns'] for r in v)/1e6,first_answer_median_ms=statistics.median(first)/1e6 if first else None,query_phase_median_ms={p:statistics.median(sum(s[p]['ns'] for s in r['samples']) for r in v)/1e6 for p in analysis.PHASES},allocation_inclusive=analysis.allocations(a),primary_requested_bytes=sum(p['memory']['requested_bytes'] for p in analysis.phases(a))-a['source_build']['memory']['requested_bytes']-sum(s['input_build']['memory']['requested_bytes'] for s in a['samples']))
        for j,(a,b) in enumerate(CONTRASTS):
            logs=[math.log(x/y) for x,y in zip(values[a],values[b])];rng=random.Random(7412+6*i+j)
            draws=sorted(math.exp(sum(rng.choices(logs,k=7))/7) for _ in range(10000));lo,hi=draws[249],draws[9749]
            row['contrasts'][a+'/'+b]=dict(ratio=math.exp(statistics.mean(logs)),interval95=[lo,hi],classification='gain' if hi<.9 else 'loss' if lo>1.1 else 'unresolved')
        results.append(row)
    for i,m in enumerate(MODES):
        for kind in ['cancel','cancel-meter']:read(kind,i,(m,'choice4',8,2,True))
    counts={a+'/'+b:{v:sum(r['contrasts'][a+'/'+b]['classification']==v for r in results) for v in ['gain','loss','unresolved']} for a,b in CONTRASTS}
    (OUT/'summary.json').write_text(json.dumps(dict(counts=counts,results=results),indent=2)+'\n');print(json.dumps(counts,indent=2))

if __name__=='__main__':
    import sys
    if '--analyze' not in sys.argv:main()
    summarize()
