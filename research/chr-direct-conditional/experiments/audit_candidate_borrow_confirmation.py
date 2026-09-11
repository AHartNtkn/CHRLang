"""Reconstruct selected median intervals without trimming observations."""
import collections,gzip,hashlib,json,math,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];PARENT=ROOT/'docs/experiments/results/s03-candidate-borrow';BASE=PARENT/'confirmation'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    f=json.loads((BASE/'freeze.json').read_text());assert sha(PARENT/'freeze.json')==f['parent_freeze_sha256'];assert sha(PARENT/'audit.json')==f['parent_audit_sha256']
    for p,s in f['sources'].items():assert sha(ROOT/p)==s['sha256']==hashlib.sha256(s['text'].encode()).hexdigest()
    pf=json.loads((PARENT/'freeze.json').read_text());assert sha(PARENT/'sources.zip')==pf['archive_sha256']
    for p,h in pf['sources'].items():assert sha(ROOT/p)==h
    for b in f['binaries'].values():assert sha(Path(b['path']))==b['sha256']
    # Recover exact endpoint metadata from the pilot itself.
    jobs=json.loads((PARENT/'jobs.json').read_text());expected={}
    with gzip.open(PARENT/'runs.jsonl.gz','rt') as stream:
        for line in stream:
            raw=json.loads(line);j=jobs[raw['index']]
            if j['kind'] in ['old-time','new-time'] and not j['cancel']:
                r=next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
                expected[tuple([*j['scenario'],j['mode']])]=r['endpoints']
    phases=[('source',0),('prepare',0)]+[(p,i) for i in range(4) for p in ['input','setup','execute_observe','engine_drop','consumer']]+[('prepared_drop',0),('consumer_drop',0)]
    values={};seen=0
    with gzip.open(BASE/'runs.jsonl.gz','rt') as stream:
        for i,line in enumerate(stream):
            raw=json.loads(line);j=f['jobs'][i];assert raw['job']==i;seen+=1
            fam,n,rev,ret,mode=f['cases'][j['index']]
            assert raw['command']==[f['binaries'][j['kind']]['path'],mode,fam,str(n),str(rev).lower(),ret,'false']
            assert raw['exit_code']==0 and not raw['timeout'] and not raw['stderr']
            r=next(json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{') and json.loads(l).get('event')=='result')
            assert not r['meter'] and r['endpoints']==expected[tuple(f['cases'][j['index']])]
            assert [(p['phase'],p['query']) for p in r['phases']]==phases
            assert all(p['reading']['memory'] is None for p in r['phases'])
            if j['rep']>=0:values[j['index'],j['rep'],j['kind']]=sum(p['reading']['ns'] for p in r['phases'])
    assert seen==len(f['jobs'])==1950 and len(values)==1920
    bound=30*sum(math.comb(64,i) for i in range(20))/2**64;assert bound==f['family_error_bound'] and bound<.05
    rows=[]
    for index,case in enumerate(f['cases']):
        ratios=[values[index,rep,'new-time']/values[index,rep,'old-time'] for rep in range(64)];ordered=sorted(ratios);lo,hi=ordered[19],ordered[44]
        sensitive=any(values[index,rep,kind]<f['clock_floor_ns'] for rep in range(64) for kind in ['old-time','new-time'])
        verdict='signal-limited' if sensitive else 'gain' if hi<.9 else 'loss' if lo>1.1 else 'within-10%' if lo>=.9 and hi<=1.1 else 'unresolved magnitude'
        rows.append(dict(case=case,median_ratio=statistics.median(ratios),interval=[lo,hi],verdict=verdict,first_half=statistics.median(ratios[:32]),last_half=statistics.median(ratios[32:]),ratios=ratios))
    out=dict(primary=1920,warmups=30,family_error_bound=bound,counts=dict(collections.Counter(r['verdict'] for r in rows)),rows=rows)
    (BASE/'audit.json').write_text(json.dumps(out,indent=2));print(json.dumps({k:v for k,v in out.items() if k!='rows'}));
    for r in rows:print(r['case'],r['interval'],r['verdict'])
if __name__=='__main__':main()
