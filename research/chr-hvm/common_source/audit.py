"""Reconstruct common-source coverage and every accepted observation."""
import collections,hashlib,json
from gate import ROOT,OUT,MODES,sources,parse,normalize

def rows(name):return [json.loads(l) for l in (OUT/name).read_text().splitlines()]
def main():
 for path,h in json.loads((OUT/'validation.json').read_text())['hashes'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
 grouped={}
 for s in sources():grouped.setdefault(json.dumps(s['rules'],sort_keys=True),[]).append(s)
 groups=list(grouped.values());assert groups==json.loads((OUT/'groups.json').read_text()) and len(groups)==22
 runs=rows('runs.jsonl');assert len(runs)==264;seen=set();counts=collections.defaultdict(collections.Counter)
 for row in runs:
  key=row['group'],row['mode'];assert key not in seen;seen.add(key);group=groups[row['group']];assert row['code']==0
  if row['stdout'].startswith('UNSUPPORTED '):
   assert row['mode'] in ['prefix','finite'];counts[row['mode']]['unsupported']+=len(group);continue
  actual=parse(row['stdout']);assert len(actual)==len(group)
  for index,(a,s) in enumerate(zip(actual,group)):
   assert a['index']==index and a['exhausted']==(not s['ongoing'])
   assert sorted(map(normalize,a['answers']))==sorted(map(normalize,s['expected'])),(key,s['name'])
   counts[row['mode']]['pass']+=1
 assert seen=={(i,m) for i in range(22) for m in MODES}
 current={(r['group'],r['mode']):r for r in runs}
 initial=rows('initial-runs.jsonl');assert len(initial)==220
 for r in initial:
  now=current[r['group'],r['mode']];assert r['code']==now['code']==0 and r['stdout']==now['stdout'] and r['stderr']==now['stderr']
 entry=rows('prefix-entry-runs.jsonl');rejections=0
 for r in entry:
  now=current[r['group'],r['mode']]
  if r['code']==0:assert r['stdout']==now['stdout'] and r['stderr']==now['stderr']
  else:
   assert r['mode']=='prefix' and 'no closed acyclic private pure prefix' in r['stderr']
   assert now['stdout']=='UNSUPPORTED no closed acyclic private pure prefix\n';rejections+=1
 for r in rows('before-lint-runs.jsonl'):
  now=current[r['group'],r['mode']];assert r==now
 finite=json.loads((OUT/'finite-admission.json').read_text());assert len(finite)==22
 accepted=[r['group'] for r in finite if ' admitted' in r['stdout']];assert accepted==[2]
 assert counts['prefix']=={'pass':20,'unsupported':105} and counts['finite']=={'pass':4,'unsupported':121}
 for m in MODES[:10]:assert counts[m]=={'pass':125}
 result=dict(groups=22,processes=264,passed_observations=sum(c['pass'] for c in counts.values()),unsupported_observations=sum(c['unsupported'] for c in counts.values()),counts={m:dict(c) for m,c in counts.items()},initial_exact_process_replays=220,adapter_admission_corrections=rejections,pre_lint_exact_process_replays=264,finite_admitted_groups=accepted)
 (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
if __name__=='__main__':main()
