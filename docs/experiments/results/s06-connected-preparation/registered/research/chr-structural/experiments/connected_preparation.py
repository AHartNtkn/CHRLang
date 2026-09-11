"""Matched clock attribution of connected preparation, preserving raw outputs."""
from pathlib import Path
import hashlib,itertools,json,random,resource,statistics,subprocess
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-connected-preparation'
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def run():
    RAW.mkdir(exist_ok=False)
    bins={kind:ROOT/f'target/s06-connected-{suffix}/release/examples/connected_cost' for kind,suffix in [('profile','profile'),('control','profile-control')]}
    sources=[ROOT/'research/chr-structural/src/joint_region.rs',ROOT/'research/chr-structural/examples/connected_cost.rs',Path(__file__),ROOT/'docs/experiments/registrations/S06-connected-preparation.md',*bins.values()]
    (RAW/'inputs.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources},indent=2)+'\n')
    cells=list(itertools.product(['star','clique','different'],[4,6],[0,1]));schedule=[];rng=random.Random(607092)
    for block in range(10):
        batch=[(block,kind,*cell)for cell in cells for kind in bins];rng.shuffle(batch);schedule.extend(batch)
    (RAW/'schedule.json').write_text(json.dumps(schedule)+'\n')
    for i,(block,kind,family,n,duplicate) in enumerate(schedule):
        cmd=[str(bins[kind]),'projection',family,str(n),str(duplicate),'1','4','1','0']
        p=subprocess.run(cmd,capture_output=True,text=True,timeout=45,preexec_fn=bounds)
        (RAW/f'{i:03}.json').write_text(json.dumps(dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr),indent=2)+'\n');assert p.returncode==0 and not p.stderr
    (RAW/'analysis.json').write_text(json.dumps(analyze(),indent=2)+'\n')
def analyze():
    values={}
    for i,(_,kind,f,n,d) in enumerate(json.loads((RAW/'schedule.json').read_text())):
        r=json.loads((RAW/f'{i:03}.json').read_text());assert r['returncode']==0 and not r['stderr']
        lines=[json.loads(x)for x in r['stdout'].splitlines()];header=lines[0];rows=[x for x in lines if 'phase' in x]
        assert len(rows)==38 and sum(x['ns']for x in rows)==header['total_ns']
        if kind=='profile':
            scopes=lines[1]['preparation_ns'];prep=next(x['ns']for x in rows if x['phase']=='prepare');assert sum(scopes)<=prep
            values.setdefault((f,n,d),[]).append([*scopes,prep-sum(scopes),prep])
    out=[]
    for cell,rows in sorted(values.items()):
        assert len(rows)==10
        out.append(dict(source=cell,median_ns=[statistics.median(x[i]for x in rows)for i in range(6)],median_share=[statistics.median(x[i]/x[-1]for x in rows)for i in range(5)]))
    assert len(out)==12
    return out
if __name__=='__main__':
    import sys
    if sys.argv[1]=='run':run()
    else:
        assert json.loads(json.dumps(analyze()))==json.loads((RAW/'analysis.json').read_text())
        print('240 terminal processes and 120 exclusive phase sums verified.')
