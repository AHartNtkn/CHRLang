"""Validate the registered matrix and emit all summary statistics."""
from collections import defaultdict
import json
from pathlib import Path
from statistics import median
import sys


def key(row):
    return row['id'], row['policy'], row['grouping'], row['mode']


def interval(values):
    return dict(median=median(values), min=min(values), max=max(values))


def summarize(timing, memory):
    assert len(timing) == 1350 and len(memory) == 450
    assert all(r['warmup'] for r in timing[:225])
    assert not any(r['warmup'] for r in timing[225:])
    assert len({key(r) for r in timing[:225]}) == 225
    groups = defaultdict(list)
    for row in timing + memory:
        assert row['status'] == 'pass'
        groups[key(row)].append(row)
    assert len(groups) == 225
    for config, rows in sorted(groups.items()):
        times = [r for r in rows if not r['traced'] and not r['warmup']]
        memories = [r for r in rows if r['traced']]
        assert len(times) == 5 and len(memories) == 2
        assert {r['repeat'] for r in times} == set(range(1,6))
        assert {r['repeat'] for r in memories} == {0,1}
        for field in ('actions', 'answers', 'answer_actions', 'raw', 'unique', 'source_jobs', 'prefix', 'output_bytes'):
            assert all(r[field] == rows[0][field] for r in rows), (config, field)
        yield dict(id=config[0], policy=config[1], grouping=config[2], mode=config[3],
                   timings_ns={field: interval([r['timings_ns'][field] for r in times])
                               for field in times[0]['timings_ns'] if times[0]['timings_ns'][field] is not None},
                   process_ns=interval([r['process_ns'] for r in times]),
                   memory=[r['memory'] for r in memories], rss_kib=[r['rss_kib'] for r in memories],
                   raw=rows[0]['raw'], unique=rows[0]['unique'], prefix=rows[0]['prefix'],
                   source_jobs=rows[0]['source_jobs'], actions=rows[0]['actions'],
                   output_bytes=rows[0]['output_bytes'], status='pass')


if __name__ == '__main__':
    timing, memory = ([json.loads(line) for line in Path(p).read_text().splitlines()] for p in sys.argv[1:])
    for row in summarize(timing, memory):
        print(json.dumps(row, sort_keys=True))
