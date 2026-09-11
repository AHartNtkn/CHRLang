"""Measure exclusive execution responsibilities with exact allocation controls."""
import gzip,hashlib,itertools,json,random,resource,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
STABLE='--stable' in sys.argv
BORROWED='--borrowed' in sys.argv or STABLE
OUT=ROOT/('docs/experiments/results/s02-stable-candidates' if STABLE else 'docs/experiments/results/s02-borrowed-cycles-allocation' if BORROWED else 'docs/experiments/results/s02-execution-attribution')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def audit():
    rows=[json.loads(s) for s in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==144
    freeze=json.loads((OUT/'freeze.json').read_text());assert sha(OUT/'sources.zip')==freeze['source_hash']
    for b in freeze['builds'].values():assert sha(Path(b['binary']))==b['sha256']
    cells={};profiles={}
    for row in rows:
        key=tuple(row['cell']);d=row['result'];assert d['validated'] and d['metered']
        phases=[d['preparation']]+[q[p] for q in d['queries'] for p in ['setup','execution_observation','engine_drop']]+[d['preparation_drop'],d['consumer_drop']]
        heaps=[p['heap'] for p in phases];root=heaps[0]['live_start']
        assert all(a['live_end']==b['live_start'] for a,b in zip(heaps,heaps[1:])) and heaps[-1]['live_end']==root
        normalized=[{k:v-root if k in ['live_start','live_end','peak_live'] else v for k,v in h.items()} for h in heaps]
        evidence=(normalized,[q['advances'] for q in d['queries']]);cells.setdefault((key,row['kind']),[]).append((row['rep'],evidence))
        if row['kind']=='profile':
            profile=row['profile'];profiles.setdefault(key,[]).append([{k:v for k,v in p.items() if k!='diagnostic_ns'} for p in profile])
            for field in ['allocation_calls','requested_bytes','deallocation_calls']:assert sum(p[field] for p in profile)==sum(q['execution_observation']['heap'][field] for q in d['queries']),(key,field)
            assert next(p['scopes'] for p in profile if p['phase']=='execution_other')==2
            if key[1]=='compiled' or key[-1]=='broad-cancel':assert all(p['scopes']==0 for p in profile if p['phase']!='execution_other')
    assert len(cells)==72
    for (key,kind),reps in cells.items():
        assert sorted(r for r,e in reps)==[0,1] and reps[0][1]==reps[1][1]
        assert reps[0][1]==cells[key,'meter'][0][1]
    assert all(p[0]==p[1] for p in profiles.values())
    analysis=[{'cell':r['cell'],'rep':r['rep'],'execution_ns':sum(q['execution_observation']['ns'] for q in r['result']['queries']),'profile':r['profile']} for r in rows if r['kind']=='profile']
    if BORROWED and not STABLE:
        old=[json.loads(s) for s in gzip.open(ROOT/'docs/experiments/results/s02-execution-attribution/runs.jsonl.gz','rt')]
        old={tuple(r['cell']):r for r in old if r['kind']=='profile' and r['rep']==0}
        contrasts=[]
        def totals(d):
            phases=[d['preparation']]+[q[p] for q in d['queries'] for p in ['setup','execution_observation','engine_drop']]+[d['preparation_drop'],d['consumer_drop']]
            return {'requested':sum(p['heap']['requested_bytes'] for p in phases),'peak':max(p['heap']['peak_live'] for p in phases)-phases[0]['heap']['live_start']}
        for row in rows:
            if row['kind']!='profile' or row['rep']!=0:continue
            prior=old[tuple(row['cell'])]
            for a,b in zip(prior['profile'],row['profile']):
                assert a['scopes']==b['scopes']
                if a['phase']!='cycles':assert {k:v for k,v in a.items() if k!='diagnostic_ns'}=={k:v for k,v in b.items() if k!='diagnostic_ns'}
            assert [q['advances'] for q in prior['result']['queries']]==[q['advances'] for q in row['result']['queries']]
            a,b=totals(prior['result']),totals(row['result'])
            if row['cell'][1]=='compiled':assert a==b
            contrasts.append({'cell':row['cell'],'owned':a,'borrowed':b})
        (OUT/'borrowed-comparison.json').write_text(json.dumps(contrasts,indent=2)+'\n')
    if STABLE:
        old=[json.loads(s) for s in gzip.open(ROOT/'docs/experiments/results/s02-borrowed-cycles-allocation/runs.jsonl.gz','rt')]
        old={tuple(r['cell']):r for r in old if r['kind']=='profile' and r['rep']==0}
        contrasts=[]
        def summary(row):
            d=row['result'];phases=[d['preparation']]+[q[p] for q in d['queries'] for p in ['setup','execution_observation','engine_drop']]+[d['preparation_drop'],d['consumer_drop']]
            return {'requested':sum(p['heap']['requested_bytes'] for p in phases),'peak':max(p['heap']['peak_live'] for p in phases)-phases[0]['heap']['live_start'],'discovery':next(p for p in row['profile'] if p['phase']=='discovery')}
        for row in rows:
            if row['kind']!='profile' or row['rep']!=0:continue
            prior=old[tuple(row['cell'])]
            assert [q['advances'] for q in prior['result']['queries']]==[q['advances'] for q in row['result']['queries']]
            a,b=summary(prior),summary(row)
            if row['cell'][1]=='compiled':assert a['requested']==b['requested'] and a['peak']==b['peak']
            contrasts.append({'cell':row['cell'],'conservative':a,'stable':b})
        (OUT/'comparison.json').write_text(json.dumps(contrasts,indent=2)+'\n')
    (OUT/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n');print(json.dumps({'processes':144,'source_configurations':36,'exact_profile_control_pairs':72,'exclusive_sums':72}))
def run():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
    for vector,kind in itertools.product([False,True],['meter','profile']):
        flags=('execution-profile' if kind=='profile' else 'alloc-meter')+(',incidence-vector' if vector else '')+(',borrowed-cycles' if BORROWED else '')+(',stable-candidates' if STABLE else '')
        cmd=['cargo','test','-p','chr-relational','--release','--no-default-features','--features',flags,'--test','readiness_lifecycle','--no-run','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{vector}-{kind}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        exe=next(json.loads(x)['executable'] for x in r.stdout.splitlines() if x.startswith('{') and json.loads(x).get('executable'))
        b=ROOT/f'target/execution-attribution-{STABLE}-{BORROWED}-{vector}-{kind}';b.write_bytes(Path(exe).read_bytes());b.chmod(0o755);builds[str((vector,kind))]={'binary':str(b),'sha256':sha(b),'command':cmd};print('built',vector,kind,flush=True)
    paths=subprocess.check_output(['git','ls-files','research/chr-relational','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-direct-conditional','research/chr-integrated','crates/chr-syntax','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()+[str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S02-execution-attribution.md']
    if BORROWED: paths += ['docs/experiments/registrations/S02-borrowed-cycles.md']
    if STABLE: paths += ['docs/experiments/registrations/S02-stable-candidates.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(paths)):z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps({'builds':builds,'source_hash':sha(OUT/'sources.zip'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True)},indent=2)+'\n')
    cases=[(8,'false','broad-forward'),(128,'true','broad-reverse'),(128,'true','broad-cancel'),(64,'false','fail'),(64,'true','success'),(64,'true','clash')]
    cells=[(v,m,*c) for v,m,c in itertools.product([False,True],['full','batch256','compiled'],cases)];rng=random.Random(7212)
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for rep in range(2):
            jobs=list(itertools.product(cells,['meter','profile']));rng.shuffle(jobs)
            for cell,kind in jobs:
                vector,mode,*case=cell;cmd=[builds[str((vector,kind))]['binary'],mode,*map(str,case),'2']
                r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                if r.returncode:(OUT/'failure.json').write_text(json.dumps({'cmd':cmd,'stderr':r.stderr},indent=2)+'\n')
                assert r.returncode==0,(cmd,r.stderr)
                lines=[json.loads(x) for x in r.stdout.splitlines()];result=next(x for x in lines if 'validated' in x);profile=next((x['execution_profile'] for x in lines if 'execution_profile' in x),None)
                out.write(json.dumps({'cell':cell,'kind':kind,'rep':rep,'result':result,'profile':profile})+'\n');out.flush()
            print('block',rep+1,'complete',flush=True)
    audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
