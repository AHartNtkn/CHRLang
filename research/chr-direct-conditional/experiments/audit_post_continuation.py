"""Reconstruct the mixed-source work gate and verify its frozen inputs."""
import hashlib
import json
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s10-post-continuation'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    freeze = json.loads((BASE / 'freeze.json').read_text())
    for name, digest in freeze['sources'].items():
        assert sha(ROOT / name) == digest, name
    traces = []
    for name in ['final-off.log', 'final-work.log']:
        data = (BASE / name).read_text()
        assert 'test result: ok. 1 passed;' in data
        assert 'test result: ok. 2 passed;' in data
        assert 'FAILED' not in data
        cases = re.findall(r'^CASE,([^\n]+)', data, re.M)
        assert len(cases) == len(set(cases)) == 216
        rows = re.findall(r'^(WORK|COUNTED),([^\n]+)', data, re.M)
        assert len(rows) == 432
        for kind, row in rows:
            family, k, d, history, reverse, seed, explicit, conditional = row.split(',')
            k, d = int(k), int(d)
            assert history in ['true', 'false'] and reverse in ['true', 'false']
            assert seed in ['0', '1']
            count = 2 ** (k - 1) if family == 'early' and k else 2 ** k
            extra = k * 2 ** (k - 1) if family == 'independent' and k else 0
            want = extra if kind == 'COUNTED' else count * d + extra
            assert int(explicit.split('=')[1]) == want
            want_conditional = extra if kind == 'COUNTED' else want if family == 'independent' else d
            assert int(conditional.split('=')[1]) == want_conditional
        traces.append((cases, rows))
    assert traces[0] == traces[1]
    for name, count in [('nonground.log', 11), ('unit.log', 11)]:
        assert f'test result: ok. {count} passed;' in (BASE / name).read_text()
    result = {'source_configurations_per_build': 216,
              'complete_executions_per_build': 5184,
              'cancellation_probes_per_build': 4320,
              'exact_work_rows_per_build': 432,
              'ordinary_timing_runs': 0,
              'source_sha256': freeze['sources'],
              'logs_sha256': {p.name: sha(p) for p in BASE.glob('*.log')}}
    (BASE / 'audit.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({k: v for k, v in result.items() if not k.endswith('sha256')}))

if __name__ == '__main__':
    main()
