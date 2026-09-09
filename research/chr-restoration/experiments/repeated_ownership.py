#!/usr/bin/env python3
"""Registered allocation-only source comparison; stop on first failure."""
import hashlib, itertools, json, pathlib, random, resource, shutil, subprocess, sys
ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s04-repeated-reunion-ownership'
BIN = ROOT / 'target/repeated-reunion-ownership/repeated_cost'
def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
def run(args, path):
    command = [str(BIN), *map(str, args)]
    try:
        p = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=60, preexec_fn=limits)
        receipt = {'command': command, 'returncode': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
    except subprocess.TimeoutExpired as e:
        receipt = {'command':command, 'timeout':60, 'stdout':str(e.stdout), 'stderr':str(e.stderr)}
    path.write_text(json.dumps(receipt, indent=2)+'\n')
    assert receipt.get('returncode') == 0, path
    return json.loads(receipt['stdout'])
def main():
    OUT.mkdir(exist_ok=False)
    BIN.parent.mkdir(exist_ok=False)
    shutil.copy2(ROOT/'target/release/examples/repeated_cost',BIN)
    sources=['research/chr-restoration/src/lib.rs','research/chr-restoration/src/reunion.rs','research/chr-restoration/examples/repeated_cost.rs','research/chr-restoration/examples/support/repeated_source.rs','research/chr-compiled/experiments/meter.rs','docs/experiments/registrations/S04-repeated-reunion-ownership.md']
    freeze={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'binary_sha256':hashlib.sha256(BIN.read_bytes()).hexdigest(),'source_sha256':{s:hashlib.sha256((ROOT/s).read_bytes()).hexdigest() for s in sources}}
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    shutil.copy2('/tmp/chr-repeated-build.log',OUT/'build.log')
    configs=list(itertools.product(['copy','reunion','repeated'],['plain','history','late'],[0,1,3],[0,12]))
    (OUT/'preflight').mkdir()
    for i,c in enumerate(configs): run((*c,1),OUT/'preflight'/f'{i:03}.json')
    print('54 preflight processes passed',flush=True)
    cells=[(*c,reuse) for c in configs for reuse in [1,4]]
    jobs=[(rep,c) for rep in range(2) for c in cells]
    random.Random(7708).shuffle(jobs)
    (OUT/'order.json').write_text(json.dumps(jobs)+'\n')
    (OUT/'runs').mkdir()
    for i,(rep,c) in enumerate(jobs):
        run(c,OUT/'runs'/f'{i:03}-r{rep}.json')
        if (i+1)%24==0: print(f'{i+1}/216 matrix processes passed',flush=True)
if __name__=='__main__': main()
