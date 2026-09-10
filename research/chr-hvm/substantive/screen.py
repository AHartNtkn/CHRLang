"""Retain every outcome; qualify substantive source semantics before cost selection."""
import collections
import importlib.util
import json
import resource
import subprocess
import sys
from pathlib import Path
sys.dont_write_bytecode = True
from cases import cases
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s10-native-substantive'

def load(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT / path)
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m

prepared = load('prepared_gate', 'research/chr-hvm/prepared/gate.py')
common = load('common_gate', 'research/chr-hvm/common_source/gate.py')

def native_bounds():
    resource.setrlimit(resource.RLIMIT_AS, (96 << 30, 96 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (10, 10))

def invoke(command, text, bounds):
    try:
        p = subprocess.run(list(map(str, command)), input=text, text=True, capture_output=True,
                           timeout=15, preexec_fn=bounds)
        return dict(code=p.returncode, stdout=p.stdout, stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        def decode(x): return x.decode() if isinstance(x, bytes) else x or ''
        return dict(timeout=True, stdout=decode(e.stdout), stderr=decode(e.stderr))

def main():
    sources = list(cases()); assert len(sources) == 96
    (OUT / 'cases.json').write_text(json.dumps(sources, indent=2) + '\n')
    refs = []
    for s in sources:
        result = prepared.choice_gate.reference(s)
        refs.append(dict(case=s['name'], result=result))
    (OUT / 'references.json').write_text(json.dumps(refs, indent=2) + '\n')
    print('All 96 independent source expectations agree with reference.', flush=True)
    groups = {}
    for s, ref in zip(sources, refs):
        groups.setdefault(json.dumps(s['rules'], sort_keys=True), []).append(dict(s, input=ref['result']['input']))
    groups = list(groups.values()); assert len(groups) == 4
    (OUT / 'groups.json').write_text(json.dumps(groups, indent=2) + '\n')
    outcomes = []
    with (OUT / 'rust.jsonl').open('w') as log:
        for g, group in enumerate(groups):
            text = group[0]['input'].rstrip('\n')
            for s in group[1:]: text += '\nNEXT\n' + '\n'.join(s['input'].splitlines()[:2])
            for mode, label in enumerate(common.MODES):
                r = invoke([common.BINARY, mode], text + '\n', common.bounds)
                log.write(json.dumps(dict(group=g, mode=label, result=r)) + '\n'); log.flush()
                status = 'process-failure'
                if r.get('code') == 0:
                    if r['stdout'].startswith('UNSUPPORTED '):
                        assert label in ['prefix', 'finite']; status = 'unsupported'
                    else:
                        actual = common.parse(r['stdout']); assert len(actual) == len(group)
                        status = 'pass' if all(a['exhausted'] and sorted(map(common.normalize, a['answers'])) == sorted(map(common.normalize, s['expected'])) for a, s in zip(actual, group)) else 'mismatch'
                outcomes.append(dict(engine=label, group=g, queries=len(group), status=status))
                print('Rust', g, label, status, flush=True)
    with (OUT / 'native.jsonl').open('w') as log:
        for g, group in enumerate(groups):
            program, preds, atoms = prepared.compile_source(dict(rules=group[0]['rules'], query=[], outputs=[]))
            path = OUT / f'rules-{g}.hvm'; path.write_text(program)
            for s in group:
                line, ps, ats = prepared.encode(s, preds, atoms, 65536, True)
                for mode, binary in [('ownership', ROOT / 'target/s10-native-prepared/prepared'), ('ordinary', ROOT / 'target/s10-native-timing-entry/ordinary')]:
                    r = invoke([binary, path], '1\n' + line + '\n', native_bounds)
                    log.write(json.dumps(dict(case=s['name'], group=g, mode=mode, result=r)) + '\n'); log.flush()
                    status = 'process-failure'; calls = None
                    if r.get('code') == 0:
                        values = prepared.output_records(r['stdout']); assert set(values) == {0}
                        answers = [prepared.normalize(prepared.parse(l, ps, ats)) for l in values[0].splitlines()]
                        expected = list(map(prepared.normalize, s['expected']))
                        event = json.loads(r['stderr'].splitlines()[0]); calls = event['calls']
                        subset = not (collections.Counter(answers) - collections.Counter(expected))
                        if event['unsupported'] or not subset: status = 'mismatch'
                        elif event['pending']: status = 'cutoff'
                        else: status = 'pass' if sorted(answers) == sorted(expected) else 'mismatch'
                    outcomes.append(dict(engine=mode, case=s['name'], status=status, calls=calls))
            print('Native group', g, 'recorded', flush=True)
    (OUT / 'outcomes.json').write_text(json.dumps(outcomes, indent=2) + '\n')
    print(collections.Counter((r['engine'], r['status']) for r in outcomes))

if __name__ == '__main__': main()
