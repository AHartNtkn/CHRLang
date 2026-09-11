"""Verify frozen probe qualification, complete cases and matched work changes."""
import hashlib,json,re,zipfile,itertools
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
FIELDS=['n','right','missing','reverse','policy','access','order','target']
WORK=['candidate','pool','structural','index','probe_starts','probe_visits','probe_candidates']
def rows(raw,prefix):return [dict(p.split('=') for p in line.split(',')[1:]) for line in re.findall(prefix+r',[^\n]+',raw['stdout'])]
def main():
    prior=None
    for stage in ['s01-selective-probe','s01-probe-key-reuse']:
        out=ROOT/'docs/experiments/results'/stage
        f=json.loads((out/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
        assert sha((out/'sources.zip').read_bytes())==f['archive_sha256']
        with zipfile.ZipFile(out/'sources.zip') as z:
            for p,h in f['sources'].items():assert sha(z.read(p))==h
        collected={};broad={}
        for feature,metrics in itertools.product([False,True],repeat=2):
            for test in ['partner_order','partner_order_bound','selective_probe']:
                repetitions=[]
                for rep in range(2 if metrics else 1):
                    raw=json.loads((out/f'{feature}-{metrics}-{test}-{rep}.json').read_text())
                    assert raw['exit_code']==0 and '0 failed;' in raw['stdout']
                    if test=='selective_probe':
                        assert 'PROBE_GROUND,cases=2048' in raw['stdout'] and 'PROBE_OPEN,cases=32' in raw['stdout'] and '4 passed;' in raw['stdout']
                        rr=rows(raw,'PROBE_BROAD');assert len(rr)==6
                        assert {(r['n'],r['access']) for r in rr}==set(itertools.product(['8','32','128'],['Scan','Indexed']))
                        if rep==0:broad[feature,metrics]=rr
                    else:
                        assert 'PARTNER_SENTINEL,validated=4' in raw['stdout'] and '2 passed;' in raw['stdout']
                        rr=rows(raw,'PARTNER');assert len(rr)==384 and [int(x['id']) for x in rr]==list(range(384))
                        expected=set()
                        for n in [8,32,128]:
                            for rest in itertools.product(['false','true'],['false','true'],['false','true'],['Global','Active'],['Scan','Indexed'],['false','true'],[0,n-1]):expected.add(tuple(map(str,(n,*rest))))
                        assert {tuple(r[k] for k in FIELDS) for r in rr}==expected
                        if rep==0:collected[feature,metrics,test]=rr
                    assert all(r['metrics']==str(metrics).lower() for r in rr)
                    if not metrics:assert all(int(v)==0 for r in rr for k,v in r.items() if k in WORK)
                    repetitions.append(rr)
                if metrics:assert repetitions[0]==repetitions[1]
        comparisons=[]
        for test in ['partner_order','partner_order_bound']:
            a=collected[False,True,test];b=collected[True,True,test]
            if prior:assert a==prior[False,True,test]
            for left,right in zip(a,b):
                assert all(left[k]==right[k] for k in FIELDS)
                if left['access']=='Scan':assert all(left[k]==right[k] for k in WORK)
                comparisons.append(dict(source=test,case={k:left[k] for k in FIELDS},control={k:int(left[k]) for k in WORK},probe={k:int(right[k]) for k in WORK}))
        if prior:
            assert all(x['probe']['index']<=x['control']['index'] for x in comparisons if x['source']=='partner_order')
            assert all(int(r['probe_starts'])==0 and int(r['probe_visits'])==0 for r in broad[True,True])
            assert all(a['candidate']==b['candidate'] and a['structural']==b['structural'] for a,b in zip(broad[False,True],broad[True,True]))
        report=dict(processes=18,partner_sessions=4608,ground_sessions=12288,open_comparisons=192,open_sessions=384,broad_sessions=36,sentinels=24*2,comparisons=comparisons,broad_control=broad[False,True],broad_probe=broad[True,True],control_reproduced=prior is not None)
        (out/'analysis.json').write_text(json.dumps(report,indent=2)+'\n')
        prior=collected
        print(stage,'18 processes, complete source/trace checks and exact repeated work verified')
        for x in comparisons:
            c=x['case']
            if x['source']=='partner_order_bound' and c['n']=='128' and c['right']=='true' and c['missing']=='false' and c['reverse']=='false' and c['target']=='127' and c['access']=='Indexed' and c['order']=='false':print(c,x['control'],x['probe'])
if __name__=='__main__':main()
