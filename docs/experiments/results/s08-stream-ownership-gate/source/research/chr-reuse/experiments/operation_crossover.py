#!/usr/bin/env python3
"""Build/gate frozen S04 binaries, then execute the registered serial pilot."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s05-operation-crossover'
FAMILIES = [f'cross-before-{result}-{d}-{b}' for result in ['success','clash'] for d in [64,256,1024] for b in [2,4,6]] + ['cross-after-success-1024-6','cross-after-clash-1024-6','cross-unique-success-64-6','cross-unique-success-1024-6']
MODES = ['ordinary','dependencies','graph','specialized']

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def sources():
    paths = [ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'lean-toolchain']
    paths = [p for p in paths if p.exists()]
    for directory in ['crates/chr-syntax', 'research/chr-reuse', 'research/chr-direct-choice', 'research/chr-direct-conditional', 'research/chr-compiled', 'research/chr-persistent', 'research/chr-observe']:
        paths += [p for p in (ROOT/directory).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
    paths += [ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs', ROOT/'docs/experiments/registrations/S05-operation-crossover.md', Path(__file__).resolve()]
    return {str(p.relative_to(ROOT)): digest(p) for p in sorted(set(paths))}

def constrained(cpu):
    def apply():
        resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
        os.sched_setaffinity(0, {cpu})
    return apply

def invoke(cmd, name, cpu=None, timeout=60):
    try:
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, timeout=timeout,
                           preexec_fn=constrained(cpu) if cpu is not None else None)
        result = {'command': cmd, 'exit_code': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
    except subprocess.TimeoutExpired as e:
        result = {'command': cmd, 'exit_code': None, 'cutoff': 'wall',
                  'stdout': (e.stdout or b'').decode() if isinstance(e.stdout, bytes) else (e.stdout or ''),
                  'stderr': (e.stderr or b'').decode() if isinstance(e.stderr, bytes) else (e.stderr or '')}
    (OUT/(name+'.json')).write_text(json.dumps(result, indent=2)+'\n')
    assert result['exit_code'] == 0, f'{name}: failed; see raw receipt'
    return result

def build():
    assert not (OUT/'freeze.json').exists(), 'frozen binaries already exist'
    OUT.mkdir(parents=True, exist_ok=True)
    cpu = min(os.sched_getaffinity(0))
    bins = {}
    for name,features in [('time', []), ('meter', ['alloc-meter'])]:
        target = ROOT/'target/s05-operation-crossover'/name
        cmd = ['cargo', 'build', '-p', 'chr-reuse', '--no-default-features', '--release', '--example', 'lifecycle', '--target-dir', str(target)]
        if features: cmd += ['--features', ','.join(features)]
        invoke(cmd, 'build-'+name, timeout=120)
        binary = target/'release/examples/lifecycle'
        for family in FAMILIES:
            gate_cmd = [str(binary), 'gate', family, ','.join(MODES)]
            invoke(gate_cmd, 'gate-final-'+name+'-'+family, cpu, timeout=60)
            print('gated', name, family, flush=True)
        bins[name] = {'path': str(binary), 'sha256': digest(binary)}
        print('built and gated', name, flush=True)
    invoke(['cargo','clippy','-p','chr-reuse','--all-targets','--all-features','--','-D','warnings'], 'clippy', timeout=120)
    invoke(['cargo','test','-p','chr-reuse','--release'], 'tests', timeout=60)
    invoke(['cargo','fmt','-p','chr-reuse','--check'], 'format')
    target=ROOT/'target/s05-operation-crossover/work'
    invoke(['cargo','build','-p','chr-reuse','--release','--example','lifecycle','--target-dir',str(target)],'build-work',timeout=120)
    work_binary=target/'release/examples/lifecycle'
    bins['work']={'path':str(work_binary),'sha256':digest(work_binary)}
    for family in FAMILIES:
        values=[]
        for rep in range(2):
            values.append(invoke([str(work_binary),'work',family],f'work-{rep}-{family}',cpu)['stdout'])
        assert values[0]==values[1],(family,'work replay')
    invoke(['cargo','tree','-p','chr-reuse','--no-default-features','-e','features'], 'feature-tree')
    freeze = {'sources': sources(), 'binaries': bins, 'cpu':cpu, 'affinity':sorted(os.sched_getaffinity(0)),
              'platform': platform.platform(), 'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
              'git_head':subprocess.check_output(['git','rev-parse','HEAD'],text=True,cwd=ROOT).strip()}
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')

def run():
    freeze=json.loads((OUT/'freeze.json').read_text())
    assert sources()==freeze['sources'], 'source freeze changed'
    for b in freeze['binaries'].values(): assert digest(Path(b['path']))==b['sha256']
    rng=random.Random(20260912)
    order=[]
    for kind,reps in [('time',5),('meter',2)]:
        for rep in range(reps):
            cells=[(f,m,r) for f in FAMILIES for m in MODES for r in [1]]
            rng.shuffle(cells)
            order += [{'kind':kind,'rep':rep,'family':f,'mode':m,'reuse':r} for f,m,r in cells]
    (OUT/'order.json').write_text(json.dumps(order,indent=2)+'\n')
    for index,cell in enumerate(order):
        name=f"{cell['kind']}-{cell['rep']}-{cell['family']}-{cell['mode']}-{cell['reuse']}"
        key=cell['kind']
        cmd=[freeze['binaries'][key]['path'], cell['mode'], cell['family'], str(cell['reuse'])]
        path=OUT/(name+'.json')
        if path.exists():
            receipt=json.loads(path.read_text())
            assert receipt['exit_code']==0 and receipt['command']==cmd
        else:
            receipt=invoke(cmd,name,freeze['cpu'],timeout=60)
        value=json.loads(receipt['stdout'])
        assert value['family']==cell['family'] and value['reuse']==cell['reuse']
        assert value['meter']==(cell['kind']=='meter')
        assert len(value['queries'])==cell['reuse']
        if (index+1)%24==0: print(f'{index+1}/{len(order)} processes validated',flush=True)
    assert sources()==freeze['sources']
    print('all registered processes complete',flush=True)

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('action',choices=['build','run'])
    args=parser.parse_args()
    build() if args.action=='build' else run()
