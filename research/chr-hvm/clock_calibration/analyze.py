"""Audit raw calibration and screen existing qualification phases without ranking engines."""
import hashlib
import importlib.util
import json
import math
import random
import statistics
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s10-clock-calibration'

def main():
    receipt=json.loads((OUT/'validation.json').read_text())
    for name,digest in receipt['hashes'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    rows=[json.loads(l) for l in (OUT/'runs.jsonl').read_text().splitlines()]
    expected=[(lang,cpu,block) for lang in ['rust','native','python'] for cpu in [0,1] for block in range(7)]
    random.Random(20260910).shuffle(expected)
    assert [(r['language'],r['cpu'],r['block']) for r in rows]==expected
    calibration={};thresholds={}
    for lang in ['rust','native','python']:
        calibration[lang]={};scale=0
        for cpu in [0,1]:
            selected=[r for r in rows if r['language']==lang and r['cpu']==cpu]
            medians=[];p99s=[];extra=[]
            for r in selected:
                d=r['result'];a=sorted(d['samples'])
                assert d['cpu']==cpu and len(a)==10000 and a[0]>=0 and d['baseline_verified']
                assert r['command'][-1]==str(r['block']%2)
                medians.append(statistics.median(a));p99s.append(a[9900])
                extra.append((d['pairs_ns']-d['baseline_ns'])/100000)
            scale=max(scale,*p99s,*extra)
            calibration[lang][cpu]=dict(empty_median_ns=statistics.median(medians),empty_p99_range_ns=[min(p99s),max(p99s)],bulk_extra_pair_ns=dict(min=min(extra),median=statistics.median(extra),max=max(extra)))
        thresholds[lang]=math.ceil(100*scale)
    spec=importlib.util.spec_from_file_location('session_gate',ROOT/'research/chr-hvm/host_session/gate.py')
    session=importlib.util.module_from_spec(spec);spec.loader.exec_module(session)
    wire=session.wire
    screens={}
    def record(owner,phase,value,scale=1):
        key=owner+'/'+phase
        row=screens.setdefault(key,dict(observations=0,flagged=0,threshold_per_pair_ns=thresholds[owner]))
        row['observations']+=1
        row['flagged']+=int(value<=thresholds[owner]*scale)
    rust_path=ROOT/'docs/experiments/results/s10-rust-lifecycle/runs.jsonl'
    for line in rust_path.read_text().splitlines():
        row=json.loads(line);data=bytes.fromhex(row['result']['stdout_hex'])
        if data.startswith(b'UNSUPPORTED'):continue
        values=wire.rust_records(data)
        events=[json.loads(l) for l in row['result']['stderr'].splitlines()][1:]
        for value,e in zip(values,events[:-1]):
            for phase in ['setup_ns','query_drop_ns']:
                record('rust',phase,e[phase])
            if value['answers']:
                record('rust','serialization_per_answer',e['serialization_ns'],len(value['answers']))
        for phase in ['input_load_ns','decode_ns','input_drop_ns','prepare_ns','consumer_setup_ns','query_batch_drop_ns','prepared_drop_ns']:
            record('rust',phase,events[-1][phase])
    host_path=ROOT/'docs/experiments/results/s10-host-session/runs.jsonl'
    for line in host_path.read_text().splitlines():
        row=json.loads(line);result=json.loads(row['stderr']);values=wire.rust_records(bytes.fromhex(row['stdout_hex']))
        for phase,value in result['phases'].items():record('python',phase,value)
        for value,e in zip(values,result['native'][:-1]):
            for phase in ['query_setup_ns','observer_setup_ns','pending_drop_ns','export_ns','query_drop_ns']:
                record('native',phase,e[phase])
            if value['answers']:record('native','serialization_per_answer',e['serialization_ns'],len(value['answers']))
    result=dict(calibration=calibration,screen_threshold_ns=thresholds,screen=screens,screen_is_error_bound=False,architecture_ranking=False,input_hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [rust_path,host_path,Path(__file__)]})
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n')
    density={}
    for lang,path in [('rust',rust_path),('native',host_path)]:
        fractions=[]
        for line in path.read_text().splitlines():
            row=json.loads(line)
            row=row['result'] if lang=='rust' else row
            data=bytes.fromhex(row['stdout_hex'])
            if data.startswith(b'UNSUPPORTED'):continue
            values=wire.rust_records(data)
            events=[json.loads(l) for l in row['stderr'].splitlines()][1:-1] if lang=='rust' else json.loads(row['stderr'])['native'][:-1]
            for value,event in zip(values,events):
                fractions.append((thresholds[lang]/100)*(len(value['answers'])+2)/max(1,event['service_ns']))
        density[lang]=dict(observations=len(fractions),above_one_percent=sum(x>=.01 for x in fractions),median=statistics.median(fractions),maximum=max(fractions))
    (OUT/'service-density.json').write_text(json.dumps(dict(exploratory=True,not_an_error_bound=True,formula='screen scale * (answer count + 2) / service_ns',results=density),indent=2)+'\n')
    print(json.dumps(dict(calibration=calibration,screen_threshold_ns=thresholds),indent=2))
    print('All 42 processes audited; screening is diagnostic, without overhead subtraction.')
if __name__=='__main__':main()
