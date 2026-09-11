"""Reconstruct all frozen complete lifecycle totals and paired median intervals."""
import collections,gzip,hashlib,itertools,json,math,random,statistics,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-unique-confirmation';PARENT=ROOT/'docs/experiments/results/s06-unique-output'
MODES=['union','unique','dedup','reduced'];FAMILIES=['overlap','disjoint','redundant','single']
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 f=read(BASE/'freeze.json');p=read(PARENT/'freeze.json')
 for n,h in f['parent'].items():assert sha(PARENT/n)==h
 assert f['binary']==p['binaries']['time'] and sha(Path(f['binary']['path']))==f['binary']['sha256']
 assert sha(PARENT/'sources.zip')==p['archive_sha256']
 with zipfile.ZipFile(PARENT/'sources.zip') as z:
  assert set(z.namelist())==set(p['sources'])
  for n,h in p['sources'].items():assert hashlib.sha256(z.read(n)).hexdigest()==h
 for n,h in f['sources'].items():assert hashlib.sha256(f['source_text'][n].encode()).hexdigest()==h
 assert sha(BASE/'qualification.json')==f['qualification_sha256'] and read(BASE/'qualification.json')['qualified']
 assert sha(BASE/'symbols.txt')==f['symbols_sha256'] and 'chr_observe' not in (BASE/'symbols.txt').read_text()
 build=read(PARENT/'build-time.json');assert build['exit_code']==0
 for line in build['stdout'].splitlines():
  x=json.loads(line)
  if x.get('reason')=='compiler-artifact' and x['target']['name'] in ['chr_structural','union_lifecycle']:assert not set(x.get('features',[]))&{'metrics','alloc-meter','phase-clock','session-clock'}
 assert f['cpu']==0 and 0 in f['affinity']
 for n,h in f['orders'].items():assert sha(BASE/f'{n}.json')==h
 scenarios=list(itertools.product(FAMILIES,[1,64],['member','full'],['set','ordered']));cells=[[m,fam,8,q,e,'all',c] for fam,q,e,c in scenarios for m in MODES];assert read(BASE/'cells.json')==cells
 blocks=list(itertools.product(range(64),range(32)));rng=random.Random(76050);rng.shuffle(blocks);jobs=[]
 for block,(rep,index) in enumerate(blocks):
  modes=MODES.copy();rng.shuffle(modes)
  for pos,m in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=index*4+MODES.index(m),position=pos))
 assert jobs==read(BASE/'jobs.json');warm=list(range(128));random.Random(76051).shuffle(warm);assert warm==read(BASE/'warmup-order.json')
 campaign=read(BASE/'campaign.json');assert campaign['primary_processes']==8192 and campaign['warmups']==128 and campaign['seconds']<1800
 for n,h in campaign['archives'].items():assert sha(BASE/f'{n}.jsonl.gz')==h
 clocks=[]
 for i in range(5):
  raw=read(BASE/f'clock-{i}.json');assert raw['command']==[f['binary']['path'],'clock-check'] and raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'];clocks.append(json.loads(raw['stdout'])['median_ns'])
 totals=collections.defaultdict(list);phases=collections.defaultdict(lambda:collections.defaultdict(list));paired={}
 for kind,order in [('warmup',warm),('runs',jobs)]:
  count=0
  with gzip.open(BASE/f'{kind}.jsonl.gz','rt') as stream:
   for i,line in enumerate(stream):
    assert i<len(order);r=json.loads(line);j=order[i];index=j if kind=='warmup' else j['index'];assert r['sequence']==i and r['index']==index;raw=r['raw'];assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'];assert raw['command']==[f['binary']['path'],*map(str,cells[index])]
    meta,*ps=[json.loads(l) for l in raw['stdout'].splitlines()];m,fam,n,q,e,c,col=cells[index];assert meta==dict(mode=m,family=fam,width=n,queries=q,full=e=='full',retention=c,collector=col,meter=False)
    assert [p['phase'] for p in ps]==['source','prepare','source-drop']+['request','execute-observe','request-drop','consumer']*q+['prepared-drop','consumer-drop'];assert all(p['memory'] is None and isinstance(p['ns'],int) and p['ns']>=0 for p in ps)
    if kind=='runs':
     ns=sum(p['ns'] for p in ps);totals[index].append(ns);assert (j['block'],m) not in paired;paired[j['block'],m]=ns
     for name in set(p['phase'] for p in ps):phases[index][name].append(sum(p['ns'] for p in ps if p['phase']==name))
    count+=1
  assert count==len(order)
 parent=read(PARENT/'audit.json')['summary'];lookup={(x['mode'],x['family'],x['width'],x['queries'],x['endpoint'],x['retention'],x['collector']):x for x in parent};summary=[]
 for i,c in enumerate(cells):
  assert len(totals[i])==64;old=lookup[tuple(c)];floor=100*max(clocks)*(4*c[3]+5)
  summary.append(dict(cell=c,median_ns=statistics.median(totals[i]),min_ns=min(totals[i]),max_ns=max(totals[i]),floor_ns=floor,clock_sensitive=min(totals[i])<floor,traffic=old['traffic'],peak=old['peak'],retained=old['retained'],phase_median_ns={n:statistics.median(v) for n,v in phases[i].items()}))
 k=max(k for k in range(1,33) if sum(math.comb(64,i) for i in range(k))*3840<=2**64);assert k==18;contrasts=[]
 for index,scenario in enumerate(scenarios):
  bs=[b for b,(_,s) in enumerate(blocks) if s==index];assert len(bs)==64
  for control in MODES[1:]:
   ratios=[paired[b,'union']/paired[b,control] for b in bs];ordered=sorted(ratios);lower,upper=ordered[k-1],ordered[64-k];sensitive=any(summary[index*4+MODES.index(m)]['clock_sensitive'] for m in ['union',control])
   label='instrumentation-sensitive' if sensitive else 'faster' if upper<.9 else 'slower' if lower>1.1 else 'within-10-percent' if lower>=.9 and upper<=1.1 else 'unresolved'
   contrasts.append(dict(scenario=scenario,control=control,median_ratio=statistics.median(ratios),min_ratio=min(ratios),max_ratio=max(ratios),interval=[lower,upper],classification=label,ratios=ratios))
 assert len(contrasts)==96
 print(json.dumps(dict(primary_processes=8192,warmups=128,cells=128,repetitions=64,contrasts_count=96,clock_medians_ns=clocks,clock_sensitive_cells=sum(s['clock_sensitive'] for s in summary),interval_ranks=[18,47],classification_counts=dict(collections.Counter(c['classification'] for c in contrasts)),summary=summary,contrasts=contrasts),indent=2))
if __name__=='__main__':main()
