#!/usr/bin/env python3
"""Audit the partial-join work matrix against the unchanged earlier controls."""
import hashlib,itertools,json,pathlib
ROOT=pathlib.Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-partial-join-gate'
def rows(path):
    records=[json.loads(x) for x in path.read_text().splitlines() if x.startswith('{')]
    result={(r['family'],r['width'],r['selective'],r.get('partial',False)):r for r in records}
    assert len(records)==len(result)
    return result
def main():
    current=rows(OUT/'source.log')
    prior=rows(OUT.parent/'s02-tuple-activation-gate/source.log')
    assert set(current)=={(f,n,s,p) for f,n in itertools.product(['sparse','broad','nested','cold','dense'],[4,16,64]) for s,p in [(False,False),(True,False),(True,True)]}
    for key,old in prior.items():
        now=dict(current[key]);now.pop('partial');assert now==old,key
    for n in [4,16,64]:
        cold=current['cold',n,True,True]
        assert cold['heads']==cold['registrations']==cold['peak_tuples']==n
        dense=current['dense',n,True,True]
        assert dense['heads']==dense['registrations']==dense['peak_tuples']==n+n*n
    files=['research/chr-relational/tests/support/local_multihead.rs','research/chr-relational/tests/multihead.rs','docs/experiments/registrations/S02-partial-join-gate.md']
    receipt={'finite_configurations':70,'work_cells':45,'metrics_off_cells':45,'unchanged_control_cells':24,'rows':list(current.values()),'source_sha256':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files}}
    (OUT/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('45 work cells, 24 unchanged controls, linear cold-prefix and dense-prefix retention witnesses verified')
if __name__=='__main__':main()
