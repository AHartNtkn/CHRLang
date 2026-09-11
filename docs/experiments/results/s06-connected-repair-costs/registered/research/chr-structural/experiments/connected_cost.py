"""Prospective connected lifecycle pilot; isolated processes and exact ownership."""
from pathlib import Path
import hashlib,itertools,json,os,platform,random,resource,statistics,subprocess,sys,time
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s06-connected-cost-pilot'
PRIMARY=ROOT/'target/s06-connected-primary/release/examples/connected_cost'
METER=ROOT/'target/s06-connected-meter/release/examples/connected_cost'
def cells():
    return list(itertools.product(['projection','enumerate'],['star','clique','different'],[4,6],[0,1],[0,1],[1,4],[0,1],[0,1]))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(name,data):
    p=RAW/name;assert not p.exists(),p;p.write_text(json.dumps(data,indent=2)+'\n')
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def invoke(binary,cell,name):
    command=[str(binary),*map(str,cell)];start=time.monotonic()
    try:
        p=subprocess.run(command,capture_output=True,text=True,timeout=45,preexec_fn=bounds)
        r=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r=dict(command=command,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    r['wall_seconds']=time.monotonic()-start;write(name,r)
    assert r.get('returncode')==0 and not r['stderr'],name
    return parsed(r)
def parsed(r):
    lines=[json.loads(x) for x in r['stdout'].splitlines()];header=lines[0];rows=lines[1:]
    assert header['total_ns']==sum(x['ns'] for x in rows)
    q=int(r['command'][-3]);assert len(rows)==6+8*q,(len(rows),q)
    phases=['consumer_create','source','prepare','source_dispose']+['input','setup','consumer_buffer','first','remaining','engine_dispose','input_dispose','consume']*q+['prepared_dispose','consumer_dispose']
    # Eight query phases, with every allocator-owned object disposed explicitly.
    assert [x['phase'] for x in rows]==phases
    return header,rows

def memory(rows):
    rs=[x['memory'] for x in rows];base=rs[0]['live_start']
    assert rs[-1]['live_end']==base
    for a,b in zip(rs,rs[1:]):assert a['live_end']==b['live_start']
    return dict(requested=sum(x['requested_bytes'] for x in rs),peak=max(x['peak_live'] for x in rs)-base,consumer=rs[-1]['live_start']-base)
def audit():
    manifest=json.loads((RAW/'manifest.json').read_text())
    for path,h in manifest['sha256'].items():
        candidate=RAW/'registered-driver.py' if path==str(Path(__file__).relative_to(ROOT)) else ROOT/path
        assert sha(candidate)==h,path
    allocation={};timing={};floors={}
    for kind,repeat,i in json.loads((RAW/'schedule.json').read_text()):
        r=json.loads((RAW/f'{kind}-{repeat:02}-{i:03}.json').read_text());assert r['returncode']==0 and not r['stderr']
        h,rows=parsed(r);cell=tuple(manifest['cells'][i]);assert h['metered']==(kind=='meter')
        if kind=='meter':
            m=memory(rows)
            if cell in allocation:assert allocation[cell]==m
            allocation[cell]=m
        else:
            timing[cell,repeat]=h['total_ns'];floors[cell,repeat]=h['clock_floor']
    assert len(allocation)==384 and len(timing)==3840
    comparisons=[]
    for cell in cells():
        if cell[0]!='projection':continue
        other=('enumerate',*cell[1:]);a,b=allocation[cell],allocation[other];assert a['consumer']==b['consumer'],(cell,a,b)
        ratios=[timing[cell,k]/timing[other,k] for k in range(10)];median=statistics.median(ratios)
        qualified=all(statistics.median(timing[c,k] for k in range(10))>max(floors[c,k] for k in range(10)) for c in [cell,other])
        verdict='gain' if qualified and median<=.9 and max(ratios)<1 else 'loss' if qualified and median>=1.1 and min(ratios)>1 else 'unresolved'
        comparisons.append(dict(scenario=cell[1:],ratio_median=median,ratio_min=min(ratios),ratio_max=max(ratios),verdict=verdict,clock_qualified=qualified,projection_allocation=a,enumerate_allocation=b,projection_ns=statistics.median(timing[cell,k] for k in range(10)),enumerate_ns=statistics.median(timing[other,k] for k in range(10))))
    return dict(cells=384,allocation_processes=768,timing_processes=3840,comparisons=comparisons)
if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False)
        (RAW/'registered-driver.py').write_bytes(Path(__file__).read_bytes())
        paths=[Path(__file__),ROOT/'research/chr-structural/examples/connected_cost.rs',ROOT/'research/chr-structural/examples/support/projection_cost.rs',ROOT/'research/chr-structural/src/joint_region.rs',ROOT/'research/chr-structural/src/projection.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S06-connected-cost-pilot.md',ROOT/'Cargo.lock',PRIMARY,METER]
        write('manifest.json',dict(cells=cells(),sha256={str(p.relative_to(ROOT)):sha(p) for p in paths},host=platform.platform(),affinity=sorted(os.sched_getaffinity(0))))
        schedule=[];rng=random.Random(607091)
        for kind,repeats in [('meter',2),('primary',10)]:
            for repeat in range(repeats):
                order=list(range(384));rng.shuffle(order);schedule.extend((kind,repeat,i) for i in order)
        write('schedule.json',schedule)
        entries={}
        for number,(kind,repeat,i) in enumerate(schedule):
            cell=cells()[i];h,rows=invoke(METER if kind=='meter' else PRIMARY,cell,f'{kind}-{repeat:02}-{i:03}.json')
            if kind=='meter':
                m=memory(rows)
                if cell in entries:assert entries[cell]==m
                entries[cell]=m
                other=('enumerate' if cell[0]=='projection' else 'projection',*cell[1:])
                if other in entries:assert m['consumer']==entries[other]['consumer']
            if number%128==127:print(number+1,'/ 4608 terminal processes',flush=True)
        write('analysis.json',audit())
    elif sys.argv[1]=='audit':
        result=audit();assert json.loads(json.dumps(result))==json.loads((RAW/'analysis.json').read_text());print('All 4608 processes, ownership pairs, phase sums and input hashes verified.')
    else:raise ValueError('run or audit')
