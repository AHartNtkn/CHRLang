#!/usr/bin/env python3
"""Bounded isolated allocation diagnostics; no comparative timing interpretation."""
from pathlib import Path
import hashlib
import json
import resource
import subprocess
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s09-worker-meter-channel'
BINARY = ROOT / 'target/release/examples/worker_meter_gate'

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    summaries = {}
    for mode in ['cross-thread', 'request-map', '0', '1', '2', '4']:
        samples = []
        for repeat in range(5):
            command = [str(BINARY), mode]
            try:
                result = subprocess.run(command, capture_output=True, text=True, timeout=60, preexec_fn=limits)
            except subprocess.TimeoutExpired as error:
                def decode(value):
                    return value.decode(errors='replace') if isinstance(value, bytes) else (value or '')
                receipt = dict(command=command, exit_code=None, cutoff=True,
                               stdout=decode(error.stdout), stderr=decode(error.stderr))
                (OUT / f'{mode}-{repeat}.json').write_text(json.dumps(receipt, indent=2) + '\n')
                raise AssertionError('diagnostic cutoff; no performance conclusion') from error
            receipt = dict(command=command, exit_code=result.returncode, cutoff=False,
                           stdout=result.stdout, stderr=result.stderr)
            (OUT / f'{mode}-{repeat}.json').write_text(json.dumps(receipt, indent=2) + '\n')
            assert result.returncode == 0, receipt
            rows = [json.loads(line) for line in result.stdout.splitlines()]
            if mode in ['cross-thread', 'request-map']:
                continue
            assert rows[1]['phase'] == 'channel-context'
            context = rows[1]['reading']
            assert context['live_end'] - context['live_start'] == 48, context
            phases = rows[2:]
            assert len(phases) == 8 * 23, len(phases)
            for i in range(8):
                cycle = phases[i*23:(i+1)*23]
                assert cycle[0]['phase'] == 'prepare' and cycle[-1]['phase'] == 'runtime-drop'
                assert cycle[0]['reading']['live_start'] == cycle[-1]['reading']['live_end'], cycle
            samples.extend(phases)
        if mode not in ['cross-thread', 'request-map']:
            summaries[mode] = {}
            for phase in sorted({row['phase'] for row in samples}):
                readings = [row['reading'] for row in samples if row['phase'] == phase]
                summaries[mode][phase] = {key: sorted({r[key] for r in readings}) for key in
                    ['allocation_calls', 'requested_bytes', 'deallocation_calls']}
                summaries[mode][phase]['live_delta'] = sorted({r['live_end']-r['live_start'] for r in readings})
        print(mode, '5 processes passed', flush=True)
    (OUT / 'phase-values.json').write_text(json.dumps(summaries, indent=2) + '\n')
    paths = [ROOT/'research/chr-factors/examples/worker_meter_gate.rs',
             ROOT/'research/chr-compiled/experiments/meter.rs',
             ROOT/'research/chr-factors/experiments/reusable_workers.rs',
             ROOT/'research/chr-factors/experiments/reusable_regions.rs',
             Path(__file__).resolve(), BINARY]
    (OUT/'source-binary-hashes.json').write_text(json.dumps({str(p.relative_to(ROOT)):
        hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}, indent=2)+'\n')

if __name__ == '__main__':
    main()
