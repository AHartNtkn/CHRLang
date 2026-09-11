"""Freeze, run and audit the registered admission allocation attribution."""
import gzip,hashlib,itertools,json,os,random,resource,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
VECTOR='--incidence-vector' in sys.argv
OUT=ROOT/('docs/experiments/results/s02-incidence-vector-entry' if VECTOR else 'docs/experiments/results/s02-admission-attribution')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def audit():
    rows=[json.loads(x) for x in gzip.open(OUT/'runs.jsonl.gz','rt')];assert len(rows)==512
    cells={};profiles={}
    for row in rows:
        key=tuple(row['cell']);kind=row['kind'];data=row['result']
        assert data['validated'] and data['metered']
        phases=[data['preparation']]+[q[p] for q in data['queries'] for p in ['setup','execution_observation','engine_drop']]+[data['preparation_drop'],data['consumer_drop']]
        heaps=[p['heap'] for p in phases]
        assert all(a['live_end']==b['live_start'] for a,b in zip(heaps,heaps[1:]))
        assert heaps[-1]['live_end']==heaps[0]['live_start']
        root=heaps[0]['live_start']
        normalized=[{k:v-root if k in ['live_start','live_end','peak_live'] else v for k,v in h.items()} for h in heaps]
        evidence=(normalized,[q['advances'] for q in data['queries']])
        cells.setdefault((key,kind),[]).append((row['rep'],evidence))
        if kind=='profile':
            profile=row['profile'];profiles.setdefault(key,[]).append(profile)
            if key[0]=='compiled':assert all(p['scopes']==0 for p in profile)
            else:
                for field in ['allocation_calls','requested_bytes','deallocation_calls']:
                    assert sum(p[field] for p in profile)==sum(q['setup']['heap'][field] for q in data['queries']),(key,field)
                assert next(p['scopes'] for p in profile if p['phase']=='admission_other')==key[-1]
    assert len(cells)==256
    analysis=[]
    for (key,kind),reps in cells.items():
        assert sorted(r for r,e in reps)==[0,1] and reps[0][1]==reps[1][1]
        assert reps[0][1]==cells[key,'meter'][0][1]
        if kind=='profile':
            assert profiles[key][0]==profiles[key][1]
            analysis.append({'cell':key,'profile':profiles[key][0],'setup_requested':sum(q['requested_bytes'] for q in reps[0][1][0][1:-2:3])})
    (OUT/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n')
    if VECTOR:
        old=[json.loads(x) for x in gzip.open(ROOT/'docs/experiments/results/s02-admission-attribution/runs.jsonl.gz','rt')]
        old={tuple(x['cell']):x['result'] for x in old if x['kind']=='meter' and x['rep']==0}
        prior_profiles={tuple(x['cell']):x['profile'] for x in json.loads((ROOT/'docs/experiments/results/s02-admission-attribution/analysis.json').read_text())}
        for row in analysis:
            for a,b in zip(prior_profiles[tuple(row['cell'])],row['profile']):
                if a['phase']!='incidence':assert a==b
        comparisons=[]
        def summary(data):
            phases=[data['preparation']]+[q[p] for q in data['queries'] for p in ['setup','execution_observation','engine_drop']]+[data['preparation_drop'],data['consumer_drop']]
            return {'requested':sum(p['heap']['requested_bytes'] for p in phases),'peak':max(p['heap']['peak_live'] for p in phases)-phases[0]['heap']['live_start'],'setup':sum(q['setup']['heap']['requested_bytes'] for q in data['queries']),'execution':sum(q['execution_observation']['heap']['requested_bytes'] for q in data['queries'])}
        for row in rows:
            if row['kind']!='meter' or row['rep']!=0:continue
            key=tuple(row['cell']);prior=old[key]
            assert [q['advances'] for q in row['result']['queries']]==[q['advances'] for q in prior['queries']]
            a,b=summary(prior),summary(row['result'])
            if key[0]=='compiled':assert a==b
            comparisons.append({'cell':key,'set':a,'vector':b})
        (OUT/'comparison.json').write_text(json.dumps(comparisons,indent=2)+'\n')
    print(json.dumps({'processes':len(rows),'cells':len(analysis),'diagnostic_control_pairs':256,'exact_setup_attribution':sum(a['cell'][0]!='compiled' for a in analysis)}))
def run():
    OUT.mkdir(parents=True,exist_ok=True);assert not (OUT/'freeze.json').exists();builds={}
    for kind,feature in [('meter','alloc-meter'),('profile','admission-profile')]:
        if VECTOR: feature += ',incidence-vector'
        cmd=['cargo','test','-p','chr-relational','--release','--no-default-features','--features',feature,'--test','readiness_lifecycle','--no-run','--message-format=json']
        r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True);(OUT/f'build-{kind}.log').write_text(r.stderr.rstrip()+'\n');assert r.returncode==0
        exe=next(json.loads(x)['executable'] for x in r.stdout.splitlines() if x.startswith('{') and json.loads(x).get('executable'))
        b=ROOT/f'target/admission-attribution-{VECTOR}-{kind}';b.write_bytes(Path(exe).read_bytes());b.chmod(0o755);builds[kind]={'binary':str(b),'sha256':sha(b),'command':cmd}
        print('built',kind,flush=True)
    paths=subprocess.check_output(['git','ls-files','research/chr-relational','research/chr-compiled','research/chr-persistent','research/chr-observe','research/chr-direct-conditional','research/chr-integrated','crates/chr-syntax','Cargo.toml','Cargo.lock'],cwd=ROOT,text=True).splitlines()
    paths += [str(Path(__file__).relative_to(ROOT)),'docs/experiments/registrations/S02-admission-attribution.md']
    if VECTOR: paths += ['docs/experiments/registrations/S02-incidence-vector-entry.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(paths)):z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps({'builds':builds,'sources_sha256':sha(OUT/'sources.zip'),'rustc':subprocess.check_output(['rustc','-Vv'],text=True)},indent=2)+'\n')
    cells=list(itertools.product(['full','selective','batch256','compiled'],[4,64],['false','true'],['success','fail','clash','cancel'],[1,16]));rng=random.Random(7210)
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for rep in range(2):
            jobs=list(itertools.product(cells,['meter','profile']));rng.shuffle(jobs)
            for cell,kind in jobs:
                r=subprocess.run([builds[kind]['binary'],*map(str,cell)],cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits)
                if r.returncode:(OUT/'failure.json').write_text(json.dumps({'cell':cell,'kind':kind,'stderr':r.stderr},indent=2)+'\n')
                assert r.returncode==0,(cell,kind,r.stderr)
                lines=[json.loads(s) for s in r.stdout.splitlines()];result=next(s for s in lines if 'validated' in s);profile=next((s['admission_profile'] for s in lines if 'admission_profile' in s),None)
                out.write(json.dumps({'cell':cell,'kind':kind,'rep':rep,'result':result,'profile':profile})+'\n');out.flush()
            print('block',rep+1,'complete',flush=True)
    audit()
if __name__=='__main__':audit() if '--audit' in sys.argv else run()
