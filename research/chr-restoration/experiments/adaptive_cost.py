"""Frozen endpoint qualification and prospectively scheduled adaptive cost pilot."""
from pathlib import Path
import gzip,hashlib,itertools,json,os,platform,random,resource,subprocess,sys,time
ROOT=Path(__file__).resolve().parents[3];RAW=ROOT/'docs/experiments/results/s04-adaptive-cost-pilot'
MODES=['copy','reunion','eager','scheduled','fixed1','fixed8','backoff1','backoff8']
BINS={k:ROOT/f'target/s04-adaptive-cost-{k}/release/examples/adaptive_cost' for k in ['primary','meter']}
def cells():return list(itertools.product(MODES,['plain','history','late'],[0,4],[1,4],['0','all'],[0,1]))
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run(command,path):
    assert not path.exists(),path
    start=time.monotonic()
    try:
        p=subprocess.run(command,timeout=60,preexec_fn=limits,capture_output=True,text=True)
        result=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:result=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    result['wall_seconds']=time.monotonic()-start
    path.write_bytes(gzip.compress((json.dumps(result,indent=2)+'\n').encode(),mtime=0))
    assert result.get('returncode')==0 and not result['stderr'],path
    return result
if __name__=='__main__':
    action=sys.argv[1];assert action in ['qualify','pilot'];matrix=cells();assert len(matrix)==384
    manifest=RAW/'manifest.json'
    if action=='qualify':
        files=[Path(__file__),ROOT/'research/chr-restoration/experiments/audit_adaptive_cost.py',ROOT/'research/chr-restoration/examples/adaptive_cost.rs',ROOT/'research/chr-restoration/examples/support/repeated_source.rs',ROOT/'research/chr-restoration/src/reunion.rs',ROOT/'research/chr-restoration/src/lib.rs',ROOT/'research/chr-restoration/Cargo.toml',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'research/chr-direct-conditional/tests/runtime_support/mod.rs',ROOT/'research/chr-observe/src/lib.rs',ROOT/'crates/chr-syntax/src/lib.rs',ROOT/'Cargo.lock',ROOT/'docs/experiments/registrations/S04-adaptive-cost-pilot.md',*BINS.values()]
        assert not manifest.exists()
        manifest.write_text(json.dumps(dict(cells=matrix,binaries={k:str(p.relative_to(ROOT)) for k,p in BINS.items()},sha256={str(p.relative_to(ROOT)):digest(p) for p in files},host=dict(platform=platform.platform(),affinity=sorted(os.sched_getaffinity(0)),rustc=subprocess.check_output(['rustc','-Vv'],text=True))),indent=2)+'\n')
        for kind,binary in BINS.items():
            for i,cell in enumerate(matrix):
                result=run([str(binary),*map(str,cell)],RAW/f'qualify-{kind}-{i:03}.json.gz')
                assert result['wall_seconds']<10,'qualification sizing bound needs review'
                if i%96==95:print(kind,i+1,'/384 qualified',flush=True)
    else:
        m=json.loads(manifest.read_text())
        for p,h in m['sha256'].items():assert digest(ROOT/p)==h,p
        assert len(list(RAW.glob('qualify-*.json.gz')))==768
        for path in RAW.glob('qualify-*.json.gz'):
            r=json.loads(gzip.decompress(path.read_bytes()));assert r['returncode']==0 and r['wall_seconds']<10
        jobs=[(kind,rep,c) for kind,count in [('primary',5),('meter',2)] for rep in range(count) for c in matrix];assert len(jobs)==2688
        random.Random(770903).shuffle(jobs)
        schedule=RAW/'schedule.json';assert not schedule.exists();schedule.write_text(json.dumps(jobs)+'\n')
        for i,(kind,rep,cell) in enumerate(jobs):
            run([str(BINS[kind]),*map(str,cell)],RAW/f'run-{i:04}.json.gz')
            if i%192==191:print(i+1,'/2688 pilot processes terminal',flush=True)
