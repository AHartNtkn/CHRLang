"""Verify archived combined-session endpoints and elapsed containment."""
import argparse
import hashlib
import json
from gate import ROOT, wire
from pathlib import Path
parser=argparse.ArgumentParser()
parser.add_argument("--input",type=Path,required=True)
OUT=parser.parse_args().input.resolve()

v=json.loads((OUT/'validation.json').read_text())
for path,digest in v['hashes'].items():
    assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()]
previous=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-host-frontend/runs.jsonl').read_text().splitlines()]
assert len(rows)==len(previous)==v['sessions']==26
assert [(r['family'],r['group']) for r in rows]==[('common',i) for i in range(22)]+[('substantive',i) for i in range(4)]
count=0
for row,old in zip(rows,previous):
    assert row['code']==0 and row['temporary_root_empty']
    record=json.loads(row['stderr'])
    assert record['endpoint']=='batch-owned-wire'
    assert all(e['serialization_ns'] is None and e['compute_traverse_ns'] is None for e in record['native'][:-1])
    phases=record['phases']
    assert set(phases)=={'input_load_ns','decode_ns','input_drop_ns','emission_ns','artifact_write_ns','query_encoding_ns','native_process_ns','consumer_assembly_ns','prepared_artifact_drop_ns','publication_ns','consumer_drop_ns'}
    assert all(n>=0 for n in phases.values())
    assert sum(phases.values())==record['named_ns']<=record['session_elapsed_ns']<=row['process_elapsed_ns']
    assert record['native'][-1]['lifecycle_ns']<=phases['native_process_ns']
    values=wire.rust_records(bytes.fromhex(row['stdout_hex']))
    payloads=wire.records(bytes.fromhex(old['result']['stdout_hex']))
    events=[json.loads(l) for l in old['result']['stderr'].splitlines()]
    assert len(values)==old['queries']==len(events)-1==len(record['native'])-1
    for i,value in enumerate(values):
        ps,ats=old['validation_snapshot']['dictionaries'][i]
        assert value['index']==i and value['predicates']==ps and value['atoms']==ats
        assert value['answers']==wire.decode(payloads[i],ps,ats)
        assert value['exhausted']==(not events[i]['pending'])
        for key in ['query','calls','pending','unsupported','dynamic_words']:
            assert record['native'][i][key]==events[i][key]
    count+=len(values)
assert count==v['queries']==479
print('26 combined sessions and 479 exact owned endpoints audited; phase containment and recorded artifact cleanup pass.')
