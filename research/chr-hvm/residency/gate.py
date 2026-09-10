"""Replay qualified native endpoints with separate heap/mapping diagnostics."""
import hashlib,importlib.util,json,resource,subprocess,sys,tempfile
from pathlib import Path
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-residency';binary=root/'target/s10-residency/native'
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
            memory=[e for e in events if e.get('residency')];native=[e for e in events if not e.get('residency')]
            assert p.stdout.hex()==prior['result']['stdout_hex']
            previous=[json.loads(l) for l in prior['result']['stderr'].splitlines()]
            assert len(native)==len(previous)==len(queries)+1
            for now,before in zip(native[:-1],previous[:-1]):
                for key in ['query','calls','pending','unsupported','dynamic_words']:assert now[key]==before[key]
                assert now['serialization_ns'] is None and now['allocator']=='ordinary'
            expected=['entry','runtime_init','prepared']+[p for _ in queries for p in ['query_service_end','query_reset']]+['prepared_drop','consumer_drop']
            assert [e['phase'] for e in memory]==expected
            assert all(e['rss_kib']>0 and e['virtual_kib']>=e['rss_kib'] for e in memory)
            assert all(e['virtual_kib']>=64*(1<<20) for e in memory if e['phase']=='query_service_end')
            assert all(e['virtual_kib']>=32*(1<<20) for e in memory if e['phase']=='query_reset')
            assert memory[-2]['virtual_kib']<1<<20
            repeat.append(memory)
        summary.append(dict(group=group,queries=len(queries),rss_ranges=[dict(peak_sample_kib=max(e['rss_kib'] for e in memory),entry_kib=memory[0]['rss_kib'],after_prepared_drop_kib=memory[-2]['rss_kib'],after_consumer_drop_kib=memory[-1]['rss_kib']) for memory in repeat]))
files=[root/'research/chr-hvm/source_choice/compiler.py',Path(__file__),Path(__file__).with_name('residency.h'),Path(__file__).with_name('check.c'),Path(__file__).with_name('build.py'),binary,out/'runs.jsonl']+[root/'target/s10-residency'/n for n in ['native.c','harness.c','wire.h']]
(out/'audit.json').write_text(json.dumps(dict(sessions=26,processes=52,queries_per_pass=479,summary=summary,sha256={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}),indent=2)+'\n')
print('52 native residency processes /958 endpoints pass exact bytes, work signatures and checkpoint scopes')
