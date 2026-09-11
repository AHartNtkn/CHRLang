"""Run and audit the registered isolated requested-allocation matrix."""
import gzip
import hashlib
import itertools
import json
from pathlib import Path
import random
import resource
import subprocess
import zipfile
import sys

ARENA_COW = "--arena-cow" in sys.argv

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / ('docs/experiments/results/s01-prepared-prefix-arena' if ARENA_COW else 'docs/experiments/results/s01-prepared-prefix-ownership')


def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (1024**3, 1024**3))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))


def audit():
    rows = [json.loads(line) for line in gzip.open(OUT / 'runs.jsonl.gz', 'rt')]
    assert len(rows) == 1024
    cells = {}
    blocks = {}
    for row in rows:
        key = tuple(row['cell'])
        blocks.setdefault(key, []).append(row['block'])
        records = row['result']['records']
        phases = [(r['phase'], r['q']) for r in records]
        expected = [('prepare',0),('prefix_prepare',0)]
        for q in range(2):
            expected += [('setup',q),('execute',q)]
            if q != 0 or key[6] != 'true': expected += [('observe',q)]
            expected += [('engine_drop',q),('answer_drop',q)]
        expected += [('prefix_drop',0),('prepared_drop',0),('retained_drop',0)]
        assert phases == expected
        heaps = [r['heap'] for r in records]
        assert row['result']['validated']
        assert all(a['live_end'] == b['live_start'] for a,b in zip(heaps, heaps[1:]))
        assert heaps[-1]['live_end'] == heaps[0]['live_start']
        key = tuple(row['cell'])
        values = {'requested': sum(h['requested_bytes'] for h in heaps),
                  'peak': max(h['peak_live'] for h in heaps)-heaps[0]['live_start'],
                  'phases': {r['phase']: sum(x['heap']['requested_bytes'] for x in records if x['phase']==r['phase']) for r in records}}
        if key in cells:
            assert cells[key][0] == heaps
        else:
            cells[key] = (heaps, values)
    assert len(cells) == 512
    assert all(sorted(b) == [0,1] for b in blocks.values())
    comparisons = []
    for key, (_, reuse) in cells.items():
        if key[-1] != 'reuse': continue
        fresh = cells[key[:-1]+('fresh',)][1]
        comparisons.append({'cell': key[:-1], 'fresh': fresh, 'reuse': reuse})
    (OUT/'analysis.json').write_text(json.dumps(comparisons, indent=2)+'\n')
    if ARENA_COW:
        baseline = ROOT/'docs/experiments/results/s01-prepared-prefix-ownership'
        old = {tuple(r['cell']):r for r in json.loads((baseline/'analysis.json').read_text())}
        contrasts = []
        for row in comparisons:
            for mode in ['fresh','reuse']:
                contrasts.append({'cell':list(row['cell'])+[mode], 'ordinary':old[tuple(row['cell'])][mode], 'shared':row[mode]})
        (OUT/'arena-comparison.json').write_text(json.dumps(contrasts,indent=2)+'\n')
        # Confirm that this is a feature intervention on the same measured engine.
        with zipfile.ZipFile(baseline/'sources.zip') as a, zipfile.ZipFile(OUT/'sources.zip') as b:
            for name in a.namelist():
                if '/src/' in name or name.endswith('examples/probe_lifecycle.rs'):
                    assert a.read(name)==b.read(name), name
    print(json.dumps({'processes':len(rows),'exact_pairs':len(cells), 'comparisons':len(comparisons)}))


def run():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT/'freeze.json').exists()
    binaries = {}
    for probe in [False, True]:
        features = 'alloc-meter' + (',selective-probe' if probe else '') + (',arena-cow' if ARENA_COW else '')
        subprocess.run(['cargo','build','-p','chr-compiled','--release','--no-default-features','--features',features,'--example','probe_lifecycle'],cwd=ROOT,check=True)
        dest = ROOT / f'target/prepared-prefix-{ARENA_COW}-{probe}'
        dest.write_bytes((ROOT/'target/release/examples/probe_lifecycle').read_bytes())
        dest.chmod(0o755)
        binaries[str(probe)] = {'path':str(dest),'sha256':hashlib.sha256(dest.read_bytes()).hexdigest()}
    sources = subprocess.check_output(['git','ls-files','research/chr-compiled','research/chr-persistent','research/chr-observe','crates/chr-syntax','Cargo.toml','Cargo.lock','rust-toolchain.toml'],cwd=ROOT,text=True).splitlines()
    sources += ['research/chr-compiled/experiments/prepared_prefix_ownership.py','docs/experiments/registrations/S01-prepared-prefix-ownership.md']
    if ARENA_COW:
        sources += ['docs/experiments/registrations/S01-prepared-prefix-arena.md']
    with zipfile.ZipFile(OUT/'sources.zip','w',zipfile.ZIP_DEFLATED) as z:
        for p in sorted(set(sources)): z.write(ROOT/p,p)
    (OUT/'freeze.json').write_text(json.dumps({'binaries':binaries,'sources_sha256':hashlib.sha256((OUT/'sources.zip').read_bytes()).hexdigest(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True)},indent=2)+'\n')
    cells = list(itertools.product([False,True],['selective','neutral','duplicate','broad'],[16,128],['global','active'],['scan','indexed'],['false','true'],['false','true'],['fresh','reuse']))
    rng=random.Random(8211 if ARENA_COW else 8210)
    with gzip.open(OUT/'runs.jsonl.gz','wt') as out:
        for block in range(2):
            rng.shuffle(cells)
            for cell in cells:
                probe,family,n,policy,access,keep,cancel,mode=cell
                result=subprocess.run([binaries[str(probe)]['path'],family,str(n),policy,access,keep,cancel,'1',mode],capture_output=True,text=True,timeout=60,preexec_fn=bounds)
                if result.returncode != 0:
                    (OUT/'failure.json').write_text(json.dumps({'cell':cell,'stderr':result.stderr},indent=2)+'\n')
                assert result.returncode==0, (cell,result.stderr)
                out.write(json.dumps({'block':block,'cell':cell,'result':json.loads(result.stdout)})+'\n')
            out.flush()
            print(f'block {block+1} complete',flush=True)
    audit()


if __name__ == '__main__':
    import sys
    audit() if '--audit' in sys.argv else run()
