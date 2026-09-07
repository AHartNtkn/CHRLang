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
