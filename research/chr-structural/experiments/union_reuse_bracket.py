"""Prospective allocation-only bracket after the main reuse extension."""
import hashlib,itertools,json,os,random,resource,subprocess,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s06-union-reuse/bracket';CPU=json.loads((BASE.parent/'freeze.json').read_text())['cpu']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
 os.sched_setaffinity(0,{CPU});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
 BASE.mkdir(exist_ok=True);assert not (BASE/'freeze.json').exists()
 cmd=['cargo','build','-p','chr-structural','--example','union_lifecycle','--release','--no-default-features','--features','alloc-meter','--target-dir','target/s06-union-reuse-bracket','--message-format=json'];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=300);(BASE/'build.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0
 binary=Path(next(json.loads(l)['executable'] for l in r.stdout.splitlines() if json.loads(l).get('executable')))
 f=json.loads((BASE.parent/'freeze.json').read_text());paths=set(f['sources'])|{'research/chr-structural/experiments/union_reuse_bracket.py','docs/experiments/registrations/S06-union-reuse-bracket.md'};hashes={p:sha(ROOT/p) for p in sorted(paths)}
 with zipfile.ZipFile(BASE/'sources.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
  for p in hashes:z.write(ROOT/p,p)
 jobs=[dict(repeat=rep,args=[mode,'overlap',8,q,'full',retention]) for rep,mode,q,retention in itertools.product(range(2),['union','separate','reduced','names','explicit','dedup'],[12,13],['immediate','window','all'])];random.Random(76033).shuffle(jobs);(BASE/'order.json').write_text(json.dumps(jobs)+'\n')
 (BASE/'freeze.json').write_text(json.dumps(dict(sources=hashes,archive_sha256=sha(BASE/'sources.zip'),binary=dict(path=str(binary),sha256=sha(binary)),cpu=CPU,order_sha256=sha(BASE/'order.json')),indent=2)+'\n');(BASE/'runs').mkdir()
 for i,j in enumerate(jobs):
  cmd=[str(binary),*map(str,j['args'])];r=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=limits);(BASE/'runs'/f'{i}.json').write_text(json.dumps(dict(command=cmd,exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr))+'\n');assert r.returncode==0,r.stderr
 assert all(sha(ROOT/p)==h for p,h in hashes.items());print('bracket completed',len(jobs))
if __name__=='__main__':main()
