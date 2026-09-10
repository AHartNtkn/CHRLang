"""Read-only gate audit, including completed answers and cancellation prefixes."""
import collections,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s03-native-identity-kernel'
def rows(name):return [json.loads(l) for l in (OUT/name).read_text().splitlines()]
def main():
 v=json.loads((OUT/'validation.json').read_text())
 for p,h in v['hashes'].items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h,p
 prior=json.loads((ROOT/'docs/experiments/results/s03-native-consuming/validation.json').read_text())
 for p in ['target/s03-native-consuming/native','target/s03-native-service/baseline']:assert v['hashes'][p]==prior['hashes'][p]
 cases=json.loads((OUT/'cases.json').read_text());expected={name:answers for name,expr,answers in cases};assert len(expected)==len(cases)==340
 runs=rows('runs.jsonl');assert len(runs)==1360
 seen=set();full={};maximum=0
 for row in runs:
  key=(row['case'],row['rep'],row['fuel']);assert key not in seen;seen.add(key)
  r=row['result'];assert r['code']==0 and sorted(r['stdout'].splitlines())==sorted(expected[row['case']])
  ev=[json.loads(l) for l in r['stderr'].splitlines()];assert ev[-1]['pending']==0 and ev[-1]['unsupported']==0
  assert ev[-1]['calls']==len(ev)-1
  assert all(e['visits']<=row['fuel'] and e['stack']==1 for e in ev[:-1]);maximum=max(maximum,len(ev)-1)
  if row['rep']==0:full[(row['case'],row['fuel'])]=r
  else:assert full[(row['case'],row['fuel'])]==r
 assert seen=={(c,r,f) for c in expected for r in [0,1] for f in [1,8]}
 diagnostics=rows('diagnostics.jsonl');assert len(diagnostics)==1700;seen=set()
 for row in diagnostics:
  fuel=row.get('fuel',1);kind=row['kind'];calls=row.get('calls');key=(row['case'],kind,fuel,calls);assert key not in seen;seen.add(key)
  original=full[(row['case'],fuel)];oe=[json.loads(l) for l in original['stderr'].splitlines()]
  r=row['result'];assert r['code']==0;ev=[json.loads(l) for l in r['stderr'].splitlines()]
  if kind=='off':
   assert r['stdout']==original['stdout']
   assert [{k:v for k,v in e.items() if k!='delta'} for e in ev]==[{k:v for k,v in e.items() if k!='delta'} for e in oe]
   assert all(e['delta']==0 for e in ev[:-1])
  else:
   assert kind=='cancel';n=min(calls,len(oe)-1);assert ev[:-1]==oe[:n] and ev[-1]['calls']==n
   assert ev[-1]['pending']==(oe[n-1]['pending'] if n else 1)
   assert r['stdout'].splitlines()==original['stdout'].splitlines()[:len(r['stdout'].splitlines())]
 assert seen=={(c,'off',f,None) for c in expected for f in [1,8]}|{(c,'cancel',1,n) for c in expected for n in [0,1,16]}
 controls=json.loads((OUT/'controls.json').read_text());assert len(controls)==20
 for row in controls:assert row['result']['code']==0 and sorted(row['result']['stdout'].splitlines())==sorted(expected[row['case']])
 print(json.dumps(dict(cases=len(cases),native_runs=len(runs),diagnostics=len(diagnostics),controls=len(controls),maximum_calls=maximum,hashes='pass',complete_answers='pass',replay='pass',cancellation='pass')))
if __name__=='__main__':main()
