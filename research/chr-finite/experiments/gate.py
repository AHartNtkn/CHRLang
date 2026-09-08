#!/usr/bin/env python3
"""Full raw source-observation gate against the independent finite oracle."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-solver'))
from source_gate import cases,encode
from oracle import evaluate

def registry():
    yield from cases()
    yield 'unsupported-value',dict(choose=('x','y'),forbid=tuple(('x','y',0,a) for a in range(3)),outputs=(('x','x'),('y','y')))
    same=tuple((f'x{i}',f'x{i+1}',a,b) for i in range(3) for a in range(3) for b in range(3) if a!=b)
    for supplied in [(),(('x0',0),), (('x0',0),('x3',1))]:
        yield 'equality-chain'+str(supplied),dict(choose=tuple(f'x{i}' for i in range(4)),given=supplied,forbid=same,outputs=tuple((f'o{i}',f'x{i}') for i in range(4)))
    yield 'asymmetric',dict(choose=('x','y'),forbid=(('x','y',0,1),('y','x',0,2),('x','y',2,2)),outputs=(('x','x'),))
    yield 'arc-consistent-unsat',dict(choose=('x','y','z'),forbid=tuple((x,y,a,b) for x,y in [('x','y'),('y','z')] for a in range(3) for b in range(3) if a!=b)+tuple(('z','x',a,a) for a in range(3)))

def main():
    binary=Path(sys.argv[1]).resolve();out=Path(sys.argv[2]);rows=[(n,q,evaluate(q)) for n,q in registry()]
    payload=''.join(encode(q,o['variables']) for _,q,o in rows)
    result={'status':'incomplete','queries':len(rows),'backends':[],'failures':[],'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'source_hashes':{}}
    files=list((ROOT/'research/chr-finite').rglob('*.rs'))+[Path(__file__),ROOT/'research/chr-direct-solver/oracle.py',ROOT/'research/chr-direct-solver/source_gate.py']
    for p in files:result['source_hashes'][str(p.relative_to(ROOT))]=hashlib.sha256(p.read_bytes()).hexdigest()
    try:
        for mode in ['native','support','conflict']:
            p=subprocess.run([str(binary),mode],input=payload,capture_output=True,text=True,check=True,timeout=60)
            actual=[json.loads(x) for x in p.stdout.splitlines()];assert len(actual)==len(rows)
            for (name,q,o),r in zip(rows,actual):
                expected=[]
                for values in o['assignments']:
                    assignment=dict(zip(o['variables'],values))
                    def value(x):return assignment[x] if isinstance(x,str) else x
                    expected.append((tuple((n,value(x)) for n,x in q.get('outputs',())),tuple(sorted((value(x),value(y),a,b) for x,y,a,b in q.get('forbid',())))))
                observed=[(tuple(tuple(p) for p in a['outputs']),tuple(sorted(tuple(p) for p in a['residual']))) for a in r['answers']]
                if not r['exhausted'] or r['raw']!=o['raw'] or sorted(expected)!=sorted(observed):result['failures'].append({'case':name,'backend':mode,'expected':expected,'actual':r})
            result['backends'].append({'backend':mode,'checked':len(actual)})
        assert not result['failures'];result['status']='passed'
    except Exception as e:result['error']=repr(e);raise
    finally:out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(result,indent=2)+'\n')
    print(f'{len(rows)} source queries x3 backends: full raw observations and exhaustion pass')
if __name__=='__main__':main()
