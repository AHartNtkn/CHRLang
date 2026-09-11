"""Audit paired core-placement matrix and pre-registered Holm sign tests."""
import zipfile
import collections,hashlib,itertools,json,math,random,statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s02-multihead-affinity'
MODES=['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed'];FAMILIES=['sparse','broad','nested','cold','dense','three']
def main():
 freeze=json.loads((OUT/'freeze.json').read_text())
 archive=json.loads((OUT/'source-archive.json').read_text());assert hashlib.sha256((OUT/'sources.zip').read_bytes()).hexdigest()==archive['sha256']
 with zipfile.ZipFile(OUT/'sources.zip') as sources:
  assert set(sources.namelist())==set(freeze['sources'])
  for path,h in freeze['sources'].items():assert hashlib.sha256(sources.read(path)).hexdigest()==h,path
 assert hashlib.sha256((ROOT/'target/s02-multihead-lifecycle/time').read_bytes()).hexdigest()==freeze['binary_sha256']
 blocks=list(itertools.product(range(17),[0,8],FAMILIES));rng=random.Random(7282);rng.shuffle(blocks);jobs=[]
 for rep,cpu,family in blocks:
  modes=MODES.copy();rng.shuffle(modes);jobs.extend((rep,cpu,family,m) for m in modes)
 assert json.loads((OUT/'order.json').read_text())==json.loads(json.dumps(jobs))
 rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()];assert len(rows)==len(jobs)==1428
 totals={};phases=collections.defaultdict(lambda:collections.defaultdict(list))
 for row,job in zip(rows,jobs):
  assert tuple(row[k] for k in ['rep','cpu','family','mode'])==job and row['code']==0 and not row['stderr']
  assert row['command'][1:]==[job[3],job[2],'64','4']
  x=json.loads(row['stdout']);assert (x['mode'],x['family'],x['width'],x['reuse'])==(job[3],job[2],64,4)
  expected=['source-build','prepare']+['input','setup','execute','observe','engine-drop','answer-drop','input-drop']*4+['cancel-input','cancel-setup','cancel-advance','cancel-engine-drop','cancel-input-drop']*2+['prepared-drop','source-drop']
  assert [p['phase'] for p in x['phases']]==expected
  assert all('memory' not in p['measurement'] and p['measurement']['ns']>=0 for p in x['phases'])
  normal=[p for p in x['phases'] if not p['phase'].startswith('cancel-')]
  totals[job]=sum(p['measurement']['ns'] for p in normal)
  sums=collections.Counter()
  for p in normal:sums[p['phase']]+=p['measurement']['ns']
  for name,ns in sums.items():phases[job[1:]][name].append(ns)
 comparisons=[];tests=[]
 for cpu,family in itertools.product([0,8],FAMILIES):
  for control,candidates in [('scan',[m for m in MODES if m!='scan']),('local-scan',['tuples','partial'])]:
   for mode in candidates:
    ratios=[totals[(rep,cpu,family,mode)]/totals[(rep,cpu,family,control)] for rep in range(17)]
    result=dict(cpu=cpu,family=family,mode=mode,control=control,ratios=ratios,median_ratio=statistics.median(ratios),classification='unresolved')
    comparisons.append(result)
    for direction,threshold in [('gain',.9),('loss',1.1)]:
     k=sum(r<threshold if direction=='gain' else r>threshold for r in ratios)
     p=sum(math.comb(17,j) for j in range(k,18))/2**17
     tests.append(dict(index=len(comparisons)-1,direction=direction,p=p,successes=k))
 assert len(comparisons)==96 and len(tests)==192
 tests.sort(key=lambda t:t['p']);maximum=0
 for rank,test in enumerate(tests):
  maximum=max(maximum,(len(tests)-rank)*test['p']);test['holm_p']=min(1,maximum)
  if test['holm_p']<=.05:
   result=comparisons[test['index']];assert result['classification']=='unresolved';result['classification']=test['direction']
 summary=[]
 for cpu,family,mode in itertools.product([0,8],FAMILIES,MODES):
  values=[totals[(rep,cpu,family,mode)] for rep in range(17)]
  summary.append(dict(cpu=cpu,family=family,mode=mode,min_ns=min(values),median_ns=statistics.median(values),max_ns=max(values),phase_medians_ns={k:statistics.median(v) for k,v in phases[(cpu,family,mode)].items()}))
 audit=dict(processes=1428,comparisons=comparisons,directional_tests=tests,summary=summary)
 (OUT/'audit.json').write_text(json.dumps(audit,indent=2)+'\n')
 for cpu in [0,8]:
  print('CPU',cpu)
  for control,candidates in [('scan',[m for m in MODES if m!='scan']),('local-scan',['tuples','partial'])]:
   for mode in candidates:print(mode,'vs',control,dict(collections.Counter(r['classification'] for r in comparisons if r['cpu']==cpu and r['mode']==mode and r['control']==control)))
if __name__=='__main__':main()
