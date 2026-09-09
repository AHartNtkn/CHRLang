"""Independent receipt/phase audit for the exploratory opportunity matrix."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-reuse-opportunity-sizing'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def without_times(value):
    if isinstance(value, dict):
        return {k: without_times(v) for k, v in value.items()
                if k not in ['ns', 'first_answer_ns']}
    if isinstance(value, list):
        return [without_times(v) for v in value]
    return value


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    for path, expected in freeze['sources'].items():
        assert digest(OUT / 'source' / path) == expected, path
    for path, expected in freeze['binaries'].items():
        assert digest(Path(path)) == expected, path
    changed = [p for p, h in freeze['sources'].items() if digest(ROOT / p) != h]
    assert changed == ['research/chr-reuse/Cargo.toml',
                       'research/chr-reuse/examples/continuation_opportunity.rs']
    summary = json.loads((OUT / 'summary.json').read_text())
    assert summary['exploratory'] and len(summary['results']) == 120
    records = {}
    receipts = sorted(p for pattern in ['ordinary-*.json', 'allocation-*.json',
                                      'cancel-*.json'] for p in OUT.glob(pattern))
    assert len(receipts) == 4344
    for path in receipts:
        raw = json.loads(path.read_text())
        assert raw['returncode'] == 0 and not raw.get('timeout'), path
        value = json.loads(raw['stdout'].splitlines()[-1])
        command = raw['command']
        mode, family, depth, count, resource, reverse, width, payload = command[1:9]
        assert value['mode'] == mode and value['family'] == family
        assert value['resource'] == bool(int(resource))
        assert value['alternatives'] == int(width) and value['payload_depth'] == int(payload)
        assert not value['counters'] and value['meter'] == (Path(command[0]).name == 'meter')
        assert len(value['samples']) == int(count) == 2
        for i, sample in enumerate(value['samples']):
            assert sample['depth'] == int(depth) + i % 2
            assert sample['complete'] == (len(command) == 9 or i % 2 == 1)
            assert sample['answers'] == int(width) if sample['complete'] else sample['answers'] <= int(width)
            assert sample['first_answer_ns'] is None or sample['first_answer_ns'] <= sample['execute_observe']['ns']
            if value['meter']:
                assert sample['input_build']['memory']['live_start'] == sample['answer_drop']['memory']['live_end']
        if value['meter']:
            assert value['source_build']['memory']['live_start'] == value['prepared_drop']['memory']['live_end']
        records[path.stem] = value
    for i, config in enumerate(freeze['configs']):
        row = summary['results'][i]
        assert row['config'] == config
        for j, mode in enumerate(freeze['modes']):
            index = i * 12 + j
            ordinary = records[f'ordinary-{index:04}']
            meter = records[f'allocation-{index:04}']
            replay = records[f'allocation-{index + 1440:04}']
            assert without_times(meter) == without_times(replay)
            for kind in ['ordinary', 'allocation']:
                raw = json.loads((OUT / f'{kind}-{index:04}.json').read_text())
                expected_args = [mode] + [str(int(x)) if isinstance(x, bool) else str(x) for x in config]
                assert raw['command'][1:] == expected_args
            def phases(v):
                return [v['preparation'], v['prepared_drop']] + [s[k] for s in v['samples']
                    for k in ['setup', 'execute_observe', 'engine_drop', 'answer_drop']]
            result = row['modes'][mode]
            primary = sum(p['ns'] for p in phases(ordinary))
            assert result['primary_ms'] == primary / 1e6
            assert result['inclusive_ms'] == (primary + ordinary['source_build']['ns'] + sum(s['input_build']['ns'] for s in ordinary['samples'])) / 1e6
            assert result['requested_bytes'] == sum(p['memory']['requested_bytes'] for p in phases(meter))
            all_phases = phases(meter) + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
            assert result['peak_growth'] == max(p['memory']['peak_live'] for p in all_phases) - meter['source_build']['memory']['live_start']
    work = [json.loads(line) for line in (OUT / 'work.jsonl').read_text().splitlines()]
    assert len(work) == 540
    groups = {}
    for row in work:
        key = tuple(row[k] for k in ['family', 'resource', 'width', 'payload'])
        groups.setdefault(key, {})[row['mode']] = row
    assert len(groups) == 60
    for key, rows in groups.items():
        for owned, compact in [('ExactIds', 'CompactExact'), ('Alpha', 'CompactAlpha'), ('AlphaLive', 'CompactLive')]:
            assert all(rows[owned][k] == rows[compact][k] for k in ['logical', 'executed', 'hits'])
        assert rows['sealed']['applications'] == rows['contracted']['applications']
        assert rows['sealed']['carrier_steps'] == 0
        assert rows['contracted']['carrier_steps'] == 31 * key[2]
    result = {'processes': 4344, 'allocation_replays': 1440,
              'frozen_sources_verified': len(freeze['sources']),
              'current_source_deviations': changed, 'work_rows': 540,
              'owned_compact_equal_work_pairs': 180,
              'source_equivalent_contraction_pairs': 60,
              'interpretation': 'Exploratory timing only; contraction added after pilot and not timed.',
              'receipt_hashes': {p.name: digest(p) for p in receipts},
              'post_pilot_sources': {p: digest(ROOT / p) for p in changed + [
                  'research/chr-reuse/examples/opportunity_work.rs',
                  'research/chr-reuse/experiments/opportunity_audit.py']}}
    (OUT / 'audit.json').write_text(json.dumps(result, indent=2) + '\n')
    print('4344 receipts, 1440 allocation replays, 180 key-work pairs and 60 contraction pairs verified')


if __name__ == '__main__':
    main()
