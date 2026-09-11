"""Qualified broad-incidence source execution and ordinary timing pilot."""
import gzip,hashlib,itertools,json,os,random,resource,statistics,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-incidence-source-timing'
VARIANTS=[(False,'full'),(True,'full'),(False,'batch256'),(True,'batch256'),(False,'compiled')]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{min(os.sched_getaffinity(0))});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(build,mode,case):
    cmd=[build['binary'],mode,*map(str,case), '2']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    if r.returncode:(OUT/'failure.json').write_text(json.dumps({'cmd':cmd,'stderr':r.stderr},indent=2)+'\n')
    assert r.returncode==0,(cmd,r.stderr)
    data=json.loads(r.stdout);assert data['validated'] and not data['metered'];assert len(data['queries'])==2
    return data

def audit():
    freeze=json.loads((OUT/'freeze.json').read_text())
    rows=[json.loads(x) for x in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==300
    assert sha(OUT/'sources.zip')==freeze['source_hash']
    for b in freeze['builds'].values():assert sha(Path(b['binary']))==b['sha256']
    assert [r['job'] for r in rows]==freeze['jobs']
    entries=[json.loads(x) for x in gzip.open(OUT/'entry.jsonl.gz','rt')];assert len(entries)==72 and all(r['result']['validated'] for r in entries)
    warm=[json.loads(x) for x in gzip.open(OUT/'warmups.jsonl.gz','rt')];assert len(warm)==60
    cells={}
    admission={}
    for row in rows:
        j=row['job'];r=row['result'];assert r['validated'] and not r['metered'] and len(r['queries'])==2
        phases=[r['preparation']]+[q[p] for q in r['queries'] for p in ['setup','execution_observation','engine_drop']]+[r['preparation_drop'],r['consumer_drop']]
        assert all('heap' not in p for p in phases)
        if j['case'][2]=='broad-cancel':assert all(q['advances']==0 for q in r['queries'])
        key=(tuple(j['case']),j['variant'])
        assert j['block'] not in cells.setdefault(key,{})
        cells[key][j['block']]=sum(p['ns'] for p in phases)
        admission.setdefault(key,{})[j['block']]=sum(q['setup']['ns'] for q in r['queries'])
    assert len(cells)==60 and all(set(x)==set(range(5)) for x in cells.values())
    results=[]
    for case in sorted(set(k[0] for k in cells)):
        for a,b in itertools.combinations(range(5),2):
            ratios=[cells[case,b][i]/cells[case,a][i] for i in range(5)];median=statistics.median(ratios)
            results.append({'case':case,'reference':VARIANTS[a],'candidate':VARIANTS[b],'median_ratio':median,'min_ratio':min(ratios),'max_ratio':max(ratios),'reference_ns':statistics.median(cells[case,a].values()),'candidate_ns':statistics.median(cells[case,b].values()),'screen':'gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'})
    bounds=[]
    for case in sorted(set(k[0] for k in cells)):
        for v in [1,3]:
            ratios=[(cells[case,v][i]-admission[case,v][i])/cells[case,4][i] for i in range(5)]
            bounds.append({'case':case,'vector_schedule':VARIANTS[v][1],'status':'optimistic zero-setup sensitivity, not an implemented engine','median_ratio':statistics.median(ratios),'min_ratio':min(ratios),'max_ratio':max(ratios)})
    (OUT/'zero-setup.json').write_text(json.dumps(bounds,indent=2)+'\n')
    (OUT/'analysis.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps({'entry_processes':len(entries),'timing_processes':len(rows),'contrasts':len(results)}))

def run():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
    for vector in [False,True]:
        cmd=['cargo','test','-p','chr-relational','--release','--no-default-features','--test','readiness_lifecycle','--no-run','--message-format=json']
        if vector:cmd+=['--features','incidence-vector']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{vector}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        exe=next(json.loads(x)['executable'] for x in r.stdout.splitlines() if x.startswith('{') and json.loads(x).get('executable'))
        b=ROOT/f'target/incidence-source-{vector}';b.write_bytes(Path(exe).read_bytes());b.chmod(0o755);builds[str(vector)]={'binary':str(b),'sha256':sha(b),'command':cmd}
    cases=[(64,order,outcome) for order,outcome in itertools.product(['false','true'],['broad-forward','broad-reverse'])]
    cases += [(128,'false','broad-forward'),(128,'true','broad-reverse'),(64,'false','broad-cancel'),(64,'true','broad-cancel'),(64,'false','fail'),(64,'true','success'),(64,'true','clash'),(4,'false','success')]
    jobs=[];rng=random.Random(7211)
    for block in range(5):
        groups=list(enumerate(cases));rng.shuffle(groups)
        for index,case in groups:
            for v in [((block+index)+i)%5 for i in range(5)]:jobs.append({'block':block,'case':case,'variant':v})
    paths=subprocess.check_output(['git','ls-files','research/chr-relational','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-direct-conditional','research/chr-integrated','crates/chr-syntax','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S02-incidence-source-timing.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(paths)):z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps({'builds':builds,'jobs':jobs,'source_hash':sha(OUT/'sources.zip'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True)},indent=2)+'\n')
    with gzip.open(OUT/'entry.jsonl.gz','wt') as out:
        for vector,mode,n,order,outcome in itertools.product([False,True],['full','batch256','compiled'],[8,64,128],['false','true'],['broad-forward','broad-reverse']):
            case=(n,order,outcome);out.write(json.dumps({'vector':vector,'mode':mode,'case':case,'result':invoke(builds[str(vector)],mode,case)})+'\n');out.flush()
    print('source entry passed',flush=True)
    with gzip.open(OUT/'warmups.jsonl.gz','wt') as out:
        for case in cases:
            for vector,mode in VARIANTS:out.write(json.dumps({'case':case,'vector':vector,'mode':mode,'result':invoke(builds[str(vector)],mode,case)})+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            vector,mode=VARIANTS[j['variant']];out.write(json.dumps({'job':j,'result':invoke(builds[str(vector)],mode,j['case'])})+'\n');out.flush()
            if (i+1)%60==0:print('block',(i+1)//60,'complete',flush=True)
    audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
