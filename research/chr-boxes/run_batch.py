"""Execute each registered case/configuration in a fresh release process."""
import argparse
import csv
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('output', type=Path)
args = parser.parse_args()
root = Path(__file__).resolve().parents[2]
binary = root / 'target/release/examples/boxes_probe'
cases = subprocess.run([binary, '--list'], check=True, capture_output=True, text=True).stdout.splitlines()
failed = False
with args.output.open('w', newline='') as output:
    writer = None
    for case in cases:
        for mode in ['0', '1', '8', '64', '256', 'Unlimited']:
            try:
                run = subprocess.run([binary, case, mode], capture_output=True, text=True, timeout=60)
            except subprocess.TimeoutExpired:
                if writer is None:
                    raise
                writer.writerow({'case': case, 'mode': mode, 'pass': 'timeout'})
                output.flush()
                failed = True
                continue
            rows = list(csv.DictReader(run.stdout.splitlines(), delimiter='\t'))
            if len(rows) != 1:
                raise RuntimeError(f'{case} {mode}: missing result: {run.stderr}')
            row = rows[0]
            if writer is None:
                writer = csv.DictWriter(output, fieldnames=list(row), delimiter='\t', lineterminator='\n')
                writer.writeheader()
            writer.writerow(row)
            output.flush()
            if run.returncode or row['pass'] != 'true':
                failed = True
                print(f'{case} {mode}: {run.stderr}', flush=True)
if failed:
    raise SystemExit(1)
