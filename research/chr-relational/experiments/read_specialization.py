"""Run the registered missing-control contrast using preserved primary binaries."""
import hashlib, json, os, random, resource, subprocess, time
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s02-read-specialization'
PARENT = ROOT / 'docs/experiments/results/s02-read-cost/freeze.json'
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0, {0})
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
def main():
    assert not (BASE / 'freeze.json').exists()
    parent = json.loads(PARENT.read_text())
    for path, digest in parent['sources'].items():
        assert sha(ROOT / path) == digest, path
    binaries = {k: parent['binaries'][k] for k in ['ordinary', 'meter']}
    for binary in binaries.values():
        assert sha(Path(binary['path'])) == binary['sha256']
    cases = [('near-repeated-8',8,1), ('near-repeated-32',8,1),
             ('near-repeated-32',8,4), ('near-unique-32',8,4), ('near-mixed-32',8,4)]
    modes = ['sealed', 'indexed', 'validated', 'contextual']
    configs = [{'case': i, 'mode': m, 'cancel': False} for i in range(5) for m in modes]
    order = [dict(c, build='meter', rep=r, warmup=False) for c in configs for r in range(2)]
    rng = random.Random(86402)
    for rep in range(-1,5):
        ids = list(range(5)); rng.shuffle(ids)
        for i in ids:
            block = [c for c in configs if c['case'] == i]; rng.shuffle(block)
            order += [dict(c, build='ordinary', rep=rep, warmup=rep<0) for c in block]
    for i in range(5):
        for mode in ['sealed','indexed']:
            for kind, rep in [('meter',0),('meter',1),('ordinary',0)]:
                order.append(dict(case=i,mode=mode,cancel=True,build=kind,rep=rep,warmup=False))
    assert len(order) == 190
    (BASE/'cases.json').write_text(json.dumps(cases,indent=2)+'\n')
    (BASE/'order.json').write_text(json.dumps(order,indent=2)+'\n')
    files = [Path(__file__), ROOT/'research/chr-relational/tests/read_specialization.rs',
             ROOT/'docs/experiments/registrations/S02-read-specialization.md', PARENT]
    (BASE/'freeze.json').write_text(json.dumps(dict(
        sources={str(p.relative_to(ROOT)):sha(p) for p in files}, binaries=binaries,
        order_hash=sha(BASE/'order.json'), cases_hash=sha(BASE/'cases.json'),
        affinity=sorted(os.sched_getaffinity(0)), cpu=0),indent=2)+'\n')
    def invoke(kind,args,path):
        r = subprocess.run([binaries[kind]['path'],*map(str,args)],cwd=ROOT,
            env=dict(os.environ,DEDUCTION_RETAIN='all'),capture_output=True,text=True,
            timeout=60,preexec_fn=limits)
        path.write_text(json.dumps(dict(args=args,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n')
        assert r.returncode == 0 and not r.stderr, path
        return r.stdout
    invoke('meter',['meter-check'],BASE/'meter-check.json')
    for i in range(5):
        invoke('ordinary',['clock-check'],BASE/f'clock-{i}.json')
    (BASE/'runs').mkdir(); start=time.monotonic()
    with (BASE/'results.jsonl').open('x') as output:
        for i, item in enumerate(order):
            assert time.monotonic()-start<600
            family, depth, queries=cases[item['case']]
            args=[item['mode'],family,depth,4 if item['cancel'] else queries,1,0]
            if item['cancel']: args.append(1)
            raw=invoke(item['build'],args,BASE/'runs'/f'{i}.json')
            result=[json.loads(x) for x in raw.splitlines() if x.startswith('{')][-1]
            assert not result['counters'] and result['retained'] and result['resource']
            assert [s['complete'] for s in result['samples']]==([False,True,False,True] if item['cancel'] else [True]*queries)
            output.write(json.dumps(dict(index=i,**item,result=result))+'\n');output.flush()
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=190,seconds=time.monotonic()-start))+'\n')
    print('Completed all 190 registered workload processes')
if __name__=='__main__':main()
