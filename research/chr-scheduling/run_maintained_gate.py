"""Bounded semantic gate; exclusive, pre-frozen evidence. Not a timing study."""
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import signal
import subprocess
import sys
import time

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'docs/experiments/results'
PREFIX='E09-maintained-gate-v2'

def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def cells():
    store=[dict(kind='store',family=f,mode=m) for f in range(4) for m in ('full','selective')]
    source=[dict(kind='source',consuming=c,n=4,branches=b,rounds=1,mode=m,quantum=q)
            for c in (False,True) for b in (1,2) for m in ('prefix','full','selective') for q in (1,8)]
    # 2 consumption variants * 2 branch counts * 3 modes * 2 quanta = 24.
    return [dict(c,seed=0) for c in store+source]+[dict(kind='source',consuming=True,n=4,branches=b,rounds=1,mode='selective',quantum=q,seed=seed) for seed in (1,2) for b in (1,2) for q in (1,8)]

def child(cell):
    if cell['kind']=='source':
        from maintained_cases import case
        from check_maintained_source import check
        return check(case(cell['n'],cell['branches'],cell['rounds'],cell['consuming']),cell['mode'],cell['quantum'],True)
    from maintained_store_cases import cases
    from maintained_join import MaintainedJoin
    from check_maintained_store import validate, expected
    fixture=list(cases())[cell['family']]
    index=MaintainedJoin(fixture['rules'],cell['mode'],8)
    rows=[]
    for label,states in fixture['updates']:
        validate(index,states)
        rows.append(dict(stage=label,contexts=len(states),counts=dict(index.counts),
                         matches={str(ctx):[expected(index.rules,state,r) for r in range(len(index.rules))] for ctx,state in states.items()},
                         peak_rows=index.peak_rows,peak_tests=index.peak_tests,
                         peak_dependencies=index.peak_dependencies,peak_index_edges=index.peak_index_edges))
    return dict(id=fixture['id'],mode=cell['mode'],stages=rows)

def bound(): resource.setrlimit(resource.RLIMIT_AS,(1073741824,1073741824))

def main():
    paths=sorted((ROOT/'research/chr-scheduling').glob('*.py'))
    paths += [ROOT/'docs/experiments/registrations/E09-maintained-gate.md']
    # The independent tree control and its transitive local implementation.
    paths += sorted((ROOT/'research/chr-symbolic').glob('*.py'))
    frozen={str(p.relative_to(ROOT)):digest(p) for p in paths}
    order=[dict(phase=phase,cell=cell) for phase in ('gate','replay') for cell in cells()]
    manifest_path=OUT/f'{PREFIX}-manifest.json'; records_path=OUT/f'{PREFIX}.jsonl'
    if manifest_path.exists() or records_path.exists(): raise FileExistsError('evidence exists')
    with manifest_path.open('x') as manifest, records_path.open('x') as records:
        json.dump(dict(sha256=frozen,order=order,timeout_seconds=30,address_space_bytes=1073741824,
                       python=sys.version,platform=platform.platform(),purpose='semantic gate; wall time is only a bound diagnostic'),manifest,indent=2)
        manifest.write('\n');manifest.flush();os.fsync(manifest.fileno())
        for entry in order:
            assert all(digest(ROOT/p)==h for p,h in frozen.items()),'frozen input changed'
            start=time.monotonic()
            proc=subprocess.Popen([sys.executable,__file__,'--child',json.dumps(entry['cell'])],cwd=ROOT,
                                  stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=bound,start_new_session=True,
                                  env={**os.environ,'PYTHONDONTWRITEBYTECODE':'1','PYTHONHASHSEED':str(entry['cell']['seed'])})
            try:
                stdout,stderr=proc.communicate(timeout=30);status=proc.returncode
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate();status='timeout'
            row={**entry,'exit':status,'stderr':stderr,'wall_seconds':time.monotonic()-start}
            try:
                assert status==0
                row['result']=json.loads(stdout)
            except (AssertionError,ValueError):row['stdout']=stdout
            records.write(json.dumps(row,sort_keys=True)+'\n');records.flush();os.fsync(records.fileno())
            if status!=0 or 'result' not in row: raise RuntimeError(f'unsuccessful child preserved: {entry}')
        assert all(digest(ROOT/p)==h for p,h in frozen.items()),'frozen input changed'
    print(f'{len(order)} isolated children recorded')

if __name__=='__main__':
    if len(sys.argv)>1 and sys.argv[1]=='--child': print(json.dumps(child(json.loads(sys.argv[2])),sort_keys=True))
    else: main()
