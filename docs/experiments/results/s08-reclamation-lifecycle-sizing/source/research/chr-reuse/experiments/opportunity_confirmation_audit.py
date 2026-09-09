"""Reconstruct registered ordering and costs independently from raw receipts."""
import hashlib
import itertools
import json
import math
import random
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-reuse-opportunity-confirmation'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def strip_times(v):
    if isinstance(v, dict):
        return {k: strip_times(x) for k, x in v.items() if k not in ['ns', 'first_answer_ns']}
    if isinstance(v, list):
        return [strip_times(x) for x in v]
    return v


def main():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    configs = [[f, n, 2, r, False, w, p] for f, n, w, p, r in itertools.product(
        ['exact', 'rename', 'history', 'history-early', 'distinct'], [0, 32], [1, 16], [0, 32], [False, True])]
    modes = [['off', m] for m in ['direct', 'alpha', 'live', 'compact-alpha', 'compact-live', 'sealed', 'dependencies', 'templates', 'lowered']] + [['on', 'sealed'], ['on', 'contracted']]
    contrasts = [[3, 1], [4, 2], [4, 0], [4, 5], [4, 10], [10, 9], [9, 5], [4, 7], [8, 4]]
    assert freeze['configs'] == configs and freeze['modes'] == modes and freeze['contrasts'] == contrasts
    for name, h in freeze['sources'].items():
        assert sha(OUT / 'source' / name) == h == sha(ROOT / name)
    for name, h in freeze['binaries'].items():
        assert sha(Path(name)) == h
    expected_order = [('allocation', i) for i in range(1760)]
    for block in range(7):
        order = list(range(880))
        random.Random(7851 + block).shuffle(order)
        expected_order += [('ordinary', 880 * block + i) for i in order]
    expected_order += [(kind, i) for i in range(11) for kind in ['cancel', 'cancel-meter']]
    manifest = json.loads((OUT / 'manifest.json').read_text())
    assert [(k, i) for k, i, _, _ in manifest] == expected_order
    values, hashes = {}, {}
    for kind, index, config, command in manifest:
        i, j = divmod(index % 880, 11)
        c = ['history-early', 16, 2, True, False, 16, 32] if kind.startswith('cancel') else configs[i]
        assert config == c
        feature, mode = modes[j]
        binary = Path('/tmp/chr-opportunity-confirm-21aae3fc9') / feature / ('meter' if kind in ['allocation', 'cancel-meter'] else 'ordinary')
        expected = [str(binary), mode] + [str(int(v)) if isinstance(v, bool) else str(v) for v in c]
        if kind.startswith('cancel'):
            expected.append('1')
        assert command == expected
        path = OUT / f'{kind}-{index:04}.json'
        raw = json.loads(path.read_text())
        assert raw['command'] == expected and raw['returncode'] == 0 and not raw.get('timeout')
        v = json.loads(raw['stdout'].splitlines()[-1])
        assert v['mode'] == mode and v['family'] == c[0] and v['resource'] == c[3]
        assert v['alternatives'] == c[5] and v['payload_depth'] == c[6]
        assert not v['counters'] and v['meter'] == (binary.name == 'meter')
        assert len(v['samples']) == 2
        for q, s in enumerate(v['samples']):
            assert s['depth'] == c[1] + q
            assert s['complete'] == (not kind.startswith('cancel') or q == 1)
            if s['complete']:
                assert s['answers'] == c[5]
            if s['first_answer_ns'] is not None:
                assert s['first_answer_ns'] <= s['execute_observe']['ns']
            if v['meter']:
                assert s['input_build']['memory']['live_start'] == s['answer_drop']['memory']['live_end']
        if v['meter']:
            assert v['source_build']['memory']['live_start'] == v['prepared_drop']['memory']['live_end']
        values[kind, index] = v
        hashes[path.name] = sha(path)
    assert len(values) == 7942
    summary = json.loads((OUT / 'summary.json').read_text())['results']
    assert len(summary) == 80

    def phases(v):
        return [v['preparation'], v['prepared_drop']] + [s[k] for s in v['samples'] for k in ['setup', 'execute_observe', 'engine_drop', 'answer_drop']]

    def total(v):
        return sum(p['ns'] for p in phases(v))

    for i, row in enumerate(summary):
        assert row['config'] == configs[i]
        times = []
        for j, (feature, mode) in enumerate(modes):
            meter = values['allocation', i * 11 + j]
            assert strip_times(meter) == strip_times(values['allocation', 880 + i * 11 + j])
            ordinary = [values['ordinary', b * 880 + i * 11 + j] for b in range(7)]
            t = [total(v) for v in ordinary]
            times.append(t)
            r = row['modes'][feature + ':' + mode]
            assert r['primary_median_ms'] == statistics.median(t) / 1e6
            assert r['inclusive_median_ms'] == statistics.median(total(v) + v['source_build']['ns'] + sum(s['input_build']['ns'] for s in v['samples']) for v in ordinary) / 1e6
            assert r['requested_bytes'] == sum(p['memory']['requested_bytes'] for p in phases(meter))
            all_p = phases(meter) + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
            assert r['peak_growth'] == max(p['memory']['peak_live'] for p in all_p) - meter['source_build']['memory']['live_start']
        for j, (a, b) in enumerate(contrasts):
            r = row['contrasts'][j]
            assert r['numerator'] == ':'.join(modes[a]) and r['denominator'] == ':'.join(modes[b])
            logs = [math.log(x / y) for x, y in zip(times[a], times[b])]
            rng = random.Random(7852 + i * 9 + j)
            draws = sorted(math.exp(sum(rng.choices(logs, k=7)) / 7) for _ in range(10000))
            lo, hi = draws[249], draws[9749]
            assert r['interval95'] == [lo, hi] and r['ratio'] == math.exp(statistics.mean(logs))
            assert r['classification'] == ('gain' if hi < .9 else 'loss' if lo > 1.1 else 'unresolved')
    (OUT / 'audit.json').write_text(json.dumps(dict(processes=7942, allocation_replays=880,
        contrasts_recomputed=720, frozen_sources=len(freeze['sources']), current_source_matches=True,
        requested_live_restoration=True, receipt_hashes=hashes), indent=2) + '\n')
    print('7942 receipts, 880 exact allocation replays, 720 recomputed paired contrasts verified')


if __name__ == '__main__':
    main()
