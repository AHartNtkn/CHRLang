#!/usr/bin/env python3
"""Post-run audit covering complete AND cutoff R06 lifecycle records."""
import hashlib
import json
import pathlib
from analyze_stream import phases

ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/r06-streaming-lifetime'
rows = [json.loads(line) for line in (OUT / 'runs.jsonl').read_text().splitlines()]
assert len(rows) == 384
partial = []
for row in rows:
    assert row.get('exit') == 0 and 'result' in row, row['cell']
    d = row['result']
    assert len(d['samples']) == d['delivered']
    assert [s['answer'] for s in d['samples']] == list(range(1,d['delivered']+1))
    assert sum(p['ns'] for p in phases(d)) == d['measured_ns']
    assert sum(s['ticks'] for s in d['samples']) + (d['terminal_service']['ticks'] if d['terminal_service'] else 0) == d['total_ticks']
    assert d['metrics'] == d['compiled_metrics'] == (row['mode'] == 'work')
    assert d['allocator_meter'] == (row['mode'] == 'allocation')
    if not d['complete']:
        assert d['cutoff'] and not d['exhausted']
        assert d['total_ticks'] == d['tick_limit'] == 20_000_000
        assert d['terminal_service']['outcome'] == 'cutoff'
        assert d['delivered'] == 42
        assert d['backend'] == 'conditional' and d['extent'] == 'unbounded' and d['n'] in [64,128]
        item = {'mode':row['mode'],'rep':row['rep'],'cell':row['cell'], 'delivered':d['delivered'],
                'measured_ms':d['measured_ns']/1e6,'final_diagnostics':d['final_diagnostics']}
        if row['mode'] == 'allocation':
            base = d['baseline']['memory']['live_end']
            item.update(retained_at_last_answer=d['samples'][-1]['consumer']['memory']['live_end']-base,
                        retained_at_cutoff=d['terminal_service']['service']['memory']['live_end']-base,
                        requested_bytes=sum(p['memory']['requested_bytes'] for p in phases(d)))
        partial.append(item)
    if row['mode'] == 'allocation':
        assert d['final']['memory']['live_end'] == d['baseline']['memory']['live_end'], row['cell']
meta=json.loads((OUT/'metadata.json').read_text())
for name, expected in meta['sources'].items():
    assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == expected, name
assert len(partial)==64
result={'processes':len(rows),'completed':320,'service_cutoffs':len(partial),
        'validated_raw_answers':sum(r['result']['delivered'] for r in rows),
        'allocation_baseline_restored':48,'source_freeze_unchanged':True,'partial_outcomes':partial}
(OUT/'outcome-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print({k:v for k,v in result.items() if k!='partial_outcomes'})
