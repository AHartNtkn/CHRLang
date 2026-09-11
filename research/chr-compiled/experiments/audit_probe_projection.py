"""Check complete per-step traces and paired projection-repair evidence."""
import collections,gzip,hashlib,itertools,json,re,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
def read(p):
    if p.exists():return json.loads(p.read_text())
    packed=p.with_suffix(p.suffix+'.gz').read_bytes();data=gzip.decompress(packed)
    manifest=json.loads((p.parent/'packed-receipts.json').read_text())[p.name]
    assert hashlib.sha256(packed).hexdigest()==manifest['gzip_sha256']
    assert hashlib.sha256(data).hexdigest()==manifest['raw_sha256']
    return json.loads(data)
def audit(out,names):
    f=read(out/'freeze.json');sha=lambda b:hashlib.sha256(b).hexdigest()
    assert sha((out/'sources.zip').read_bytes())==f['archive_sha256']
    with zipfile.ZipFile(out/'sources.zip') as z:
        for p,h in f['sources'].items():assert sha(z.read(p))==h
    reports={}
    for name in names:
        raw=read(out/(name+'.json'));assert raw['exit_code']==0 and '0 failed' in raw['stdout']
        rows={};digest=hashlib.sha256()
        for match in re.finditer('PROJECT_STEP,[^\n]+',raw['stdout']):
            line=match.group();digest.update((line+'\n').encode())
            r=dict(x.split('=') for x in line.split(',')[1:]);key=(int(r['n']),r['duplicates']=='true',int(r['query']))
            acc=rows.setdefault(key,dict(steps=0,maximum={k:0 for k in ['probe','bucket','pool','candidate','empty']},total={k:0 for k in ['probe','bucket','pool','candidate','empty']}))
            acc['steps']+=1;assert int(r['step'])==acc['steps']
            for k in acc['maximum']:
                v=int(r.get(k,0));assert v>=0;acc['maximum'][k]=max(acc['maximum'][k],v);acc['total'][k]+=v
        done=[dict(x.split('=') for x in line.split(',')[1:]) for line in re.findall('PROJECT_DONE,[^\n]+',raw['stdout'])]
        assert len(done)==12 and set(rows)==set(itertools.product([16,64,256],[False,True],[0,1]))
        summary=[]
        for d in done:
            key=(int(d['n']),d['duplicates']=='true',int(d['query']));r=rows[key];assert r['steps']==int(d['steps'])
            if d['metrics']=='false':assert all(v==0 for v in r['total'].values())
            summary.append(dict(n=key[0],duplicates=key[1],query=key[2],metrics=d['metrics']=='true',**r))
        reports[name]=dict(summary=summary,rows_sha256=digest.hexdigest())
    return reports

def main():
    base=ROOT/'docs/experiments/results/s01-probe-projection';old=audit(base,['on-0','on-1','off-0']);assert old['on-0']==old['on-1']
    out=ROOT/'docs/experiments/results/s01-probe-projection-repair'
    names=[f'{f}-{m}-probe_projection-{r}' for f,m in itertools.product([False,True],repeat=2) for r in range(2 if m else 1)]
    new=audit(out,names)
    for f in [False,True]:assert new[f'{f}-True-probe_projection-0']==new[f'{f}-True-probe_projection-1']
    for name in names:
        for s in new[name]['summary']:
            if name.startswith('True-True'):assert s['maximum']['bucket']<=2*s['n']
    # Recheck all frozen semantic gates and repeat work; their own assertions
    # independently validate answers/occurrence tuples before process success.
    for feature,metrics in itertools.product([False,True],repeat=2):
        for test,count in [('partner_order',384),('partner_order_bound',384),('selective_probe',None)]:
            previous=None
            for rep in range(2 if metrics else 1):
                raw=read(out/f'{feature}-{metrics}-{test}-{rep}.json');assert raw['exit_code']==0 and '0 failed;' in raw['stdout']
                if count:
                    lines=re.findall('PARTNER,id=[^\n]+',raw['stdout']);assert len(lines)==count and 'PARTNER_SENTINEL,validated=4' in raw['stdout']
                else:
                    assert 'PROBE_GROUND,cases=2048' in raw['stdout'] and 'PROBE_OPEN,cases=32' in raw['stdout'];lines=re.findall('PROBE_BROAD,[^\n]+',raw['stdout']);assert len(lines)==6
                if previous is not None:assert previous==lines
                previous=lines
    result=dict(old={k:v['summary'] for k,v in old.items()},new={k:v['summary'] for k,v in new.items()},confirmation_processes=24,old_processes=3)
    (out/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    for name in ['on-0']:
        for s in old[name]['summary']:
            if s['query']==0:print('old',s['n'],s['duplicates'],s['steps'],s['maximum'])
    for s in new['True-True-probe_projection-0']['summary']:
        if s['query']==0:print('repaired',s['n'],s['duplicates'],s['steps'],s['maximum'])
    print('Verified 27 processes, complete step traces, repeated diagnostics and source gates')
if __name__=='__main__':main()
