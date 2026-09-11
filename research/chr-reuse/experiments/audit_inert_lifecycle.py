"""Verify frozen lifecycle receipts and summarize only completed measurements."""
import collections
import gzip
import hashlib
import json
from pathlib import Path
import statistics
import zipfile

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s05-inert-lifecycle'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    freeze = json.loads((BASE / 'freeze.json').read_text())
    assert sha(BASE / 'jobs.json') == freeze['jobs_sha256']
    assert sha(BASE / 'sources.zip') == freeze['archive_sha256']
    with zipfile.ZipFile(BASE / 'sources.zip') as archive:
        for path, digest in freeze['sources'].items():
            assert hashlib.sha256(archive.read(path)).hexdigest() == digest
            assert sha(ROOT / path) == digest, path
    for binary in freeze['binaries'].values():
        assert sha(Path(binary['path'])) == binary['sha256']
    jobs = json.loads((BASE / 'jobs.json').read_text())
    clocks = json.loads((BASE / 'clocks.json').read_text())
    floor = max(json.loads(c['stdout'])['median_ns'] for c in clocks)
    groups = collections.defaultdict(list)
    failed = []
    skipped = []
    failure_counts = collections.Counter()
    seen = 0
    with gzip.open(BASE / 'runs.jsonl.gz', 'rt') as stream:
        for index, line in enumerate(stream):
            raw = json.loads(line)
            assert raw['index'] == index
            seen += 1
            job = jobs[index]
            key = (job['kind'], tuple(job['args']))
            if 'skipped' in raw:
                assert failure_counts[key] >= 2
                skipped.append(index)
                continue
            assert raw['command'] == [freeze['binaries'][job['kind']]['path'], *job['args']]
            if raw['exit_code'] != 0:
                failure_counts[key] += 1
                failed.append(dict(index=index, job=job, exit_code=raw['exit_code'],
                                   timeout=raw['timeout'], stderr=raw['stderr'][-500:]))
                continue
            assert not raw['timeout'] and not raw['stderr']
            data = [json.loads(line) for line in raw['stdout'].splitlines()]
            summary, rows = data[0], data[1:]
            mode, family, depth, reuse, keep, cancel, distinct = job['args']
            assert cancel == '0'
            n = 1 if family == '5' else 2
            reuse = int(reuse)
            assert summary['counts'] == [n] * reuse
            retained = 0 if keep == '0' else 1 if keep == '1' else n * reuse
            assert summary['retained'] == retained
            expected = [('source', 0), ('prepare', 0), ('source_dispose', 0)]
            for q in range(reuse):
                expected += [('input', q), ('setup', q)]
                expected += [('service_observe', q), ('consume', q)] * n
                expected += [('service_observe', q), ('engine_dispose', q), ('input_dispose', q)]
            expected += [('prepared_dispose', reuse), ('consumer_dispose', reuse)]
            assert [(r['phase'], r['query']) for r in rows] == expected
            if job['kind'] == 'meter':
                assert summary['unreleased_bytes'] == 0
                baseline = rows[0]['memory']['live_start']
                assert rows[-1]['memory']['live_end'] == baseline
                assert summary['requested_bytes'] == sum(r['memory']['requested_bytes'] for r in rows)
                assert summary['consumer_bytes'] == rows[-1]['memory']['live_start'] - baseline
                for r in rows:
                    assert r['ns'] is None
                    m = r['memory']
                    assert m['peak_live'] >= max(m['live_start'], m['live_end'])
                    for field in ['live_start', 'live_end', 'peak_live']:
                        m[field] -= baseline
                if keep == '0':
                    assert all(r['memory']['live_end'] == rows[2]['memory']['live_end']
                               for r in rows if r['phase'] == 'input_dispose')
                value = dict(summary=summary, rows=rows)
            else:
                assert all(summary[k] is None for k in ['consumer_bytes', 'requested_bytes', 'unreleased_bytes'])
                assert all(r['memory'] is None and isinstance(r['ns'], int) and r['ns'] >= 0 for r in rows)
                phases = collections.Counter()
                for r in rows:
                    phases[r['phase']] += r['ns']
                first = [next(r['ns'] for r in rows if r['query'] == q and r['phase'] == 'service_observe')
                         for q in range(reuse)]
                total = sum(phases.values())
                value = dict(rep=job['rep'], total_ns=total, phases=dict(phases),
                             first_ns=first, sensitive=total < 100 * floor * len(rows))
            groups[key].append(value)
    cells = []
    for (kind, args), values in sorted(groups.items()):
        if kind == 'meter':
            assert len(values) == 2 and values[0] == values[1], (kind, args)
            v = values[0]
            cell = dict(kind=kind, args=args, requested_bytes=v['summary']['requested_bytes'],
                        peak_excess=max(r['memory']['peak_live'] for r in v['rows']),
                        consumer_bytes=v['summary']['consumer_bytes'])
        else:
            assert len(values) == 5, (kind, args, len(values))
            cell = dict(kind=kind, args=args,
                        median_ns=statistics.median(v['total_ns'] for v in values),
                        min_ns=min(v['total_ns'] for v in values), max_ns=max(v['total_ns'] for v in values),
                        sensitive=any(v['sensitive'] for v in values),
                        phases={p: statistics.median(v['phases'][p] for v in values) for p in values[0]['phases']},
                        first_ns=statistics.median(v['first_ns'][0] for v in values), samples=values)
        cells.append(cell)
    assert seen == len(jobs), (seen, len(jobs))
    assert len(jobs) == 20160
    out = dict(scheduled=len(jobs), recorded=seen, successful=sum(len(v) for v in groups.values()),
               failures=failed, skipped=skipped, clock_median_max_ns=floor,
               cells=cells)
    (BASE / 'audit.json').write_text(json.dumps(out, indent=2))
    print(json.dumps({k: v for k, v in out.items() if k not in ['cells', 'failures', 'skipped']}))
    print('failures', len(failed), 'skipped', len(skipped), 'cells', len(cells))


if __name__ == '__main__':
    main()
