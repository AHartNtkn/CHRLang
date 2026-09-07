"""Independent matrix and accounting audit, with exploratory sizing summaries."""
import hashlib
import json
from pathlib import Path

out = Path('docs/experiments/results')
prefix = 'E16-regions-pilot'
manifest = json.loads((out / f'{prefix}-manifest.json').read_text())
rows = [json.loads(s) for s in (out / f'{prefix}.jsonl').read_text().splitlines()]
ids = ['one-zero', 'one-work', 'two-work', 'owner-product', 'asym-first', 'asym-last',
       'duplicate-eight', 'refute-loop', 'stream-prefix', 'mixed-add-infer']
modes = ['Inline', 'Threads1', 'Threads2']
cells = []
for case in ids:
    cells += [(case, 'Baseline', 1, 0)]
    cells += [(case, mode, q, 4) for q in [1, 8] for mode in modes]
cells += [(case, mode, 64, 4) for case in ['one-work', 'two-work'] for mode in modes]
cells += [('two-work', mode, 8, 1) for mode in modes]
expected = [(kind, *cell) for kind in ['time', 'memory'] for cell in cells]
def key(r): return tuple(r[k] for k in ['kind', 'case', 'mode', 'quantum', 'limit'])
assert len(expected) == 158 and len(set(expected)) == 158
assert [key(r) for r in rows] == expected
assert [key(r) for r in manifest['order']] == expected
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest() == h for p,h in manifest['sha256'].items())
lookup = {key(r): r['measurement'] for r in rows if r['exit'] == 0 and 'measurement' in r}
assert len(lookup) == 158
for row in rows:
    m = row['measurement']
    n = lambda k: int(m[k])
    assert m['pass'] == 'true' and row['exit'] == 0
    assert (m['case'], m['mode'], n('quantum'), n('limit')) == key(row)[1:]
    assert m['metered'] == str(row['kind'] == 'memory').lower()
    assert n('wall_through_drop_including_diagnostics_ns') == n('operational_ns') + n('diagnostic_gap_ns')
    for name in m:
        if name.startswith('stop_') and (name.startswith(('stop_accepted_', 'stop_global_')) or name in ['stop_raw', 'stop_raw_available', 'stop_owner_steps', 'stop_products']):
            assert m[name] == m[name.replace('stop_', 'joined_', 1)], (key(row), name)
    if row['mode'] != 'Baseline':
        assert n('joined_transport_issued') == n('joined_transport_received')
        assert n('joined_transport_issued') - n('joined_transport_accepted') == n('joined_transport_unaccepted_at_shutdown')
        assert n('joined_transport_outstanding') == n('joined_transport_buffered') == 0
        assert n('joined_transport_max_outstanding') <= row['limit']
        assert n('joined_transport_actual_source_steps') == n('joined_actual_source_steps')
        assert n('joined_transport_accepted_source_steps') == n('joined_accepted_source_steps')
        assert n('joined_actual_source_steps') >= n('joined_accepted_source_steps')
    if row['case'] == 'stream-prefix':
        assert n('answers') == 8 and m['exhausted'] == m['stop_raw_available'] == 'false'
    elif row['case'] == 'duplicate-eight':
        assert n('answers') == 1 and n('stop_raw') == 256
    elif row['case'] == 'refute-loop':
        assert n('answers') == 0 and m['exhausted'] == 'true' and n('stop_raw') == 0
    # Accepted source work and final observations must match the corresponding
    # same-Q inline control; speculative actual work may differ at early stops.
    control = lookup[(row['kind'], row['case'], 'Inline', row['quantum'], 4)]
    for name in m:
        if name.startswith(('stop_accepted_', 'stop_global_')) or name in ['answers', 'exhausted', 'stop_raw', 'stop_raw_available']:
            assert m[name] == control[name], (key(row), name, m[name], control[name])
for case, mode, q, k in cells:
    a, b = lookup[('time',case,mode,q,k)], lookup[('memory',case,mode,q,k)]
    for name in a:
        if name.startswith(('stop_accepted_', 'stop_global_')) or name in ['answers','exhausted','stop_raw','stop_raw_available']:
            assert a[name] == b[name], ((case,mode,q,k),name)
summary = {
    'pass': True, 'cells':79, 'children':158,
    'max_child_wall_seconds':max(r['child_wall_seconds'] for r in rows),
    'max_rss_kib':max(int(r['rss_report'].strip()) for r in rows),
    'prefix_and_refutation_work':[
        {**{k:r[k] for k in ['kind','case','mode','quantum','limit']},
         **{k:v for k,v in r['measurement'].items() if k.startswith('joined_transport_')}}
        for r in rows if r['case'] in ['stream-prefix','refute-loop']],
    'time_cells':[
        {**{k:r[k] for k in ['case','mode','quantum','limit']},
         **{k:v for k,v in r['measurement'].items() if k.endswith('_ns')}}
        for r in rows if r['kind']=='time'],
}
(out / f'{prefix}-audit.json').write_text(json.dumps(summary, indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in ['prefix_and_refutation_work','time_cells']}))
