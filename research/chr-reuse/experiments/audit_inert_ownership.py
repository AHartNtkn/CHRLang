"""Audit frozen receipts, exact phase endpoints and the post-lint replay."""
from pathlib import Path
import collections,gzip,hashlib,json
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s05-inert-ownership'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def normalized(data):
 data=json.loads(json.dumps(data));base=data[1]['memory']['live_start']
 for row in data[1:]:
  for field in ['live_start','live_end','peak_live']:row['memory'][field]-=base
 return data
if __name__=='__main__':
 matrices=[]
 for folder in [RAW,RAW/'final',RAW/'compact']:
  m=json.loads((folder/'manifest.json').read_text())
  for p,h in m['sha256'].items():
   path=ROOT/p
   if p=='research/chr-reuse/examples/inert_ownership.rs':
    if folder==RAW:path=RAW/'before/inert_ownership.rs'
    elif folder==RAW/'final':path=RAW/'before/inert_ownership-linted.rs'
   assert digest(path)==h,p
  expected=144 if folder==RAW/'compact' else 576
  assert len(m['cells'])==expected and len(set(map(tuple,m['cells'])))==expected
  matrix=[]
  for i,cell in enumerate(m['cells']):
   pair=[]
   for rep in range(2):
    r=json.loads(gzip.decompress((folder/f'run-{i:03}-{rep}.json.gz').read_bytes()));assert r['returncode']==0 and not r['stderr']
    assert r['command']==[str(ROOT/m['binary']),*map(str,cell)]
    data=[json.loads(l) for l in r['stdout'].splitlines()];head,*rows=data;mode,family,depth,reuse,keep,cancel=cell
    count=1 if cancel or family==5 else 2
    assert head['counts']==[count]*reuse
    assert head['retained']==(0 if keep=='0' else 1 if keep=='1' else count*reuse)
    phases=[('source',0),('prepare',0),('source_dispose',0)]
    for q in range(reuse):
     phases += [('input',q),('setup',q)]+[x for _ in range(count) for x in [('service_observe',q),('consume',q)]]
     if not cancel:phases.append(('service_observe',q))
     phases += [('engine_dispose',q),('input_dispose',q)]
    phases += [('prepared_dispose',reuse),('consumer_dispose',reuse)]
    assert [(r['phase'],r['query']) for r in rows]==phases
    base=rows[0]['memory']['live_start'];assert rows[-1]['memory']['live_end']==base
    assert head['consumer_bytes']==rows[-2]['memory']['live_end']-base
    assert head['requested_bytes']==sum(r['memory']['requested_bytes'] for r in rows)
    assert head['unreleased_bytes']==0
    if keep=='0':assert all(r['memory']['live_end']==rows[2]['memory']['live_end'] for r in rows if r['phase']=='input_dispose')
    pair.append(data)
   assert pair[0]==pair[1],cell
   matrix.append(normalized(pair[0]))
  matrices.append(matrix)
 assert matrices[0]==matrices[1],'post-lint repair changed relative heap observations'
 print('Verified 2592 processes, exact phase coverage, pair equality, final ownership, source provenance and identical relative heap after lint repair.')
