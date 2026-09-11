"""Complete ownership of live call traces and matched existing controls."""
from pathlib import Path
import hashlib,itertools,json,random,resource,shutil,subprocess,sys
ROOT=Path(__file__).resolve().parents[3]
RAW=ROOT/'docs/experiments/results/s05-call-trace-ownership'
def cells():
    return [(m,f,n,q,k,c,u)for m,f,n,(q,u),k,c in itertools.product(['trace','direct','memo','memo16'],range(4),[4,32,128],[(1,0),(4,0),(4,1)],['0','all'],[0,1])]
def binary(kind):return ROOT/f'target/s05-call-trace-{kind}/release/examples/call_trace_ownership'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30));resource.setrlimit(resource.RLIMIT_CPU,(30,30))
def parse(r):
    assert r['returncode']==0 and not r['stderr']
    data=[json.loads(x)for x in r['stdout'].splitlines()];h,rows=data[0],data[1:]
    assert [x['phase']for x in rows[:3]]==['source','prepare','source_dispose']
    assert [x['phase']for x in rows[-2:]]==['prepared_dispose','consumer_dispose']
    assert all(x['phase'] in ['source','prepare','source_dispose','input','setup','service_observe','consume','engine_dispose','input_dispose','maintenance','prepared_dispose','consumer_dispose']for x in rows)
    if rows[0]['memory'] is None:return h,None
    rs=[x['memory']for x in rows];base=rs[0]['live_start']
    assert rs[-1]['live_end']==base and h['unreleased_bytes']==0
    assert rs[-1]['live_start']-base==h['consumer_bytes']
    for a,b in zip(rs,rs[1:]):assert a['live_end']==b['live_start']
    assert sum(x['requested_bytes']for x in rs)==h['requested_bytes']
    return h,dict(requested=h['requested_bytes'],peak=max(x['peak_live']for x in rs)-base,consumer=h['consumer_bytes'],query_retained=[x['memory']['live_end']-base for x in rows if x['phase']=='input_dispose'])
def audit():
    m=json.loads((RAW/'manifest.json').read_text())
    for name,h in m['sha256'].items():assert sha(ROOT/name if name.startswith('target/') else RAW/'registered'/name)==h,name
    memory={};observations={};seen=set()
    for kind,k,i in json.loads((RAW/'schedule.json').read_text()):
        assert (kind,k,i) not in seen;seen.add((kind,k,i));r=json.loads((RAW/f'{kind}-{k}-{i:03}.json').read_text());cell=tuple(m['cells'][i])
        assert r['command']==[str(binary(kind)),*map(str,cell)]
        h,mem=parse(r);obs=(h['counts'],h['retained'])
        if cell in observations:assert observations[cell]==obs
        observations[cell]=obs
        if kind=='meter':
            if cell in memory:assert memory[cell]==mem
            memory[cell]=mem
        else:assert mem is None
    assert len(seen)==1728 and len(memory)==576
    out=[]
    for cell in cells():
        if cell[0]!='trace':continue
        for mode in ['direct','memo','memo16']:
            other=(mode,*cell[1:]);a,b=memory[cell],memory[other];assert a['consumer']==b['consumer'];assert observations[cell]==observations[other]
            out.append(dict(scenario=list(cell[1:]),control=mode,trace=a,other=b))
    return out
if __name__=='__main__':
    if sys.argv[1]=='run':
        RAW.mkdir(exist_ok=False)
        paths=[Path(__file__),ROOT/'research/chr-reuse/examples/call_trace_ownership.rs',ROOT/'research/chr-reuse/examples/support/call_trace_source.rs',ROOT/'research/chr-reuse/src/calls.rs',ROOT/'research/chr-reuse/src/calls/trace.rs',ROOT/'research/chr-reuse/src/calls/trace/caller.rs',ROOT/'research/chr-reuse/src/residuals.rs',ROOT/'research/chr-persistent/src/state.rs',ROOT/'research/chr-persistent/src/continuations.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'docs/experiments/registrations/S05-call-trace-ownership.md',binary('meter'),binary('primary')]
        for p in paths:
            if str(p.relative_to(ROOT)).startswith('target/'):continue
            dest=RAW/'registered'/p.relative_to(ROOT);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dest)
        (RAW/'manifest.json').write_text(json.dumps(dict(cells=cells(),sha256={str(p.relative_to(ROOT)):sha(p)for p in paths}),indent=2)+'\n')
        rng=random.Random(607094);schedule=[]
        for kind,count in [('meter',2),('primary',1)]:
            for k in range(count):
                order=list(range(576));rng.shuffle(order);schedule.extend((kind,k,i)for i in order)
        (RAW/'schedule.json').write_text(json.dumps(schedule)+'\n');entries={}
        for number,(kind,k,i) in enumerate(schedule):
            cell=cells()[i];cmd=[str(binary(kind)),*map(str,cell)]
            try:
                p=subprocess.run(cmd,capture_output=True,text=True,timeout=45,preexec_fn=bounds);r=dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
            except subprocess.TimeoutExpired as e:
                r=dict(command=cmd,timeout=True,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode())
            (RAW/f'{kind}-{k}-{i:03}.json').write_text(json.dumps(r,indent=2)+'\n');h,mem=parse(r)
            if mem is not None:
                if cell in entries:assert entries[cell]==mem
                entries[cell]=mem
                for mode in ['trace','direct','memo','memo16']:
                    other=(mode,*cell[1:])
                    if other in entries:assert entries[other]['consumer']==mem['consumer'],(cell,mem,entries[other])
            if number%144==143:print(number+1,'/ 1728 terminal processes',flush=True)
        (RAW/'analysis.json').write_text(json.dumps(audit(),indent=2)+'\n')
    else:
        assert audit()==json.loads((RAW/'analysis.json').read_text());print('1728 processes, exact ownership, ordered observations and frozen inputs verified.')
