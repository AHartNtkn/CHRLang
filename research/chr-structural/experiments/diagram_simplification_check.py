"""Check source-specific analytical simplifications against full assignment enumeration."""
import collections,itertools,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
rows=[];checks=0
for family,n,k in itertools.product(['free','star','clique','duplicate','overlap','union'],[3,6],[2,3]):
 star=[(a,h) for h in range(2,n) for a in [0,1]]
 clique=list(itertools.combinations(range(n),2))
 branches={'free':[[]],'star':[star],'clique':[clique],'duplicate':[[],[]],'overlap':[[(0,1)],[(0,2)]],'union':[star,clique]}[family]
 observed={(a[0],a[1]) for a in itertools.product(range(k),repeat=n) if any(all(a[x]!=a[y] for x,y in edges) for edges in branches)}
 if family in ['free','duplicate','overlap']:form='true'
 elif family in ['star','union']:form='true' if k>=3 else 'equal'
 else:form='false' if n>k else 'different'
 for x,y in itertools.product(range(k),repeat=2):
  predicted={'true':True,'false':False,'equal':x==y,'different':x!=y}[form]
  assert predicted==((x,y) in observed),(family,n,k,x,y)
  checks+=1
 rows.append({'family':family,'variables':n,'alphabet':k,'derived_form':form,'visible_pairs':sorted(observed)})
assert checks==156
print(json.dumps({'source_configurations':len(rows),'pair_checks':checks,'forms':dict(collections.Counter(r['derived_form'] for r in rows)),'cases':rows},indent=2))
