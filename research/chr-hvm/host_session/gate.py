"""Validate combined lifecycle from source text through external owned bytes."""
import hashlib
import importlib.util
import json
import resource
import subprocess
import sys
import tempfile
from pathlib import Path
from time import perf_counter_ns as clock
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s10-host-session'
sys.path.insert(0, str(ROOT / 'research/chr-hvm/host_frontend'))
spec = importlib.util.spec_from_file_location('frontend_gate', ROOT / 'research/chr-hvm/host_frontend/gate.py')
frontend = importlib.util.module_from_spec(spec);spec.loader.exec_module(frontend)
wire = frontend.wire

def bounds():
    resource.setrlimit(resource.RLIMIT_AS, (96<<30, 96<<30))
    resource.setrlimit(resource.RLIMIT_CPU, (20, 20))

def main():
    plans = json.loads((ROOT/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
    groups = json.loads((ROOT/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
    old = [json.loads(l) for l in (ROOT/'docs/experiments/results/s10-host-frontend/runs.jsonl').read_text().splitlines()]
    inputs = [(p['queries'], frontend.joined([q['source'] for q in p['queries']])) for p in plans]
    inputs += [([dict(source=s,limit=1048576,kind='complete') for s in g],frontend.joined(g)) for g in groups]
    with tempfile.TemporaryDirectory(prefix='chr-host-gate-') as directory, (OUT/'runs.jsonl').open('w') as log:
        for (queries, text), previous in zip(inputs, old):
            limits = ','.join(str(q['limit']) for q in queries)
            command = [sys.executable,str(Path(__file__).with_name('run.py')),limits,directory]
            start=clock()
            p=subprocess.run(command,input=text.encode(),capture_output=True,timeout=30,preexec_fn=bounds)
            elapsed=clock()-start
            row=dict(family=previous['family'],group=previous['group'],command=command,code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode(),process_elapsed_ns=elapsed,temporary_root_empty=not list(Path(directory).iterdir()))
            log.write(json.dumps(row)+'\n');log.flush()
            assert p.returncode==0,row['stderr']
            assert row['temporary_root_empty']
            result=json.loads(row['stderr'])
            values=wire.rust_records(p.stdout)
            expected=wire.records(bytes.fromhex(previous['result']['stdout_hex']))
            old_events=[json.loads(l) for l in previous['result']['stderr'].splitlines()]
            assert len(values)==len(queries)==len(result['native'])-1
            for i,(q,value) in enumerate(zip(queries,values)):
                ps,ats=previous['validation_snapshot']['dictionaries'][i]
                assert value['index']==i and value['predicates']==ps and value['atoms']==ats
                answers=wire.decode(expected[i],ps,ats)
                assert value['answers']==answers
                assert value['exhausted']==(not old_events[i]['pending'])
                for key in ['query','calls','pending','unsupported','dynamic_words']:
                    assert result['native'][i][key]==old_events[i][key]
                if q.get('kind','complete')=='complete':
                    assert sorted(map(wire.common.normalize,value['answers']))==sorted(map(wire.common.normalize,q['source']['expected']))
            assert result['named_ns']==sum(result['phases'].values())
            assert 0<=result['named_ns']<=result['session_elapsed_ns']<=elapsed
            assert result['native'][-1]['lifecycle_ns']<=result['phases']['native_process_ns']
            print(row['family'],row['group'],'combined session passes',flush=True)
    paths=list(Path(__file__).parent.glob('*.py'))+[OUT/'runs.jsonl',ROOT/'target/s10-answer-wire/ordinary/run',ROOT/'docs/experiments/registrations/S10-host-session.md']
    paths += [ROOT/p for p in ['research/chr-hvm/host_frontend/frontend.py','research/chr-hvm/host_frontend/gate.py','research/chr-hvm/prepared/gate.py','research/chr-hvm/source_choice/compiler.py','research/chr-hvm/identity/build_kernel.py','research/chr-hvm/identity/kernel.hvm','docs/experiments/results/s10-host-frontend/runs.jsonl','docs/experiments/results/s10-native-prepared/plans.json','docs/experiments/results/s10-native-prepared/symbol-references.json','docs/experiments/results/s10-native-substantive/groups.json']]
    (OUT/'validation.json').write_text(json.dumps(dict(sessions=26,queries=479,comparative_timing=False,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
    print('479 combined host/native queries pass.')
if __name__=='__main__':main()
