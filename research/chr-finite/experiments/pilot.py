#!/usr/bin/env python3
"""Execute only the prospective R04 lifecycle registry, preserving incomplete runs."""
import hashlib,json,os,random,resource,signal,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/R04-lifecycle-pilot'
BINARY=Path('/tmp/chr-r04-lifecycle/release/chr-finite-cost')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    freeze=json.loads((OUT/'freeze.json').read_text())
    for p,h in freeze['sources'].items():assert digest(ROOT/p)==h,p
    assert digest(BINARY)==freeze['binary_sha256']
    workloads=[(f,n) for f in ['weak','chain','unsupported','contradiction','asymmetric'] for n in ([4,6] if f in ['weak','unsupported'] else [4,8])]
    variants=['native','support','conflict','generated-global-scan','generic-global-scan','generated-active-indexed']
    jobs=[(mode,rep,f,n,v,q) for mode,reps in [('time',5),('memory',1)] for rep in range(reps) for f,n in workloads for v in variants for q in [1,4]]
    random.Random(20260915).shuffle(jobs);cpu=min(os.sched_getaffinity(0))
    (OUT/'order.json').write_text(json.dumps({'seed':20260915,'cpu':cpu,'jobs':jobs},indent=2)+'\n')
    def limits():
        os.sched_setaffinity(0,{cpu});resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3));resource.setrlimit(resource.RLIMIT_CPU,(20,20));resource.setrlimit(resource.RLIMIT_CORE,(0,0))
    start=time.monotonic()
    with (OUT/'raw.jsonl').open('x') as out:
        for i,(mode,rep,f,n,v,q) in enumerate(jobs):
            if time.monotonic()-start>900:raise RuntimeError('15-minute batch bound')
            cmd=[str(BINARY),mode,f,str(n),v,str(q)];memory=OUT/f'memory-{i}.txt'
            if mode=='memory':cmd=['/usr/bin/time','-f','%M','-o',str(memory),*cmd]
            record={'index':i,'mode':mode,'rep':rep,'family':f,'size':n,'variant':v,'queries':q,'command':cmd}
            t=time.monotonic_ns();p=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,preexec_fn=limits,start_new_session=True)
            try:
                stdout,stderr=p.communicate(timeout=30);record.update(returncode=p.returncode,stderr=stderr,process_ns=time.monotonic_ns()-t)
                if p.returncode==0:
                    r=json.loads(stdout);record['report']=r
                    assert not r['engine_metrics'] and not r['kernel_metrics'];assert r['memory_run']==(mode=='memory')
                    record['complete']=len(r['samples'])==q and all(s['complete'] for s in r['samples'])
                elif p.returncode in [-9,-24] or mode=='memory' and p.returncode in [137,152]:
                    record.update(resource_failure=True,stdout=stdout)
                else:record['stdout']=stdout;raise RuntimeError(f'validation/runner failure {p.returncode}: {stderr}')
                if mode=='memory' and memory.exists():
                    record['memory_output']=memory.read_text();last=record['memory_output'].splitlines()[-1]
                    if last.isdigit():record['max_rss_kib']=int(last)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid,signal.SIGKILL);stdout,stderr=p.communicate();record.update(timeout=True,stdout=stdout,stderr=stderr,process_ns=time.monotonic_ns()-t)
            except Exception as error:
                record['validation_error']=str(error);out.write(json.dumps(record)+'\n');out.flush();raise
            out.write(json.dumps(record)+'\n');out.flush()
            if (i+1)%60==0:print(f'{i+1}/{len(jobs)} processes recorded',flush=True)
if __name__=='__main__':main()
