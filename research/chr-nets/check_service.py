"""Whole-request yield gate over independently expected E06 workloads."""
import argparse
import json
from pathlib import Path
from check_unification import canonical, encoded, listing, items, number
from direct_unification import decoded, extract_tables
from measure import nat
from net import data_system
from service import UnificationJob
from unification_workloads import cases


def run(output):
    system = data_system()
    all_passed = True
    with output.open('w') as out:
        for case in cases():
            table = listing([('Pair', (nat(k), encoded(v))) for k, v in case['initial']])
            pending = listing([('Pair', (encoded(a), encoded(b))) for a, b in case['equations']])
            expected = [canonical(a) for a in case['expected']]
            baseline = None
            for quantum in (1, 8, 64):
                job = UnificationJob(table, pending, system)
                actions = resumes = 0
                while not job.done and actions < 250000:
                    limit = min(quantum, 250000 - actions)
                    used = job.advance(limit)
                    assert 0 < used <= limit
                    actions += used
                    resumes += 1
                observed = None
                passed = False
                if job.done:
                    before, option = job.observe()
                    bindings = None if option[0] == 'None' else [
                        [number(p[1][0]), decoded(p[1][1])] for p in items(option[1][0])]
                    _, result, values = extract_tables(case['initial'], bindings, case['selected'])
                    observed = [] if result is None else [canonical(values)]
                    passed = before == table and observed == expected
                    job.net.check()
                counts = job.counts.copy()
                assert actions == sum(counts.values())
                if baseline is None:
                    baseline = counts
                assert baseline == counts, case['id']
                all_passed &= passed
                out.write(json.dumps(dict(id=case['id'], quantum=quantum, done=job.done,
                                          passed=passed, counts=counts, resumes=resumes,
                                          observed=observed, interactions=job.net.interactions), sort_keys=True) + '\n')
    return all_passed


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('output', type=Path)
    a = p.parse_args()
    if not run(a.output):
        raise SystemExit('whole-service yield mismatch; inspect outcomes')
