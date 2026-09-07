"""Source/net interface check using separately exported Rust reference requests."""
import json
from pathlib import Path
import sys
from scheduler import Job
from net_bridge import NetEquality
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'chr-symbolic'))
from check_cases import freeze, equivalent
from machine import resolve


if __name__ == '__main__':
    cases = [freeze(json.loads(line)) for line in sys.stdin]
    assert len(cases) == 432
    service = NetEquality(('a','b','f','p'), status=sys.argv[1] if len(sys.argv)>1 else 'scan')
    for case in cases:
        previous = None
        for quantum in (1,8,64):
            original = case['initial']
            job = Job(service.solve(original, (case['equation'],)))
            actions = 0
            while not job.done:
                actions += job.advance(quantum)
                assert actions <= 2_000_000, (case['id'], 'action cap')
            result = job.observe()
            assert (result is not None) == bool(case['answers']), (case['id'], 'success')
            if result is not None:
                for key, value in result:
                    assert 0 <= key < 3, 'unexpected source variable ID'
                    pending = [value]
                    while pending:
                        term = pending.pop()
                        if isinstance(term, int):
                            assert 0 <= term < 3, 'unexpected source variable ID'
                        else:
                            pending.extend(term[1])
                env = dict(result)
                actual = dict(outputs=[resolve(v, env) for v in range(3)], residual=[])
                assert equivalent(actual, dict(outputs=case['answers'][0], residual=[])), case['id']
            row = dict(id=case['id'], quantum=quantum, success=result is not None,
                       actions=dict(job.counts), result=result, status='pass')
            invariant = {k:v for k,v in row.items() if k != 'quantum'}
            assert previous is None or previous == invariant
            previous = invariant
            print(json.dumps(row, sort_keys=True), flush=True)
