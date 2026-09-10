"""Replay frozen common-source schedules; timings are qualification records only."""
import hashlib
import json
import resource
import subprocess
from pathlib import Path
from build import ROOT, OUT, BUILD
from check import check

PRIOR = ROOT / 'docs/experiments/results/s10-native-prepared'

def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (96 << 30, 96 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (10, 10))

def main():
    plans = json.loads((PRIOR / 'plans.json').read_text())
    expected = [json.loads(line) for line in (PRIOR / 'runs.jsonl').read_text().splitlines()]
    counts = {}
    for name in ['ordinary', 'ubsan']:
        count = 0
        with (OUT / f'{name}.jsonl').open('w') as log:
            for plan, old in zip(plans, expected):
                command = [str(BUILD / name), str(ROOT / plan['program'])]
                p = subprocess.run(command, input=plan['input'], capture_output=True, text=True, timeout=15, preexec_fn=bounds)
                row = dict(group=plan['group'], code=p.returncode, stdout=p.stdout, stderr=p.stderr)
                log.write(json.dumps(row) + '\n'); log.flush()
                check(plan, row, old)
                count += len(plan['queries'])
                print(name, 'group', plan['group'], 'passes', flush=True)
        counts[name] = count
    groups = [0, 21, 0]
    command = [str(BUILD / 'ordinary'), '--sessions'] + [str(ROOT / plans[g]['program']) for g in groups]
    p = subprocess.run(command, input=''.join(plans[g]['input'] for g in groups), capture_output=True, text=True, timeout=15, preexec_fn=bounds)
    row = dict(groups=groups, code=p.returncode, stdout=p.stdout, stderr=p.stderr)
    (OUT / 'sessions.json').write_text(json.dumps(row, indent=2) + '\n')
    assert p.returncode == 0, p.stderr
    parts = p.stdout.split('SESSION '); assert len(parts) == 4 and parts[0] == ''
    lines = p.stderr.splitlines(); offset = 0
    for i, (part, g) in enumerate(zip(parts[1:], groups)):
        number, body = part.split('\n', 1); assert int(number) == i
        n = len(plans[g]['queries']) + 1
        check(plans[g], dict(code=0, stdout=body, stderr='\n'.join(lines[offset:offset+n])), expected[g])
        offset += n
    assert offset == len(lines)
    counts['session_queries'] = sum(len(plans[g]['queries']) for g in groups)
    paths = list(Path(__file__).parent.glob('*.py')) + [Path(__file__).with_name('harness.c')]
    paths += [BUILD / x for x in ['ordinary', 'ubsan', 'native.c', 'harness.c']]
    paths += [PRIOR / x for x in ['plans.json', 'runs.jsonl', 'validation.json']]
    paths += [OUT / x for x in ['ordinary.jsonl', 'ubsan.jsonl', 'sessions.json', 'build.json']]
    paths += [ROOT / 'docs/experiments/registrations/S10-native-timing-entry.md']
    (OUT / 'validation.json').write_text(json.dumps(dict(counts=counts, hashes={str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}), indent=2) + '\n')
    print(counts)

if __name__ == '__main__':
    main()
