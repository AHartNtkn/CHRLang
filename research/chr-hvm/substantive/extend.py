"""Bound-only extension with exact initial-prefix and changing-query replays."""
import hashlib
import json
import subprocess
from pathlib import Path
from screen import ROOT, OUT, prepared, invoke, native_bounds

BUILD = ROOT / 'target/s10-native-substantive'

def fields(row):
    return [{k: e[k] for k in ['query', 'calls', 'pending', 'unsupported', 'dynamic_words', 'retained_bytes']}
            for e in [json.loads(l) for l in row['stderr'].splitlines()][:-1]]

def build():
    records = []
    for mode, old in [('ownership', 's10-native-prepared'), ('ordinary', 's10-native-timing-entry')]:
        path = BUILD / mode; path.mkdir(parents=True, exist_ok=True)
        source = (ROOT / 'target' / old / 'harness.c').read_text()
        assert source.count('*limit<=65536') == 1
        (path / 'harness.c').write_text(source.replace('*limit<=65536', '*limit<=1048576'))
        (path / 'native.c').write_bytes((ROOT / 'target' / old / 'native.c').read_bytes())
        command = ['clang', '-O2', '-Wall', str(path / 'harness.c'), '-o', str(path / 'run')]
        p = subprocess.run(command, capture_output=True, text=True, timeout=60)
        records.append(dict(mode=mode, command=command, code=p.returncode, stdout=p.stdout, stderr=p.stderr))
        (OUT / 'extension-build.json').write_text(json.dumps(records, indent=2) + '\n')
        assert p.returncode == 0, p.stderr

def main():
    build()
    groups = json.loads((OUT / 'groups.json').read_text())
    initial = {(r['mode'], r['case']): r['result'] for r in map(json.loads, (OUT / 'native.jsonl').read_text().splitlines())}
    extended = {}; prefix_checks = 0
    with (OUT / 'extension.jsonl').open('w') as log, (OUT / 'prefix-replays.jsonl').open('w') as prefixes:
        for g, group in enumerate(groups):
            _, preds, atoms = prepared.compile_source(dict(rules=group[0]['rules'], query=[], outputs=[]))
            for s in group:
                for mode in ['ownership', 'ordinary']:
                    line, ps, ats = prepared.encode(s, preds, atoms, 1048576, True)
                    result = invoke([BUILD / mode / 'run', OUT / f'rules-{g}.hvm'], '1\n' + line + '\n', native_bounds)
                    log.write(json.dumps(dict(group=g, case=s['name'], mode=mode, result=result)) + '\n'); log.flush()
                    assert result.get('code') == 0, (s['name'], mode, result)
                    event = fields(result)[0]
                    values = prepared.output_records(result['stdout']); assert set(values) == {0}
                    actual = sorted(prepared.normalize(prepared.parse(l, ps, ats)) for l in values[0].splitlines())
                    assert not event['pending'] and not event['unsupported'], (s['name'], event)
                    assert actual == sorted(map(prepared.normalize, s['expected'])), s['name']
                    old = initial[mode, s['name']]
                    if fields(old)[0]['pending']:
                        line, _, _ = prepared.encode(s, preds, atoms, 65536, True)
                        replay = invoke([BUILD / mode / 'run', OUT / f'rules-{g}.hvm'], '1\n' + line + '\n', native_bounds)
                        prefixes.write(json.dumps(dict(case=s['name'], mode=mode, result=replay)) + '\n'); prefixes.flush()
                        assert replay.get('code') == 0 and replay['stdout'] == old['stdout'] and fields(replay) == fields(old)
                        prefix_checks += 1
                    else:
                        assert result['stdout'] == old['stdout'] and fields(result) == fields(old)
                    extended[mode, s['name']] = result
            print('Extended group', g, 'passes', flush=True)
    grouped = 0
    with (OUT / 'reuse.jsonl').open('w') as log:
        for g, group in enumerate(groups):
            _, preds, atoms = prepared.compile_source(dict(rules=group[0]['rules'], query=[], outputs=[]))
            lines = [prepared.encode(s, preds, atoms, 1048576, True)[0] for s in group]
            for mode in ['ownership', 'ordinary']:
                result = invoke([BUILD / mode / 'run', OUT / f'rules-{g}.hvm'], str(len(lines)) + '\n' + '\n'.join(lines) + '\n', native_bounds)
                log.write(json.dumps(dict(group=g, mode=mode, result=result)) + '\n'); log.flush()
                assert result.get('code') == 0, result
                values = prepared.output_records(result['stdout']); events = fields(result)
                assert set(values) == set(range(len(group))) and len(events) == len(group)
                for i, s in enumerate(group):
                    old = extended[mode, s['name']]
                    assert values[i] == prepared.output_records(old['stdout'])[0]
                    expected = dict(fields(old)[0], query=i); assert events[i] == expected
                    grouped += 1
                if mode == 'ownership':
                    end = json.loads(result['stderr'].splitlines()[-1]); assert end['tracked_live'] == 0 and end['consumer_survives_prepared_drop']
    paths = list(Path(__file__).parent.glob('*.py'))
    paths += [ROOT / 'target/debug/examples/native_choice_reference', ROOT / 'target/debug/examples/native_common_source']
    paths += [p for p in BUILD.rglob('*') if p.is_file()]
    paths += list(OUT.glob('*.hvm'))
    paths += [OUT / n for n in ['cases.json', 'groups.json', 'references.json', 'native.jsonl', 'rust.jsonl', 'outcomes.json', 'extension.jsonl', 'prefix-replays.jsonl', 'reuse.jsonl', 'extension-build.json']]
    paths += [ROOT / 'docs/experiments/registrations/S10-native-substantive.md']
    (OUT / 'validation.json').write_text(json.dumps(dict(extended_queries=len(extended), prefix_replays=prefix_checks, reuse_queries=grouped, hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}), indent=2) + '\n')
    print('Extended queries', len(extended), 'prefix checks', prefix_checks, 'reuse queries', grouped)

if __name__ == '__main__': main()
