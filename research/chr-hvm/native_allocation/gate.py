"""Replay qualified native endpoints with separate heap/mapping diagnostics."""
import hashlib,importlib.util,json,resource,subprocess,sys,tempfile
from pathlib import Path
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-native-allocation';binary=root/'target/s10-native-allocation/native'
spec=importlib.util.spec_from_file_location('host_gate',root/'research/chr-hvm/host_session/gate.py')
host=importlib.util.module_from_spec(spec);spec.loader.exec_module(host)
sys.path.insert(0,str(root/'research/chr-hvm/host_frontend'))
from frontend import decode
spec=importlib.util.spec_from_file_location('prepared',root/'research/chr-hvm/prepared/gate.py')
prepared=importlib.util.module_from_spec(spec);spec.loader.exec_module(prepared)
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30));resource.setrlimit(resource.RLIMIT_CPU,(20,20))
plans=json.loads((root/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
groups=json.loads((root/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
old=[json.loads(l) for l in (root/'docs/experiments/results/s10-host-frontend/runs.jsonl').read_text().splitlines()]
inputs=[(p['queries'],host.frontend.joined([q['source'] for q in p['queries']])) for p in plans]
inputs += [([dict(source=s,limit=1048576,kind='complete') for s in g],host.frontend.joined(g)) for g in groups]
summary=[]
with tempfile.TemporaryDirectory(prefix='chr-native-meter-') as directory,(out/'runs.jsonl').open('w') as log:
    for group,((queries,text),prior) in enumerate(zip(inputs,old)):
        rules,qs=decode(text);program,preds,atoms=prepared.compile_source(dict(rules=rules,query=[],outputs=[]))
        artifact=Path(directory)/f'rules-{group}.hvm';artifact.write_text(program)
        encoded=[prepared.encode(q,preds,atoms,queries[i]['limit'],queries[i].get('keep',True))[0] for i,q in enumerate(qs)]
        protocol=(str(len(encoded))+'\n'+'\n'.join(encoded)+'\n').encode()
        repeat=[]
        for attempt in range(2):
            p=subprocess.run([binary,artifact],input=protocol,capture_output=True,timeout=30,preexec_fn=bounds)
            row=dict(group=group,repeat=attempt,code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode())
            log.write(json.dumps(row)+'\n');log.flush()
            assert p.returncode==0,row['stderr']
            events=[json.loads(l) for l in p.stderr.splitlines()]
            memory=[e for e in events if e.get('memory')];native=[e for e in events if not e.get('memory')]
            assert p.stdout.hex()==prior['result']['stdout_hex']
            previous=[json.loads(l) for l in prior['result']['stderr'].splitlines()]
            assert len(native)==len(previous)==len(queries)+1
            for now,before in zip(native[:-1],previous[:-1]):
                for key in ['query','calls','pending','unsupported','dynamic_words']:assert now[key]==before[key]
                assert now['serialization_ns'] is None and now['allocator']=='direct-call-meter'
            expected=['runtime_init','prepared','source_drop']+['query_reset']*len(queries)+['prepared_drop','consumer_drop']
            assert [e['phase'] for e in memory]==expected
            assert memory[0]['live']==(256<<20)+20 and memory[0]['mapped']==32<<30
            assert all(e['mapped']==32<<30 for e in memory if e['phase']=='query_reset')
            assert memory[-2]['mapped']==0
            assert memory[-1]['live']==memory[-1]['mapped']==0
            repeat.append(memory)
        assert repeat[0]==repeat[1]
        summary.append(dict(group=group,queries=len(queries),final=repeat[0][-1],consumer_live_after_prepared_drop=repeat[0][-2]['live']))
files=[Path(__file__),Path(__file__).with_name('meter.h'),Path(__file__).with_name('check.c'),Path(__file__).with_name('build.py'),binary,out/'runs.jsonl']+[root/'target/s10-native-allocation'/n for n in ['native.c','harness.c','wire.h']]
(out/'audit.json').write_text(json.dumps(dict(sessions=26,processes=52,queries_per_pass=479,summary=summary,sha256={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}),indent=2)+'\n')
print('52 native diagnostic processes /958 query endpoints pass exact bytes, work signatures, repeatable memory and final release')
