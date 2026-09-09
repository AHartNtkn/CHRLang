"""Prospectively registered, paired opportunity/contraction comparison."""
import hashlib
import itertools
import json
import math
import random
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'research/chr-direct-conditional/experiments'))
import derivation_sizing as runner
from shared_template_analysis import phases, primary

OUT = ROOT / 'docs/experiments/results/s05-reuse-opportunity-confirmation'
BIN = Path('/tmp/chr-opportunity-confirm-21aae3fc9')
MODES = [('off', m) for m in ['direct', 'alpha', 'live', 'compact-alpha',
         'compact-live', 'sealed', 'dependencies', 'templates', 'lowered']]
MODES += [('on', 'sealed'), ('on', 'contracted')]
CONFIGS = [(f, n, 2, r, False, w, p) for f, n, w, p, r in itertools.product(
    ['exact', 'rename', 'history', 'history-early', 'distinct'],
    [0, 32], [1, 16], [0, 32], [False, True])]
CONTRASTS = [(3, 1), (4, 2), (4, 0), (4, 5), (4, 10), (10, 9), (9, 5), (4, 7), (8, 4)]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def command(kind, j, config, cancel=None):
    feature, mode = MODES[j]
    binary = BIN / feature / ('meter' if kind in ['allocation', 'cancel-meter'] else 'ordinary')
    cmd = [str(binary), mode] + [str(int(x)) if isinstance(x, bool) else str(x) for x in config]
    return cmd + ([str(cancel)] if cancel is not None else [])


def invoke(cmd, path):
    assert not path.exists(), path
    start = time.monotonic()
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=60, preexec_fn=runner.limits)
        raw = dict(command=cmd, returncode=p.returncode, stdout=p.stdout,
                   stderr=p.stderr, wall_seconds=time.monotonic() - start)
    except subprocess.TimeoutExpired as e:
        path.write_text(json.dumps(dict(command=cmd, timeout=True, stdout=str(e.stdout), stderr=str(e.stderr)), indent=2) + '\n')
        raise
    path.write_text(json.dumps(raw, indent=2) + '\n')
    assert p.returncode == 0, path
    return raw


def validate(raw, config, kind):
    v = json.loads(raw['stdout'].splitlines()[-1])
    assert v['event'] == 'result' and not v['counters']
    assert v['meter'] == (kind in ['allocation', 'cancel-meter'])
    assert v['mode'] == raw['command'][1] and v['family'] == config[0]
    assert v['resource'] == config[3] and v['alternatives'] == config[5]
    assert v['payload_depth'] == config[6] and len(v['samples']) == 2
    for i, s in enumerate(v['samples']):
        assert s['depth'] == config[1] + i % 2
        assert s['complete'] == (not kind.startswith('cancel') or i % 2 == 1)
        if s['complete']:
            assert s['answers'] == config[5]
        if v['meter']:
            assert s['input_build']['memory']['live_start'] == s['answer_drop']['memory']['live_end']
    if v['meter']:
        assert v['source_build']['memory']['live_start'] == v['prepared_drop']['memory']['live_end']
    return v


def read(kind, index, config):
    raw = json.loads((OUT / f'{kind}-{index:04}.json').read_text())
    assert raw['returncode'] == 0 and not raw.get('timeout')
    return validate(raw, config, kind)


def analyze():
    freeze = json.loads((OUT / 'freeze.json').read_text())
    for p, h in freeze['sources'].items():
        assert digest(OUT / 'source' / p) == h and digest(ROOT / p) == h
    for p, h in freeze['binaries'].items():
        assert digest(Path(p)) == h
    results = []
    for i, config in enumerate(CONFIGS):
        row = dict(config=config, modes={}, contrasts=[])
        values = [[read('ordinary', b * 880 + i * 11 + j, config) for b in range(7)] for j in range(11)]
        for j, (feature, mode) in enumerate(MODES):
            meter = read('allocation', i * 11 + j, config)
            replay = read('allocation', 880 + i * 11 + j, config)
            assert runner.allocation_records(meter) == runner.allocation_records(replay)
            ps = list(phases(meter).values())
            all_ps = ps + [meter['source_build']] + [s['input_build'] for s in meter['samples']]
            row['modes'][feature + ':' + mode] = {
                'primary_median_ms': statistics.median(primary(v) for v in values[j]) / 1e6,
                'inclusive_median_ms': statistics.median(primary(v) + v['source_build']['ns'] + sum(s['input_build']['ns'] for s in v['samples']) for v in values[j]) / 1e6,
                'requested_bytes': sum(p['memory']['requested_bytes'] for p in ps),
                'peak_growth': max(p['memory']['peak_live'] for p in all_ps) - meter['source_build']['memory']['live_start'],
                'first_answer_median_ms': statistics.median(s['first_answer_ns'] for v in values[j] for s in v['samples']) / 1e6,
                'phase_median_ms': {k: statistics.median(phases(v)[k]['ns'] for v in values[j]) / 1e6 for k in phases(values[j][0])},
            }
        for j, (a, b) in enumerate(CONTRASTS):
            logs = [math.log(primary(x) / primary(y)) for x, y in zip(values[a], values[b])]
            rng = random.Random(7852 + 9 * i + j)
            draws = sorted(math.exp(sum(rng.choices(logs, k=7)) / 7) for _ in range(10000))
            lo, hi = draws[249], draws[9749]
            row['contrasts'].append(dict(numerator=':'.join(MODES[a]), denominator=':'.join(MODES[b]),
                ratio=math.exp(statistics.mean(logs)), interval95=[lo, hi],
                classification='gain' if hi < .9 else 'loss' if lo > 1.1 else 'unresolved'))
        results.append(row)
    (OUT / 'summary.json').write_text(json.dumps(dict(results=results), indent=2) + '\n')
    print('7942 processes complete; 880 exact allocation replays; paired analysis complete', flush=True)


def main():
    if len(sys.argv) == 2 and sys.argv[1] == 'analyze':
        analyze()
        return
    assert len(CONFIGS) == 80 and len(MODES) == 11
    OUT.mkdir(exist_ok=False)
    paths = ['Cargo.toml', 'Cargo.lock', 'docs/experiments/registrations/S05-reuse-opportunity-confirmation.md',
             'research/chr-direct-conditional/experiments/shared_template_analysis.py',
             'research/chr-direct-conditional/experiments/derivation_sizing.py']
    for directory in ['crates/chr-syntax', 'research/chr-reuse', 'research/chr-persistent',
                      'research/chr-compiled', 'research/chr-observe', 'research/chr-direct-choice', 'research/chr-direct-conditional']:
        paths += [str(p.relative_to(ROOT)) for p in (ROOT / directory).rglob('*') if p.is_file() and p.suffix in ['.rs', '.toml', '.py']]
    freeze = dict(commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
        rustc=subprocess.check_output(['rustc', '-Vv'], text=True), cpu=runner.CPU,
        configs=CONFIGS, modes=MODES, contrasts=CONTRASTS, sources={}, binaries={})
    for p in sorted(set(paths)):
        dst = OUT / 'source' / p
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / p, dst)
        freeze['sources'][p] = digest(dst)
    for feature, allocator in itertools.product(['off', 'on'], ['ordinary', 'meter']):
        binary = BIN / feature / allocator
        freeze['binaries'][str(binary)] = digest(binary)
        shutil.copyfile(BIN / (feature + '-' + allocator + '.log'), OUT / (feature + '-' + allocator + '-build.log'))
    manifest = []
    for rep in range(2):
        for i, c in enumerate(CONFIGS):
            for j in range(11):
                manifest.append(('allocation', rep * 880 + i * 11 + j, c, command('allocation', j, c)))
    for block in range(7):
        order = list(range(880))
        random.Random(7851 + block).shuffle(order)
        for index in order:
            i, j = divmod(index, 11)
            manifest.append(('ordinary', block * 880 + index, CONFIGS[i], command('ordinary', j, CONFIGS[i])))
    c = ('history-early', 16, 2, True, False, 16, 32)
    for j in range(11):
        for kind in ['cancel', 'cancel-meter']:
            manifest.append((kind, j, c, command(kind, j, c, 1)))
    (OUT / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    (OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    for feature in ['off', 'on']:
        invoke([str(BIN / feature / 'meter'), 'meter-check'], OUT / (feature + '-meter-check.json'))
        for kind in ['ordinary', 'allocation']:
            j = 5 if feature == 'off' else 10
            raw = invoke(command(kind, j, CONFIGS[0]), OUT / (feature + '-' + kind + '-check.json'))
            validate(raw, CONFIGS[0], kind)
    allocations = {}
    for count, (kind, index, config, cmd) in enumerate(manifest, 1):
        v = validate(invoke(cmd, OUT / f'{kind}-{index:04}.json'), config, kind)
        if kind == 'allocation':
            record = runner.allocation_records(v)
            if index < 880:
                allocations[index] = record
            else:
                assert allocations[index - 880] == record
        if count % 880 == 0:
            print(count, 'registered processes complete', flush=True)
    analyze()


if __name__ == '__main__':
    main()
