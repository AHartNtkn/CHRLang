"""Prospectively frozen paired timing confirmation using the existing binary."""
import hashlib,itertools,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s01-prefix-confirmation'
PARENT=ROOT/'docs/experiments/results/s01-prefix-lifecycle'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists()
    parent=json.loads((PARENT/'freeze.json').read_text());binary=parent['binaries']['time'];assert sha(Path(binary['path']))==binary['sha256'];assert sha(PARENT/'sources.zip')==parent['archive_sha256'];assert 0 in os.sched_getaffinity(0)
    scenarios=list(itertools.product([f'prefix-{f}-{d}' for f in ['sparse','keyed','kill','miss'] for d in [0,32]],[4,8],[1,4],['immediate','all']))
    blocks=list(itertools.product(range(64),range(64)));rng=random.Random(81210);rng.shuffle(blocks);jobs=[]
    for block,(rep,index) in enumerate(blocks):
        modes=['intermediate','local-scan','native-indexed' if scenarios[index][0].startswith('prefix-kill-') else 'active-native-indexed'];rng.shuffle(modes)
        for position,mode in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=index,position=position,mode=mode))
    for name,data in [('scenarios',scenarios),('jobs',jobs)]: (BASE/f'{name}.json').write_text(json.dumps(data)+'\n')
    paths=['docs/experiments/registrations/S01-prefix-confirmation.md','research/chr-relational/experiments/prefix_confirmation.py','research/chr-relational/experiments/audit_prefix_confirmation.py']
    freeze=dict(binary=binary,parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_archive_sha256=sha(PARENT/'sources.zip'),sources={p:sha(ROOT/p) for p in paths},source_text={p:(ROOT/p).read_text() for p in paths},orders={n:sha(BASE/f'{n}.json') for n in ['scenarios','jobs']},cpu=0,affinity=sorted(os.sched_getaffinity(0)),platform=os.uname().release,commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    def invoke(args,path):
        cmd=[binary['path'],*map(str,args)]
        try:
            r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
            raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
            raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
        path.write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],path
    invoke([],BASE/'smoke.json');invoke(['prefix-check'],BASE/'prefix-smoke.json')
    for rep in range(5):invoke(['clock-check'],BASE/f'clock-{rep}.json')
    (BASE/'runs').mkdir();start=time.monotonic()
    for i,job in enumerate(jobs):
        assert time.monotonic()-start<1800
        fam,width,reuse,consumer=scenarios[job['index']]
        invoke([job['mode'],fam,width,reuse,consumer],BASE/'runs'/f'{i}.json')
        if (i+1)%1024==0:print(i+1,'/',len(jobs),flush=True)
    assert sha(Path(binary['path']))==binary['sha256']
    assert all(sha(ROOT/p)==h for p,h in freeze['sources'].items())
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=len(jobs),seconds=time.monotonic()-start))+'\n')
if __name__=='__main__':main()
