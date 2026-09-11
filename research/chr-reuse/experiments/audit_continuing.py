"""Audit frozen continuing runs and emit descriptive paired lifecycle costs."""
import collections
import gzip
import hashlib
import json
import statistics
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s08-continuing-lifecycle'
COST = ROOT / 'docs/experiments/results/s08-binding-coverage-cost'

def read(p):
    return json.loads(p.read_text())

def rows(p):
    with gzip.open(p, 'rt') as f:
        return [json.loads(x) for x in f]

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def lifecycle(raw, n, keep):
    assert raw['exit_code'] == 0
    d = json.loads(raw['stdout'])
    assert d['validated']
    records = d['records']
    counts = collections.Counter(x['phase'] for x in records)
    assert counts == dict(prepare=1, setup=2, produce=2*n, pause=2*n,
                          deliver=2*n, export=2*n, consumer=2*n, cancel=2,
                          prepared_drop=1, consumer_drop=1), counts
    ends = [x for x in d['snapshots'] if x['stage'] == 'before_cancel']
    assert len(ends) == 2
    for q, end in enumerate(ends):
        assert (end['query'], end['delivered'], end['queued']) == (q, n, 3)
        assert end['held'] == (n*(q+1) if keep == 'all' else int(keep))
    if d['meter']:
        root = records[0]['reading']['memory']['live_start']
        assert records[-1]['reading']['memory']['live_end'] == root
        for x in records:
            m = x['reading']['memory']
            if x['phase'] == 'pause':
                assert m['requested_bytes'] == m['allocation_calls'] == 0
                assert m['live_start'] == m['live_end']
    return d

def main():
    for name, archive in [('freeze.json', 'sources.zip'), ('coverage-freeze.json', 'coverage-sources.zip')]:
        frozen = read(BASE/name)
        assert sha(BASE/archive) == frozen['archive_sha256']
        with zipfile.ZipFile(BASE/archive) as z:
            for path, expected in frozen['sources'].items():
                assert hashlib.sha256(z.read(path)).hexdigest() == expected
        for binary in frozen['binaries'].values():
            assert sha(Path(binary['path'])) == binary['sha256']
    original = rows(BASE/'qualification.jsonl.gz') + rows(BASE/'remaining.jsonl.gz')
    assert [x['job'] for x in original] == read(BASE/'freeze.json')['jobs']
    outcomes = collections.Counter()
    for r in original:
        j, raw = r['job'], r['raw']
        if raw['exit_code'] == 0:
            lifecycle(raw, j['demand'], 'all'); outcomes['pass'] += 1
        elif raw['exit_code'] == 101:
            assert j['mode'] == 'conditional' and j['resource'] and j['demand'] == 512
            assert 'service' in raw['stderr']; outcomes['service_cutoff'] += 1
        else:
            assert raw['exit_code'] is None and j['mode'] in ['dependencies', 'dependencies-reclaim'] and j['demand'] == 512
            outcomes['wall_timeout'] += 1
    assert outcomes == {'pass': 98, 'service_cutoff': 2, 'wall_timeout': 8}
    base_diag = [read(BASE/f'diagnostic-{i}.json') for i in range(4)]
    assert all(x['exit_code'] == 0 for x in base_diag)
    assert base_diag[2]['stdout'] == base_diag[3]['stdout']
    coverage = rows(BASE/'coverage.jsonl.gz')
    assert [x['job'] for x in coverage] == read(BASE/'coverage-freeze.json')['jobs']
    diag = {}
    for r in coverage:
        j = r['job']; assert r['raw']['exit_code'] == 0
        if j['kind'] == 'diagnostic':
            key = (j['resource'], j['demand'])
            if key in diag: assert diag[key] == r['raw']['stdout']
            diag[key] = r['raw']['stdout']
        else: lifecycle(r['raw'], j['demand'], 'all')
    graph = [read(BASE/f'graph-{i}.json') for i in range(20)]
    assert [x['job'] for x in graph] == read(BASE/'graph-freeze.json')['jobs']
    for i, r in enumerate(graph):
        assert r['raw']['exit_code'] == 0
        if i < 16 and i % 2: assert graph[i-1]['raw']['stdout'] == r['raw']['stdout']
    gf = read(BASE/'graph-freeze.json')
    assert sha(Path(gf['binary']['path'])) == gf['binary']['sha256']
    assert sha(ROOT/'research/chr-reuse/examples/continuing_graph_attribution.rs') == gf['source_sha256']
    # Backend inputs are independently retained in the coverage archive.
    with zipfile.ZipFile(BASE/'coverage-sources.zip') as z:
        for name in z.namelist():
            if name.startswith('research/chr-direct-choice/'):
                assert z.read(name) == (ROOT/name).read_bytes()
    frozen = read(COST/'freeze.json'); runs = rows(COST/'runs.jsonl.gz')
    assert [x['job'] for x in runs] == frozen['jobs']
    for name, value in frozen['parent_sha256'].items(): assert sha(BASE/name) == value
    assert sha(ROOT/'research/chr-reuse/experiments/binding_coverage_cost.py') == frozen['runner_sha256']
    assert sha(ROOT/'docs/experiments/registrations/S08-binding-coverage-cost.md') == frozen['registration_sha256']
    assert collections.Counter(x['job']['stage'] for x in runs) == dict(warmup=32, primary=160, allocation=64, rss=64)
    cells = {}; memory = {}
    for r in runs:
        j = r['job']; resource, n, keep, packing, cover = j['case']
        d = lifecycle(r['raw'], n, keep)
        key = tuple(j['case']); cells[j['stage'], j['rep'], key] = d
        if j['stage'] == 'allocation':
            mm = [x['reading']['memory'] for x in d['records']]
            if key in memory: assert mm == memory[key]
            memory[key] = mm
    summary = []
    for resource in [False, True]:
        for n in [32, 128]:
            for keep in ['0', 'all']:
                for packing in [False, True]:
                    scenario = (resource,n,keep,packing)
                    times = [[sum(x['reading']['ns'] for x in cells['primary',rep,scenario+(cover,)]['records']) for rep in range(5)] for cover in [False,True]]
                    ratios = [b/a for a,b in zip(*times)]
                    heaps = []
                    for cover in [False,True]:
                        mm = memory[scenario+(cover,)]; root = mm[0]['live_start']
                        heaps.append(dict(requested=sum(m['requested_bytes'] for m in mm), peak=max(m['peak_live'] for m in mm)-root, retained=mm[-1]['live_start']-root))
                    summary.append(dict(resource=resource,demand=n,keep=keep,packing=packing,control_ms=statistics.median(times[0])/1e6,coverage_ms=statistics.median(times[1])/1e6,ratio_median=statistics.median(ratios),ratio_range=[min(ratios),max(ratios)],saved_ms_median=statistics.median([a-b for a,b in zip(*times)])/1e6,heap_control=heaps[0],heap_coverage=heaps[1]))
    result = dict(original=dict(outcomes),coverage_processes=len(coverage),graph_processes=len(graph),cost_processes=len(runs),allocation_repeats_exact=True,rows=summary)
    (COST/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))

if __name__ == '__main__':
    main()
