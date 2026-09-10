import importlib.util,json,subprocess,tempfile,sys,hashlib
from pathlib import Path
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-residency'
spec=importlib.util.spec_from_file_location('host_gate',root/'research/chr-hvm/host_session/gate.py')
host=importlib.util.module_from_spec(spec);spec.loader.exec_module(host)
plans=json.loads((root/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
groups=json.loads((root/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
inputs=[(p['queries'],host.frontend.joined([q['source'] for q in p['queries']])) for p in plans]
inputs += [([dict(source=s,limit=1048576,kind='complete') for s in g],host.frontend.joined(g)) for g in groups]
prior=[json.loads(l) for l in (root/'docs/experiments/results/s10-host-primary/runs.jsonl').read_text().splitlines()]
summary=[]
with tempfile.TemporaryDirectory(prefix='chr-host-memory-') as directory,(out/'host.jsonl').open('w') as log:
 for group,((queries,text),old) in enumerate(zip(inputs,prior)):
  for repeat in range(2):
   command=[sys.executable,str(Path(__file__).with_name('host.py')),','.join(str(q['limit']) for q in queries),directory,str(root/'target/s10-publication-clock/native-primary')]
   p=subprocess.run(command,input=text.encode(),capture_output=True,timeout=30,preexec_fn=host.bounds)
   row=dict(group=group,repeat=repeat,code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode(),temporary_root_empty=not list(Path(directory).iterdir()))
   log.write(json.dumps(row)+'\n');log.flush()
   assert p.returncode==0 and row['temporary_root_empty'],row['stderr']
   assert row['stdout_hex']==old['stdout_hex']
   execution,memory=map(json.loads,p.stderr.splitlines())
   old_execution=json.loads(old['stderr'])
   for a,b in zip(execution['native'][:-1],old_execution['native'][:-1]):
    for key in ['query','calls','pending','unsupported','dynamic_words']:assert a[key]==b[key]
   assert memory['host_memory'] and [p['phase'] for p in memory['phases']]==list(execution['phases'])
   assert all(p['traced_peak']>=p['traced_current']>=0 and p['host_rss_kib']>0 for p in memory['phases'])
   summary.append(dict(group=group,repeat=repeat,maximum_phase_traced_bytes=max(p['traced_peak'] for p in memory['phases']),maximum_phase_host_rss_kib=max(p['host_rss_kib'] for p in memory['phases']),returned_traced_bytes=memory['returned_current']))
files=[Path(__file__),Path(__file__).with_name('host.py'),root/'research/chr-hvm/host_session/run.py',root/'target/s10-publication-clock/native-primary',out/'host.jsonl']
(out/'host-audit.json').write_text(json.dumps(dict(processes=52,query_endpoints=958,summary=summary,sha256={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}),indent=2)+'\n')
print('52 traced host processes /958 exact endpoints and cleanup pass; traces are diagnostic scopes, not total heap traffic')
