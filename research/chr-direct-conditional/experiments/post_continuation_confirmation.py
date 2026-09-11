"""Prospectively registered confirmation using the unchanged pilot artifact."""
import gzip,hashlib,json,os,random,resource,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];PARENT=ROOT/'docs/experiments/results/s10-post-continuation-cost';BASE=PARENT/'confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    BASE.mkdir(exist_ok=True);assert not (BASE/'freeze.json').exists()
    parent=json.loads((PARENT/'freeze.json').read_text());binary=parent['binaries']['time'];assert sha(Path(binary['path']))==binary['sha256']
    for p,h in parent['sources'].items():assert sha(ROOT/p)==h,p
    cases=[]
    for family in ['common','independent','early']:
        case=[family,3,16,True,False,'all']
        for demand in ['birth','birth-miss','birth-miss-template']:
            for control in ['native-scan','active-native-scan','sealed-scan']:cases.append(dict(case=case,left=[demand,True],right=[control,True]))
        for mode in ['birth-miss','conditional','native-scan']:cases.append(dict(case=case,left=[mode,True],right=[mode,False]))
    for mode in ['birth-miss','conditional','native-scan']:cases.append(dict(case=['common',0,0,True,False,'all'],left=[mode,True],right=[mode,False]))
    assert len(cases)==39
    paths=[Path(__file__),ROOT/'research/chr-direct-conditional/experiments/audit_post_continuation_confirmation.py',ROOT/'docs/experiments/registrations/S10-post-continuation-confirmation.md']
    freeze=dict(cases=cases,binary=binary,parent_sha256=sha(PARENT/'freeze.json'),sources={str(p.relative_to(ROOT)):sha(p) for p in paths},clock_floor_ns=43200)
    (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2));rng=random.Random(781042);start=time.monotonic();n=0
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for rep in range(-1,64):
            order=list(range(39));rng.shuffle(order)
            for index in order:
                case=cases[index];sides=['left','right'];rng.shuffle(sides)
                for side in sides:
                    assert time.monotonic()-start<600
                    mode,counted=case[side];family,k,d,h,r,consumer=case['case'];args=[mode,family,k,d,h,r,consumer,counted,False]
                    cmd=[binary['path']]+[str(x).lower() if isinstance(x,bool) else str(x) for x in args]
                    try:
                        raw=subprocess.run(cmd,capture_output=True,text=True,timeout=60,preexec_fn=limits);record=dict(exit_code=raw.returncode,stdout=raw.stdout,stderr=raw.stderr)
                    except subprocess.TimeoutExpired as e:
                        record=dict(exit_code=None,stdout=str(e.stdout),stderr=str(e.stderr))
                    out.write(json.dumps(dict(index=index,rep=rep,side=side,command=cmd,raw=record))+'\n');out.flush();n+=1;assert record['exit_code']==0,record
            if rep%8==0:print(rep,n,round(time.monotonic()-start,2),flush=True)
    (BASE/'campaign.json').write_text(json.dumps(dict(processes=n,seconds=time.monotonic()-start),indent=2))
if __name__=='__main__':main()
