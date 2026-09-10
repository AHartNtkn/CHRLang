"""Independent reconstruction of the registered scale matrix and sign tests."""
import collections,hashlib,itertools,json,math,random,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-multihead-scale'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed']
FAMILIES=['sparse','broad','nested','cold','dense','three'];PAIRS=[(4,1),(4,4),(16,1),(16,4),(64,1)]
def main():
 for path,h in json.loads((OUT/'freeze.json').read_text()).items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 blocks=list(itertools.product(range(21),[0,8],FAMILIES,PAIRS));rng=random.Random(7283);rng.shuffle(blocks);jobs=[]
 for rep,cpu,family,(width,reuse) in blocks:
  modes=MODES.copy();rng.shuffle(modes);jobs.extend((rep,cpu,family,width,reuse,m) for m in modes)
 assert json.loads((OUT/'order.json').read_text())==json.loads(json.dumps(jobs))
 rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()];assert len(rows)==len(jobs)==8820
 assert json.loads((OUT/'complete.json').read_text())['processes']==8820
 totals={};phases=collections.defaultdict(lambda:collections.defaultdict(list))
 for row,job in zip(rows,jobs):
  rep,cpu,family,width,reuse,mode=job
  assert tuple(row[k] for k in ['rep','cpu','family','width','reuse','mode'])==job
  assert row['code']==0 and not row['stderr']
  assert row['command']==[str(ROOT/'target/s02-multihead-lifecycle/time'),mode,family,str(width),str(reuse)]
  x=json.loads(row['stdout']);assert (x['mode'],x['family'],x['width'],x['reuse'])==(mode,family,width,reuse)
  expected=['source-build','prepare']+['input','setup','execute','observe','engine-drop','answer-drop','input-drop']*reuse+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop','source-drop']
  assert [p['phase'] for p in x['phases']]==expected
  assert all('memory' not in p['measurement'] and isinstance(p['measurement']['ns'],int) and p['measurement']['ns']>=0 for p in x['phases'])
  normal=[p for p in x['phases'] if not p['phase'].startswith('cancel-')]
  totals[job]=sum(p['measurement']['ns'] for p in normal);assert totals[job]>0
  sums=collections.Counter()
  for p in normal:sums[p['phase']]+=p['measurement']['ns']
  for name,ns in sums.items():phases[job[1:]][name].append(ns)
 comparisons=[];tests=[];summary=[]
 for cpu,family,(width,reuse) in itertools.product([0,8],FAMILIES,PAIRS):
  for control,candidates in [('scan',[m for m in MODES if m!='scan']),('local-scan',['tuples','partial'])]:
   for mode in candidates:
    ratios=[totals[(rep,cpu,family,width,reuse,mode)]/totals[(rep,cpu,family,width,reuse,control)] for rep in range(21)]
    comparisons.append(dict(cpu=cpu,family=family,width=width,reuse=reuse,mode=mode,control=control,ratios=ratios,median_ratio=statistics.median(ratios),classification='unresolved'))
    for direction,threshold in [('gain',.9),('loss',1.1)]:
     k=sum(r<threshold if direction=='gain' else r>threshold for r in ratios)
     tests.append(dict(index=len(comparisons)-1,direction=direction,p=sum(math.comb(21,j) for j in range(k,22))/2**21,successes=k))
  for mode in MODES:
   values=[totals[(rep,cpu,family,width,reuse,mode)] for rep in range(21)]
   summary.append(dict(cpu=cpu,family=family,width=width,reuse=reuse,mode=mode,min_ns=min(values),median_ns=statistics.median(values),max_ns=max(values),phase_medians_ns={k:statistics.median(v) for k,v in phases[(cpu,family,width,reuse,mode)].items()}))
 assert len(comparisons)==480 and len(tests)==960
 tests.sort(key=lambda t:t['p']);maximum=0
 for rank,test in enumerate(tests):
  maximum=max(maximum,(len(tests)-rank)*test['p']);test['holm_p']=min(1,maximum)
  if test['holm_p']<=.05:
   result=comparisons[test['index']];assert result['classification']=='unresolved';result['classification']=test['direction']
 (OUT/'audit.json').write_text(json.dumps(dict(processes=8820,comparisons=comparisons,directional_tests=tests,summary=summary),indent=2)+'\n')
 for control in ['scan','local-scan']:
  for mode in MODES:
   subset=[r for r in comparisons if r['control']==control and r['mode']==mode]
   if subset:print(mode,'vs',control,dict(collections.Counter(r['classification'] for r in subset)))
 print('Coverage, counter-free phases, frozen inputs and registered analysis pass.')
if __name__=='__main__':main()
