"""Fixed paired confirmation using the qualified post binary; compressed raw receipts."""
import gzip,hashlib,itertools,json,os,random,resource,subprocess,time,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];BASE=ROOT/'docs/experiments/results/s03-post-confirmation';PARENT=ROOT/'docs/experiments/results/s03-post-control-cost'
FAMILIES=['post-input','post-output','post-forward','post-miss','post-duplicate','post-template'];MODES=['birth','birth-miss','birth-miss-template','scan','indexed','sealed','active-scan','active-indexed','native-scan','native-indexed','active-native-scan','active-native-indexed']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    os.sched_setaffinity(0,{0});resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def main():
    assert not (BASE/'freeze.json').exists();parent=json.loads((PARENT/'freeze.json').read_text());binary=parent['binaries']['time'];assert sha(Path(binary['path']))==binary['sha256'];assert sha(PARENT/'sources.zip')==parent['archive_sha256'];assert 0 in os.sched_getaffinity(0)
    build=json.loads((PARENT/'build-time.json').read_text());assert build['exit_code']==0
    artifacts=[json.loads(l) for l in build['stdout'].splitlines() if json.loads(l).get('reason')=='compiler-artifact']
    assert artifacts and any(x.get('executable')==binary['path'] for x in artifacts)
    assert all(not(set(x.get('features',[]))&{'metrics','kernel-metrics','work-diagnostics','candidate-profile','alloc-meter'}) for x in artifacts)
    with zipfile.ZipFile(PARENT/'sources.zip') as archive:
        assert set(archive.namelist())==set(parent['sources'])
        for p,h in parent['sources'].items():assert hashlib.sha256(archive.read(p)).hexdigest()==h
    scenarios=list(itertools.product(FAMILIES,[8,32],[False,True],['immediate','window','all']));cells=[[*s,m] for s in scenarios for m in MODES]
    blocks=list(itertools.product(range(64),range(72)));rng=random.Random(71330);rng.shuffle(blocks);jobs=[]
    for block,(rep,index) in enumerate(blocks):
        modes=MODES.copy();rng.shuffle(modes)
        for pos,mode in enumerate(modes):jobs.append(dict(block=block,rep=rep,index=cells.index([*scenarios[index],mode]),position=pos))
    warmup=list(range(864));random.Random(71331).shuffle(warmup)
    for name,data in [('cells',cells),('jobs',jobs),('warmup-order',warmup)]: (BASE/f'{name}.json').write_text(json.dumps(data)+'\n')
    paths=['docs/experiments/registrations/S03-post-confirmation.md','research/chr-direct-conditional/experiments/post_confirmation.py','research/chr-direct-conditional/experiments/audit_post_confirmation.py']
    f=dict(binary=binary,parent_freeze_sha256=sha(PARENT/'freeze.json'),parent_archive_sha256=sha(PARENT/'sources.zip'),parent_build_sha256=sha(PARENT/'build-time.json'),parent_audit_sha256=sha(PARENT/'audit.json'),sources={p:sha(ROOT/p) for p in paths},source_text={p:(ROOT/p).read_text() for p in paths},orders={n:sha(BASE/f'{n}.json') for n in ['cells','jobs','warmup-order']},cpu=0,affinity=sorted(os.sched_getaffinity(0)),platform=os.uname().release,commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip())
    (BASE/'freeze.json').write_text(json.dumps(f,indent=2)+'\n')
    def invoke(args):
        cmd=[binary['path'],*args]
        try:
            r=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,DEPENDENCY_FIRST_CLOCK='off'),capture_output=True,text=True,timeout=60,preexec_fn=limits);return dict(command=cmd,first_clock='off',exit_code=r.returncode,stdout=r.stdout,stderr=r.stderr,timeout=False)
        except subprocess.TimeoutExpired as e:
            dec=lambda x:x.decode() if isinstance(x,bytes) else x or ''
            return dict(command=cmd,first_clock='off',exit_code=None,stdout=dec(e.stdout),stderr=dec(e.stderr),timeout=True)
    for i in range(5):
        raw=invoke(['clock-check']);(BASE/f'clock-{i}.json').write_text(json.dumps(raw)+'\n');assert raw['exit_code']==0 and not raw['stderr']
    start=time.monotonic()
    for kind,order in [('warmup',warmup),('runs',jobs)]:
        with gzip.GzipFile(filename=str(BASE/f'{kind}.jsonl.gz'),mode='wb',compresslevel=1,mtime=0) as stream:
            for i,j in enumerate(order):
                assert time.monotonic()-start<1800;index=j if kind=='warmup' else j['index'];fam,n,rev,c,m=cells[index];raw=invoke([m,fam,str(n),str(rev).lower(),c,'false'])
                stream.write((json.dumps(dict(sequence=i,index=index,raw=raw))+'\n').encode())
                if kind=='warmup' or (i+1)%12==0:stream.flush()
                assert raw['exit_code']==0 and not raw['stderr'] and not raw['timeout'],(kind,i)
                if (i+1)%2304==0:print(kind,i+1,'/',len(order),flush=True)
    assert sha(Path(binary['path']))==binary['sha256'];assert all(sha(ROOT/p)==h for p,h in f['sources'].items())
    (BASE/'campaign.json').write_text(json.dumps(dict(primary_processes=55296,warmups=864,seconds=time.monotonic()-start,archives={n:sha(BASE/f'{n}.jsonl.gz') for n in ['warmup','runs']}))+'\n')
if __name__=='__main__':main()
