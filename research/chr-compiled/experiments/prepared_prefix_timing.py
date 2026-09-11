"""Frozen, paired ordinary-allocator lifecycle comparison and raw-data audit."""
import gzip, hashlib, itertools, json, os, random, resource, statistics, subprocess, sys, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s01-prepared-prefix-timing'
VARIANTS=[(False,'fresh'),(False,'reuse'),(True,'fresh'),(True,'reuse')]

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0})
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def invoke(binary,case,mode):
    family,n,policy,keep,cancel=case
    cmd=[binary,family,str(n),policy,'indexed',keep,cancel,'50',mode]
    r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
    if r.returncode:
        (OUT/'failure.json').write_text(json.dumps({'cmd':cmd,'stderr':r.stderr},indent=2)+'\n')
    assert r.returncode==0,(cmd,r.stderr)
    sessions=[json.loads(s) for s in r.stdout.splitlines()]
    assert len(sessions)==50 and all(s['validated'] for s in sessions)
    return sessions

def audit():
    rows=[json.loads(s) for s in gzip.open(OUT/'runs.jsonl.gz','rt')]
    assert len(rows)==1920
    freeze=json.loads((OUT/'freeze.json').read_text())
    assert [r['job'] for r in rows]==freeze['jobs']
    assert sha(OUT/'sources.zip')==freeze['sources_sha256']
    for build in freeze['builds'].values(): assert sha(Path(build['binary']))==build['sha256']
    warmups=[json.loads(s) for s in gzip.open(OUT/'warmups.jsonl.gz','rt')]
    assert len(warmups)==96 and all(len(w['sessions'])==50 and all(s['validated'] for s in w['sessions']) for w in warmups)
    cells={}
    distributions={}
    outliers=[]
    for row in rows:
        j=row['job'];sessions=row['sessions'];assert len(sessions)==50
        totals=[]
        for s in sessions:
            assert s['validated'] and s['feature']==j['probe']
            expected=[('prepare',0),('prefix_prepare',0)]
            for q in range(2):
                expected += [('setup',q),('execute',q)]
                if q!=0 or j['case'][4]!='true':expected += [('observe',q)]
                expected += [('engine_drop',q),('answer_drop',q)]
            expected += [('prefix_drop',0),('prepared_drop',0),('retained_drop',0)]
            assert [(r['phase'],r['q']) for r in s['records']]==expected
            assert all(r['heap'] is None and r['cpu_ns'] is None for r in s['records'])
            totals.append(sum(r['ns'] for r in s['records']))
        key=(tuple(j['case']),j['probe'],j['variant'])
        distributions[key+(j["block"],)]=totals
        outliers.append(max(totals)/statistics.median(totals))
        cell=cells.setdefault(key,{})
        assert j['block'] not in cell
        cell[j['block']]=statistics.mean(totals)
    assert len(cells)==96 and all(set(c)==set(range(20)) for c in cells.values())
    comparisons=[]
    groups=sorted(set((c,p) for c,p,v in cells))
    for case,probe in groups:
        for a,b in itertools.combinations(range(4),2):
            ratios=[cells[case,probe,b][i]/cells[case,probe,a][i] for i in range(20)]
            median=statistics.median(ratios)
            verdict='gain' if median<=.9 and max(ratios)<1 else 'loss' if median>=1.1 and min(ratios)>1 else 'unresolved'
            comparisons.append({'case':case,'probe':probe,'reference':VARIANTS[a],'candidate':VARIANTS[b], 'median_ratio':median,'min_ratio':min(ratios),'max_ratio':max(ratios),'verdict':verdict,'reference_median_ns':statistics.median(cells[case,probe,a].values()),'candidate_median_ns':statistics.median(cells[case,probe,b].values())})
    (OUT/'analysis.json').write_text(json.dumps(comparisons,indent=2)+'\n')
    diagnostic=[]
    for case,probe in groups:
        means=[];medians=[]
        for block in range(20):
            a=distributions[case,probe,1,block];b=distributions[case,probe,3,block]
            means.append(statistics.mean(b)/statistics.mean(a));medians.append(statistics.median(b)/statistics.median(a))
        diagnostic.append({'case':case,'probe':probe,'mean_pairs_above_one':sum(x>1 for x in means),'median_pairs_above_one':sum(x>1 for x in medians),'primary_median_ratio':statistics.median(means),'diagnostic_median_ratio':statistics.median(medians)})
    detail={'status':'post-hoc diagnostic; does not replace primary mean-of-sessions endpoint','process_max_over_session_median':{'median':statistics.median(outliers),'max':max(outliers),'above_two':sum(x>2 for x in outliers),'processes':len(outliers)},'arena_reuse_contrasts':diagnostic}
    (OUT/'batch-diagnostic.json').write_text(json.dumps(detail,indent=2)+'\n')
    print(json.dumps({'processes':len(rows),'sessions':len(rows)*50,'comparisons':len(comparisons)}))

def run():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists()
    builds={}
    for cow,probe in itertools.product([False,True],repeat=2):
        features=(['arena-cow'] if cow else [])+(['selective-probe'] if probe else [])
        cmd=['cargo','build','-p','chr-compiled','--release','--no-default-features','--example','probe_lifecycle']
        if features:cmd+=['--features',','.join(features)]
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
        (OUT/f'build-{cow}-{probe}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        b=ROOT/f'target/prepared-timing-{cow}-{probe}';b.write_bytes((ROOT/'target/release/examples/probe_lifecycle').read_bytes());b.chmod(0o755)
        builds[str((cow,probe))]={'binary':str(b),'sha256':sha(b),'command':cmd}
        print('built',cow,probe,flush=True)
    cases=[(f,128,p,'false','false') for f,p in itertools.product(['selective','neutral','duplicate','broad'],['global','active'])]
    cases += [('selective',16,'global','false','false'),('selective',128,'global','false','true'),('selective',128,'active','false','true'),('selective',128,'global','true','false')]
    groups=list(itertools.product(cases,[False,True]));rng=random.Random(8212);jobs=[]
    for block in range(20):
        rng.shuffle(groups)
        for g,(case,probe) in enumerate(groups):
            start=(block+list(itertools.product(cases,[False,True])).index((case,probe)))%4
            for v in [(start+i)%4 for i in range(4)]:jobs.append({'block':block,'case':case,'probe':probe,'variant':v})
    paths=subprocess.check_output(['git','ls-files','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths += [str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S01-prepared-prefix-timing.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(paths)):z.write(ROOT/p,p)
    freeze={'builds':builds,'jobs':jobs,'sources_sha256':sha(OUT/'sources.zip'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'affinity':sorted(os.sched_getaffinity(0)),'platform':os.uname()._asdict() if hasattr(os.uname(),'_asdict') else list(os.uname())}
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    with gzip.open(OUT/'warmups.jsonl.gz','wt') as out:
        for case,probe in itertools.product(cases,[False,True]):
            for v,(cow,mode) in enumerate(VARIANTS):out.write(json.dumps({'case':case,'probe':probe,'variant':v,'sessions':invoke(builds[str((cow,probe))]['binary'],case,mode)})+'\n')
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for i,j in enumerate(jobs):
            cow,mode=VARIANTS[j['variant']]
            out.write(json.dumps({'job':j,'sessions':invoke(builds[str((cow,j['probe']))]['binary'],j['case'],mode)})+'\n');out.flush()
            if (i+1)%96==0:print('block',(i+1)//96,'complete',flush=True)
    audit()
if __name__=='__main__': audit() if '--audit' in sys.argv else run()
