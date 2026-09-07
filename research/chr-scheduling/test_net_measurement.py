"""Session reset and oracle checks exercise actual measurement subprocesses."""
import json
from pathlib import Path
import subprocess
import sys
import unittest


class NetMeasurement(unittest.TestCase):
    def test_session_warmups_precede_measured_repetitions(self):
        from run_net_measurements import ordered_jobs
        configs = [('case-a', 'direct', 'async', False), ('case-b', 'count', 'round', True)]
        jobs = ordered_jobs(configs, False)
        self.assertEqual(len(jobs), 12)
        self.assertEqual([j[-1] for j in jobs[:2]], [0, 0])
        self.assertTrue(all(j[-1] > 0 for j in jobs[2:]))
        self.assertEqual(set(jobs), {c+(r,) for c in configs for r in range(6)})
        self.assertEqual(jobs, ordered_jobs(configs, False))

    def test_validation_occurs_after_release_and_is_collected_before_next_query(self):
        script = """
import json, sys, weakref
sys.path.insert(0, sys.argv[1])
import measure_net as m
original_search, original_equivalent = m.Search, m.equivalent
searches, garbage = [], []
class Cycle:
    def __init__(self): self.link = self

def tracked_search(*args, **kwargs):
    assert all(ref() is None for ref in garbage), 'validator garbage reached next query'
    value = original_search(*args, **kwargs)
    searches.append(weakref.ref(value))
    return value

def checked_equivalent(a, b):
    assert searches[-1]() is None, 'oracle ran before search release'
    value = Cycle()
    garbage.append(weakref.ref(value))
    return original_equivalent(a,b)

m.Search, m.equivalent = tracked_search, checked_equivalent
case = dict(id='release-boundary', rules=[], constraints=[['p',[0]]], outputs=[0],
            prefix=False, raw=1, expected=[dict(outputs=[0], residual=[['p',[0]]])])
m.measure(dict(case_json=json.dumps(case), service='direct', policy='async',
               grouping=False, traced=True, queries=3))
assert len(searches) == 3 and all(ref() is None for ref in garbage)
"""
        child = subprocess.run([sys.executable, '-c', script, str(Path(__file__).resolve().parent)],
                               text=True, capture_output=True, timeout=30)
        self.assertEqual(child.returncode, 0, child.stderr)

    def test_prepared_session_repeats_private_search_with_residual_alias(self):
        case = dict(id='session-alias', rules=[dict(kept=[], guards=[], removed=[['start',[0]]],
                    body=['and',['eq',0,['a',[]]],['post',['left',[1]]],['post',['right',[1]]]])],
                    constraints=[['start',[0]]], outputs=[0], prefix=False, raw=1,
                    expected=[dict(outputs=[['a',[]]], residual=[['left',[0]],['right',[0]]])])
        for service in ('direct','scan','count'):
            with self.subTest(service=service):
                request = dict(case_json=json.dumps(case), service=service, policy='async',
                               grouping=True, queries=3, traced=True)
                child = subprocess.run([sys.executable, str(Path(__file__).with_name('measure_net.py'))],
                                       input=json.dumps(request), text=True, capture_output=True, timeout=30)
                self.assertEqual(child.returncode, 0, child.stderr)
                row = json.loads(child.stdout)
                self.assertEqual(len(row['queries']), 3)
                self.assertEqual(len(set(row['outputs'])), 1)
                self.assertEqual(row['queries'][0]['actions'], row['queries'][2]['actions'])
                self.assertEqual(row['queries'][0]['raw'], 1)
                self.assertIn('2.released', row['memory'])
                self.assertGreater(row['queries'][0]['timings_ns']['first_answer'], 0)


if __name__ == '__main__':
    unittest.main()
