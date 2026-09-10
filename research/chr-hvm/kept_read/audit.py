"""Audit checked-read admissions, caller repair and historical source correspondence."""
import hashlib
import json
import re
from gate import ROOT, OUT, common, verify

def read_rows(path):
    return [json.loads(l) for l in path.read_text().splitlines()]

def main():
    v=json.loads((OUT/'validation.json').read_text())
    for path,digest in v['hashes'].items():
        assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
    supplement=json.loads((OUT/'supplement-hashes.json').read_text())
    for path,digest in supplement.items():
        assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
    oldpaths={
        'research/chr-direct-conditional/examples/native_common_source.rs':OUT/'initial-common-source.rs',
        'target/debug/examples/native_common_source':ROOT/'target/s10-finite-kept-read/initial-common-source',
    }
    historical=[]
    for name in ['s10-native-common-source','s10-native-substantive']:
        prior=ROOT/'docs/experiments/results'/name/'validation.json'
        data=json.loads(prior.read_text());count=0
        for path,digest in data['hashes'].items():
            actual=oldpaths.get(path,ROOT/path)
            assert hashlib.sha256(actual.read_bytes()).hexdigest()==digest,(name,path)
            count+=1
        historical.append(dict(package=name,hashes_checked=count,snapshot_overrides=[p for p in data['hashes'] if p in oldpaths]))
    (OUT/'historical-validation.json').write_text(json.dumps(historical,indent=2)+'\n')
    oldroot=ROOT/'docs/experiments/results/s10-native-common-source'
    old={(r['group'],r['mode']):r for r in read_rows(oldroot/'runs.jsonl')}
    replays=read_rows(OUT/'common-replays.jsonl');assert len(replays)==264
    assert {(r['group'],r['mode']) for r in replays}==set(old)
    for r in replays:
        x=r['result'];before=old[r['group'],r['mode']]
        assert x['code']==0 and x['stdout']==before['stdout'] and x['stderr']==before['stderr']
    groups=json.loads((ROOT/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
    sources={s['name']:s for g in groups for s in g}
    rows=read_rows(OUT/'substantive.jsonl');assert len(rows)==96 and {r['case'] for r in rows}==set(sources)
    admitted=unsupported=0
    for r in rows:
        s=sources[r['case']];x=r['result'];assert x['code']==0
        if s['parameters']['late']:
            assert x['stdout']=='UNSUPPORTED Source("initial kept-read occurrences are missing")\n';unsupported+=1
        else:verify([s],x);admitted+=1
    reused=read_rows(OUT/'reuse.jsonl');assert len(reused)==2
    for r in reused:verify(groups[r['group']],r['result'])
    probes=json.loads((OUT/'caller-probes.json').read_text());assert len(probes)==3
    for p in probes:
        assert p['result']['code']==p['scan']['code']==p['reference']['code']==0
        header,body=p['reference']['stdout'].split('\n',1)
        assert p['result']['stdout']==p['scan']['stdout']=='QUERY 0 '+header+'\n'+body
    initial=json.loads((OUT/'late-private-probe.json').read_text())
    assert initial[1]['stdout']!=initial[2]['stdout']
    header,body=initial[0]['stdout'].split('\n',1)
    assert initial[1]['stdout']=='QUERY 0 '+header+'\n'+body==probes[0]['result']['stdout']
    for mode in ['off','default']:
        text=(OUT/f'tests-{mode}.log').read_text()
        counts=list(map(int,re.findall(r'test result: ok\. (\d+) passed;',text)))
        assert sorted(counts)==[1,2,9,11] and 'FAILED' not in text
    assert 'error:' not in (OUT/'clippy.log').read_text()
    result=dict(common_exact_replays=264,substantive_admitted=admitted,substantive_unsupported=unsupported,
                reused_queries=48,independent_caller_probes=3,semantic_tests_per_feature_build=23,
                caller_adapter_repaired=True,finite_solver_changed=False,comparative_costs=False)
    (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result)

if __name__=='__main__':main()
