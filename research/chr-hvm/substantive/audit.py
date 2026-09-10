"""Audit the full screen, resolved cutoffs and prepared-session observations."""
import collections
import hashlib
import json
from pathlib import Path
from screen import ROOT, OUT, prepared, common
from extend import BUILD, fields

def rows(name):
    return [json.loads(l) for l in (OUT / name).read_text().splitlines()]

def observations(result, ps, ats):
    return {i: sorted(prepared.normalize(prepared.parse(l, ps, ats)) for l in text.splitlines())
            for i, text in prepared.output_records(result['stdout']).items()}

def main():
    v = json.loads((OUT / 'validation.json').read_text())
    for path, digest in v['hashes'].items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
    supplement=json.loads((OUT/'supplement-hashes.json').read_text())
    for path,digest in supplement.items():
        assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
    admission=json.loads((OUT/'finite-admission.json').read_text());assert len(admission)==4
    assert all(r['code']==0 and r['stdout'].count('private rule must be unguarded single-head consumption')==4 for r in admission)
    sources = json.loads((OUT / 'cases.json').read_text()); assert len(sources) == 96
    byname = {s['name']: s for s in sources}; assert len(byname) == 96
    refs = json.loads((OUT / 'references.json').read_text()); assert len(refs) == 96
    for s, ref in zip(sources, refs):
        assert s['name'] == ref['case']
        p = s['parameters']; expected_count = 1 if p['failure'] else 2 ** p['choices']
        assert len(s['expected']) == expected_count
        for output, residual in s['expected']:
            counts = collections.Counter(name for name, _ in residual)
            assert counts == collections.Counter(dict(ready=1,watch=1,seen=p['chain']+1,edge=p['noise']))
            assert len(output) == p['choices'] and all(a in ['a', 'b'] for a in output)
            assert not p['failure'] or output == ['a'] * p['choices']
            assert [args for name,args in residual if name=='seen'] == [[output[0]]] * (p['chain']+1)
        header, body = ref['result']['stdout'].split('\n',1)
        exhausted, count = header.split(); assert exhausted == 'true' and int(count) == expected_count
        parsed = common.parse(f'QUERY 0 {exhausted} {count}\n' + body)[0]
        assert sorted(map(common.normalize, parsed['answers'])) == sorted(map(common.normalize, s['expected']))
    groups = json.loads((OUT / 'groups.json').read_text()); assert len(groups) == 4
    assert {s['name'] for group in groups for s in group} == set(byname)
    for group in groups:
        assert len(group) == 24 and all(s['rules'] == group[0]['rules'] for s in group)
    rust = rows('rust.jsonl'); assert len(rust) == 48
    rust_pass = rust_unsupported = 0
    for r in rust:
        group = groups[r['group']]; result = r['result']; assert result['code'] == 0
        if r['mode'] in ['prefix','finite']:
            assert result['stdout'].startswith('UNSUPPORTED '); rust_unsupported += len(group)
        else:
            actual = common.parse(result['stdout']); assert len(actual) == len(group)
            for i, (s,a) in enumerate(zip(group,actual)):
                assert a['index']==i and a['exhausted']
                assert sorted(map(common.normalize,a['answers'])) == sorted(map(common.normalize,s['expected']))
                rust_pass += 1
    initial = {(r['mode'],r['case']):r for r in rows('native.jsonl')}
    ext = {(r['mode'],r['case']):r for r in rows('extension.jsonl')}
    assert len(initial) == len(ext) == 192
    cutoffs = complete_replays = 0; max_calls = max_words = 0
    for key, r in ext.items():
        s = byname[r['case']]; result = r['result']; assert result['code'] == 0
        program, preds, atoms = prepared.compile_source(dict(rules=s['rules'],query=[],outputs=[]))
        assert (OUT/f"rules-{r['group']}.hvm").read_text() == program
        _, ps, ats = prepared.encode(s,preds,atoms,1048576,True)
        assert observations(result,ps,ats)=={0:sorted(map(prepared.normalize,s['expected']))}
        e = fields(result)[0]; assert e['pending']==e['unsupported']==0 and e['calls']<=1048576
        max_calls=max(max_calls,e['calls']);max_words=max(max_words,e['dynamic_words'])
        old = initial[key]['result']; assert old['code']==0
        old_e = fields(old)[0]
        if old_e['pending']:
            assert old_e['calls']==65536;cutoffs+=1
            assert not (collections.Counter(observations(old,ps,ats)[0])-collections.Counter(observations(result,ps,ats)[0]))
        else:
            assert result['stdout']==old['stdout'] and fields(result)==fields(old);complete_replays+=1
        other = ext[('ordinary' if r['mode']=='ownership' else 'ownership',r['case'])]['result']
        assert result['stdout']==other['stdout'] and fields(result)==fields(other)
    prefixes=rows('prefix-replays.jsonl'); assert len(prefixes)==cutoffs==20
    for r in prefixes:
        old=initial[r['mode'],r['case']]['result']; result=r['result']
        assert result['code']==0 and result['stdout']==old['stdout'] and fields(result)==fields(old)
    reuse=rows('reuse.jsonl');assert len(reuse)==8
    for r in reuse:
        result=r['result'];assert result['code']==0
        group=groups[r['group']];values=prepared.output_records(result['stdout']);events=fields(result)
        assert set(values)==set(range(len(group))) and len(events)==len(group)
        for i,s in enumerate(group):
            old=ext[r['mode'],s['name']]['result']
            assert values[i]==prepared.output_records(old['stdout'])[0] and events[i]==dict(fields(old)[0],query=i)
        if r['mode']=='ownership':
            raw=[json.loads(l) for l in result['stderr'].splitlines()]
            assert all(e['prepared_unchanged'] and e['query_restored'] for e in raw[:-1])
            assert raw[-1]['tracked_live']==0 and raw[-1]['consumer_survives_prepared_drop']
    for mode,old in [('ownership','s10-native-prepared'),('ordinary','s10-native-timing-entry')]:
        original=(ROOT/'target'/old/'harness.c').read_text()
        assert (BUILD/mode/'harness.c').read_text()==original.replace('*limit<=65536','*limit<=1048576')
        assert (BUILD/mode/'native.c').read_bytes()==(ROOT/'target'/old/'native.c').read_bytes()
    result=dict(sources=96,rulesets=4,reference_checks=96,rust_complete_checks=rust_pass,
                rust_unsupported=rust_unsupported,initial_native_queries=192,initial_cutoffs=cutoffs,
                extended_complete_queries=192,unchanged_complete_replays=complete_replays,
                cutoff_prefix_replays=20,prepared_reuse_queries=192,max_native_calls=max_calls,
                max_dynamic_words=max_words,ownership_final_live=0,comparative_timing=False)
    (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(result)

if __name__=='__main__':main()
