"""Recheck archived lifecycle results, independent dictionaries and reuse controls."""
import hashlib
import json
from collections import Counter
from gate import ROOT, OUT, check, common, wire


def rows(name):
    return [json.loads(line) for line in (OUT / name).read_text().splitlines()]


def main():
    validation = json.loads((OUT / 'validation.json').read_text())
    for name, expected in validation['hashes'].items():
        assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == expected, name
    families = {name: json.loads((ROOT / 'docs/experiments/results' / folder / 'groups.json').read_text())
                for name, folder in [('common', 's10-native-common-source'), ('substantive', 's10-native-substantive')]}
    runs = rows('runs.jsonl')
    keys = [(r['family'], r['group'], r['mode']) for r in runs]
    expected_keys = [(name, g, mode) for name, groups in families.items()
                     for g in range(len(groups)) for mode in range(13)]
    assert keys == expected_keys and len(keys) == 338
    initial = rows('initial-runs.jsonl')
    prior_wire = [json.loads(l) for l in (ROOT / 'docs/experiments/results/s10-answer-wire/rust.jsonl').read_text().splitlines()]
    assert len(initial) == len(prior_wire) == len(runs)
    counts = Counter()
    selection = {}
    for r, old, previous in zip(runs, initial, prior_wire):
        key = (r['family'], r['group'], r['mode'])
        assert key == (old['family'], old['group'], old['mode']) == (previous['family'], previous['group'], previous['mode'])
        assert r['result']['stdout_hex'] == old['result']['stdout_hex'] == previous['result']['stdout_hex']
        group = families[r['family']][r['group']]
        values = check(r['result'], group)
        counts['unsupported' if values is None else 'passed'] += len(group)
        if values is None:
            assert r['mode'] in [10, 11, 12]
            continue
        events = [json.loads(l) for l in r['result']['stderr'].splitlines()][1:-1]
        for source, value, event in zip(group, values, events):
            _, ps, ats = wire.prepared.compile_source(dict(rules=source['rules'], query=[], outputs=[]))
            _, ps, ats = wire.prepared.encode(source, ps, ats, 65536, True)
            assert value['predicates'] == ps and value['atoms'] == ats
            if event['exhausted'] and (r['mode'] not in selection or event['calls'] > selection[r['mode']]['calls']):
                selection[r['mode']] = dict(source=source, calls=event['calls'])
    assert dict(counts) == validation['counts'] == dict(passed=2286, unsupported=587)
    cancellations = rows('cancellations.jsonl')
    assert len(cancellations) == 52
    assert Counter((r['mode'], r['limit']) for r in cancellations) == Counter((m, n) for m in range(13) for n in [200000, 0, 1, 64])
    full = {}
    for r in cancellations:
        values = check(r['result'], [r['source']], r['limit'] == 200000)
        got = list(map(common.normalize, values[0]['answers']))
        if r['limit'] == 200000:
            full[r['mode']] = got
        else:
            assert got == full[r['mode']][:len(got)]
        event = json.loads(r['result']['stderr'].splitlines()[1])
        assert event['calls'] <= r['limit']
        if r['limit'] == 0:
            assert event['calls'] == 0 and not values[0]['answers'] and not event['exhausted']
    assert {int(k): v for k, v in json.loads((OUT / 'reuse-selection.json').read_text()).items()} == selection
    retained = rows('retained-reuse.jsonl')
    assert sorted(r['mode'] for r in retained) == list(range(13))
    for r in retained:
        source = selection[r['mode']]['source']
        budgets = [0, 1, 64, selection[r['mode']]['calls'] // 2, 200000]
        assert r['budgets'] == budgets and len(r['standalone']) == 5
        values = check(r['result'], [source] * 5, False)
        events = [json.loads(l) for l in r['result']['stderr'].splitlines()][1:-1]
        for i, (budget, alone) in enumerate(zip(budgets, r['standalone'])):
            assert alone['budget'] == budget
            value = check(alone['result'], [source], i == 4)[0]
            assert values[i]['answers'] == value['answers'] and values[i]['exhausted'] == value['exhausted']
            assert events[i]['calls'] <= budget
            if i == 0:
                assert events[i]['calls'] == 0 and not value['answers'] and not value['exhausted']
            if i == 3:
                assert not value['exhausted'], 'halfway cancellation must interrupt live work'
        assert values[-1]['exhausted']
    print('Audited 338 corpus processes, 52 cancellation controls and 78 retained/standalone processes; complete outputs match both prior runners and independent source expectations.')


if __name__ == '__main__':
    main()
