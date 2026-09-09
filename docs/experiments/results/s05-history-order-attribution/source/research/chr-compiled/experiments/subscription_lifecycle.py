#!/usr/bin/env python3
"""Build/gate frozen S01 subscription binaries, then execute the registered serial pilot."""
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
OUT = ROOT / 'docs/experiments/results/s01-subscription-lifecycle'
FAMILIES=['selective','dense','inactive','churn-sparse','churn-broad','reopen','unique','binding','consuming']
MODES=['indexed','eager','subscribed','compiled']
CONFIGS=[(f,n,r) for f in FAMILIES for n in [2,4] for r in [2,8]]

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def sources():
    paths = [ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/'lean-toolchain']
    paths = [p for p in paths if p.exists()]
    for directory in ['crates/chr-syntax', 'research/chr-reuse', 'research/chr-direct-choice', 'research/chr-direct-conditional', 'research/chr-compiled', 'research/chr-persistent', 'research/chr-observe']:
        paths += [p for p in (ROOT/directory).rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
    paths += [ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs', ROOT/'docs/experiments/registrations/S01-subscription-lifecycle.md', Path(__file__).resolve()]
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
    assert not (OUT/'freeze.json').exists(), 'freeze already exists'
    OUT.mkdir(parents=True,exist_ok=True)
    cpu=min(os.sched_getaffinity(0));bins={}
    for kind in ['time','meter','work']:
        target=ROOT/'target/s01-subscription-lifecycle'/kind
        cmd=['cargo','build','-p','chr-compiled','--release','--example','s01_subscription_lifecycle','--target-dir',str(target)]
        if kind!='work': cmd+=['--no-default-features']
        if kind=='meter':cmd+=['--features','alloc-meter']
        invoke(cmd,'build-'+kind,timeout=120)
        binary=target/'release/examples/s01_subscription_lifecycle'
        bins[kind]={'path':str(binary),'sha256':digest(binary)}
        for f,n,r in CONFIGS:
            for mode in MODES if kind!='work' else MODES[:3]:
                if kind!='work':invoke([str(binary),'gate',mode,f,str(n),str(r)],f'gate-{kind}-{f}-{n}-{r}-{mode}',cpu)
                else:
                    values=[]
                    for rep in range(2):values.append(invoke([str(binary),'work',mode,f,str(n),str(r)],f'work-{rep}-{f}-{n}-{r}-{mode}',cpu)['stdout'])
                    assert values[0]==values[1],(f,n,r,mode,'work replay')
                    for line in values[0].splitlines():
                        row=json.loads(line)
                        if mode=='indexed':built=retained=0
                        elif mode=='subscribed':
                            retained=0
                            built=1 if f=='selective' else r*n**3 if f in ['reopen','unique','consuming'] else n**3+r*(n if f=='churn-broad' else 1)*n**2 if f.startswith('churn') else n**3
                        else:
                            retained=n if f=='selective' else n**3 if f=='dense' else 0 if f=='consuming' else 8*n**3
                            built=n if f=='selective' else n**3 if f=='dense' else r*n**3 if f=='consuming' else 8*n**3+r*(n if f=='churn-broad' else 1)*n**2 if f.startswith('churn') else 8*n**3
                        assert (row['constructed'],row['invalidated'],row['retained'])==(built,built-retained,retained),(f,n,r,mode,row,built,retained)
            print('gated',kind,f,n,r,flush=True)
    invoke(['cargo','clippy','-p','chr-compiled','--no-default-features','--example','s01_subscription_lifecycle','--','-D','warnings'],'clippy',timeout=120)
    invoke(['cargo','tree','-p','chr-compiled','--no-default-features','-e','features'],'features')
    freeze={'sources':sources(),'binaries':bins,'cpu':cpu,'affinity':sorted(os.sched_getaffinity(0)),'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'git_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()}
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
def run():
    freeze=json.loads((OUT/'freeze.json').read_text());assert sources()==freeze['sources']
    for b in freeze['binaries'].values():assert digest(Path(b['path']))==b['sha256']
    rng=random.Random(20260915);order=[]
    for kind,reps in [('time',5),('meter',2)]:
        for rep in range(reps):
            cells=[(f,n,r,m) for f,n,r in CONFIGS for m in MODES];rng.shuffle(cells)
            order.extend(dict(kind=kind,rep=rep,family=f,n=n,rounds=r,mode=m) for f,n,r,m in cells)
    p=OUT/'order.json'
    if p.exists():assert json.loads(p.read_text())==order
    else:p.write_text(json.dumps(order,indent=2)+'\n')
    for i,cell in enumerate(order):
        f,n,r,m,k,rep=[cell[x] for x in ['family','n','rounds','mode','kind','rep']]
        name=f'{k}-{rep}-{f}-{n}-{r}-{m}'
        cmd=[freeze['binaries'][k]['path'],'cell',m,f,str(n),str(r)]
        if (OUT/(name+'.json')).exists():
            old=json.loads((OUT/(name+'.json')).read_text());assert old['exit_code']==0 and old['command']==cmd
        else:invoke(cmd,name,freeze['cpu'])
        if (i+1)%16==0:print('complete',i+1,'of',len(order),flush=True)
    print('all registered processes complete',flush=True)
if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('action',choices=['build','run']);args=parser.parse_args()
    {'build':build,'run':run}[args.action]()
