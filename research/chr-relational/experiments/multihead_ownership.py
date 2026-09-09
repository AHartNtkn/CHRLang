#!/usr/bin/env python3
import hashlib,itertools,json,pathlib,random,resource,shutil,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[3]
OUT=ROOT/'docs/experiments/results/s02-multihead-ownership'
BIN=ROOT/'target/multihead-ownership/runner'
def bounds():resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
def run(args,path):
    command=[str(BIN),*map(str,args)]
    try:
        p=subprocess.run(command,cwd=ROOT,capture_output=True,text=True,timeout=60,preexec_fn=bounds)
        receipt={'command':command,'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
    except subprocess.TimeoutExpired as e:receipt={'command':command,'timeout':60,'stdout':str(e.stdout),'stderr':str(e.stderr)}
    path.write_text(json.dumps(receipt,indent=2)+'\n');assert receipt.get('returncode')==0,path
    return json.loads(receipt['stdout'])
def main():
    OUT.mkdir(exist_ok=False);BIN.parent.mkdir(exist_ok=False)
    build=[json.loads(x) for x in pathlib.Path('/tmp/chr-multihead-build.jsonl').read_text().splitlines()]
    binary=[x['executable'] for x in build if x.get('reason')=='compiler-artifact' and x.get('target',{}).get('name')=='multihead_ownership' and x.get('executable')];assert len(binary)==1
    shutil.copy2(binary[0],BIN);shutil.copy2('/tmp/chr-multihead-build.log',OUT/'build.log')
    files=['research/chr-relational/tests/multihead_ownership.rs','research/chr-relational/tests/support/multihead_source.rs','research/chr-relational/tests/support/local_multihead.rs','research/chr-relational/tests/support/local_ports.rs','research/chr-compiled/experiments/meter.rs','docs/experiments/registrations/S02-multihead-ownership.md']
    freeze={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'binary_sha256':hashlib.sha256(BIN.read_bytes()).hexdigest(),'source_sha256':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files}}
    (OUT/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
    configs=list(itertools.product(['local-scan','tuples','partial','scan','indexed','special-scan','special-indexed'],['sparse','broad','nested','cold','dense','three'],[4,16,64]))
    (OUT/'preflight').mkdir()
    for i,c in enumerate(configs):run((*c,1),OUT/'preflight'/f'{i:03}.json')
    print('126 preflight processes passed',flush=True)
    jobs=[(rep,(*c,reuse)) for rep in range(2) for c in configs for reuse in [1,4]];random.Random(7204).shuffle(jobs)
    (OUT/'order.json').write_text(json.dumps(jobs)+'\n');(OUT/'runs').mkdir()
    for i,(rep,c) in enumerate(jobs):
        run(c,OUT/'runs'/f'{i:03}-r{rep}.json')
        if (i+1)%42==0:print(f'{i+1}/504 matrix processes passed',flush=True)
if __name__=='__main__':main()
