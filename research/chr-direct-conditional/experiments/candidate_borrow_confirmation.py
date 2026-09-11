"""Prospectively registered confirmation of 15 selected paired contrasts."""
import collections,gzip,hashlib,json,math,os,random,resource,statistics,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];PARENT=ROOT/'docs/experiments/results/s03-candidate-borrow';BASE=PARENT/'confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    BASE.mkdir(exist_ok=True);assert not (BASE/'freeze.json').exists()
    parent=json.loads((PARENT/'freeze.json').read_text());audit=json.loads((PARENT/'audit.json').read_text())
    bins={k:parent['binaries'][k] for k in ['old-time','new-time']}
    for b in bins.values():assert sha(Path(b['path']))==b['sha256']
    scenarios=[['post-duplicate',8,False,'window'],['post-duplicate',32,True,'immediate'],['post-miss',32,False,'all'],['post-forward',8,False,'all'],['post-output',32,True,'all']]
    cases=[[*s,m] for s in scenarios for m in ['birth','birth-miss','birth-miss-template']]
    rng=random.Random(710734);jobs=[]
    for rep in [-1,*range(64)]:
        order=list(range(15));rng.shuffle(order)
        for index in order:
            kinds=list(bins);rng.shuffle(kinds)
            for kind in kinds:jobs.append(dict(index=index,kind=kind,rep=rep))
    paths=['research/chr-direct-conditional/experiments/candidate_borrow_confirmation.py','research/chr-direct-conditional/experiments/audit_candidate_borrow_confirmation.py','docs/experiments/registrations/S03-candidate-borrow-confirmation.md']
    (BASE/'freeze.json').write_text(json.dumps(dict(parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_audit_sha256=sha(PARENT/'audit.json'),binaries=bins,cases=cases,jobs=jobs,sources={p:dict(sha256=sha(ROOT/p),text=(ROOT/p).read_text()) for p in paths},family_error_bound=30*sum(math.comb(64,i) for i in range(20))/2**64,clock_floor_ns=audit['clock_floor_ns']),indent=2))
    start=time.monotonic()
    with gzip.open(BASE/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            assert time.monotonic()-start<1800
            fam,n,rev,ret,mode=cases[j['index']];cmd=[bins[j['kind']]['path'],mode,fam,str(n),str(rev).lower(),ret,'false']
            try:
                r=subprocess.run(cmd,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits);raw=dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
            except subprocess.TimeoutExpired as e:
                dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
                raw=dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
            out.write(json.dumps(dict(job=i,**raw))+'\n');out.flush();assert raw['exit_code']==0 and not raw['stderr'],(i,raw)
    (BASE/'campaign.json').write_text(json.dumps(dict(primary=1920,warmups=30,seconds=time.monotonic()-start)))
if __name__=='__main__':main()
