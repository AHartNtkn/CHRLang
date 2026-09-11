"""Requested heap costs of reclaiming call traces between query windows."""
import itertools,json,random,resource,statistics,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
import call_trace_ownership as own
ROOT=own.ROOT
OUTPUT=ROOT/'docs/experiments/results/S05-call-reclamation.json'
MODES=['trace','trace1','trace4','trace16','direct','planned-sealed']
def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(1<<30,1<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(60,60))
def run():
    rows=[];consumers={}
    for family,policy,cancel,keep,mode in itertools.product(range(4),[0,2,3],[0,1],['0','all'],MODES):
        cell=[mode,family,8,32,keep,cancel,policy];previous=None
        for repetition in range(2):
            command=[str(ROOT/'target/s05-call-reclamation/release/examples/call_trace_ownership'),*map(str,cell)]
            p=subprocess.run(command,capture_output=True,text=True,timeout=90,preexec_fn=bounds,cwd=ROOT)
            receipt=dict(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
            if p.returncode:
                OUTPUT.with_suffix('.failure.json').write_text(json.dumps(dict(command=command,**receipt),indent=2)+'\n')
            header,allocation=own.parse(receipt)
            assert header['counts']==[1 if cancel or family==1 else 2]*32
            assert header['oracle_classes']=={0:1,2:4,3:16}[policy]
            phases=[json.loads(line)for line in p.stdout.splitlines()[1:]]
            base=phases[0]['memory']['live_start'];maintenance=[r['memory']for r in phases if r['phase']=='maintenance']
            assert len(maintenance)==32
            allocation.pop('query_retained')
            allocation['retained_after_maintenance']=[m['live_end']-base for m in maintenance]
            allocation['maintenance_freed']=[m['live_start']-m['live_end']for m in maintenance]
            allocation['phase_requested']={name:sum(r['memory']['requested_bytes']for r in phases if r['phase']==name)for name in sorted(set(r['phase']for r in phases))}
            if previous is not None:assert previous==allocation,(cell,previous,allocation)
            previous=allocation
        key=family,policy,cancel,keep
        if key in consumers:assert consumers[key]==allocation['consumer'],cell
        consumers[key]=allocation['consumer']
        rows.append(dict(cell=cell,repetitions=2,**allocation))
        OUTPUT.write_text(json.dumps(rows,indent=2)+'\n')
        print('qualified',cell,flush=True)
    assert len(rows)==288
    print('576 allocation runs; complete answers, repeatability and final heap restoration pass.',flush=True)
def timing():
    cells=[x['cell']for x in json.loads(OUTPUT.read_text())];samples={};rng=random.Random(607100)
    clock=json.loads((ROOT/'docs/experiments/results/s05-call-amortization-planned/freeze.json').read_text())['clock']['path']
    clocks=[json.loads(subprocess.check_output([clock,'clock-check'],text=True))for _ in range(3)]
    floor=100*max(x['p99_ns']for x in clocks)
    for block in range(5):
        order=list(range(len(cells)));rng.shuffle(order)
        for i in order:
            cell=cells[i];command=[str(ROOT/'target/s05-call-reclamation-time/release/examples/call_trace_ownership'),*map(str,cell)]
            p=subprocess.run(command,capture_output=True,text=True,timeout=90,preexec_fn=bounds,cwd=ROOT)
            h,mem=own.parse(dict(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert mem is None
            assert h['counts']==[1 if cell[5]or cell[1]==1 else 2]*32
            phases=[json.loads(line)for line in p.stdout.splitlines()[1:]]
            samples[i,block]=sum(x['ns']for x in phases)
        print('timing block',block,'complete',flush=True)
    comparisons=[];index={tuple(c):i for i,c in enumerate(cells)}
    for i,cell in enumerate(cells):
        if not cell[0].startswith('trace'):continue
        for control in ['direct','planned-sealed']+(['trace']if cell[0]!='trace'else[]):
            j=index[control,*cell[1:]];ratios=[samples[i,k]/samples[j,k]for k in range(5)];median=statistics.median(ratios)
            qualified=min(statistics.median(samples[x,k]for k in range(5))for x in [i,j])>=floor
            status='gain'if qualified and median<=.9 and max(ratios)<1 else'loss'if qualified and median>=1.1 and min(ratios)>1 else'uncertain'
            comparisons.append(dict(cell=cell,control=control,median=median,min=min(ratios),max=max(ratios),status=status,qualified=qualified))
    assert len(comparisons)==528
    OUTPUT.with_name('S05-call-reclamation-time.json').write_text(json.dumps(dict(clocks=clocks,floor_ns=floor,samples=[dict(cell=c,ns=[samples[i,k]for k in range(5)])for i,c in enumerate(cells)],comparisons=comparisons),indent=2)+'\n')
def peak_diagnosis():
    rows=json.loads(OUTPUT.read_text());lookup={tuple(x['cell']):x for x in rows};out=[]
    for mode in ['trace1','trace4','trace16']:
        selected=max((x for x in rows if x['cell'][0]==mode),key=lambda x:x['peak']/lookup['trace',*x['cell'][1:]]['peak'])
        for control in [mode,'trace']:
            cell=[control,*selected['cell'][1:]]
            p=subprocess.run([str(ROOT/'target/s05-call-reclamation/release/examples/call_trace_ownership'),*map(str,cell)],capture_output=True,text=True,timeout=90,preexec_fn=bounds)
            _,memory=own.parse(dict(returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert memory['peak']==lookup[tuple(cell)]['peak']
            phases=[json.loads(line)for line in p.stdout.splitlines()[1:]];base=phases[0]['memory']['live_start'];peak=max(phases,key=lambda r:r['memory']['peak_live'])
            out.append(dict(cell=cell,peak_phase=peak['phase'],peak_query=peak['query'],peak_bytes=memory['peak'],live_before_peak=peak['memory']['live_start']-base,consumer_bytes=memory['consumer']))
    OUTPUT.with_name('S05-call-reclamation-peaks.json').write_text(json.dumps(out,indent=2)+'\n')
if __name__=='__main__':
    if sys.argv[1:] == ['time']:timing()
    elif sys.argv[1:] == ['peaks']:peak_diagnosis()
    else:run()
