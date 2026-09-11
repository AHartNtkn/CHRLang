"""Ordered caller lifecycle: compact numerical results, separate allocator builds."""
import itertools,json,random,statistics,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
import connected_cost as common
ROOT=common.ROOT
OUT=ROOT/'docs/experiments/results/S06-ordered-cost.json'
CELLS=list(itertools.product(['projected','enumerate','direct'],[2,4],[0,1],[0,3],[1,8],[0,1],[0,1]))
CHALLENGE=sys.argv[1:]==["challenge"]
COLD=sys.argv[1:]==["cold"]
REUSE=sys.argv[1:] in [["reuse"],["cold"]]
if REUSE:
    OUT=OUT.with_name("S06-caller-reuse-cost.json")
    CELLS=list(itertools.product(["projected","enumerate","lazy-enumerate","memo-projected","memo-enumerate","memo-lazy-enumerate","direct"],[2,4],[0,1],[0,3],[1,8],[0,1],[0,1]))
if CHALLENGE:
    OUT=OUT.with_name("S06-ordered-challenge.json")
    scenarios=[(4,1,p,8,k,c)for p,k in [(0,0),(0,1),(3,0),(3,1)]for c in [0,1]]
    CELLS=[(m,*s)for m in ["projected","enumerate","direct","lazy-enumerate"]for s in scenarios]
if COLD:
    OUT=OUT.with_name("S06-caller-reuse-once.json")
    CELLS=list(itertools.product(["projected","enumerate","lazy-enumerate","memo-projected","memo-enumerate","memo-lazy-enumerate"],[2,4],[0,1],[0,3],[1],[0,1],[1]))
def invoke(kind,cell):
    cmd=[str(ROOT/f'target/s06-ordered-{kind}/release/examples/ordered_cost'),*map(str,cell)]
    p=subprocess.run(cmd,capture_output=True,text=True,timeout=45,preexec_fn=common.bounds,cwd=ROOT)
    r=dict(command=cmd,stdout=p.stdout,stderr=p.stderr,returncode=p.returncode)
    if p.returncode or p.stderr:OUT.with_suffix('.failure.json').write_text(json.dumps(r,indent=2)+'\n')
    assert p.returncode==0 and not p.stderr,(cell,p.stderr)
    h,rows=common.parsed(r);assert h['metered']==(kind=='meter')
    return h,rows
if __name__=='__main__':
    memory={};samples={};rng=random.Random(607110 if COLD else 607109 if REUSE else 607108 if CHALLENGE else 607107)
    for i,c in enumerate(CELLS):
        for rep in range(2):
            _,rows=invoke('meter',c);m=common.memory(rows)
            m['phase_requested']={p:sum(r['memory']['requested_bytes']for r in rows if r['phase']==p)for p in sorted(set(r['phase']for r in rows))}
            if rep:assert memory[c]==m,c
            memory[c]=m
        if i%32==31:print('allocation',i+1,'/',len(CELLS),flush=True)
    for c in CELLS:assert memory[c]['consumer']==memory[('enumerate',*c[1:])]['consumer'],c
    if COLD:
        for c in CELLS:
            if c[0].startswith('memo-'):
                for key in ['requested','peak','consumer']:assert memory[c][key]==memory[(c[0][5:],*c[1:])][key],(c,key)
    OUT.with_name('S06-caller-reuse-once-allocation.json'if COLD else'S06-caller-reuse-allocation.json'if REUSE else'S06-ordered-challenge-allocation.json'if CHALLENGE else'S06-ordered-allocation.json').write_text(json.dumps([dict(cell=c,**memory[c])for c in CELLS],indent=2)+'\n')
    for block in range(5):
        order=CELLS.copy();rng.shuffle(order)
        for c in order:
            h,rows=invoke('primary',c)
            samples[c,block]=dict(total_ns=h['total_ns'],floor_ns=h['clock_floor'],phases={p:sum(r['ns']for r in rows if r['phase']==p)for p in sorted(set(r['phase']for r in rows))})
        print('ordinary block',block+1,'/5',flush=True)
    comparisons=[]
    for c in CELLS:
        if REUSE:
            if not c[0].startswith('memo-'):continue
            controls=[c[0][5:]]+(['memo-enumerate','memo-lazy-enumerate','direct']if c[0]=='memo-projected'and not COLD else[])
        else:
            if c[0]!='projected':continue
            controls=['enumerate','direct','lazy-enumerate']if CHALLENGE else['enumerate','direct']
        for mode in controls:
            other=(mode,*c[1:]);ratios=[samples[c,k]['total_ns']/samples[other,k]['total_ns']for k in range(5)];median=statistics.median(ratios)
            qualified=all(statistics.median(samples[x,k]['total_ns']for k in range(5))>max(samples[x,k]['floor_ns']for k in range(5))for x in [c,other])
            status='gain'if qualified and median<=.9 and max(ratios)<1 else'loss'if qualified and median>=1.1 and min(ratios)>1 else'uncertain'
            comparisons.append(dict(cell=c,control=mode,median=median,min=min(ratios),max=max(ratios),qualified=qualified,status=status))
    OUT.write_text(json.dumps(dict(samples=[dict(cell=c,runs=[samples[c,k]for k in range(5)])for c in CELLS],comparisons=comparisons),indent=2)+'\n')
    print(len(CELLS)*2,'allocation +',len(CELLS)*5,'ordinary processes complete',flush=True)
