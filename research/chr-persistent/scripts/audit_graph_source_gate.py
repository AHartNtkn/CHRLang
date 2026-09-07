"""Independent coverage, exact replay and immutable-input audit."""
import hashlib,json,tarfile
from run_graph_source_gate import ROOT,OUT,PREFIX

def main():
    m=json.loads((OUT/f'{PREFIX}-manifest.json').read_text())
    rows=[json.loads(s) for s in (OUT/f'{PREFIX}.jsonl').read_text().splitlines()]
    order=[dict(phase=p,index=i) for p in ('gate','replay') for i in range(64)]
    assert m['order']==order and [dict(phase=r['phase'],index=r['index']) for r in rows]==order
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in m['sha256'].items())
    with tarfile.open(OUT/f'{PREFIX}-inputs.tar.gz','r:gz') as a:
        assert all(hashlib.sha256(a.extractfile(p).read()).hexdigest()==h for p,h in m['sha256'].items())
    parsed=[]
    for row in rows:
        assert row['exit']==0 and not row['stderr'] and row['wall_seconds']<30
        lines=[s for s in row['stdout'].splitlines() if s.startswith('SOURCE\t')]
        assert len(lines)==1
        fields=lines[0].split('\t');assert len(fields)==12 and int(fields[1])==row['index']
        assert int(fields[3])>=int(fields[4]) and fields[5] in ('true','false')
        parsed.append(fields)
    assert parsed[:64]==parsed[64:]
    previous=[json.loads(s) for s in (OUT/'E14-graph-source-gate-v1.jsonl').read_text().splitlines()]
    assert [next(s for s in r['stdout'].splitlines() if s.startswith('SOURCE\t')).split('\t') for r in previous]==parsed
    assert len({f[2] for f in parsed[:64]})==64
    result=dict(passed=True,isolated_children=128,registry_cases=64,exact_semantic_and_counter_replay=True,
                raw_completions=sum(int(f[3]) for f in parsed[:64]),unique_observations=sum(int(f[4]) for f in parsed[:64]),
                exhausted_cases=sum(f[5]=='true' for f in parsed[:64]),max_child_wall_seconds=max(r['wall_seconds'] for r in rows))
    with (OUT/f'{PREFIX}-audit.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(json.dumps(result))
if __name__=='__main__':main()
