"""Native-choice outcome and label-provenance probes, including faulty encodings."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from continuation_gate import parse_data
from cases import cases, program


def multiset(values):
    return Counter(json.dumps(v, sort_keys=True) for v in values)


def collapse(binary, path, case):
    try:
        result = subprocess.run([str(binary), str(path), '-s', '-C' + str(case['limit'])],
                                text=True, capture_output=True, timeout=0.5 if case['timeout'] else 5)
    except subprocess.TimeoutExpired as error:
        def text(value):
            return value.decode() if isinstance(value, bytes) else value or ''
        return dict(timeout=True, stdout=text(error.stdout), stderr=text(error.stderr), values=None)
    values = []
    errors = []
    for line in result.stdout.splitlines():
        if not line.strip() or line.startswith('- '):
            continue
        try:
            values.append(parse_data(line, annotation=True))
        except ValueError:
            errors.append(line)
    events = []
    stderr_other = []
    for line in result.stderr.splitlines():
        try:
            events.append(json.loads(line))
        except ValueError:
            stderr_other.append(line)
    return dict(timeout=False, exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr,
                values=values, decode_errors=errors, events=events, stderr_other=stderr_other)


def verify_provenance(rows):
    by_id = {r['case']['id']: r for r in rows}
    assert len(by_id) == len(rows) == 21
    for row in rows:
        if row['case']['graph_trace']:
            trace = row['graph_trace']
            assert not trace.get('timeout') and trace['exit_code'] == 0
        for event in row['instrumented'].get('events', []):
            if event['event'] in ('dsu_num', 'ddu_num'):
                assert event['effective'] == event['requested'] & ((1 << 24) - 1)
            if event['event'] == 'dsu_num':
                assert event['requested'] in row['case']['birth_labels']
    def events(name):
        return by_id[name]['instrumented']['events']
    assert any(e['event'] == 'dup_sup' and e['dup_label'] != e['sup_label']
               for e in events('copy-distinct-label'))
    assert any(e['event'] == 'dup_sup' and e['dup_label'] == e['sup_label']
               for e in events('copy-same-label'))
    assert not any(e['event'] == 'dsu_num' for e in events('lambda-copy'))
    assert any(e['event'] == 'dup_lam' and e['introduces_sup'] for e in events('lambda-copy'))
    assert 'DUP-LAM' in by_id['lambda-copy']['graph_trace']['stdout']
    assert any(e['event'] == 'dup_lam' and e['introduces_sup'] and e['label'] == 1
               for e in events('guarded-branch'))
    assert any(e['event'] == 'dsu_num' and e['requested'] == 16777217 and e['effective'] == 1
               for e in events('wrapped-birth-label'))
    for depth in (1, 2, 3):
        assert {e['requested'] for e in events(f'recursive-{depth}') if e['event'] == 'dsu_num'} == set(range(1, depth + 1))


def run(directory, output):
    rows = []
    with tempfile.TemporaryDirectory(prefix='chr-hvm-native-') as scratch:
        for case in cases():
            source = program(case)
            path = Path(scratch) / (case['id'] + '.hvm')
            path.write_text(source)
            baseline = collapse(directory / 'baseline', path, case)
            instrumented = collapse(directory / 'trace', path, case)
            predicted = None if case['prediction'] is None else [parse_data(x) for x in case['prediction']]
            intended = [parse_data(x) for x in case['source_expected']]
            actual = baseline['values']
            if case['timeout']:
                prediction_match = baseline['timeout'] and instrumented['timeout']
                source_match = raw_match = None
                instrumentation_match = baseline['timeout'] == instrumented['timeout']
            else:
                well_formed = all(not r['timeout'] and r['exit_code'] == 0 and not r['decode_errors']
                                  and not r['stderr_other'] for r in (baseline, instrumented))
                prediction_match = well_formed and multiset(actual) == multiset(predicted)
                instrumentation_match = well_formed and multiset(actual) == multiset(instrumented['values'])
                source_match = well_formed and set(multiset(actual)) == set(multiset(intended))
                raw_match = well_formed and multiset(actual) == multiset(intended)
            row = dict(case=case, source=source, source_sha256=hashlib.sha256(source.encode()).hexdigest(),
                       baseline=baseline, instrumented=instrumented, prediction_match=prediction_match,
                       instrumentation_match=instrumentation_match, source_set_match=source_match,
                       source_multiplicity_match=raw_match)
            if case['graph_trace']:
                try:
                    trace = subprocess.run([str(directory / 'trace'), str(path), '-D'],
                                           text=True, capture_output=True, timeout=5)
                    row['graph_trace'] = dict(exit_code=trace.returncode, stdout=trace.stdout, stderr=trace.stderr)
                except subprocess.TimeoutExpired:
                    row['graph_trace'] = dict(timeout=True)
            rows.append(row)
    output.write_text(''.join(json.dumps(row, sort_keys=True) + '\n' for row in rows))
    verify_provenance(rows)
    return all(r['prediction_match'] and r['instrumentation_match'] for r in rows)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('build_directory', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    if not run(a.build_directory.resolve(), a.output):
        raise SystemExit('native probe differs from prediction; inspect retained evidence')
