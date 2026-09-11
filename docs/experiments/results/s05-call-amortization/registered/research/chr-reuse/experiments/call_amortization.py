"""Registered per-program compiler costs and observed prepared-session reuse."""
import gzip,hashlib,itertools,json,os,random,resource,shutil,statistics,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-amortization'
TARGET=ROOT/'target/s05-call-amortization'
REGIMES=[(1,128,0),(2,128,2),(3,32,2),(0,4,0)]
MODES=['trace','direct','scan','indexed','sealed','generated','generated-sealed']
SOURCES=['generic','generated']
manifest={}
def source_type(mode):return 'generated'if mode.startswith('generated')else'generic'
def stem(kind,family,source,rep):return f'{kind}-f{family}-{source}-{rep}'
def binary(kind,family,mode,rep=0):return TARGET/stem(kind,family,source_type(mode),rep)
def record(name,r):
    data=(json.dumps(r)+'\n').encode();encoded=gzip.compress(data,mtime=0);(RAW/(name+'.json.gz')).write_bytes(encoded)
    manifest[name]=dict(raw_sha256=hashlib.sha256(data).hexdigest(),gzip_sha256=hashlib.sha256(encoded).hexdigest())
    (RAW/'receipts.json').write_text(json.dumps(manifest,indent=2)+'\n')
def read(name):return json.loads(gzip.decompress((RAW/(name+'.json.gz')).read_bytes()))
def invoke(name,cmd,compiler=False):
    assert not (RAW/(name+'.json.gz')).exists()
    def bounds():
        limit=(4 if compiler else 1)<<30
        resource.setrlimit(resource.RLIMIT_AS,(limit,limit));resource.setrlimit(resource.RLIMIT_CPU,(120,120))
    before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.monotonic_ns()
    try:
        p=subprocess.run(list(map(str,cmd)),cwd=ROOT,capture_output=True,text=True,timeout=180,preexec_fn=bounds)
        r=dict(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:r=dict(timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    after=resource.getrusage(resource.RUSAGE_CHILDREN)
    r.update(command=list(map(str,cmd)),wall_ns=time.monotonic_ns()-start,cpu_seconds=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime)
    record(name,r);assert r.get('returncode')==0,(name,r.get('stderr','')[-1000:]);return r

def validate(r,cell,metered):
    mode,f,n,q,policy=cell;h,m=own.parse(r)
    assert len(h['counts'])==q and all(v==(1 if f==1 else 2)for v in h['counts'])
    assert h['retained']==0 and h['oracle_classes']==(1 if policy==0 else min(q,4))
    assert (m is not None)==metered
    return m

def args(cell):
    m,f,n,q,p=cell;return [m,f,n,q,'0',0,p]
def assess(rs,qualified=True):
    med=statistics.median(rs)
    status='gain'if qualified and med<=.9 and max(rs)<1 else'loss'if qualified and med>=1.1 and min(rs)>1 else'uncertain'
    return dict(median=med,min=min(rs),max=max(rs),status=status)
def program_cost(kind,f,source,rep,disposal):
    s=stem(kind,f,source,rep);e=json.loads(read('emit-'+s)['stdout'])
    return sum(e[k]for k in ['emission_ns','write_ns','buffer_drop_ns'])+read('compile-'+s)['wall_ns']+disposal.get(s,0)
def comparisons(scenarios,disposal):
    floor=100*max(json.loads(read(f'clock-{i}')['stdout'])['p99_ns']for i in range(3));out=[]
    for si,(f,n,q,p)in enumerate(scenarios):
        values={};walls={}
        for mode in MODES:
            for k in range(5):
                r=read(f'time-{si}-{k}-{mode}');validate(r,(mode,f,n,q,p),False)
                rows=[json.loads(line)for line in r['stdout'].splitlines()][1:]
                values[mode,k]=sum(x['ns']for x in rows);walls[mode,k]=r['wall_ns']
        for a in MODES[-2:]:
            for b in MODES[:5]:
                qualified=min(statistics.median(values[m,k]for k in range(5))for m in [a,b])>=floor
                installed=[];packaged=[];envelope=[]
                for j in range(5):
                    ca=program_cost('primary',f,'generated',j,disposal);cb=program_cost('primary',f,'generic',j,disposal)
                    for k in range(5):
                        installed.append((ca+values[a,k])/values[b,k]);packaged.append((ca+values[a,k])/(cb+values[b,k]));envelope.append((ca+walls[a,k])/walls[b,k])
                out.append(dict(scenario=[f,n,q,p],candidate=a,control=b,runtime=assess([values[a,k]/values[b,k]for k in range(5)],qualified),installed=assess(installed,qualified),packaged=assess(packaged,qualified),process_envelope=assess(envelope,qualified),candidate_ns=statistics.median(values[a,k]for k in range(5)),control_ns=statistics.median(values[b,k]for k in range(5))))
    return dict(floor_ns=floor,comparisons=out)
def run_sessions(scenarios,start,rng):
    for si in range(start,len(scenarios)):
        f,n,q,p=scenarios[si]
        for mode in MODES:
            cell=(mode,f,n,q,p);previous=None
            for k in range(2):
                mem=validate(invoke(f'alloc-{si}-{k}-{mode}',[binary('meter',f,mode),*args(cell)]),cell,True)
                if previous is not None:assert previous==mem
                previous=mem
            validate(invoke(f'check-{si}-{mode}',[binary('primary',f,mode),*args(cell)]),cell,False)
        print('ownership/source scenario',si,'qualified',flush=True)
    schedule=[]
    for k in range(5):
        order=list(range(start,len(scenarios)));rng.shuffle(order)
        for si in order:
            modes=MODES.copy();rng.shuffle(modes)
            for mode in modes:schedule.append((si,k,mode))
    (RAW/f'schedule-{start}.json').write_text(json.dumps(schedule)+'\n')
    for si,k,mode in schedule:
        f,n,q,p=scenarios[si];cell=(mode,f,n,q,p)
        validate(invoke(f'time-{si}-{k}-{mode}',[binary('primary',f,mode),*args(cell)]),cell,False)
        if mode==schedule[-1][2] and (si,k,mode)==schedule[-1]:pass
        print('timing',si,k,mode,'terminal',flush=True)

def audit():
    f=json.loads((RAW/'freeze.json').read_text())
    for path,h in f['inputs'].items():assert own.sha(RAW/'registered'/path)==h,path
    for path,h in f['libraries'].items():assert own.sha(ROOT/path)==h,path
    for name,h in json.loads((RAW/'receipts.json').read_text()).items():
        enc=(RAW/(name+'.json.gz')).read_bytes();data=gzip.decompress(enc)
        assert hashlib.sha256(enc).hexdigest()==h['gzip_sha256'] and hashlib.sha256(data).hexdigest()==h['raw_sha256'];assert json.loads(data)['returncode']==0,name
    for name,v in f['artifacts'].items():
        assert not (TARGET/name).exists() and not (TARGET/(name+'.rs')).exists()
        assert own.sha(RAW/'sources'/(name+'.rs'))==v['source_sha256']
    scenarios=json.loads((RAW/'scenarios.json').read_text())
    for si,(family,n,q,p)in enumerate(scenarios):
        consumers=[]
        for mode in MODES:
            cell=(mode,family,n,q,p)
            a=validate(read(f'alloc-{si}-0-{mode}'),cell,True);b=validate(read(f'alloc-{si}-1-{mode}'),cell,True);assert a==b;consumers.append(a['consumer'])
            validate(read(f'check-{si}-{mode}'),cell,False)
        assert len(set(consumers))==1
    disposal=json.loads((RAW/'disposal.json').read_text())
    return comparisons(scenarios,disposal)

if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False);TARGET.mkdir(exist_ok=False);(RAW/'sources').mkdir()
        for kind,features in [('primary',[]),('meter',['--features','chr-reuse/alloc-meter','--features','chr-compiled/alloc-meter'])]:
            invoke('build-'+kind,['cargo','build','--offline','--release','--jobs','2','-p','chr-reuse','-p','chr-compiled','-p','chr-syntax','-p','chr-observe','--lib','--no-default-features','--target-dir',TARGET/kind,*features],True)
        invoke('build-emitter',['cargo','build','--offline','--release','--jobs','2','-p','chr-reuse','--no-default-features','--example','call_program_emit','--target-dir',TARGET/'primary'],True)
        paths=[ROOT/p for p in subprocess.check_output(['git','ls-files','research','crates'],cwd=ROOT,text=True).splitlines()if p.endswith('.rs')or p.endswith('Cargo.toml')]
        paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),Path(own.__file__),ROOT/'research/chr-reuse/examples/call_program_emit.rs',ROOT/'docs/experiments/registrations/S05-call-amortization.md']
        for p in paths:
            dst=RAW/'registered'/p.relative_to(ROOT);dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        libs=list(TARGET.glob('*/release/*.rlib'))+list(TARGET.glob('*/release/deps/*.rlib'))
        freeze=dict(inputs={str(p.relative_to(ROOT)):own.sha(p)for p in paths},libraries={str(p.relative_to(ROOT)):own.sha(p)for p in libs},artifacts={},rustc=subprocess.check_output(['rustc','-Vv'],text=True),parent_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),affinity=sorted(os.sched_getaffinity(0)))
        rng=random.Random(607098)
        for kind,reps in [('primary',5),('meter',1)]:
            for rep in range(reps):
                order=list(itertools.product(range(4),SOURCES));rng.shuffle(order)
                for family,source in order:
                    name=stem(kind,family,source,rep);src=TARGET/(name+'.rs');bin=TARGET/name
                    invoke('emit-'+name,[TARGET/'primary/release/examples/call_program_emit',source,family,src])
                    cmd=['rustc','--edition','2024','-C','opt-level=3','-C','codegen-units=1','-C','lto=off',src,'-L','dependency='+str(TARGET/kind/'release/deps'),'-o',bin]
                    for crate in ['chr_reuse','chr_compiled','chr_syntax','chr_observe']:cmd+=['--extern',crate+'='+str(TARGET/kind/'release'/('lib'+crate+'.rlib'))]
                    if kind=='meter':cmd+=['--cfg','feature="alloc-meter"']
                    invoke('compile-'+name,cmd,True)
                    mode='generated'if source=='generated'else'direct';n,p=next((n,p)for f,n,p in REGIMES if f==family)
                    validate(invoke('preflight-'+name,[bin,*args((mode,family,n,4,p))]),(mode,family,n,4,p),kind=='meter')
                    shutil.copyfile(src,RAW/'sources'/src.name)
                    freeze['artifacts'][name]=dict(source_sha256=own.sha(src),binary_sha256=own.sha(bin),binary_bytes=bin.stat().st_size)
                print('compiler block',kind,rep,'qualified',flush=True)
        previous=json.loads((ROOT/'docs/experiments/results/s05-recognition-stride-timing/freeze.json').read_text());clock=ROOT/previous['binary'];assert own.sha(clock)==previous['binary_hash']
        freeze['clock']=dict(path=str(clock),sha256=own.sha(clock));(RAW/'freeze.json').write_text(json.dumps(freeze,indent=2)+'\n')
        for i in range(3):invoke(f'clock-{i}',[clock,'clock-check'])
        scenarios=[(f,n,q,p)for f,n,p in REGIMES for q in [64,8192]];rng=random.Random(607099)
        run_sessions(scenarios,0,rng)
        preliminary=comparisons(scenarios,{})
        (RAW/'pre-disposal.json').write_text(json.dumps(preliminary,indent=2)+'\n')
        eligible=[]
        for ri,(f,n,p)in enumerate(REGIMES):
            for mi,mode in enumerate(MODES[-2:]):
                xs=[x for x in preliminary['comparisons']if x['scenario']==[f,n,8192,p]and x['candidate']==mode and x['control']in ['direct','scan','sealed']]
                if all(x['runtime']['median']<=.9 for x in xs)and not any(x['installed']['status']=='gain'for x in xs):eligible.append((min(x['control_ns']-x['candidate_ns']for x in xs),-ri,-mi))
        selection=None
        if eligible:
            score,ri,mi=max(eligible);f,n,p=REGIMES[-ri];selection=dict(source_index=-ri,mode=MODES[-2:][-mi],minimum_saving_ns=score);scenarios.append((f,n,32768,p));run_sessions(scenarios,8,rng)
        (RAW/'selection.json').write_text(json.dumps(dict(eligible=eligible,selected=selection),indent=2)+'\n');(RAW/'scenarios.json').write_text(json.dumps(scenarios)+'\n')
        disposal={}
        for name,entry in freeze['artifacts'].items():
            bin=TARGET/name;src=TARGET/(name+'.rs');assert own.sha(bin)==entry['binary_sha256'] and own.sha(src)==entry['source_sha256']
            start=time.monotonic_ns();src.unlink();bin.unlink();disposal[name]=time.monotonic_ns()-start
        (RAW/'disposal.json').write_text(json.dumps(disposal,indent=2)+'\n')
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n');print('amortization campaign complete',flush=True)
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text());print('Compiler, runtime, ownership, source and disposal evidence verified.')
