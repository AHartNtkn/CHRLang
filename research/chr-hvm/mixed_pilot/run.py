"""Frozen complete-query eligibility followed by the registered primary timing pilot."""
from pathlib import Path
import hashlib,importlib.util,json,os,random,resource,subprocess,sys,tempfile,time
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-mixed-pilot'
def load(name,path):
 spec=importlib.util.spec_from_file_location(name,root/path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
rust=load('rust_lifecycle','research/chr-hvm/rust_lifecycle/gate.py')
host=load('host_gate','research/chr-hvm/host_session/gate.py')
build=root/'target/s10-publication-clock'
known=json.loads((root/'docs/experiments/results/s10-publication-clock/validation.json').read_text())['hashes']
for name in ['rust-primary','native-primary']:assert hashlib.sha256((build/name).read_bytes()).hexdigest()==known[f'target/s10-publication-clock/{name}']
assert {0,1}<=os.sched_getaffinity(0)
selection=[];excluded=[]
for family,folder in [('common','s10-native-common-source'),('substantive','s10-native-substantive')]:
 groups=json.loads((root/'docs/experiments/results'/folder/'groups.json').read_text())
 for index,group in enumerate(groups):
  complete=[s for s in group if not s['ongoing']]
  if not complete:excluded.append(dict(family=family,group=index,reason='ongoing-only'));continue
  selection.append(dict(family=family,group=index,sources=complete,text=rust.joined(complete)))
assert len(selection)==24
(out/'selection.json').write_text(json.dumps(dict(batches=selection,excluded=excluded),indent=2)+'\n')
start_all=time.monotonic();cpu_start=resource.getrusage(resource.RUSAGE_CHILDREN)
def invoke(batch,mode,cpu,directory):
 usage=resource.getrusage(resource.RUSAGE_CHILDREN)
 assert time.monotonic()-start_all<1200 and usage.ru_utime+usage.ru_stime-cpu_start.ru_utime-cpu_start.ru_stime<600,'whole-pilot resource bound'
 def bounds():
  os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_CPU,(20,20));resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30))
 command=([build/'rust-primary',mode,200000] if mode!='native' else [sys.executable,root/'research/chr-hvm/host_session/run.py',1048576,directory,build/'native-primary'])
 text=selection[batch]['text'].encode()
 start=time.perf_counter_ns();p=subprocess.run(list(map(str,command)),input=text,capture_output=True,timeout=30,preexec_fn=bounds);elapsed=time.perf_counter_ns()-start
 row=dict(batch=batch,mode=mode,cpu=cpu,process_ns=elapsed,result=dict(code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode()))
 assert p.returncode==0,row
 if mode!='native':
  values=rust.check(row['result'],selection[batch]['sources'])
  row['supported']=values is not None
  if values is not None:
   events=[json.loads(l) for l in p.stderr.splitlines()][1:]
   assert all(e['serialization_ns'] is None for e in events[:-1])
   row['service_ns']=sum(e['service_ns'] for e in events[:-1]);row['named_ns']=events[-1]['lifecycle_ns']
 else:
  events=json.loads(p.stderr);values=rust.wire.rust_records(p.stdout)
  assert len(values)==len(selection[batch]['sources']) and not list(Path(directory).iterdir())
  for source,value,event in zip(selection[batch]['sources'],values,events['native'][:-1]):
   assert value['exhausted'] and not event['pending'] and not event['unsupported']
   assert sorted(map(rust.common.normalize,value['answers']))==sorted(map(rust.common.normalize,source['expected']))
   assert event['serialization_ns'] is None
  row['supported']=True;row['service_ns']=sum(e['service_ns'] for e in events['native'][:-1]);row['named_ns']=events['named_ns']
 return row
modes=list(range(13))+['native'];eligible=[]
with tempfile.TemporaryDirectory(prefix='chr-mixed-pilot-') as directory:
 with (out/'eligibility.jsonl').open('w') as log:
  for batch in range(24):
   for mode in modes:
    row=invoke(batch,mode,0,directory);log.write(json.dumps(row)+'\n');log.flush()
    if row['supported']:eligible.append((batch,mode))
 (out/'eligible.json').write_text(json.dumps(eligible)+'\n')
 print(f'Eligibility: {len(eligible)}/336 pairs complete and admitted',flush=True)
 warm=[(b,m,c,-1) for b,m in eligible for c in [0,1]]
 jobs=[(b,m,c,r) for b,m in eligible for c in [0,1] for r in range(5)]
 rng=random.Random(20260917);rng.shuffle(warm);rng.shuffle(jobs)
 with (out/'runs.jsonl').open('w') as log:
  for i,(batch,mode,cpu,rep) in enumerate(warm+jobs):
   row=invoke(batch,mode,cpu,directory);assert row['supported'];row['repetition']=rep
   log.write(json.dumps(row)+'\n');log.flush()
   if (i+1)%100==0:print(f'{i+1}/{len(warm)+len(jobs)} timing/warmup processes validated',flush=True)
paths=[Path(__file__),out/'selection.json',out/'eligible.json',out/'eligibility.jsonl',out/'runs.jsonl',build/'rust-primary',build/'native-primary']+[root/p for p in ['research/chr-hvm/host_session/run.py','research/chr-hvm/source_choice/compiler.py','research/chr-hvm/host_frontend/frontend.py','docs/experiments/registrations/S10-mixed-pilot.md']]
(out/'validation.json').write_text(json.dumps(dict(eligible=len(eligible),warmups=len(warm),measured=len(jobs),hashes={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
print('Registered pilot complete',flush=True)
