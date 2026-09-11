"""Complete connected sparse/Cartesian/enumeration costs, without artifact bookkeeping."""
import itertools,json,random,statistics,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
import connected_cost as common
ROOT=common.ROOT
OUTPUT=ROOT/'docs/experiments/results/S06-sparse-cost.json'
CELLS=list(itertools.product(['sparse','projection','enumerate'],['star','clique','different','dense'],[4,6],[0,1],[0,1],[1,4],[0,1],[0,1]))
def invoke(kind,cell):
    command=[str(ROOT/f'target/s06-sparse-{kind}/release/examples/connected_cost'),*map(str,cell)]
    p=subprocess.run(command,capture_output=True,text=True,timeout=45,preexec_fn=common.bounds,cwd=ROOT)
    receipt=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    if p.returncode or p.stderr:
        OUTPUT.with_suffix('.failure.json').write_text(json.dumps(receipt,indent=2)+'\n')
    assert p.returncode==0 and not p.stderr,(cell,p.stderr)
    return common.parsed(receipt)
def run():
    memory={};samples={};rng=random.Random(607104 if LAZY else 607103 if LONG else 607102 if CHALLENGE else 607101)
    for i,cell in enumerate(CELLS):
        for repeat in range(2):
            header,rows=invoke('meter',cell);assert header['metered']
            value=common.memory(rows)
            value['phase_requested']={phase:sum(r['memory']['requested_bytes']for r in rows if r['phase']==phase)for phase in sorted(set(r['phase']for r in rows))}
            if repeat:assert memory[cell]==value,cell
            memory[cell]=value
        if i%64==63:print('allocation cells',i+1,'/',len(CELLS),flush=True)
    for cell in CELLS:
        assert memory[cell]['consumer']==memory['enumerate',*cell[1:]]['consumer'],cell
    OUTPUT.with_name('S06-lazy-allocation.json'if LAZY else'S06-projection-reuse-allocation.json'if LONG else'S06-sparse-dense-allocation.json'if CHALLENGE else'S06-sparse-allocation.json').write_text(json.dumps([dict(cell=c,**memory[c])for c in CELLS],indent=2)+'\n')
    for block in range(5):
        order=CELLS.copy();rng.shuffle(order)
        for cell in order:
            header,rows=invoke('primary',cell);assert not header['metered']
            phases={phase:sum(r['ns']for r in rows if r['phase']==phase)for phase in sorted(set(r['phase']for r in rows))}
            samples[cell,block]=dict(total_ns=header['total_ns'],floor_ns=header['clock_floor'],phases=phases)
        print('ordinary block',block,'complete',flush=True)
    comparisons=[]
    for cell in CELLS:
        if cell[0]!=('sparse-lazy'if LAZY else'sparse'):continue
        for mode in (['sparse','projection-lazy','enumerate']if LAZY else ['projection','enumerate']+(['separable']if CHALLENGE else [])):
            other=(mode,*cell[1:]);ratios=[samples[cell,k]['total_ns']/samples[other,k]['total_ns']for k in range(5)]
            median=statistics.median(ratios)
            qualified=all(statistics.median(samples[c,k]['total_ns']for k in range(5))>max(samples[c,k]['floor_ns']for k in range(5))for c in [cell,other])
            status='gain'if qualified and median<=.9 and max(ratios)<1 else'loss'if qualified and median>=1.1 and min(ratios)>1 else'uncertain'
            comparisons.append(dict(cell=cell,control=mode,median=median,min=min(ratios),max=max(ratios),status=status,qualified=qualified))
    assert len(comparisons)==(3*len(CELLS)//4 if LAZY else 192 if CHALLENGE else 2*len(CELLS)//3)
    OUTPUT.write_text(json.dumps(dict(samples=[dict(cell=c,runs=[samples[c,k]for k in range(5)])for c in CELLS],comparisons=comparisons),indent=2)+'\n')
    print(len(CELLS)*2,'allocation and',len(CELLS)*5,'ordinary processes complete.',flush=True)
def profile():
    out=[]
    for family,mode in itertools.product(['star','clique','different','dense'],['sparse','projection']):
        cell=[mode,family,6,1,0,4,0,0];runs=[]
        for _ in range(3):
            command=[str(ROOT/'target/s06-sparse-profile/release/examples/connected_cost'),*map(str,cell)]
            p=subprocess.run(command,capture_output=True,text=True,timeout=45,preexec_fn=common.bounds,cwd=ROOT)
            assert p.returncode==0 and not p.stderr,(cell,p.stderr)
            lines=[json.loads(x)for x in p.stdout.splitlines()];phases=lines[1]['preparation_ns']
            prepare=next(x['ns']for x in lines[2:]if x['phase']=='prepare')
            assert sum(phases)<=prepare
            runs.append(dict(phases=phases,prepare_ns=prepare))
        out.append(dict(cell=cell,runs=runs))
    OUTPUT.with_name('S06-sparse-preparation.json').write_text(json.dumps(out,indent=2)+'\n')
LAZY=sys.argv[1:]==['lazy']
LONG=sys.argv[1:]==['long']
CHALLENGE=sys.argv[1:]==['dense']
if CHALLENGE:
    OUTPUT=OUTPUT.with_name('S06-sparse-dense-cost.json')
    CELLS=list(itertools.product(['sparse','projection','enumerate','separable'],['dense'],[4,6],[0,1],[0,1],[1,4],[0,1],[0,1]))
if LONG:
    OUTPUT=OUTPUT.with_name('S06-projection-reuse.json')
    CELLS=list(itertools.product(['sparse','projection','enumerate'],['star','clique','different'],[6],[0,1],[0,1],[4,16,64,256],[0,1],[0,1]))
if LAZY:
    OUTPUT=OUTPUT.with_name('S06-lazy-cost.json')
    CELLS=list(itertools.product(['sparse','sparse-lazy','projection-lazy','enumerate'],['star','clique','different'],[6],[0,1],[0,1],[4,16,64,256],[0,1],[0,1]))
if __name__=='__main__':
    if sys.argv[1:]==['profile']:profile()
    else:run()
