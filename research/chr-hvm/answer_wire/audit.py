"""Audit full observations, consumer ownership, phase arithmetic and source freezes."""
import hashlib,json,re
from gate import ROOT,OUT,BUILD,prepared,common,native_check,rust_records
from codec import records

def rows(name):return [json.loads(l) for l in (OUT/name).read_text().splitlines()]
def phases(row):
    if row.get('mode')!='ordinary':return
    result=row['result'];payloads=records(bytes.fromhex(result['stdout_hex']))
    events=[json.loads(l) for l in result['stderr'].splitlines()]
    for e in events[:-1]:
        assert (e['first_observation_ns'] is not None)==bool(payloads[e['query']])
        if e['first_observation_ns'] is not None:assert 0<=e['first_observation_ns']<=e['service_ns']
        assert e['serialization_ns']+e['compute_traverse_ns']==e['service_ns']
        assert e['query_total_ns']==sum(e[p] for p in ['query_setup_ns','observer_setup_ns','service_ns','pending_drop_ns','export_ns','query_drop_ns'])
    final=events[-1]
    assert final['lifecycle_ns']==sum(final[p] for p in ['runtime_init_ns','source_load_ns','prepare_ns','source_drop_ns','consumer_setup_ns','prepared_drop_ns','consumer_drop_ns'])+sum(e['query_total_ns']+e['consumer_drop_ns'] for e in events[:-1])

def main():
    v=json.loads((OUT/'validation.json').read_text())
    for path,h in v['hashes'].items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
    supplement=json.loads((OUT/'supplement-hashes.json').read_text())
    for path,h in supplement.items():assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==h,path
    oldv=json.loads((ROOT/'docs/experiments/results/s10-finite-kept-read/validation.json').read_text())
    for path,snapshot in [('research/chr-direct-conditional/examples/native_common_source.rs',OUT/'initial-common-source.rs'),('target/debug/examples/native_common_source',BUILD/'initial-common-source')]:
        assert hashlib.sha256(snapshot.read_bytes()).hexdigest()==oldv['hashes'][path]
    plans=json.loads((ROOT/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
    old=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-native-prepared/runs.jsonl').read_text().splitlines()]
    first=rows('native-prepared.jsonl');sanitized=rows('sanitized.jsonl');assert len(first)==44 and len(sanitized)==22
    for r in first+sanitized:native_check(plans[r['group']],r['result'],old[r['group']]);phases(r)
    original={(r['group'],r['mode']):r['result'] for r in first}
    for r in sanitized:assert r['result']==original[r['group'],'ownership']
    root=ROOT/'docs/experiments/results/s10-native-substantive';groups=json.loads((root/'groups.json').read_text())
    old={(r['group'],r['mode']):r['result'] for r in map(json.loads,(root/'reuse.jsonl').read_text().splitlines())}
    substantive=rows('native-substantive.jsonl');assert len(substantive)==8
    for r in substantive:
        group=groups[r['group']];_,preds,atoms=prepared.compile_source(dict(rules=group[0]['rules'],query=[],outputs=[]));qs=[]
        for s in group:
            _,ps,ats=prepared.encode(s,preds,atoms,1048576,True);qs.append(dict(source=s,predicates=ps,atoms=ats))
        native_check(dict(queries=qs),r['result'],old[r['group'],r['mode']]);phases(r)
    counts={};rust=rows('rust.jsonl');assert len(rust)==338
    families={'common':json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text()),'substantive':groups}
    for r in rust:
        group=families[r['family']][r['group']];result=r['result'];assert result['code']==0;data=bytes.fromhex(result['stdout_hex'])
        if data.startswith(b'UNSUPPORTED '):assert r['mode'] in [10,11,12];counts['unsupported']=counts.get('unsupported',0)+len(group);continue
        decoded=rust_records(data);assert len(decoded)==len(group)
        for i,(s,a) in enumerate(zip(group,decoded)):
            _,ps,ats=prepared.compile_source(dict(rules=s['rules'],query=[],outputs=[]));_,ps,ats=prepared.encode(s,ps,ats,65536,True)
            assert a['predicates']==ps and a['atoms']==ats and a['index']==i and a['exhausted']==(not s['ongoing'])
            assert sorted(map(common.normalize,a['answers']))==sorted(map(common.normalize,s['expected']))
        counts['passed']=counts.get('passed',0)+len(group)
    assert counts==v['rust']==dict(passed=2286,unsupported=587)
    prior={(r['group'],r['mode']):r['result'] for r in map(json.loads,(ROOT/'docs/experiments/results/s10-finite-kept-read/common-replays.jsonl').read_text().splitlines())}
    regressions=rows('ordinary-output-replays.jsonl');assert len(regressions)==264
    for r in regressions:
        result=r['result'];old=prior[r['group'],r['mode']]
        assert result['code']==0 and bytes.fromhex(result['stdout_hex']).decode()==old['stdout'] and result['stderr']==old['stderr']
    for mode in ['ownership','ordinary']:
        native=(BUILD/mode/'native.c').read_text();prior=(ROOT/'target/s10-native-substantive'/mode/'native.c').read_text()
        if mode=='ownership':assert native.replace('    wire_publish(term);','    print_term_quoted(term); printf("\\n");')==prior
        else:assert native==prior
        harness=(BUILD/mode/'harness.c').read_text();old=(ROOT/'target/s10-native-substantive'/mode/'harness.c').read_text()
        for start,end in [('static Term ctr(','static void session_free(')] if mode=='ordinary' else [('static Term ctr(','static OwnedAnswer execute(')]:
            assert old[old.index(start):old.index(end)] in harness
        assert 'open_memstream' not in harness
    for mode in ['off','default']:
        text=(OUT/f'tests-{mode}.log').read_text();assert sorted(map(int,re.findall(r'test result: ok\. (\d+) passed;',text)))==[2,9]
    result=dict(native_queries=958,sanitized_exact_queries=383,rust_complete_checks=2286,rust_unsupported=587,ordinary_output_exact_replays=264,ownership_final_live=0,rust_tests_per_build=11,comparative_costs=False)
    (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
if __name__=='__main__':main()
