"""Check complete qualified tests, frozen inputs and paired owner conservation."""
from pathlib import Path
import hashlib,json,re
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-projection-lifecycle-gate'
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
    m=json.loads((RAW/'manifest.json').read_text())
    for p,h in m['sha256'].items(): assert digest(ROOT/p)==h,p
    prior=json.loads((ROOT/'docs/experiments/results/s06-projection-entry/qualified/manifest.json').read_text())
    assert digest(RAW/'prior/projection.rs')==prior['sha256']['research/chr-structural/src/projection.rs']
    names={};seen=set()
    for build,test,binary in m['binaries']:
        r=json.loads((RAW/f'{build}-{test}.json').read_text())
        assert r['command']==[str(ROOT/binary),'--test-threads=1','--nocapture']
        assert r['returncode']==0 and not r['stderr']
        count={'projection_lifecycle':7,'projection':10,'finite_paths':16}[test]
        assert f'test result: ok. {count} passed; 0 failed; 0 ignored;' in r['stdout']
        tests=re.findall(r'^test (\S+) \.\.\.',r['stdout'],re.M)
        assert len(set(tests))==count
        if test in names: assert names[test]==tests
        else: names[test]=tests
        if test=='projection_lifecycle':assert 'complete weighted-map comparisons: 12288' in r['stdout']
        seen.add((build,test))
    assert seen=={(b,t) for b in ['default','counter-free'] for t in names} and len(seen)==6
    cells=[]
    for family in ['independent','star','dense']:
        for retain in ['0','4','all']:
            for cancel in ['0','1']:
                pair=[]
                for repeat in range(2):
                    r=json.loads((RAW/f'owner-{family}-{retain}-{cancel}-{repeat}.json').read_text())
                    assert r['returncode']==0 and not r['stderr']
                    assert r['command']==[str(ROOT/m['ownership']),family,retain,cancel]
                    data=[json.loads(s) for s in r['stdout'].splitlines()]
                    summary=data[0];rows=data[1:]
                    assert (summary['family'],summary['retain'],summary['cancel'])==(family,retain,cancel=='1')
                    expected={'independent':[4096,2048,2048,4096],'star':[129,65,64,129],'dense':[1024,512,512,1024]}[family]
                    assert summary['expected']==expected
                    assert summary['delivered']==([min(8,n) for n in expected] if cancel=='1' else expected)
                    assert summary['unreleased_bytes']==0
                    assert len(rows)==18
                    start=rows[0]['memory']['live_start'];held=rows[-2]['memory']['live_end'];end=rows[-1]['memory']['live_end']
                    assert held-start==summary['consumer_bytes'] and end==start
                    assert rows[-1]['phase']=='consumer_dispose'
                    assert [r['query'] for r in rows if r['phase']=='weighted_iterator_setup']==[0,1,2,3]
                    assert [r['query'] for r in rows if r['phase']=='producer_dispose']==[3]
                    pair.append(data)
                assert pair[0]==pair[1],(family,retain,cancel)
                cells.append(pair[0][0])
    (RAW/'audit.json').write_text(json.dumps(dict(status='passed',correctness_processes=6,owner_processes=36,map_comparisons_per_build=12288,cells=cells),indent=2)+'\n')
    print('Verified six correctness processes, 12288 map comparisons per build, and 36 exactly paired ownership diagnostics.')
