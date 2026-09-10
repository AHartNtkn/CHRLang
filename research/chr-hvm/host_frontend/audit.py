"""Audit frontend evidence against independent stored syntax and native records."""
import hashlib
import json
from gate import ROOT, OUT, wire, joined
from frontend import decode

v = json.loads((OUT / 'validation.json').read_text())
for path, digest in v['hashes'].items():
    assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
rows = [json.loads(l) for l in (OUT / 'runs.jsonl').read_text().splitlines()]
assert [(r['family'], r['group']) for r in rows] == [('common', i) for i in range(22)] + [('substantive', i) for i in range(4)]
assert v['sessions'] == 26 and v['queries'] == sum(r['queries'] for r in rows) == 479
plans = json.loads((ROOT / 'docs/experiments/results/s10-native-prepared/plans.json').read_text())
groups = json.loads((ROOT / 'docs/experiments/results/s10-native-substantive/groups.json').read_text())
for row in rows:
    g = row['group']
    sources = [q['source'] for q in plans[g]['queries']] if row['family'] == 'common' else groups[g]
    rules, queries = decode(joined(sources))
    assert rules == sources[0]['rules']
    assert queries == [dict(query=s['query'], outputs=s['outputs']) for s in sources]
    artifact = OUT / f"{row['family']}-{g}.hvm"
    frozen = ROOT / (plans[g]['program'] if row['family'] == 'common' else f'docs/experiments/results/s10-native-substantive/rules-{g}.hvm')
    assert artifact.read_bytes() == frozen.read_bytes()
    snapshot = row['validation_snapshot']
    assert hashlib.sha256(artifact.read_bytes()).hexdigest() == snapshot['program_sha256']
    assert len(row['encoding_ns']) == len(snapshot['dictionaries']) == row['queries']
    assert all(x >= 0 for x in row['encoding_ns'])
    for key in ['decode_ns','emission_ns','protocol_assembly_ns','release_ns']:
        assert row[key] >= 0
    previous_path = 'native-prepared.jsonl' if row['family'] == 'common' else 'native-substantive.jsonl'
    previous = [json.loads(l) for l in (ROOT / 'docs/experiments/results/s10-answer-wire' / previous_path).read_text().splitlines()]
    previous = next(r['result'] for r in previous if r['mode'] == 'ordinary' and r['group'] == g)
    assert row['result']['stdout_hex'] == previous['stdout_hex']
    actual_events = [json.loads(l) for l in row['result']['stderr'].splitlines()]
    prior_events = [json.loads(l) for l in previous['stderr'].splitlines()]
    assert len(actual_events) == len(prior_events) == row['queries'] + 1
    for actual, prior in zip(actual_events[:-1], prior_events[:-1]):
        for key in ['query','calls','pending','unsupported','dynamic_words']:
            assert actual[key] == prior[key]
    assert row['result']['code'] == 0
print('26 source artifacts, 479 query syntaxes and 479 exact native observations audited; no comparative timing inference.')
