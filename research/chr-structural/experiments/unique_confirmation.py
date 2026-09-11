"""Prospectively fixed compact-vs-direct timing confirmation."""
import gzip, hashlib, itertools, json, os, random, resource, subprocess, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-unique-confirmation';PARENT=ROOT/'docs/experiments/results/s06-unique-output'
MODES=['union','unique','dedup','reduced'];FAMILIES=['overlap','disjoint','redundant','single']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_CPU,(60,60));resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def main():
 assert not (BASE/'freeze.json').exists();p=json.loads((PARENT/'freeze.json').read_text());binary=p['binaries']['time'];assert sha(Path(binary['path']))==binary['sha256'] and 0 in os.sched_getaffinity(0)
 assert json.loads((BASE/'qualification.json').read_text())['qualified']
 scenarios=list(itertools.product(FAMILIES,[1,64],['member','full'],['set','ordered']));cells=[[m,f,8,q,e,'all',c] for f,q,e,c in scenarios for m in MODES]
 blocks=list(itertools.product(range(64),range(32)));rng=random.Random(76050);rng.shuffle(blocks);jobs=[]
 for block,(rep,index) in enumerate(blocks):
  modes=MODES.copy();rng.shuffle(modes)
  for pos,m in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=index*4+MODES.index(m),position=pos))
 warm=list(range(128));random.Random(76051).shuffle(warm)
 for name,data in [('cells',cells),('jobs',jobs),('warmup-order',warm)]: (BASE/f'{name}.json').write_text(json.dumps(data)+'\n')
 paths=['docs/experiments/registrations/S06-unique-confirmation.md','research/chr-structural/experiments/unique_confirmation.py','research/chr-structural/experiments/audit_unique_confirmation.py'];texts={n:(ROOT/n).read_text() for n in paths}
 freeze=dict(binary=binary,source_text=texts,sources={n:sha(ROOT/n) for n in paths},parent={n:sha(PARENT/n) for n in ['freeze.json','sources.zip','build-time.json','audit.json']},orders={n:sha(BASE/f'{n}.json') for n in ['cells','jobs','warmup-order']},qualification_sha256=sha(BASE/'qualification.json'),symbols_sha256=sha(BASE/'symbols.txt'),cpu=0,affinity=sorted(os.sched_getaffinity(0)),platform=os.uname().release,commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
 (BASE/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
 def invoke(args):
  cmd=[binary['path'],*map(str,args)]
  try:
   r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits);return dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
  except subprocess.TimeoutExpired as e:
   dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
   return dict(command=cmd,exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
 for i in range(5):
  raw=invoke(['clock-check']);(BASE/f'clock-{i}.json').write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr']
 start=time.monotonic()
 for kind,order in [('warmup',warm),('runs',jobs)]:
  with gzip.GzipFile(filename=str(BASE/f'{kind}.jsonl.gz'),mode='wb',compresslevel=1,mtime=0) as stream:
   for i,j in enumerate(order):
    assert time.monotonic()-start<1800;index=j if kind=='warmup' else j['index'];raw=invoke(cells[index]);stream.write((json.dumps(dict(sequence=i,index=index,raw=raw))+'\n').encode());stream.flush()
    assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],(kind,i)
    if (i+1)%1024==0:print(kind,i+1,'/',len(order),flush=True)
 assert sha(Path(binary['path']))==binary['sha256'] and all(sha(ROOT/n)==h for n,h in freeze['sources'].items())
 (BASE/'campaign.json').write_text(json.dumps(dict(primary_processes=8192,warmups=128,seconds=time.monotonic()-start,archives={n:sha(BASE/f'{n}.jsonl.gz') for n in ['warmup','runs']}))+'\n')
if __name__=='__main__':main()
