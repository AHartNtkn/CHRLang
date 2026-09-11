"""Matched compiled controls: ownership qualification before ordinary timing."""
from pathlib import Path
import itertools,json,random,shutil,statistics,subprocess,sys
sys.dont_write_bytecode=True
import call_trace_ownership as own
ROOT=own.ROOT
RAW=ROOT/'docs/experiments/results/s05-call-compiled-cost'
MODES=['trace','direct','scan','indexed','sealed']
CELLS=list(itertools.product(MODES,range(4),[4,32,128],[(1,0),(4,0),(4,1)],['0','all'],[0,1]))
CELLS=[(m,f,n,q,k,c,u)for m,f,n,(q,u),k,c in CELLS]
def binary(kind):return ROOT/f'target/s05-call-compiled-{kind}/release/examples/call_trace_ownership'
def invoke(cmd,path):
    assert not path.exists()
    try:
        p=subprocess.run(cmd,capture_output=True,text=True,timeout=45,preexec_fn=own.bounds)
        r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    except subprocess.TimeoutExpired as e:
        r=dict(command=cmd,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
    path.write_text(json.dumps(r)+'\n')
    assert r.get('returncode')==0 and not r['stderr'],path
    return r

def audit(complete=True):
    freeze=json.loads((RAW/'freeze.json').read_text())
    for path,h in freeze['sha256'].items():
        assert own.sha(ROOT/path if path.startswith('target/') else RAW/'registered'/path)==h,path
    memories={};observations={};values={};seen=set()
    for kind,k,i in json.loads((RAW/'schedule.json').read_text()):
        if not complete and kind=='primary':continue
        key=(kind,k,i);assert key not in seen;seen.add(key)
        r=json.loads((RAW/f'{kind}-{k:02}-{i:03}.json').read_text());cell=CELLS[i]
        assert r['command']==[str(binary(kind)),*map(str,cell)]
        h,mem=own.parse(r);obs=(h['counts'],h['retained'])
        scenario=cell[1:]
        if scenario in observations:assert observations[scenario]==obs
        observations[scenario]=obs
        if kind=='meter':
            if cell in memories:assert memories[cell]==mem
            assert mem is not None;memories[cell]=mem
        else:
            assert mem is None
            rows=[json.loads(x)for x in r['stdout'].splitlines()][1:]
            assert all(isinstance(x['ns'],int)for x in rows)
            values[cell,k]=sum(x['ns']for x in rows)
    assert len(memories)==720
    for cell,mem in memories.items():assert mem['consumer']==memories[('trace',*cell[1:])]['consumer']
    if not complete:
        assert len(seen)==1440
        return
    assert len(seen)==8640 and len(values)==7200
    clocks=[json.loads(json.loads((RAW/f'clock-{i}.json').read_text())['stdout'])for i in range(3)]
    floor=100*max(x['p99_ns']for x in clocks)
    comparisons=[]
    for c in CELLS:
        if c[0]!='trace':continue
        for mode in MODES[1:]:
            other=(mode,*c[1:]);ratios=[values[c,k]/values[other,k]for k in range(10)]
            med=statistics.median(ratios)
            qualified=min(statistics.median(values[x,k]for k in range(10))for x in [c,other])>=floor
            verdict='gain' if qualified and med<=.9 and max(ratios)<1 else 'loss' if qualified and med>=1.1 and min(ratios)>1 else 'unresolved'
            comparisons.append(dict(scenario=list(c[1:]),control=mode,median=med,min=min(ratios),max=max(ratios),clock_qualified=qualified,verdict=verdict,trace_ns=statistics.median(values[c,k]for k in range(10)),control_ns=statistics.median(values[other,k]for k in range(10)),trace_memory=memories[c],control_memory=memories[other]))
    return dict(floor_ns=floor,comparisons=comparisons)

if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False)
        # Archive tracked workspace compilation inputs; preserve unrelated untracked work.
        paths=[ROOT/p for p in subprocess.check_output(['git','ls-files','research','crates'],cwd=ROOT,text=True).splitlines()if p.endswith('.rs') or p.endswith('Cargo.toml')]
        paths += [ROOT/'Cargo.toml',ROOT/'Cargo.lock',Path(__file__),Path(own.__file__),ROOT/'docs/experiments/registrations/S05-call-compiled-cost.md',binary('primary'),binary('meter')]
        previous=json.loads((ROOT/'docs/experiments/results/s05-recognition-stride-timing/freeze.json').read_text())
        clock=ROOT/previous['binary'];assert own.sha(clock)==previous['binary_hash'];paths.append(clock)
        for p in paths:
            rel=p.relative_to(ROOT)
            if str(rel).startswith('target/'):continue
            dst=RAW/'registered'/rel;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
        (RAW/'freeze.json').write_text(json.dumps(dict(cells=CELLS,sha256={str(p.relative_to(ROOT)):own.sha(p)for p in paths}),indent=2)+'\n')
        rng=random.Random(607096);schedule=[]
        for kind,reps in [('meter',2),('primary',10)]:
            for k in range(reps):
                order=list(range(720));rng.shuffle(order);schedule.extend((kind,k,i)for i in order)
        (RAW/'schedule.json').write_text(json.dumps(schedule)+'\n')
        for number,(kind,k,i)in enumerate(schedule):
            if number==1440:
                audit(False)
                for j in range(3):invoke([str(clock),'clock-check'],RAW/f'clock-{j}.json')
                print('ownership qualified; starting primary timing',flush=True)
            invoke([str(binary(kind)),*map(str,CELLS[i])],RAW/f'{kind}-{k:02}-{i:03}.json')
            if number%720==719:print(number+1,'/8640 processes',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n')
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text())
        print('8640 terminal processes, exact ownership and 576 timing comparisons verified.')
