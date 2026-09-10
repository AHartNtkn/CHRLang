"""Prospectively registered paired resource-phase costs; ordinary allocator only."""
import collections
import hashlib
import itertools
import json
import os
import random
import resource
import statistics
import subprocess
import sys
import time
from pathlib import Path
import phase_ownership as gate

ROOT = gate.ROOT
OUT = ROOT/'docs/experiments/results/s06-capacity-phase-costs'
BASE = [(0,'empty',1,1,'complete'),(2,'tight',1,1,'complete'),
        (4,'tight',1,4,'complete'),(4,'empty',1,4,'complete'),
        (4,'spare',2,4,'complete'),(4,'spare',2,4,'first')]
SCENARIOS = [(n,s,w,r,h,c,stop) for (n,s,w,r,stop),h,c in itertools.product(BASE,['need','token'],['take','later'])]
BINARY = ROOT/'target/s06-capacity-phase-ownership/primary/release/examples/capacity_phase_lifecycle'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def analyze():
    manifest = json.loads((OUT/'manifest.json').read_text())
    assert manifest['scenarios'] == [list(s) for s in SCENARIOS]
    for file,sha in manifest['hashes'].items():
        assert digest(ROOT/file) == sha, file
    receipts = [json.loads(line) for line in (OUT/'runs.jsonl').read_text().splitlines()]
    assert len(receipts) == 1920
    assert [[r[k] for k in ['scenario','cpu','block','mode']] for r in receipts] == manifest['order']
    rows = {}
    for r in receipts:
        assert r['returncode'] == 0 and not r['stderr'],r
        row = json.loads(r['stdout'])
        gate.validate(row,(r['mode'],*SCENARIOS[r['scenario']]),'primary')
        r['row'] = row
        r['total_ns'] = sum(p['ns'] for p in row['phases'])
        rows[r['scenario'],r['cpu'],r['block'],r['mode']] = r
    assert len(rows) == 1920
    comparisons = []
    for i,control in itertools.product(range(24),gate.MODES[1:]):
        cpus = []
        for cpu in [0,1]:
            a = [rows[i,cpu,b,'phase']['total_ns'] for b in range(7)]
            c = [rows[i,cpu,b,control]['total_ns'] for b in range(7)]
            ratios = [x/y for x,y in zip(a,c)]
            median = statistics.median(ratios)
            direction = 'gain' if median <= .9 and max(ratios) < 1 else 'loss' if median >= 1.1 and min(ratios) > 1 else 'unresolved'
            cpus.append(dict(cpu=cpu,median_ratio=median,ratios=ratios,direction=direction,
                             phase_range=[min(a),max(a)],control_range=[min(c),max(c)]))
        def phases(mode):
            rs = [rows[i,cpu,b,mode] for cpu in [0,1] for b in range(7)]
            names = {p['name'] for p in rs[0]['row']['phases']}
            return dict(total_ns=statistics.median(r['total_ns'] for r in rs),
                        phases={name:statistics.median(sum(p['ns'] for p in r['row']['phases'] if p['name']==name) for r in rs) for name in sorted(names)})
        direction = cpus[0]['direction'] if cpus[0]['direction'] == cpus[1]['direction'] else 'unresolved'
        comparisons.append(dict(scenario=SCENARIOS[i],control=control,direction=direction,cpus=cpus,
                                phase=phases('phase'),ordinary=phases(control)))
    result = dict(processes=1920,measured=1680,warmups=240,
                  runs_sha256=digest(OUT/'runs.jsonl'),manifest_sha256=digest(OUT/'manifest.json'),
                  comparisons=comparisons,
                  counts={m:dict(collections.Counter(c['direction'] for c in comparisons if c['control']==m)) for m in gate.MODES[1:]})
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    print(result['counts'])
    for c in comparisons:
        if c['control']=='specialized-scan':
            print(c['scenario'],c['direction'],[round(p['median_ratio'],3) for p in c['cpus']])


def run():
    assert {0,1} <= os.sched_getaffinity(0)
    assert not (OUT/'manifest.json').exists() and not (OUT/'runs.jsonl').exists()
    entry = json.loads((gate.RAW/'manifest.json').read_text())
    paths = [Path(__file__),ROOT/'docs/experiments/registrations/S06-capacity-phase-costs.md',
             gate.RAW/'manifest.json',gate.RAW/'runs.jsonl',gate.RAW/'audit.json']
    hashes = {**entry['sha256'], **{str(p.relative_to(ROOT)):digest(p) for p in paths}}
    for file,sha in hashes.items(): assert digest(ROOT/file)==sha,file
    rng = random.Random(20260923); order=[]
    for block in [-1,*range(7)]:
        jobs = list(itertools.product(range(24),[0,1]));rng.shuffle(jobs)
        for i,cpu in jobs:
            modes = gate.MODES.copy();rng.shuffle(modes)
            order.extend((i,cpu,block,m) for m in modes)
    manifest = dict(scenarios=SCENARIOS,order=order,hashes=hashes,
                    binary=str(BINARY.relative_to(ROOT)),command_arguments=['mode','requests','supply','weight','reuse','head','caller','stop'],
                    affinity=sorted(os.sched_getaffinity(0)),uname=list(os.uname()),
                    cpuinfo=Path('/proc/cpuinfo').read_text())
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    deadline = time.monotonic()+600
    with (OUT/'runs.jsonl').open('x') as out:
        for j,(i,cpu,block,mode) in enumerate(order):
            remaining = deadline-time.monotonic();assert remaining>0,'launcher bound'
            def bounds():
                os.sched_setaffinity(0,{cpu})
                resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
                resource.setrlimit(resource.RLIMIT_CPU,(60,60))
            command = [str(BINARY),mode,*map(str,SCENARIOS[i])]
            receipt = dict(scenario=i,cpu=cpu,block=block,mode=mode,command=command)
            try:
                p = subprocess.run(command,capture_output=True,text=True,timeout=min(60,remaining),preexec_fn=bounds)
                receipt.update(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
            except subprocess.TimeoutExpired as error:
                receipt.update(timeout=True,stdout=str(error.stdout),stderr=str(error.stderr))
                out.write(json.dumps(receipt)+'\n');out.flush();raise
            out.write(json.dumps(receipt)+'\n');out.flush()
            assert p.returncode==0 and not p.stderr,receipt
            gate.validate(json.loads(p.stdout),(mode,*SCENARIOS[i]),'primary')
            if (j+1)%240==0: print(j+1,'/1920 primary processes validated',flush=True)
    analyze()


if __name__ == '__main__':
    assert sys.argv[1:] in [[],['--analyze']]
    analyze() if sys.argv[1:] else run()
