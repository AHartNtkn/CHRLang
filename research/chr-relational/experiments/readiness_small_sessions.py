"""Registered repeated complete-session follow-up for five noisy contrasts."""
import json
import random
import subprocess
import zipfile
from pathlib import Path
from readiness_lifecycle import ROOT, CPU, invoke, sha

OUT = ROOT / 'docs/experiments/results/s02-readiness-small-sessions'


def main():
    OUT.mkdir(exist_ok=True)
    assert not (OUT / 'freeze.json').exists()
    binaries, builds = {}, {}
    for flavor in ['ordinary', 'meter']:
        command = ['cargo','test','-p','chr-relational','--test','readiness_lifecycle','--release',
                   '--no-default-features','--no-run','--message-format=json','--target-dir',str(ROOT/'target'/'readiness-lifecycle'/flavor)]
        if flavor == 'meter': command += ['--features','alloc-meter']
        r = subprocess.run(command,cwd=ROOT,capture_output=True,text=True)
        (OUT/f'build-{flavor}.log').write_text(r.stderr.rstrip()+'\n'); assert r.returncode == 0
        artifacts = [json.loads(s) for s in r.stdout.splitlines() if s.startswith('{')]
        b = Path(next(x['executable'] for x in artifacts if x.get('executable') and x.get('target',{}).get('name')=='readiness_lifecycle'))
        binaries[flavor]=b; builds[flavor]=dict(command=command,binary=str(b),sha256=sha(b))
    previous=json.loads((ROOT/'docs/experiments/results/s02-readiness-lifecycle/freeze.json').read_text())
    assert previous['cpu']==CPU
    paths=list(previous['sources'])+[str(Path(__file__).relative_to(ROOT)), 'docs/experiments/registrations/S02-readiness-small-sessions.md']
    hashes={p:sha(ROOT/p) for p in paths}
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in hashes:z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps(dict(builds=builds,sources=hashes,archive_sha256=sha(OUT/'sources.zip'),cpu=CPU,seed=7241),indent=2)+'\n')
    cells=[(m,'false',o) for o in ['cancel','fail'] for m in ['selective','compiled']]+[(m,'true','cancel') for m in ['selective','batch8','batch256','compiled']]
    rng=random.Random(7241)
    with (OUT/'samples.jsonl').open('x') as out:
        for flavor,repetitions in [('ordinary',9),('meter',2)]:
            for rep in range(repetitions):
                order=cells.copy();rng.shuffle(order)
                for mode,shared,outcome in order:
                    receipt=invoke(binaries[flavor],[mode,4,shared,outcome,1,50])
                    row=dict(flavor=flavor,rep=rep,mode=mode,shared=shared,outcome=outcome,receipt=receipt)
                    out.write(json.dumps(row)+'\n');out.flush()
                    assert receipt['exit_code']==0,row
                    samples=[json.loads(s) for s in receipt['stdout'].splitlines()]
                    assert len(samples)==50 and all(s['validated'] for s in samples)
                print(flavor,rep,'complete',flush=True)


if __name__=='__main__':main()
