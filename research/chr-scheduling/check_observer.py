"""Registered E09 observer gate; E00 fixtures on stdin, aggregate JSONL out."""
from collections import Counter
from itertools import product
import json
from pathlib import Path
import sys
from observer import EquivalenceJob
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import equivalent, freeze


def check(pairs, label):
    expected = [equivalent(a, b) for a, b in pairs]
    previous = None
    for quantum in (1, 8, 64):
        counts = Counter()
        for (a, b), wanted in zip(pairs, expected):
            job = EquivalenceJob(a, b)
            actions = 0
            while not job.done:
                actions += job.advance(quantum)
                assert actions <= 2_000_000
            assert job.observe() == wanted, (label, a, b, wanted)
            counts.update(job.counts)
        assert previous is None or previous == counts
        previous = counts
        print(json.dumps(dict(group=label, quantum=quantum, comparisons=len(pairs),
                              equivalent=sum(expected), actions=dict(counts), status='pass'), sort_keys=True))


if __name__ == '__main__':
    residuals = tuple((p, (v,)) for p in ('p', 'q') for v in (0, 1))
    observations = [dict(outputs=o, residual=r) for o in ((), (0,), (1,), (0, 1))
                    for r in product(residuals, repeat=2)]
    assert len(observations) == 64
    check(list(product(observations, repeat=2)), 'generated')
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 64
    pairs = [(a, b) for c in cases for a in c['expected'] for b in c['reference']]
    check(pairs, 'applications-and-semantics')
