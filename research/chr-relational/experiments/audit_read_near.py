"""Audit bounded near-miss source confirmations and exact diagnostic witnesses."""
import hashlib,itertools,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];B=ROOT/'docs/experiments/results/s02-read-near-miss'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=read(B/'source-freeze.json')
for p,h in f['source_hashes'].items():assert sha(ROOT/p)==h,p
receipts=read(B/'gate.json')['receipts'];assert len(receipts)==3
for r in receipts:
 assert r['exit_code']==0 and sha(Path(r['binary']))==r['sha256']
 assert r['cpu_seconds_limit']==r['wall_seconds_limit']==120 and r['address_space_limit']==1<<30
 b=read(B/f"binary-{r['kind']}.json");assert b['binary']==r['binary'] and b['sha256']==r['sha256']
def lines(i):return [l[l.index('NEAR_'):] for l in (B/f'bounded-diagnostic-{i}.log').read_text().splitlines() if 'NEAR_' in l]
a=lines(0);assert a==lines(1)
source={};operations=[]
for l in a:
 p=l.split(',')
 if p[0]=='NEAR_OPERATION':
  n,i,checks,reads=map(int,p[1:]);assert checks==reads==i;operations.append((n,i))
 else:
  n,depth=int(p[1]),int(p[2]);kind,token,reverse,mode=p[3:7];checks,reads,hits=map(int,p[7:]);k=(n,depth,kind,token,reverse,mode);assert k not in source;source[k]=(checks,reads,hits)
  if mode.endswith('validated'):
   expected=(depth+1)*(n-1) if kind=='repeated' else (depth+1)*n*(n-1)//2
   assert checks==expected,(k,checks,expected)
   assert hits==((depth+1)*(n-1) if kind=='repeated' else 0)
   assert reads>=checks
  else:assert checks==reads==0
assert operations==[(n,i) for n in [1,8,32] for i in range(n)]
expected=set(itertools.product([1,8,32],[1,8],['unique','repeated','mixed'],['false','true'],['false','true'],['plain','exact','relevant','persistent-relevant','validated','persistent-validated']))
assert set(source)==expected
for k,v in source.items():
 if k[-1]=='validated':assert v==source[(*k[:-1],'persistent-validated')]
assert 'test result: ok. 1 passed' in (B/'bounded-primary-0.log').read_text()
assert 'PASS: candidate-count omission' in (B/'mutation-audit.log').read_text()
print(json.dumps({'source_configurations':72,'compared_modes_per_configuration':8,'bounded_confirmation_processes':3,'diagnostic_source_rows_per_repeat':432,'diagnostic_repeats_equal':True,'exact_operation_cases':41,'counter_omission_mutation_rejected':True,'selected_cases':[{'alternatives':n,'depth':8,'kind':kind,'candidates':source[n,8,kind,'true','false','validated'][0],'reads':source[n,8,kind,'true','false','validated'][1],'hits':source[n,8,kind,'true','false','validated'][2]} for n in [1,8,32] for kind in ['unique','repeated','mixed']]},indent=2))
