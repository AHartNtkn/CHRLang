#!/usr/bin/env python3
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess

out = Path('docs/experiments/results/s08-cross-query-ownership')
binary = Path('target/s08-cross-query/ownership').resolve()
configs = [(m, f, n, 1, k, 0, fail) for m,f,n,k,fail in itertools.product(
    ['conditional','inferred','scan','resumable'], ['aliases','distinct'], [16,64], ['0','4','all'], [0,1])]
configs += [(m,'aliases',64,1,k,1,0) for m,k in itertools.product(['conditional','inferred','scan','resumable'], ['0','4','all'])]

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3,1024**3))
    resource.setrlimit(resource.RLIMIT_CPU, (120,120))

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    out.mkdir(exist_ok=False)
    files = [Path(__file__), Path('Cargo.lock'), Path('Cargo.toml'), Path('docs/experiments/registrations/S08-cross-query-ownership.md')]
    for directory in ['research/chr-direct-conditional','research/chr-compiled','research/chr-persistent','research/chr-relational','research/chr-observe','research/chr-reuse','crates/chr-syntax']:
        files.extend(Path(directory).rglob('*.rs')); files.append(Path(directory)/'Cargo.toml')
    freeze = {str(p):digest(p) for p in files+[binary]}
    (out/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    (out/'toolchain.txt').write_text(subprocess.check_output(['rustc','-Vv'],text=True))
    jobs = [(rep,*c) for rep in range(2) for c in configs]
    random.Random(7860).shuffle(jobs)
    (out/'order.json').write_text(json.dumps(jobs))
    check = subprocess.run([str(binary),'meter-check'],capture_output=True,text=True,timeout=150,preexec_fn=limits)
    (out/'meter-check.log').write_text(check.stdout+check.stderr); assert check.returncode == 0
    prior = {}
    for i, (rep,*c) in enumerate(jobs):
        command = [str(binary),*map(str,c)]
        run = subprocess.run(command,capture_output=True,text=True,timeout=150,preexec_fn=limits)
        (out/f'run-{i:03}.json').write_text(json.dumps(dict(command=command,returncode=run.returncode,stdout=run.stdout,stderr=run.stderr),indent=2)+'\n')
        assert run.returncode == 0, i
        data = json.loads(run.stdout.splitlines()[-1]); key=tuple(c)
        if key in prior: assert data == prior[key], key
        prior[key] = data
        if (i+1)%24 == 0: print(i+1, 'of',len(jobs),'processes pass',flush=True)
    assert all(digest(Path(p)) == h for p,h in freeze.items())
    (out/'completion.json').write_text(json.dumps(dict(processes=len(jobs),cells=len(configs),replays=len(prior),timing='not_run'))+'\n')

if __name__ == '__main__': main()
