"""Long prepared sessions with exact source and ownership validation."""
import gzip,hashlib,itertools,json,resource,shutil,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-long-cached-oracle'
PROJECT=ROOT/'target/s05-call-long-cached-project'
MODES=['trace','direct','scan','indexed','sealed','generated','generated-sealed']
CELLS=list(itertools.product(MODES,range(4),[32,128],[64],['0','all'],[0,1],[0,2]))
CELLS += [(m,f,32,16,'0',0,1)for m in MODES for f in range(4)]
CELLS += [(m,0,128,8192,'0',0,p)for m in MODES for p in [0,2]]
assert len(CELLS)==490

def binary(kind,mode):return ROOT/f'target/s05-call-long-cached-{kind}/release'/('call-generated-artifact'if mode.startswith('generated')else'examples/call_trace_ownership')
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
def invoke(cmd,path,build=False):
    assert not path.exists();start=time.monotonic_ns()
    try:
        p=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True,timeout=120 if build else 180,preexec_fn=None if build else bounds)
        r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,wall_ns=time.monotonic_ns()-start)
    except subprocess.TimeoutExpired as e:
        r=dict(command=cmd,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r)+'\n');assert r.get('returncode')==0,path
    return r

def pack():
    manifest={}
    for p in RAW.glob('*.json'):
        data=p.read_bytes()
        if b'"stdout":' not in data:continue
        encoded=gzip.compress(data,mtime=0);target=p.with_suffix('.json.gz');target.write_bytes(encoded)
        assert gzip.decompress(target.read_bytes())==data
        manifest[p.name]=dict(raw_sha256=hashlib.sha256(data).hexdigest(),gzip_sha256=hashlib.sha256(encoded).hexdigest(),raw_bytes=len(data),packed_bytes=len(encoded))
        p.unlink()
    (RAW/'packed-receipts.json').write_text(json.dumps(manifest,indent=2)+'\n')

def receipt(name,packed):
    if not packed:return json.loads((RAW/name).read_text())
    encoded=(RAW/(name+'.gz')).read_bytes();data=gzip.decompress(encoded)
    return json.loads(data)

def audit(packed=True):
    if packed:
        for name,v in json.loads((RAW/'packed-receipts.json').read_text()).items():
            encoded=(RAW/(name+'.gz')).read_bytes();data=gzip.decompress(encoded)
            assert hashlib.sha256(encoded).hexdigest()==v['gzip_sha256'] and hashlib.sha256(data).hexdigest()==v['raw_sha256']
            assert len(data)==v['raw_bytes'] and len(encoded)==v['packed_bytes']
    f=json.loads((RAW/'freeze.json').read_text())
    for p,h in f['sha256'].items():
        assert own.sha(ROOT/p if '/release/'in p and p.startswith('target/')else RAW/'registered'/p)==h,p
    for k in ['primary','meter']:
        gate=receipt(f'gate-{k}.json',packed);assert gate['returncode']==0 and 'complete_queries=768 cancellation_restarts=2304 retained_outputs_checked=true' in gate['stderr']
    mems={};obs={};consumer={}
    for kind,reps in [('meter',2),('primary',1)]:
        for k in range(reps):
            for i,c in enumerate(CELLS):
                r=receipt(f'{kind}-{k}-{i:03}.json',packed);assert r['command']==[str(binary(kind,c[0])),*map(str,c)]
                h,mem=own.parse(r);assert h['oracle_classes']==(1 if c[6]==0 else min(c[3],4)if c[6]==2 else c[3]);assert len(h['counts'])==c[3] and all(n==(1 if c[5]or c[1]==1 else 2)for n in h['counts'])
                value=(h['counts'],h['retained'])
                if c[1:]in obs:assert obs[c[1:]]==value
                obs[c[1:]]=value
                if kind=='meter':
                    assert mem is not None
                    if c in mems:assert mems[c]==mem
                    mems[c]=mem
                    if c[1:]in consumer:assert consumer[c[1:]]==mem['consumer']
                    consumer[c[1:]]=mem['consumer']
                else:assert mem is None
    return [dict(cell=list(c),memory=mems[c])for c in CELLS]

if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False)
        for kind,feature in [('primary',[]),('meter',['--features','alloc-meter'])]:
            target=str(ROOT/f'target/s05-call-long-cached-{kind}')
            invoke(['cargo','build','--offline','--release','--jobs','2','-p','chr-reuse','--no-default-features','--example','call_trace_ownership','--target-dir',target,*feature],RAW/f'build-generic-{kind}.json',True)
            invoke(['cargo','build','--offline','--release','--jobs','2','--manifest-path',str(PROJECT/'Cargo.toml'),'--target-dir',target,*feature],RAW/f'build-generated-{kind}.json',True)
            invoke([str(binary(kind,'generated')),'gate'],RAW/f'gate-{kind}.json')
            print(kind,'builds and source gate pass',flush=True)
        paths=[ROOT/p for p in subprocess.check_output(['git','ls-files','research','crates'],cwd=ROOT,text=True).splitlines()if p.endswith('.rs')or p.endswith('Cargo.toml')]
        paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',PROJECT/'Cargo.toml',PROJECT/'Cargo.lock',PROJECT/'src/main.rs',PROJECT/'src/generated.rs',Path(__file__),Path(own.__file__),ROOT/'docs/experiments/registrations/S05-call-long-entry.md',ROOT/'docs/experiments/registrations/S05-call-long-cached-oracle.md']
        for p in paths:
            dst=RAW/'registered'/p.relative_to(ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        paths += [binary(k,m)for k in ['primary','meter']for m in ['direct','generated']]
        (RAW/'freeze.json').write_text(json.dumps(dict(cells=CELLS,sha256={str(p.relative_to(ROOT)):own.sha(p)for p in paths}),indent=2)+'\n')
        # Finish every smaller cell before testing long scaling sentinels.
        for lo,hi in [(0,476),(476,490)]:
            for kind,reps in [('meter',2),('primary',1)]:
                for k in range(reps):
                    for i in range(lo,hi):
                        c=CELLS[i];r=invoke([str(binary(kind,c[0])),*map(str,c)],RAW/f'{kind}-{k}-{i:03}.json');own.parse(r)
                        if i%28==27 or i==hi-1:print(kind,k,i+1,'cells terminal',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(False),indent=2)+'\n')
        pack()
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text());print('1470 processes, long-query counts, exact ownership and source checks verified.')
