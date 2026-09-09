#!/usr/bin/env python3
"""Audit the registered full-caller pilot; emit descriptive, not inferential, results."""
import collections
import hashlib
import itertools
import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-caller-cost-pilot'
FIELDS = ('mode', 'build', 'depth', 'tail', 'payload', 'queries', 'distinct', 'cancel')
MODES = ('direct', 'whole', 'scan', 'sealed', 'call-direct', 'call-memo', 'lowered')

def key(row):
    return tuple(row[f] for f in FIELDS)

def expected_keys():
    normal = {(m, *c, False) for m in MODES for c in itertools.product(
        (False, True), (0, 32), (0, 8), (0, 8), (1, 8), (False, True))}
    cancel = {(m, b, 32, 8, 8, 8, d, True) for m in MODES
              for b, d in itertools.product((False, True), repeat=2)}
    return normal | cancel

def interval(values):
    return dict(median=statistics.median(values), minimum=min(values), maximum=max(values))

def audit(records):
    groups = collections.defaultdict(list)
    for r in records:
        k = key(r)
        assert k in expected_keys(), k
        assert r['kind'] in ('ordinary', 'meter')
        groups[k].append(r)
        q = r['queries']
        expected = [('prepare', 0)] + [(p, i) for i in range(q) for p in
            ('input', 'setup', 'execute_observe', 'caller_drop', 'consumer_drop')] + [('prepared_drop', 0)]
        assert [(x['phase'], x['query']) for x in r['rows']] == expected, k
        assert r['answers'] == [1 if r['cancel'] else 2] * q, k
        assert len(r['first']) == q
        for i in range(q):
            execution = next(x for x in r['rows'] if x['phase'] == 'execute_observe' and x['query'] == i)
            assert 0 < r['first'][i] <= execution['ns'], k
        if r['kind'] == 'ordinary':
            assert all(x['memory'] is None for x in r['rows'])
        else:
            ms = [x['memory'] for x in r['rows']]
            assert ms[0]['live_start'] == ms[-1]['live_end'], k
            assert all(a['live_end'] == b['live_start'] for a,b in zip(ms, ms[1:])), k
            assert all(x['peak_live'] >= max(x['live_start'], x['live_end']) for x in ms)
    assert set(groups) == expected_keys()
    summaries = []
    for k, rs in sorted(groups.items()):
        timing = [r for r in rs if r['kind'] == 'ordinary']
        meter = sorted((r for r in rs if r['kind'] == 'meter'), key=lambda r:r['rep'])
        assert sorted(r['rep'] for r in timing) == list(range(1 if k[-1] else 5)), k
        assert [r['rep'] for r in meter] == [0, 1], k
        signature = lambda r: [(x['phase'], x['query'], x['memory']) for x in r['rows']]
        assert signature(meter[0]) == signature(meter[1]), k
        s = dict(zip(FIELDS, k))
        s['primary_ns'] = interval([sum(x['ns'] for x in r['rows'] if x['phase'] != 'input') for r in timing])
        s['input_inclusive_ns'] = interval([sum(x['ns'] for x in r['rows']) for r in timing])
        s['phase_ns'] = {p:interval([sum(x['ns'] for x in r['rows'] if x['phase'] == p) for r in timing]) for p in {x['phase'] for x in timing[0]['rows']}}
        s['first_query_ns'] = [interval([r['first'][i] + next(x['ns'] for x in r['rows'] if x['phase']=='setup' and x['query']==i) for r in timing]) for i in range(k[5])]
        ms = meter[0]['rows']
        s['requested_bytes'] = sum(x['memory']['requested_bytes'] for x in ms if x['phase'] != 'input')
        s['input_inclusive_bytes'] = sum(x['memory']['requested_bytes'] for x in ms)
        s['peak_growth_bytes'] = max(x['memory']['peak_live'] for x in ms) - ms[0]['memory']['live_start']
        s['phase_bytes'] = {p:sum(x['memory']['requested_bytes'] for x in ms if x['phase']==p) for p in s['phase_ns']}
        summaries.append(s)
    return summaries

if __name__ == '__main__':
    verified = []
    for line in (OUT/'freeze.sha256').read_text().splitlines():
        digest, name = line.split('  ', 1)
        p = Path(name)
        assert hashlib.sha256(p.read_bytes()).hexdigest() == digest, name
        verified.append(name)
    records = [json.loads(line) for line in (OUT/'runs.jsonl').open()]
    summaries = audit(records)
    (OUT/'summary.json').write_text(json.dumps(summaries, indent=2, sort_keys=True)+'\n')
    receipt = dict(records=len(records), cells=len(summaries), allocation_replays=len(summaries),
                   ordinary=sum(r['kind']=='ordinary' for r in records), meter=sum(r['kind']=='meter' for r in records),
                   verified_frozen_files=verified, interpretation='Exploratory medians and ranges only')
    (OUT/'audit.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print(json.dumps(receipt, indent=2))
