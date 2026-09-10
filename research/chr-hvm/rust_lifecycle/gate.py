"""Complete observations and lifecycle arithmetic; no cost classifications."""
import hashlib,importlib.util,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-rust-lifecycle';BINARY=ROOT/'target/release/examples/native_lifecycle'
def load(name,path):
    s=importlib.util.spec_from_file_location(name,ROOT/path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
sys.path.insert(0,str(ROOT/'research/chr-hvm/answer_wire'))
wire=load('wire_gate','research/chr-hvm/answer_wire/gate.py');common=wire.common

def invoke(mode,text,limit=200000):
    p=subprocess.run([str(BINARY),str(mode),str(limit)],input=text.encode(),capture_output=True,timeout=15,preexec_fn=common.bounds)
    return dict(code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode())
def check(row,group,complete=True):
    assert row['code']==0,row
    events=[json.loads(l) for l in row['stderr'].splitlines()]
    flags=events.pop(0);assert flags==dict(configuration=True,compiled_metrics=False,observe_metrics=False,conditional_metrics=False)
    data=bytes.fromhex(row['stdout_hex'])
    if data.startswith(b'UNSUPPORTED '):return None
    values=wire.rust_records(data);assert len(values)==len(group) and len(events)==len(group)+1
    for i,(s,a,e) in enumerate(zip(group,values,events)):
        assert a['index']==e['query']==i and a['exhausted']==e['exhausted']
        if complete:
            assert a['exhausted']==(not s['ongoing']) and sorted(map(common.normalize,a['answers']))==sorted(map(common.normalize,s['expected'])),s['name']
        else:assert not (set(map(common.normalize,a['answers']))-set(map(common.normalize,s['expected'])))
        assert e['wire_bytes']==a['bytes'] and e['wire_capacity']>=e['wire_bytes']
        assert (e['first_observation_ns'] is not None)==bool(a['answers'])
        if e['first_observation_ns'] is not None:assert 0<=e['first_observation_ns']<=e['service_ns']
        assert e['service_ns']==e['serialization_ns']+e['compute_observe_ns']
        assert e['query_total_ns']==e['setup_ns']+e['service_ns']+e['query_drop_ns']
    end=events[-1];assert end['session_disposed']
    assert end['lifecycle_ns']==sum(end[k] for k in ['input_load_ns','decode_ns','input_drop_ns','prepare_ns','consumer_setup_ns','query_batch_drop_ns','prepared_drop_ns','consumer_drop_ns'])+sum(e['query_total_ns'] for e in events[:-1])
    return values

def joined(group):
    text=group[0]['input'].rstrip('\n')
    for s in group[1:]:text+='\nNEXT\n'+'\n'.join(s['input'].splitlines()[:2])
    return text+'\n'
def main():
    counts={};representatives={}
    with (OUT/'runs.jsonl').open('w') as log:
        for family in ['common','substantive']:
            folder='s10-native-common-source' if family=='common' else 's10-native-substantive'
            groups=json.loads((ROOT/'docs/experiments/results'/folder/'groups.json').read_text())
            for g,group in enumerate(groups):
                for mode in range(13):
                    r=invoke(mode,joined(group));log.write(json.dumps(dict(family=family,group=g,mode=mode,result=r))+'\n');log.flush()
                    a=check(r,group)
                    status='unsupported' if a is None else 'passed';counts[status]=counts.get(status,0)+len(group)
                    if a is not None and mode not in representatives and not group[0]['ongoing']:representatives[mode]=group[0]
            print(family,'complete lifecycle observations pass',flush=True)
    assert set(representatives)==set(range(13))
    with (OUT/'cancellations.jsonl').open('w') as log:
        for mode,s in representatives.items():
            full=invoke(mode,s['input']);values=check(full,[s]);assert values is not None and values[0]['exhausted']
            log.write(json.dumps(dict(mode=mode,limit=200000,source=s,result=full))+'\n');log.flush()
            expected=list(map(common.normalize,values[0]['answers']))
            for limit in [0,1,64]:
                r=invoke(mode,s['input'],limit);log.write(json.dumps(dict(mode=mode,limit=limit,source=s,result=r))+'\n');log.flush()
                a=check(r,[s],False);assert a is not None
                got=list(map(common.normalize,a[0]['answers']));assert got==expected[:len(got)]
                e=json.loads(r['stderr'].splitlines()[1]);assert e['calls']<=limit
                if limit==0:assert e['calls']==0 and not a[0]['answers'] and not e['exhausted']
    paths=[Path(__file__),BINARY,ROOT/'research/chr-direct-conditional/examples/native_lifecycle.rs',ROOT/'research/chr-direct-conditional/examples/native_common_source.rs',ROOT/'research/chr-direct-conditional/examples/support/answer_wire.rs',ROOT/'docs/experiments/registrations/S10-rust-lifecycle.md']
    paths += [OUT/n for n in ['runs.jsonl','cancellations.jsonl','release-build.log','features.txt','initial-answer-wire.rs']]
    (OUT/'validation.json').write_text(json.dumps(dict(counts=counts,processes=338,cancellation_queries=39,complete_rechecks=13,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n');print(counts)
if __name__=='__main__':main()
