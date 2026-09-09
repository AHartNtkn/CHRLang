#!/usr/bin/env python3
"""Registered T057 work-only screen; elapsed process times are resource diagnostics."""
import argparse
import hashlib
import json
import os
import pathlib
import random
import resource
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]
REG = 'docs/experiments/registrations/R01-current-join-screen.md'

def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def jobs():
    result = [dict(rep=r, n=n, requests=q, policy=p, access=a)
              for r in range(2) for n in (16, 128) for q in (1, 32)
              for p in ('global', 'active') for a in ('scan', 'indexed')]
    random.Random(57057).shuffle(result)
    return result

def run(out):
    out.mkdir(exist_ok=False)
    files = [ROOT/'Cargo.toml', ROOT/'Cargo.lock', ROOT/REG]
    for directory in ('research/chr-compiled', 'research/chr-persistent', 'research/chr-observe', 'crates/chr-syntax'):
        files.extend(p for p in (ROOT/directory).rglob('*') if p.suffix in ('.rs', '.toml', '.py'))
    binary = ROOT/'target'/out.name/'release/chr-join-screen'
    command = ['cargo', 'build', '--locked', '--offline', '--release', '-p', 'chr-compiled',
               '--features', 'experiment', '--bin', 'chr-join-screen', '--target-dir', str(binary.parents[1])]
    meta = dict(seed=57057, jobs=jobs(), cpu=min(os.sched_getaffinity(0)),
                base_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                rustc=subprocess.check_output(['rustc', '-Vv'], text=True), uname=list(os.uname()),
                sources={str(p.relative_to(ROOT)):digest(p) for p in sorted(set(files))},
                command=command, binary=str(binary),
                bounds=dict(process_seconds=30, address_bytes=1024**3, ticks=20_000_000, build_seconds=180, execution_seconds=1200))
    for job in meta['jobs']:
        job['command']=[str(binary),str(job['n']),str(job['requests']),job['policy'],job['access']]
    def save():
        (out/'metadata.json').write_text(json.dumps(meta, indent=2)+'\n')
    save()
    try:
        with (out/'build.log').open('w') as log:
            built = subprocess.run(command,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,timeout=180)
        meta['build_exit']=built.returncode
        built.check_returncode()
        meta['binary_sha256']=digest(binary)
    except (OSError, subprocess.SubprocessError) as error:
        meta['build_error']=repr(error)
        raise
    finally:
        save()
    def limits():
        os.sched_setaffinity(0,{meta['cpu']})
        resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
    start=time.monotonic()
    with (out/'runs.jsonl').open('x') as log:
        for job in meta['jobs']:
            if time.monotonic()-start>1200:
                break
            row=dict(job)
            before=time.monotonic()
            try:
                result=subprocess.run(job['command'],capture_output=True,text=True,timeout=30,preexec_fn=limits)
                row.update(exit=result.returncode,stdout=result.stdout,stderr=result.stderr)
                try:
                    row['result']=json.loads(result.stdout)
                except json.JSONDecodeError as error:
                    row['decode_error']=str(error)
            except subprocess.TimeoutExpired as error:
                def decoded(value):
                    return value.decode(errors='replace') if isinstance(value,bytes) else value
                row.update(error=repr(error),timeout=True,stdout=decoded(error.stdout),stderr=decoded(error.stderr))
            except (OSError,subprocess.SubprocessError) as error:
                row['error']=repr(error)
            row['wall_s']=time.monotonic()-before
            log.write(json.dumps(row)+'\n');log.flush()
    return audit(out)

def validate(row):
    assert row['exit']==0
    d=row['result']
    assert d['schema']==1 and d['metrics'] is True
    for key in ('n','requests','policy','access'):
        assert d[key]==row[key]
    assert d['complete'] is True and d['failed'] is False and d['validated'] is True
    assert d['kernel_metrics'] is True and d['observer_metrics'] is True
    assert d['tick_limit']==20_000_000 and 0<d['ticks']<=d['tick_limit']
    assert d['validated_receipts']==row['requests']
    assert d['applications']==3*row['requests']+row['n']+2
    assert isinstance(d['work'],dict) and d['work']
    assert all(isinstance(v,int) and v>=0 for v in d['work'].values())
    assert len(d['phases'])==row['requests']
    previous={k:0 for k in d['work']}
    for i,phase in enumerate(d['phases'],1):
        assert phase['completed_requests']==i and phase['applications']==3*i
        assert phase['work'].keys()==d['work'].keys()
        assert all(isinstance(v,int) and v>=0 for v in phase['work'].values())
        assert all(previous[k]<=v<=d['work'][k] for k,v in phase['work'].items() if k not in ('index_entries','dependency_edges'))
        previous=phase['work']

def audit(out):
    meta=json.loads((out/'metadata.json').read_text())
    expected=jobs()
    for j in expected:
        j['command']=[meta['binary'],str(j['n']),str(j['requests']),j['policy'],j['access']]
    assert meta['jobs']==expected and meta['seed']==57057
    assert meta['bounds']==dict(process_seconds=30,address_bytes=1024**3,ticks=20_000_000,build_seconds=180,execution_seconds=1200)
    expected_command=['cargo','build','--locked','--offline','--release','-p','chr-compiled','--features','experiment','--bin','chr-join-screen','--target-dir',str(pathlib.Path(meta['binary']).parents[1])]
    assert meta['command']==expected_command and meta.get('build_exit')==0
    rows=[json.loads(l) for l in (out/'runs.jsonl').read_text().splitlines()]
    failures=[];valid=[]
    for i,row in enumerate(rows):
        try:
            assert {k:row[k] for k in expected[i]}==expected[i]
            validate(row)
            valid.append(row)
        except (AssertionError,KeyError,TypeError,IndexError) as error:
            failures.append(dict(index=i,error=repr(error),row=row))
    groups={}
    for row in valid:
        key=(row['n'],row['requests'],row['policy'],row['access'])
        groups.setdefault(key,[]).append(row['result'])
    summary=[]
    for key in [(n,q,p,a) for n in (16,128) for q in (1,32) for p in ('global','active') for a in ('scan','indexed')]:
        group=groups.get(key,[])
        same=len(group)==2 and group[0]==group[1]
        summary.append(dict(cell=key,complete=same,results=group))
        if len(group)==2 and not same:
            failures.append(dict(cell=key,error='non-time results differ across repetitions'))
    freeze=[n for n,h in meta['sources'].items() if digest(ROOT/n)!=h]
    if digest(pathlib.Path(meta['binary']))!=meta['binary_sha256']:
        freeze.append('binary')
    result=dict(planned=32,recorded=len(rows),missing=32-len(rows),failures=failures,
                validated_runs=len(valid),complete_cells=sum(s['complete'] for s in summary),freeze_errors=freeze)
    (out/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
    (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(result))
    return bool(result['missing'] or failures or freeze)

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('mode',choices=['run','audit'])
    parser.add_argument('--output',type=pathlib.Path,required=True)
    args=parser.parse_args()
    raise SystemExit((run if args.mode=='run' else audit)(args.output.resolve()))
