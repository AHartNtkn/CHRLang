"""Recheck complete receipts and summarize paired requested-heap effects."""
import gzip, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
BASE=ROOT/'docs/experiments/results/s08-preparation-ownership'
def main():
    rows=[json.loads(s) for s in gzip.open(BASE/'runs.jsonl.gz','rt')]
    assert len(rows)==576
    groups={}
    for row in rows:
        assert row['exit_code']==0
        result=json.loads(row['stdout'].splitlines()[-1]);assert result['meter']
        assert len(result['phases'])==36
        assert len(result['endpoints'])==4
        case=row['case'];cancel=case[3]
        for i,e in enumerate(result['endpoints']):
            if not cancel or i%2: assert e['complete']
        phases=result['phases'];root=phases[0]['reading']['memory']['live_start']
        assert phases[-1]['reading']['memory']['live_end']==root
        memory=[p['reading']['memory'] for p in phases]
        key=json.dumps((case,row['kind']))
        if key in groups: assert memory==groups[key]['memory']
        groups[key]=dict(memory=memory,traffic=sum(m['requested_bytes'] for m in memory),peak=max(m['peak_live'] for m in memory)-root,prepared=next(p['reading']['memory']['live_end'] for p in phases if p['phase']=='source_drop')-root)
    output=[]
    for key,v in groups.items():
        case,kind=json.loads(key)
        if kind!='detached':continue
        old=groups[json.dumps((case,'retained'))]
        output.append(dict(case=case,retained={k:old[k] for k in ['traffic','peak','prepared']},detached={k:v[k] for k in ['traffic','peak','prepared']},delta={k:v[k]-old[k] for k in ['traffic','peak','prepared']}))
    assert len(output)==144
    counts={metric:{sign:sum((r['delta'][metric]<0 if sign=='lower' else r['delta'][metric]>0 if sign=='higher' else r['delta'][metric]==0) for r in output) for sign in ['lower','equal','higher']} for metric in ['traffic','peak','prepared']}
    report=dict(processes=len(rows),exact_pairs=len(groups),comparisons=len(output),counts=counts,results=output)
    (BASE/'audit.json').write_text(json.dumps(report,indent=2));print(json.dumps(counts,indent=2))
if __name__=='__main__':main()
