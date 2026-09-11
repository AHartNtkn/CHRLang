"""Audit both frozen source gates and isolate matched order work."""
import hashlib,json,re,zipfile,itertools
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
FIELDS=['n','right','missing','reverse','policy','access','order','target']
def main():
    for family in ['entry','bound']:
        out=ROOT/f'docs/experiments/results/s01-partner-order-{family}'
        f=json.loads((out/'freeze.json').read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
        assert sha((out/'sources.zip').read_bytes())==f['archive_sha256']
        with zipfile.ZipFile(out/'sources.zip') as z:
            for p,h in f['sources'].items():assert sha(z.read(p))==h
        allrows=[]
        for name in ['on-0','on-1','off-0']:
            r=json.loads((out/f'{name}.json').read_text());assert r['exit_code']==0 and '2 passed; 0 failed' in r['stdout'] and 'PARTNER_SENTINEL,validated=4' in r['stdout']
            rows=[dict(p.split('=') for p in line.split(',')[1:]) for line in re.findall('PARTNER,id=[^\n]+',r['stdout'])]
            assert len(rows)==384 and [int(x['id']) for x in rows]==list(range(384))
            expected=set()
            for n in [8,32,128]:
                for right,missing,reverse,policy,access,order,target in itertools.product(['false','true'],['false','true'],['false','true'],['Global','Active'],['Scan','Indexed'],['false','true'],[0,n-1]):expected.add(tuple(map(str,[n,right,missing,reverse,policy,access,order,target])))
            assert {tuple(x[k] for k in FIELDS) for x in rows}==expected
            assert all(x['metrics']==('false' if name.startswith('off') else 'true') for x in rows)
            if name.startswith('off'):assert all(int(x[k])==0 for x in rows for k in ['candidate','pool','structural','index'])
            allrows.append(rows)
        assert allrows[0]==allrows[1]
        assert [{k:v for k,v in x.items() if k in FIELDS} for x in allrows[0]]==[{k:v for k,v in x.items() if k in FIELDS} for x in allrows[2]]
        table={tuple(x[k] for k in FIELDS):x for x in allrows[0]}
        pairs=[]
        for key,row in table.items():
            if row['order']!='false':continue
            other=list(key);other[-2]='true';b=table[tuple(other)]
            pairs.append(dict(case={k:row[k] for k in FIELDS if k!='order'},left={k:int(row[k]) for k in ['candidate','pool','structural','index']},right={k:int(b[k]) for k in ['candidate','pool','structural','index']}))
        (out/'analysis.json').write_text(json.dumps(dict(sessions=1152,sentinels=12,exact_diagnostic_rows=384,pairs=pairs),indent=2)+'\n')
        print(family,'verified 1152 complete sessions, 12 sentinels and exact repeated diagnostics')
        for p in pairs:
            c=p['case']
            if c['n']=='128' and c['missing']=='false' and c['reverse']=='false' and c['target']=='127' and c['access']=='Indexed':print(c,p['left'],p['right'])
if __name__=='__main__':main()
