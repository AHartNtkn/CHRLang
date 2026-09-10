"""Audit registered matrix positions, analytical multiplicities and replayed work."""
import hashlib,itertools,json,re,subprocess
from collections import Counter
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s06-learned-regions'

def read(name):
    text=(OUT/name).read_text()
    assert 'FAILED' not in text and 'error:' not in text
    counts=list(map(int,re.findall(r'test result: ok\. (\d+) passed;',text)))
    assert sorted(counts)==[1,2,7,9,11],counts
    rows=[line.split(',')[1:] for line in text.splitlines() if line.startswith('REGION,')]
    common=[list(map(int,line.split(',')[1:])) for line in text.splitlines() if line.startswith('COMMON,')]
    return rows,common

def main():
    rows,common=read('gate-off.log');other,other_common=read('gate-default.log')
    assert rows==other and common==other_common
    expected=[]
    for mask,weight,capacity,seed,seed_alias in itertools.product([0,273,238,511],[1,2],[0,1,4],[1,3],['false','true']):
        for (left,right),alias,base in itertools.product([(1,1),(3,3),(7,7),(1,4),(4,1)],['false','true'],[10,1000]):
            expected.append(list(map(str,[mask,weight,capacity,seed,seed_alias,left,right,alias,base])))
    assert [r[:9] for r in rows]==expected and len(rows)==1920
    counts=Counter();successful_savings=0
    increases=[]
    for row in rows:
        mask,weight,capacity,seed=int(row[0]),int(row[1]),int(row[2]),int(row[3])
        left,right=int(row[5]),int(row[6]);aliased=row[7]=='true'
        baseline,learned,exact,answers=map(int,row[9:])
        pairs=[(a,b) for a in range(3) for b in range(3) if left&(1<<a) and right&(1<<b) and (not aliased or a==b) and mask&(1<<(3*a+b))]
        assert answers==len(pairs)*weight*weight
        assert exact==baseline # Every matrix query is new to its exact-query cache.
        if capacity==0:assert learned==baseline
        direction='lower' if learned<baseline else 'higher' if learned>baseline else 'unchanged'
        counts[direction]+=1
        if direction=='lower' and answers:successful_savings+=1
        if direction=='higher':
            assert mask==0 and capacity==4 and baseline==2
            increases.append(row)
    assert counts==dict(lower=266,higher=14,unchanged=1640)
    assert successful_savings==0
    assert common==[[d,w,d+10,2*d+9,7,7] for d in [0,1,4,16] for w in [1,2]]
    assert 'Finished' in (OUT/'clippy.log').read_text() and 'error:' not in (OUT/'clippy.log').read_text()
    result=dict(seed_sessions_per_build=96,followup_sessions_per_build=1920,attribution_variants_per_build=8,tests_per_build=30,counts=dict(counts),successful_matrix_savings=successful_savings,common_work=common,increases=increases,base_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
    paths=[ROOT/p for p in ['research/chr-compiled/experiments/finite_phase.rs','research/chr-compiled/experiments/finite_learning.rs','research/chr-direct-conditional/tests/finite_learning_gate.rs','research/chr-direct-conditional/tests/runtime_support/mod.rs','research/chr-direct-conditional/tests/composition_support/mod.rs','research/chr-direct-conditional/examples/support/finite_bridge.rs','docs/experiments/registrations/S06-learned-regions.md','docs/experiments/results/S06-learned-regions.md']]
    paths+=[OUT/n for n in ['gate-off.log','gate-default.log','clippy.log']]+[Path(__file__)]
    result['hashes']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
    (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    print('1920 matrix rows replay exactly across builds; analytical multiplicities and all eight common-work contrasts pass.')
if __name__=='__main__':main()
