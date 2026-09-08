#!/usr/bin/env python3
"""Registered before/after matcher comparison in isolated build packages."""
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s04-matcher-paired'
FAMILIES = ['mutation', 'retained', 'work', 'compatible-small', 'compatible-large', 'compatible-alias']
MODES = ['copy', 'trail', 'checkpoint1', 'indexed']

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def write(name, data):
    (OUT / (name + '.json')).write_text(json.dumps(data, indent=2) + '\n')

def bounded(cpu):
    def apply():
        resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
        os.sched_setaffinity(0, {cpu})
    return apply

def invoke(cmd, name, cpu=None, timeout=60):
    try:
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, timeout=timeout,
                           preexec_fn=bounded(cpu) if cpu is not None else None)
        r = dict(command=cmd, exit_code=p.returncode, stdout=p.stdout, stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r = dict(command=cmd, exit_code=None, cutoff='wall',
                 stdout=str(e.stdout), stderr=str(e.stderr))
    write(name, r)
    assert r['exit_code'] == 0, name
    return r

def source_paths():
    paths = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', Path(__file__).resolve(),
             ROOT / 'docs/experiments/registrations/S04-matcher-paired-cost.md',
             ROOT / 'docs/experiments/results/s04-matcher-copy/before-lib.rs',
             ROOT / 'research/chr-direct-conditional/tests/runtime_support/mod.rs']
    for d in ['crates/chr-syntax', 'research/chr-restoration', 'research/chr-compiled',
              'research/chr-persistent', 'research/chr-observe']:
        paths.extend(p for p in (ROOT/d).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml'))
    return {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(paths))}

def build():
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT/'freeze.json').exists()
    base = Path(tempfile.mkdtemp(prefix='chr-s04-matcher-'))
    cpu = min(os.sched_getaffinity(0))
    binaries, packages = {}, {}
    for version in ['before', 'after']:
        package = base/version
        (package/'src').mkdir(parents=True)
        (package/'examples').mkdir()
        library = ROOT/('docs/experiments/results/s04-matcher-copy/before-lib.rs' if version == 'before' else 'research/chr-restoration/src/lib.rs')
        (package/'src/lib.rs').write_bytes(library.read_bytes())
        harness = (ROOT/'research/chr-restoration/examples/lifecycle.rs').read_text()
        harness = harness.replace('../../chr-direct-conditional/tests/runtime_support/mod.rs', str(ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs'))
        (package/'examples/lifecycle.rs').write_text(harness)
        manifest = '[package]\nname="chr-restoration"\nversion="0.1.0"\nedition="2024"\n[workspace]\nresolver="3"\n[dependencies]\n'
        manifest += f'chr-syntax={{path="{ROOT}/crates/chr-syntax"}}\n[dev-dependencies]\n'
        for dependency in ['chr-observe','chr-compiled']:
            manifest += f'{dependency}={{path="{ROOT}/research/{dependency}",default-features=false}}\n'
        manifest += '[features]\nalloc-meter=["chr-compiled/alloc-meter"]\narena-cow=["chr-compiled/arena-cow"]\nreplay-diagnostic=[]\n'
        (package/'Cargo.toml').write_text(manifest)
        for kind in ['time','meter']:
            target = ROOT/'target/s04-matcher-paired'/version/kind
            cmd = ['cargo','build','--offline','--release','--manifest-path',str(package/'Cargo.toml'), '--example','lifecycle','--target-dir',str(target)]
            if kind == 'meter': cmd += ['--features','alloc-meter']
            invoke(cmd, f'build-{version}-{kind}', timeout=120)
            binary = target/'release/examples/lifecycle'
            for family in FAMILIES:
                for mode in MODES:
                    invoke([str(binary),'gate',family,mode], f'gate-{version}-{kind}-{family}-{mode}', cpu)
            binaries[f'{version}-{kind}'] = dict(path=str(binary), sha256=digest(binary))
            print('built and gated',version,kind,flush=True)
        packages[version] = {str(p.relative_to(package)): p.read_text() for p in [package/'Cargo.toml',package/'Cargo.lock',package/'src/lib.rs',package/'examples/lifecycle.rs']}
    write('packages',packages)
    write('freeze',dict(sources=source_paths(), binaries=binaries, cpu=cpu,
                       packages_sha256=digest(OUT/'packages.json'),
                       rustc=subprocess.check_output(['rustc','-Vv'],text=True),
                       git_head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()))

def run():
    freeze=json.loads((OUT/'freeze.json').read_text())
    assert source_paths()==freeze['sources']
    assert digest(OUT/'packages.json')==freeze['packages_sha256']
    for b in freeze['binaries'].values(): assert digest(Path(b['path']))==b['sha256']
    rng=random.Random(20260909)
    order=[]
    for kind,reps in [('time',5),('meter',2)]:
        for rep in range(reps):
            cells=[(f,m,r) for f in FAMILIES for m in MODES for r in [1,8]]
            rng.shuffle(cells)
            for f,m,r in cells:
                versions=['before','after']; rng.shuffle(versions)
                for v in versions:
                    order.append(dict(kind=kind,rep=rep,family=f,mode=m,reuse=r,version=v))
    write('order',order)
    for i,c in enumerate(order):
        name='-'.join(str(c[k]) for k in ['kind','rep','family','mode','reuse','version'])
        cmd=[freeze['binaries'][c['version']+'-'+c['kind']]['path'],c['mode'],c['family'],str(c['reuse'])]
        path=OUT/(name+'.json')
        if path.exists():
            receipt=json.loads(path.read_text())
            assert receipt['exit_code']==0 and receipt['command']==cmd
        else: receipt=invoke(cmd,name,freeze['cpu'])
        value=json.loads(receipt['stdout'])
        assert value['family']==c['family'] and value['mode']==c['mode'] and value['reuse']==c['reuse']
        assert value['meter']==(c['kind']=='meter') and not value['cow']
        if (i+1)%48==0: print(f'{i+1}/{len(order)} validated processes',flush=True)
    assert source_paths()==freeze['sources']
    print('all registered processes complete',flush=True)

if __name__=='__main__':
    {'build':build,'run':run}[sys.argv[1]]()
