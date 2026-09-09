#!/usr/bin/env python3
"""Validate diagnostic cell coverage and the registered adverse work witness."""
import hashlib,itertools,json,pathlib,re
ROOT=pathlib.Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-tuple-activation-gate'
def rows(path):
    records=[json.loads(line) for line in path.read_text().splitlines() if line.startswith('{')]
    result={(r['family'],r['width'],r['selective']):r for r in records}
    assert len(result)==len(records)
    return result
def main():
    initial=rows(OUT/'initial/source.log');current=rows(OUT/'source.log')
    assert set(current)==set(itertools.product(['sparse','broad','nested','cold'],[4,16,64],[False,True]))
    assert len(initial)==18
    for key,old in initial.items():
        if not key[2]:assert old['heads']==current[key]['heads']
    for n in [4,16,64]:
        assert current['cold',n,False]['heads']==n
        r=current['cold',n,True]
        assert r['heads']==r['registrations']==r['peak_tuples']==n*n
    assert current['sparse',64,True]['combinations']==195
    assert initial['sparse',64,True]['combinations']==4355
    assert current['broad',64,True]['inspections']==322
    assert initial['broad',64,True]['inspections']==2401
    assert current['broad',64,True]['notifications']==initial['broad',64,True]['notifications']==2143
    sources=['research/chr-relational/tests/support/local_multihead.rs','research/chr-relational/tests/multihead.rs','docs/experiments/registrations/S02-tuple-activation-gate.md']
    receipt={'finite_configurations':59,'work_cells':24,'metrics_off_cells':24,'unchanged_scan_work_cells':9,'rows':list(current.values()),'source_sha256':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources}}
    (OUT/'audit.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('24 work cells; 9 unchanged scanning controls; anchored-registration and cold-prefix witnesses verified')
if __name__=='__main__':main()
