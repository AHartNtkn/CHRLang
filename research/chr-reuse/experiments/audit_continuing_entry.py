"""Audit the incomplete ownership preflight and its diagnostic evidence.

This never promotes preflights into paired measurements or reruns experiments.
"""
from pathlib import Path
import hashlib
import importlib.util
import json
import sys

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / 'docs/experiments/results/s08-continuing-ownership'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def receipt(name):
    return json.loads((RAW / name).read_text())


def rows(name):
    return [json.loads(line) for line in receipt(name)['stdout'].splitlines()]


def audit():
    spec = importlib.util.spec_from_file_location(
        'ownership', ROOT / 'research/chr-reuse/experiments/continuing_ownership.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    manifest = receipt('manifest.json')
    for name, expected in manifest['sha256'].items():
        path = ROOT / name
        if name == 'research/chr-reuse/Cargo.toml':
            path = RAW / 'Cargo.preflight.toml'
        assert digest(path) == expected, name
    assert not list(RAW.glob('run-*.json')), 'Not a partial-preflight audit anymore'
    assert len(manifest['configurations']) == 162
    peaks = {}
    for index, mode in enumerate(['direct', 'compact-live', 'scan', 'sealed']):
        r = receipt(f'preflight-{index}.json')
        assert r['returncode'] == 0 and not r['stderr']
        row = json.loads(r['stdout'])
        module.validate(row, [mode, 512, 'all', 4])
        peaks[mode] = row['snapshots'][-1]['memory']['peak_live'] - row['baseline']
    failure = receipt('preflight-4.json')
    assert failure['returncode'] == 101 and 'producer service cutoff' in failure['stderr']
    assert receipt('preflight-5.json')['timeout']
    conditional = {}
    for name, reached in [('service-probe.json', 240),
                          ('service-probe-strong.json', 240),
                          ('service-probe-reverse.json', 211),
                          ('service-probe-stages.json', 240),
                          ('progress-conditional-inferred.json', 240)]:
        r = receipt(name)
        assert r['returncode'] == 101
        assert f'probe cutoff after {reached} answers' in r['stderr']
        rr = rows(name)
        assert [x['answers'] for x in rr] == [1, 8, 32, 64, 128]
        conditional[name] = rr[-1]
    strong = rows('service-probe-strong.json')
    stages = rows('service-probe-stages.json')
    assert [r['calls'] for r in strong] == [r['calls'] for r in stages]
    assert all(sum(r['stages']) == r['calls'] for r in stages)
    for mode in ['dependencies', 'templates', 'dependencies-reclaim', 'templates-reclaim']:
        name = f'progress-{mode}.json'
        r = receipt(name)
        assert r['returncode'] == 0 and not r['stderr']
        rr = rows(name)
        assert [x['answers'] for x in rr] == [1, 8, 32, 64, 128, 256, 512]
        assert all(x['removed'] == 0 for x in rr)
    metered = receipt('progress-metered-dependencies.json')
    assert metered['returncode'] == 0 and not metered['stderr']
    assert rows('progress-metered-dependencies.json')[-1]['answers'] == 516
    phases = receipt('phases-dependencies.json')
    assert 'validation-started' in phases['stderr']
    # Diagnostic hashes are a retrospective inventory, not prospective registration.
    paths = list(RAW.glob('*.json')) + list(RAW.glob('*.log'))
    paths += list((ROOT / 'research/chr-reuse/examples').glob('continuing_*.rs'))
    paths += [ROOT / 'research/chr-reuse/Cargo.toml', Path(__file__)]
    paths = [p for p in paths if p.name != 'partial-audit.json']
    binaries = {
        'base': ('s08-continuing-probe', 'continuing_service_probe'),
        'strong': ('s08-continuing-probe-strong', 'continuing_service_probe'),
        'reverse': ('s08-continuing-probe-reverse', 'continuing_service_probe'),
        'stages': ('s08-continuing-probe-stages', 'continuing_service_stages'),
        'progress': ('s08-continuing-progress', 'continuing_progress_probe'),
        'metered': ('s08-continuing-metered-probe', 'continuing_metered_probe'),
        'phases': ('s08-continuing-metered-probe', 'continuing_ownership_phases'),
    }
    binary_inventory = {}
    for mode, (target, example) in binaries.items():
        path = ROOT / 'target' / target / 'release/examples' / example
        binary_inventory[mode] = dict(path=str(path.relative_to(ROOT)), sha256=digest(path))
    assert binary_inventory['metered']['sha256'] == metered['sha256']
    assert binary_inventory['phases']['sha256'] == phases['sha256']
    result = dict(
        status='incomplete; no comparative matrix', diagnostic_binaries=binary_inventory,
        successful_preflights=4,
        conditional_service_cutoff=True, dependency_wall_timeout=True,
        unrun_preflights=3, comparative_processes=0, qualification_peaks=peaks,
        conditional_at_128=conditional,
        metered_dependency_prefixes=rows('progress-metered-dependencies.json'),
        phase_probe={k: v for k, v in phases.items() if k != 'stdout'},
        retrospective_sha256={str(p.relative_to(ROOT)): digest(p) for p in sorted(paths)})
    (RAW / 'partial-audit.json').write_text(json.dumps(result, indent=2) + '\n')
    print('Verified 4 passing preflights, 2 bounded obstructions, 0 comparative runs, and diagnostic receipts.')


if __name__ == '__main__':
    audit()
