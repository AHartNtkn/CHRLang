#!/usr/bin/env python3
"""Reconstruct cells and ownership independently from recorded commands/readings."""
import collections, hashlib, itertools, json, pathlib, random
ROOT=pathlib.Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s04-repeated-reunion-ownership'
def read(path):
    r=json.loads(path.read_text()); assert r['returncode']==0 and not r['stderr'],path
    x=json.loads(r['stdout']); assert r['command'][1:]==[x['mode'],x['family'],str(x['rounds']),str(x['depth']),str(x['reuse'])]
    assert not x['cow']
    phases=x['phases']; names=['prepare']+['input','setup','execute','engine-drop','answers-drop','input-drop']*x['reuse']+['cancel-input','cancel-setup','cancel-first','cancel-engine-drop','cancel-answers-drop','cancel-input-drop','prepared-drop']
    assert [p['phase'] for p in phases]==names
    m=[p['memory'] for p in phases]; base=m[0]['live_start']; prepared=m[0]['live_end']
    assert all(a['live_end']==b['live_start'] for a,b in zip(m,m[1:]))
    assert m[-1]['live_end']==base
    for p in phases:
        q=p['memory']; assert q['peak_live']>=max(q['live_start'],q['live_end'])
        if p['phase'] in ['input-drop','cancel-input-drop']: assert q['live_end']==prepared
    key=tuple(x[k] for k in ['mode','family','rounds','depth','reuse'])
    normal=[p['memory'] for p in phases if not p['phase'].startswith('cancel-')]
    return key,phases,{'traffic':sum(p['requested_bytes'] for p in normal),'peak_growth':max(p['peak_live'] for p in normal)-base,'prepared_live':prepared-base,'cancel_traffic':sum(p['memory']['requested_bytes'] for p in phases if p['phase'].startswith('cancel-'))}
def main():
    cells=collections.defaultdict(list)
    configs=list(itertools.product(['copy','reunion','repeated'],['plain','history','late'],[0,1,3],[0,12],[1,4]))
    order=[(rep,c) for rep in range(2) for c in configs]
    random.Random(7708).shuffle(order)
    assert json.loads((OUT/'order.json').read_text())==json.loads(json.dumps(order))
    for i,(rep,c) in enumerate(order):
        receipt=json.loads((OUT/'runs'/f'{i:03}-r{rep}.json').read_text())
        assert receipt['command'][1:]==list(map(str,c))
    for p in sorted((OUT/'runs').glob('*.json')):
        k,phases,summary=read(p);cells[k].append((phases,summary))
    expected=set(itertools.product(['copy','reunion','repeated'],['plain','history','late'],[0,1,3],[0,12],[1,4]))
    assert set(cells)==expected
    rows=[]
    for key,entries in sorted(cells.items()):
        assert len(entries)==2 and entries[0]==entries[1],key
        rows.append(dict(zip(['mode','family','rounds','depth','reuse'],key))|entries[0][1])
    preflight=[read(p)[0] for p in (OUT/'preflight').glob('*.json')]
    assert set(preflight)=={k for k in expected if k[-1]==1} and len(preflight)==54
    freeze=json.loads((OUT/'freeze.json').read_text())
    assert hashlib.sha256((ROOT/'target/repeated-reunion-ownership/repeated_cost').read_bytes()).hexdigest()==freeze['binary_sha256']
    archived={'research/chr-restoration/src/reunion.rs':OUT/'measured-source/reunion.rs','docs/experiments/registrations/S04-repeated-reunion-ownership.md':OUT/'measured-source/registration.md'}
    for p,h in freeze['source_sha256'].items():assert hashlib.sha256(archived.get(p,ROOT/p).read_bytes()).hexdigest()==h,p
    report={'processes':216,'preflight':54,'exact_pairs':108,'rows':rows}
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n')
    print('216 processes; 108 exact allocation pairs; 54 preflights; all owner baselines pass')
    for r in rows:
        if r['reuse']==4 and r['rounds'] in [0,3]:print(r)
if __name__=='__main__':main()
