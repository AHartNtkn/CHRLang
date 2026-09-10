"""Registered resource-phase ownership qualification; clocks are not compared."""
import hashlib
import itertools
import json
import resource
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / 'docs/experiments/results/s06-capacity-phase-ownership'
BIN = ROOT / 'target/s06-capacity-phase-ownership'
MODES = ['phase', 'scan', 'index', 'specialized-scan', 'specialized-index']
CONFIGS = [(m,n,s,1,r,h,c,'complete') for m,n,s,r,h,c in itertools.product(
    MODES, [2,4], ['empty','tight','spare'], [1,4], ['need','token'], ['take','later'])]
CONFIGS += [(m,0,'empty',1,1,h,c,'complete') for m,h,c in itertools.product(MODES,['need','token'],['take','later'])]
CONFIGS += [(m,4,'spare',2,4,h,c,stop) for stop,m,h,c in itertools.product(['complete','first'],MODES,['need','token'],['take','later'])]
assert len(CONFIGS) == len(set(CONFIGS)) == 300


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))


def expected(config):
    _, n, supply, weight, reuse, _, caller, stop = config
    caps = {'empty': (0,0), 'tight': (n//2,n-n//2), 'spare': (n,n)}[supply]
    counts = []
    for query in range(reuse):
        valid = 0
        for values in itertools.product([0,1], repeat=n):
            if query % 2 and len(set(values)) > 1:
                continue
            if all(values.count(i) <= caps[i] for i in [0,1]):
                valid += 1
        count = valid * weight**n * (weight if caller == 'later' else 1)
        counts.append(min(1,count) if stop == 'first' and query == 0 else count)
    return counts


def signature(row):
    return {k: ([{a:b for a,b in p.items() if a != 'ns'} for p in v]
                if k == 'phases' else v) for k,v in row.items()}


def validate(row, config, kind):
    for key,value in zip(['mode','requests','supply','weight','reuse','head','caller','stop'], config):
        assert row[key] == value, (key, config, row)
    assert row['counts'] == expected(config), (config, row['counts'], expected(config))
    assert row['meter'] == row['diagnostics'] == (kind == 'diagnostic')
    names = [('source_build',0), ('prepare',0)]
    for i in range(config[4]):
        names.append(('input_build',i))
        names.extend((name,i) for name in (['phase_solve','caller_transport_execute_observe_drop']
                     if config[0] == 'phase' else ['setup_execute_observe_drop']))
        names.append(('input_drop',i))
    names.extend((name,0) for name in ['prepared_drop','consumer_drop','source_drop'])
    phases = row['phases']
    assert [(p['name'],p['query']) for p in phases] == names
    if kind == 'primary':
        assert all(p['memory'] is None for p in phases)
        return
    baseline = phases[0]['memory']['live_start']
    assert phases[-1]['memory']['live_end'] == baseline
    for a,b in zip(phases,phases[1:]):
        assert a['memory']['live_end'] == b['memory']['live_start'], (config,a,b)
    for p in phases:
        m = p['memory']
        assert m['peak_live'] >= max(m['live_start'],m['live_end'])


def analyze():
    manifest = json.loads((RAW/'manifest.json').read_text())
    records = [json.loads(line) for line in (RAW/'runs.jsonl').read_text().splitlines()]
    assert manifest['configurations'] == [list(c) for c in CONFIGS]
    assert len(records) == 900
    results = []
    for cell,config in enumerate(CONFIGS):
        rows = []
        for repeat,kind in enumerate(['primary','diagnostic','diagnostic']):
            receipt = records[cell*3+repeat]
            assert (receipt['cell'],receipt['repeat'],receipt['kind']) == (cell,repeat,kind)
            assert receipt['returncode'] == 0 and not receipt['stderr'], receipt
            row = json.loads(receipt['stdout'])
            validate(row,config,kind)
            rows.append(row)
        assert signature(rows[1]) == signature(rows[2]), config
        phases = rows[1]['phases']; baseline = phases[0]['memory']['live_start']
        results.append({'config':config, 'counts':rows[1]['counts'],
                        'requested':sum(p['memory']['requested_bytes'] for p in phases),
                        'peak':max(p['memory']['peak_live'] for p in phases)-baseline,
                        'phases':signature(rows[1])['phases']})
    comparisons = []
    lookup = {tuple(r['config']):r for r in results}
    for config,row in lookup.items():
        if config[0] != 'phase':
            continue
        for control in MODES[1:]:
            other = lookup[(control,*config[1:])]
            comparisons.append({'config':config,'control':control,
                                'requested':row['requested'],'control_requested':other['requested'],
                                'peak':row['peak'],'control_peak':other['peak']})
    audit = {'configurations':300,'processes':900,'diagnostic_pairs':300,
             'metric':'requested heap bytes; not time or RSS', 'results':results,
             'comparisons':comparisons,
             'runs_sha256':hashlib.sha256((RAW/'runs.jsonl').read_bytes()).hexdigest()}
    (RAW/'audit.json').write_text(json.dumps(audit,indent=2)+'\n')
    print('Validated 900 processes: complete independent outcomes, 300 exact allocation pairs, continuous accounting and restored heap.')


if __name__ == '__main__':
    import sys
    if sys.argv[1:] == ['--analyze']:
        analyze()
        raise SystemExit(0)
    assert not sys.argv[1:]
    assert not (RAW/'manifest.json').exists() and not (RAW/'runs.jsonl').exists(), 'receipt already exists'
    binaries = {kind:BIN/kind/'release/examples/capacity_phase_lifecycle' for kind in ['primary','diagnostic']}
    files = {Path(__file__), *binaries.values(), ROOT/'Cargo.lock', ROOT/'Cargo.toml',
             ROOT/'docs/experiments/registrations/S06-capacity-phase-ownership.md'}
    for directory in ['crates/chr-syntax','research/chr-compiled','research/chr-direct-conditional',
                      'research/chr-observe','research/chr-direct-choice','research/chr-relational','research/chr-persistent']:
        files.update((ROOT/directory).rglob('*.rs'))
        files.add(ROOT/directory/'Cargo.toml')
    hashes = {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(files)}
    manifest = {'configurations':CONFIGS,'replays':['primary','diagnostic','diagnostic'],
                'commands':{k:[str(v.relative_to(ROOT)), 'mode requests supply weight reuse head caller stop'] for k,v in binaries.items()},
                'sha256':hashes}
    (RAW/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    check = subprocess.run([binaries['diagnostic'],'meter-check'],capture_output=True,text=True,timeout=60,preexec_fn=limits)
    (RAW/'meter-check.log').write_text(check.stdout+check.stderr)
    assert check.returncode == 0
    deadline = time.monotonic()+600
    with (RAW/'runs.jsonl').open('x') as out:
        for cell,config in enumerate(CONFIGS):
            for repeat,kind in enumerate(['primary','diagnostic','diagnostic']):
                remaining = deadline-time.monotonic()
                assert remaining > 0, 'launcher bound'
                command = [str(binaries[kind]), *map(str,config)]
                try:
                    proc = subprocess.run(command,capture_output=True,text=True,timeout=min(60,remaining),preexec_fn=limits)
                    receipt = {'cell':cell,'repeat':repeat,'kind':kind,'command':command,
                               'returncode':proc.returncode,'stdout':proc.stdout,'stderr':proc.stderr}
                except subprocess.TimeoutExpired as error:
                    receipt = {'cell':cell,'repeat':repeat,'kind':kind,'command':command,
                               'timeout':True,'stdout':str(error.stdout),'stderr':str(error.stderr)}
                    out.write(json.dumps(receipt)+'\n'); out.flush()
                    raise
                out.write(json.dumps(receipt)+'\n'); out.flush()
                assert proc.returncode == 0 and not proc.stderr, receipt
                validate(json.loads(proc.stdout),config,kind)
            if (cell+1)%50 == 0:
                print(f'{cell+1}/300 configurations',flush=True)
    for name,digest in hashes.items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == digest, name
    analyze()
