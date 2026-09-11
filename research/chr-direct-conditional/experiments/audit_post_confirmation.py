"""Audit frozen raw post confirmation and conservative paired median intervals."""
import collections, gzip, hashlib, itertools, json, math, random, statistics, zipfile
from pathlib import Path
ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s03-post-confirmation'
PARENT = ROOT / 'docs/experiments/results/s03-post-control-cost'
FAMILIES = ['post-input', 'post-output', 'post-forward', 'post-miss', 'post-duplicate', 'post-template']
MODES = ['birth', 'birth-miss', 'birth-miss-template', 'scan', 'indexed', 'sealed', 'active-scan', 'active-indexed', 'native-scan', 'native-indexed', 'active-native-scan', 'active-native-indexed']
def read(p): return json.loads(p.read_text())
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def result(raw):
    rows = [json.loads(l) for l in raw['stdout'].splitlines() if l.startswith('{')]
    results = [r for r in rows if r.get('event') == 'result']
    assert len(results) == 1
    return results[0]
def main():
    f = read(BASE / 'freeze.json'); parent = read(PARENT / 'freeze.json')
    for name, key in [('freeze.json', 'parent_freeze_sha256'), ('sources.zip', 'parent_archive_sha256'), ('build-time.json', 'parent_build_sha256'), ('audit.json', 'parent_audit_sha256')]:
        assert sha(PARENT / name) == f[key]
    assert f['binary'] == parent['binaries']['time']
    assert sha(Path(f['binary']['path'])) == f['binary']['sha256']
    assert sha(PARENT / 'sources.zip') == parent['archive_sha256']
    with zipfile.ZipFile(PARENT / 'sources.zip') as archive:
        assert set(archive.namelist()) == set(parent['sources'])
        for p, h in parent['sources'].items(): assert hashlib.sha256(archive.read(p)).hexdigest() == h
    assert set(f['sources']) == set(f['source_text'])
    for p, h in f['sources'].items(): assert hashlib.sha256(f['source_text'][p].encode()).hexdigest() == h
    build = read(PARENT / 'build-time.json'); assert build['exit_code'] == 0
    artifacts = [json.loads(l) for l in build['stdout'].splitlines() if json.loads(l).get('reason') == 'compiler-artifact']
    assert artifacts and any(x.get('executable') == f['binary']['path'] for x in artifacts)
    assert all(not (set(x.get('features', [])) & {'metrics', 'kernel-metrics', 'work-diagnostics', 'candidate-profile', 'alloc-meter'}) for x in artifacts)
    for n, h in f['orders'].items(): assert sha(BASE / f'{n}.json') == h
    assert f['cpu'] == 0 and 0 in f['affinity']
    scenarios = list(itertools.product(FAMILIES, [8, 32], [False, True], ['immediate', 'window', 'all']))
    cells = [[*s, m] for s in scenarios for m in MODES]; assert read(BASE / 'cells.json') == cells == read(PARENT / 'cells.json')
    blocks = list(itertools.product(range(64), range(72))); rng = random.Random(71330); rng.shuffle(blocks); jobs = []
    for block, (rep, index) in enumerate(blocks):
        modes = MODES.copy(); rng.shuffle(modes)
        for pos, mode in enumerate(modes): jobs.append(dict(block=block, rep=rep, index=cells.index([*scenarios[index], mode]), position=pos))
    assert read(BASE / 'jobs.json') == jobs
    warmup = list(range(864)); random.Random(71331).shuffle(warmup); assert read(BASE / 'warmup-order.json') == warmup
    campaign = read(BASE / 'campaign.json'); assert campaign['primary_processes'] == 55296 and campaign['warmups'] == 864 and campaign['seconds'] < 1800
    for n, h in campaign['archives'].items(): assert sha(BASE / f'{n}.jsonl.gz') == h
    clocks = []
    for i in range(5):
        raw = read(BASE / f'clock-{i}.json'); assert raw['exit_code'] == 0 and not raw['stderr'] and not raw['timeout'] and raw['first_clock'] == 'off'
        assert raw['command'] == [f['binary']['path'], 'clock-check']
        r = json.loads(raw['stdout']); assert r['samples'] == 100000; clocks.append(r['median'])
    floor = 100 * max(clocks) * 24
    phases = [('source', 0), ('prepare', 0)] + [(p, q) for q in range(4) for p in ['input', 'setup', 'execute_observe', 'engine_drop', 'consumer']] + [('prepared_drop', 0), ('consumer_drop', 0)]
    endpoints = {}
    for i, j in enumerate(read(PARENT / 'time-order.json')):
        r = result(read(PARENT / 'time' / f'{i}.json'))
        if j['index'] in endpoints: assert endpoints[j['index']] == r['endpoints']
        endpoints[j['index']] = r['endpoints']
    totals = collections.defaultdict(list); phase_times = collections.defaultdict(lambda: collections.defaultdict(list)); paired = {}
    for kind, order in [('warmup', warmup), ('runs', jobs)]:
        count = 0
        with gzip.open(BASE / f'{kind}.jsonl.gz', 'rt') as stream:
            for i, line in enumerate(stream):
                assert i < len(order); item = json.loads(line); job = order[i]; index = job if kind == 'warmup' else job['index']
                assert item['sequence'] == i and item['index'] == index; raw = item['raw']; fam, n, rev, consumer, mode = cells[index]
                assert raw['exit_code'] == 0 and not raw['stderr'] and not raw['timeout'] and raw['first_clock'] == 'off'
                assert raw['command'] == [f['binary']['path'], mode, fam, str(n), str(rev).lower(), consumer, 'false']
                r = result(raw); assert r['meter'] is False and r['endpoints'] == endpoints[index]
                assert len(r['endpoints']) == 4 and all(e['complete'] and e['answers'] == 1 and e['first_ns'] is None for e in r['endpoints'])
                assert [(x['phase'], x['query']) for x in r['phases']] == phases
                assert all(x['reading']['memory'] is None and isinstance(x['reading']['ns'], int) and x['reading']['ns'] >= 0 for x in r['phases'])
                if kind == 'runs':
                    p = collections.Counter()
                    for x in r['phases']: p[x['phase']] += x['reading']['ns']
                    total = sum(p.values()); totals[index].append(total); assert (job['block'], mode) not in paired; paired[job['block'], mode] = total
                    for name, value in p.items(): phase_times[index][name].append(value)
                count += 1
        assert count == len(order)
    parent_summary = read(PARENT / 'audit.json')['summary']; summary = []
    for index, cell in enumerate(cells):
        assert len(totals[index]) == 64 and parent_summary[index]['cell'] == cell
        summary.append(dict(cell=cell, median_ns=statistics.median(totals[index]), min_ns=min(totals[index]), max_ns=max(totals[index]), clock_sensitive=min(totals[index]) < floor, requested_bytes=parent_summary[index]['requested_bytes'], peak_excess=parent_summary[index]['peak_excess'], phase_median_ns={p: statistics.median(v) for p, v in phase_times[index].items()}))
    k = max(k for k in range(1, 33) if sum(math.comb(64, i) for i in range(k)) * 77760 <= 2**64); assert k == 16
    contrasts = []
    for index, scenario in enumerate(scenarios):
        bs = [b for b, (_, s) in enumerate(blocks) if s == index]; assert len(bs) == 64
        for demand, control in itertools.product(MODES[:3], MODES[3:]):
            ratios = [paired[b, demand] / paired[b, control] for b in bs]; ordered = sorted(ratios); lower, upper = ordered[k-1], ordered[64-k]
            sensitive = any(summary[cells.index([*scenario, m])]['clock_sensitive'] for m in [demand, control])
            label = 'instrumentation-sensitive' if sensitive else 'faster' if upper < .9 else 'slower' if lower > 1.1 else 'within-10-percent' if lower >= .9 and upper <= 1.1 else 'unresolved'
            contrasts.append(dict(scenario=scenario, demand=demand, control=control, median_ratio=statistics.median(ratios), min_ratio=min(ratios), max_ratio=max(ratios), interval=[lower, upper], classification=label, ratios=ratios))
    assert len(contrasts) == 1944
    print(json.dumps(dict(cells=864, primary_processes=55296, warmups=864, repetitions=64, paired_blocks=4608, comparisons=1944, interval_ranks=[16,49], clock_medians_ns=clocks, clock_floor_ns=floor, clock_sensitive_cells=sum(x['clock_sensitive'] for x in summary), parent_endpoints_match=True, classification_counts=dict(collections.Counter(x['classification'] for x in contrasts)), summary=summary, contrasts=contrasts), indent=2))
if __name__ == '__main__': main()
