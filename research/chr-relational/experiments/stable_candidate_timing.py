"""Interleaved lifecycle timing of inferred invalidation and serious controls."""
import gzip,hashlib,itertools,json,os,random,resource,statistics,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-stable-candidate-timing'
VARIANTS=[(False,'full'),(True,'full'),(False,'batch256'),(True,'batch256'),(False,'compiled')]
CPU=min(os.sched_getaffinity(0))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(build,mode,case):
    cmd=[build['binary'],mode,*map(str,case),'2','10']
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    if r.returncode:(OUT/'failure.json').write_text(json.dumps({'cmd':cmd,'stderr':r.stderr},indent=2)+'\n')
    assert r.returncode==0,(cmd,r.stderr)
    sessions=[json.loads(s) for s in r.stdout.splitlines()];assert len(sessions)==10 and all(s['validated'] and not s['metered'] for s in sessions)
    return sessions

def audit():
    freeze=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==freeze['source_hash']
    for b in freeze['builds'].values():assert sha(Path(b['binary']))==b['sha256']
    rows=[json.loads(s) for s in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==1200 and [r['job'] for r in rows]==freeze['jobs']
    warm=[json.loads(s) for s in gzip.open(OUT/'warmups.jsonl.gz','rt')];assert len(warm)==60 and all(len(r['sessions'])==10 and all(s['validated'] for s in r['sessions']) for r in warm)
    cells={}
    for row in rows:
        j=row['job'];sessions=row['sessions'];assert len(sessions)==10;totals=[]
        for s in sessions:
            assert s['validated'] and not s['metered'] and len(s['queries'])==2
            phases=[s['preparation']]+[q[p] for q in s['queries'] for p in ['setup','execution_observation','engine_drop']]+[s['preparation_drop'],s['consumer_drop']]
            assert all(set(p)=={'ns'} for p in phases)
            if j['case'][2]=='broad-cancel':assert all(q['advances']==0 for q in s['queries'])
            totals.append(sum(p['ns'] for p in phases))
        cell=cells.setdefault((tuple(j['case']),j['variant']),{});assert j['block'] not in cell;cell[j['block']]=statistics.mean(totals)
    assert len(cells)==60 and all(set(c)==set(range(20)) for c in cells.values())
    results=[]
    for case in sorted(set(k[0] for k in cells)):
        for a,b in itertools.combinations(range(5),2):
            ratios=[cells[case,b][i]/cells[case,a][i] for i in range(20)];median=statistics.median(ratios)
            results.append({'case':case,'reference':VARIANTS[a],'candidate':VARIANTS[b],'median_ratio':median,'min_ratio':min(ratios),'max_ratio':max(ratios),'reference_ns':statistics.median(cells[case,a].values()),'candidate_ns':statistics.median(cells[case,b].values()),'verdict':'gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'})
    (OUT/'analysis.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps({'processes':1200,'sessions':12000,'contrasts':120}))

def run():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
    for stable in [False,True]:
        features='borrowed-cycles,incidence-vector'+(',stable-candidates' if stable else '')
        cmd=['cargo','test','-p','chr-relational','--release','--no-default-features','--features',features,'--test','readiness_lifecycle','--no-run','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{stable}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        exe=next(json.loads(x)['executable'] for x in r.stdout.splitlines() if x.startswith('{') and json.loads(x).get('executable'))
        b=ROOT/f'target/stable-candidate-timing-{stable}';b.write_bytes(Path(exe).read_bytes());b.chmod(0o755);builds[str(stable)]={'binary':str(b),'sha256':sha(b),'command':cmd};print('built',stable,flush=True)
    cases=[(64,o,k) for o,k in itertools.product(['false','true'],['broad-forward','broad-reverse'])]+[(128,'false','broad-forward'),(128,'true','broad-reverse'),(64,'false','broad-cancel'),(64,'true','broad-cancel'),(64,'false','fail'),(64,'true','success'),(64,'true','clash'),(4,'false','success')]
    jobs=[];rng=random.Random(7214)
    for block in range(20):
        groups=list(enumerate(cases));rng.shuffle(groups)
        for i,case in groups:
            for v in [(i+block+j)%5 for j in range(5)]:jobs.append({'block':block,'case':case,'variant':v})
    paths=subprocess.check_output(['git','ls-files','research/chr-relational','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-direct-conditional','research/chr-integrated','crates/chr-syntax','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S02-stable-candidate-timing.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(paths)):z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps({'builds':builds,'jobs':jobs,'source_hash':sha(OUT/'sources.zip'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'cpu':CPU,'platform':list(os.uname())},indent=2)+'\n')
    with gzip.open(OUT/'warmups.jsonl.gz','wt') as out:
        for case in cases:
            for stable,mode in VARIANTS:out.write(json.dumps({'case':case,'stable':stable,'mode':mode,'sessions':invoke(builds[str(stable)],mode,case)})+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            stable,mode=VARIANTS[j['variant']];out.write(json.dumps({'job':j,'sessions':invoke(builds[str(stable)],mode,j['case'])})+'\n');out.flush()
            if (i+1)%60==0:print('block',(i+1)//60,'complete',flush=True)
    audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
