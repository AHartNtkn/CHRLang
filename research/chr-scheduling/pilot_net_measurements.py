"""Exploratory resource pilot; retain errors and continue independent cells."""
import json
from pathlib import Path
import subprocess
import sys
import time
from run_measurements import cases


if __name__ == '__main__':
    selected = [c for c in cases(sys.argv[1]) if c['id'] in ('opaque-64', 'app-sk-ignored-hole')]
    assert len(selected) == 2, [c['id'] for c in cases(sys.argv[1])]
    for traced in (False, True):
        for case in selected:
            for service in ('direct', 'scan', 'count'):
                request = dict(case_json=json.dumps(case), service=service, policy='async',
                               grouping=False, queries=1, traced=traced)
                start = time.perf_counter_ns()
                try:
                    child = subprocess.run([sys.executable, str(Path(__file__).with_name('measure_net.py'))],
                                           input=json.dumps(request), text=True, capture_output=True, timeout=600)
                    row = json.loads(child.stdout) if child.returncode == 0 else dict(
                        status='error', stderr=child.stderr, returncode=child.returncode)
                except subprocess.TimeoutExpired:
                    row = dict(status='timeout')
                row.update(id=case['id'], service=service, traced=traced,
                           process_ns=time.perf_counter_ns()-start, exploratory=True)
                print(json.dumps(row, sort_keys=True), flush=True)
