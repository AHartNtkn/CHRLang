#!/usr/bin/env python3
import hashlib,itertools,json,resource,subprocess
from pathlib import Path
root=Path('docs/experiments/results/s08-support-repetition')
binary=Path('target/s08-repetition/stream').resolve()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
def main():
    root.mkdir(exist_ok=False)
    files=[Path(__file__),Path('Cargo.lock'),Path('docs/experiments/registrations/S08-support-repetition.md')]
    for d in ['research/chr-direct-conditional','research/chr-reuse','crates/chr-syntax']:
        files.extend(Path(d).rglob('*.rs'));files.append(Path(d)/'Cargo.toml')
    freeze={str(p):sha(p) for p in files+[binary]};(root/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    (root/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    cells=list(itertools.product(['conditional','inferred'],['aliases','distinct'],[0,1,16,64]));summary=[]
    old=json.loads(Path('docs/experiments/results/s08-stream-allocation-attribution/summary.json').read_text())
    for i,c in enumerate(cells):
        prior=None
        for rep in range(2):
            run=subprocess.run([str(binary),*map(str,c)],capture_output=True,text=True,timeout=150,preexec_fn=limits)
            (root/f'{rep}-{i}.json').write_text(json.dumps(dict(command=[str(binary),*map(str,c)],returncode=run.returncode,stdout=run.stdout,stderr=run.stderr),indent=2)+'\n');assert run.returncode==0,c
            d=json.loads(run.stdout)
            if prior is not None:assert prior==d
            prior=d
        if c[2]>=16:
            control=next(x for x in old if (x['mode'],x['family'],x['depth'])==c)
            assert d['ticks']==sum(x[0] for x in control['stages'])
        seen={};counts=dict(jobs=0,cheap=0,substantive=0,unique_substantive=0,repeated_substantive=0,first_frames=0,repeated_frames=0,unfinished=0)
        for op,a,b,cheap,frames,result in d['trace']:
            counts['jobs']+=1
            if result==2**64-1:counts['unfinished']+=1;continue
            if cheap:
                assert frames==1
                counts['cheap']+=1;continue
            counts['substantive']+=1;key=(op,a,b)
            if key in seen:
                assert seen[key]==result
                counts['repeated_substantive']+=1;counts['repeated_frames']+=frames
            else:
                seen[key]=result;counts['unique_substantive']+=1;counts['first_frames']+=frames
        row=dict(mode=c[0],family=c[1],depth=c[2],ticks=d['ticks'],**counts);summary.append(row);print(row,flush=True)
    assert all(sha(Path(p))==h for p,h in freeze.items())
    (root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    (root/'audit.json').write_text(json.dumps(dict(processes=32,exact_replays=16,prior_tick_controls=8,consistent_completed_roots=True,timing='not_run',allocation='not_measured'),indent=2)+'\n')
if __name__=='__main__':main()
