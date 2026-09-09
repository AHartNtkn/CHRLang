"""Check exact row coverage, expected/actual agreement and replay."""
import hashlib,json,tarfile
from run_graph_gate import ROOT,OUT,PREFIX

def main():
    m=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
    rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    groups=['targeted_templates','common_roots_different_environments','all_directed_graph_pairs']
    order=[dict(phase=p,group=g) for p in ('gate','replay') for g in groups]
    assert m['order']==order and [dict(phase=r['phase'],group=r['group']) for r in rows]==order
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in m['sha256'].items())
    with tarfile.open(OUT/f'{PREFIX}-inputs.tar.gz','r:gz') as archive:
        assert all(hashlib.sha256(archive.extractfile(p).read()).hexdigest()==h for p,h in m['sha256'].items())
    expected={groups[0]:[f'target-{i}-{layout}-{alias}-{rev}' for i in range(12) for layout in range(4) for alias in range(2) for rev in range(2)],
              groups[1]:['same-root-equal','same-root-unequal'],
              groups[2]:[f'graph-{a}-{b}' for a in range(64) for b in range(64)]}
    for group in (groups[0],groups[2]):
        expected[group]=[base+suffix for base in expected[group] for suffix in ('','-tree-left','-tree-right','-trees')]
    parsed=[]
    for row in rows:
        assert row['exit']==0 and not row['stderr'] and row['wall_seconds']<30
        values=[s.split('\t')[1:] for s in row['stdout'].splitlines() if s.startswith('GRAPH\t')]
        assert [v[0] for v in values]==expected[row['group']]
        assert all(len(v)==9 and v[1]==v[2] and v[1] in ('true','false') and all(int(n)>=0 for n in v[3:]) for v in values)
        parsed.append(values)
    assert parsed[:3]==parsed[3:]
    counts=[len(v) for v in parsed[:3]]
    result=dict(passed=True,isolated_children=6,counts=counts,total_comparisons_per_pass=sum(counts),
                exact_observation_and_counter_replay=True,positive=sum(v[1]=='true' for group in parsed[:3] for v in group),
                max_child_wall_seconds=max(r['wall_seconds'] for r in rows))
    with (OUT/f'{PREFIX}-audit.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps(result))
if __name__=='__main__':main()
