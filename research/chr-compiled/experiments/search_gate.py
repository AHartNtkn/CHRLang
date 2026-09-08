#!/usr/bin/env python3
"""Independent finite-source correspondence for compiled explicit search."""
import hashlib
import itertools
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-direct-solver'))
from source_gate import cases,encode,observed
from oracle import evaluate

def main():
    binary=Path(sys.argv[1]).resolve();receipt=Path(sys.argv[2]);counted=sys.argv[3]=="work"
    assert sys.argv[3] in ["work","off"]
    registry=[(n,q,evaluate(q)) for n,q in cases()]
    payload=''.join(encode(q,o['variables']) for _,q,o in registry)
    report={'status':'incomplete','queries':len(registry),'configurations':[],'failures':[],
            'metrics_expected':counted,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'source_hashes':{}}
    files=[Path(__file__),ROOT/'research/chr-direct-solver/source_gate.py',ROOT/'research/chr-direct-solver/oracle.py']
    for part in ['research/chr-compiled','research/chr-persistent','crates/chr-syntax']:
        files+=list((ROOT/part).rglob('*.rs'))
    for p in files:report['source_hashes'][str(p.relative_to(ROOT))]=hashlib.sha256(p.read_bytes()).hexdigest()
    report['input_sha256']=hashlib.sha256(payload.encode()).hexdigest()
    try:
        for execution,policy,access,order in itertools.product(['generic','generated'],['global','active'],['scan','indexed'],range(6)):
            proc=subprocess.run([str(binary),str(order),execution,policy,access],input=payload,text=True,capture_output=True,check=True,timeout=60)
            actual=[json.loads(x) for x in proc.stdout.splitlines()];assert len(actual)==len(registry)
            for (name,q,oracle),r in zip(registry,actual):
                assert r['metrics']==counted and r['kernel_metrics']==counted,'instrumentation mismatch'
                expected_raw=[]
                for values in oracle['assignments']:
                    assignment=dict(zip(oracle['variables'],values))
                    def value(t):return assignment[t] if isinstance(t,str) else t
                    expected_raw.append((tuple((name,value(t)) for name,t in q.get('outputs',())),tuple(sorted((value(l),value(r),a,b) for l,r,a,b in q.get('forbid',())))))
                actual_raw=[(tuple(tuple(pair) for pair in a['outputs']),tuple(sorted(tuple(row) for row in a['residual']))) for a in r['answers']]
                if sorted(actual_raw)!=sorted(expected_raw) or not r['exhausted'] or r['raw']!=oracle['raw'] or observed(r)!=oracle['answers'] or (not q.get('choose') and r['splits']!=0):
                    report['failures'].append({'case':name,'configuration':[execution,policy,access,order],'expected':oracle,'actual':r})
            report['configurations'].append({'execution':execution,'policy':policy,'access':access,'order':order,'checked':len(actual)})
        assert not report['failures'],'independent source mismatch'
        report['status']='passed'
    except Exception as e:
        report['error']=repr(e);raise
    finally:
        receipt.parent.mkdir(parents=True,exist_ok=True);receipt.write_text(json.dumps(report,indent=2)+'\n')
    print(f'{len(registry)} queries x48 configurations passed: full observations, raw completions, exhaustion')
if __name__=='__main__':main()
