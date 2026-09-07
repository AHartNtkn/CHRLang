"""Exploratory scaling; preserve every success and censored outcome."""
import json
import os
import platform
import signal
import subprocess
import sys
import time
from run_maintained_gate import ROOT,OUT,digest,bound

PREFIX='E09-maintained-sizing-v1'

def cells():
    return [dict(consuming=c,n=n,branches=b,rounds=r,mode=m,quantum=8,seed=0)
            for c in (False,True) for n,b,r in ((4,1,1),(16,2,4),(64,1,1),(64,8,4))
            for m in ('prefix','full','selective')]

def main():
    gate=json.loads((OUT/'E09-maintained-gate-v2-audit.json').read_text())
    assert gate['passed']
    paths=sorted((ROOT/'research/chr-scheduling').glob('*.py'))+sorted((ROOT/'research/chr-symbolic').glob('*.py'))
    paths+=[ROOT/'docs/experiments/registrations/E09-maintained-sizing.md']
    frozen={str(p.relative_to(ROOT)):digest(p) for p in paths}
    manifest_path=OUT/f'{PREFIX}-manifest.json';records_path=OUT/f'{PREFIX}.jsonl'
    if manifest_path.exists() or records_path.exists():raise FileExistsError('evidence exists')
    with manifest_path.open('x') as mf,records_path.open('x') as rf:
        json.dump(dict(sha256=frozen,order=cells(),python=sys.version,platform=platform.platform(),
                       timeout_seconds=30,address_space_bytes=1073741824,purpose='exploratory resource sizing, not timing comparison'),mf,indent=2)
        mf.write('\n');mf.flush();os.fsync(mf.fileno())
        for cell in cells():
            assert all(digest(ROOT/p)==h for p,h in frozen.items())
            start=time.monotonic()
            command=[sys.executable,__file__,'--child',json.dumps(cell)]
            proc=subprocess.Popen(command,cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,
                                  preexec_fn=bound,start_new_session=True,
                                  env={**os.environ,'PYTHONDONTWRITEBYTECODE':'1','PYTHONHASHSEED':'0'})
            try:stdout,stderr=proc.communicate(timeout=30);status=proc.returncode
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid,signal.SIGKILL);stdout,stderr=proc.communicate();status='timeout'
            row=dict(cell=cell,command=command,exit=status,stderr=stderr,wall_seconds=time.monotonic()-start)
            try:
                assert status==0
                row['result']=json.loads(stdout)
            except (AssertionError,ValueError):row['stdout']=stdout
            rf.write(json.dumps(row,sort_keys=True)+'\n');rf.flush();os.fsync(rf.fileno())
        assert all(digest(ROOT/p)==h for p,h in frozen.items())
    print('24 sizing outcomes recorded')

if __name__=='__main__':
    if len(sys.argv)>1:
        from maintained_cases import case
        from check_maintained_source import check
        c=json.loads(sys.argv[2])
        print(json.dumps(check(case(c['n'],c['branches'],c['rounds'],c['consuming']),c['mode'],8,False),sort_keys=True))
    else:main()
