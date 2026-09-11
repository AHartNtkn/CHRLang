"""Matched complete repaired/original/enumeration costs using the existing runner."""
import collections,itertools,json,random,shutil,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
import connected_cost as common
ROOT=common.ROOT
RAW=ROOT/'docs/experiments/results/s06-connected-repair-costs'
def cells():return [(mode,*c[1:]) for c in common.cells() if c[0]=='projection' for mode in ['repaired','original','enumerate']]
def binary(kind,mode):
    folder=f's06-connected-{kind}' if mode=='original' else f's06-connected-repair-{kind}'
    return ROOT/f'target/{folder}/release/examples/connected_cost'
def inspect():
    manifest=json.loads((RAW/'manifest.json').read_text())
    for name,h in manifest['sha256'].items():
        p=ROOT/name if name.startswith('target/') else RAW/'registered'/name
        assert common.sha(p)==h,name
    mem={};timings={};floors={}
    schedule=json.loads((RAW/'schedule.json').read_text());assert len(schedule)==6912
    for kind,k,i in schedule:
        r=json.loads((RAW/f'{kind}-{k:02}-{i:03}.json').read_text());assert r['returncode']==0 and not r['stderr']
        h,rows=common.parsed(r);c=tuple(manifest['cells'][i]);assert h['metered']==(kind=='meter')
        expected=[str(binary(kind,c[0])),'enumerate' if c[0]=='enumerate' else 'projection',*map(str,c[1:])];assert r['command']==expected
        if kind=='meter':
            reading=common.memory(rows)
            if c in mem:assert mem[c]==reading
            mem[c]=reading
        else:timings[c,k]=h['total_ns'];floors[c,k]=h['clock_floor']
    assert len(mem)==576 and len(timings)==5760
    out=[]
    for c in cells():
        if c[0]!='repaired':continue
        for mode in ['original','enumerate']:
            other=(mode,*c[1:]);a,b=mem[c],mem[other];assert a['consumer']==b['consumer']
            ratios=[timings[c,k]/timings[other,k] for k in range(10)];median=statistics.median(ratios)
            qualified=all(statistics.median(timings[x,k] for k in range(10))>max(floors[x,k] for k in range(10)) for x in [c,other])
            verdict='gain' if qualified and median<=.9 and max(ratios)<1 else 'loss' if qualified and median>=1.1 and min(ratios)>1 else 'unresolved'
            out.append(dict(scenario=list(c[1:]),denominator=mode,ratio_median=median,ratio_min=min(ratios),ratio_max=max(ratios),verdict=verdict,clock_qualified=qualified,repaired_memory=a,control_memory=b))
    return out
if __name__=='__main__':
    if sys.argv[1]=='run':
        old=json.loads((common.RAW/'manifest.json').read_text())
        for kind in ['primary','meter']:
            p=binary(kind,'original');assert common.sha(p)==old['sha256'][str(p.relative_to(ROOT))]
        RAW.mkdir(exist_ok=False);common.RAW=RAW
        paths=[Path(__file__),Path(common.__file__),ROOT/'docs/experiments/registrations/S06-connected-repair-costs.md',ROOT/'research/chr-structural/examples/connected_cost.rs',ROOT/'research/chr-structural/examples/support/projection_cost.rs',ROOT/'research/chr-structural/src/joint_region.rs',ROOT/'research/chr-structural/src/projection.rs',ROOT/'research/chr-compiled/experiments/meter.rs',ROOT/'Cargo.lock']
        paths+=list({binary(kind,mode) for kind in ['primary','meter'] for mode in ['original','repaired']})
        for p in paths:
            if str(p.relative_to(ROOT)).startswith('target/'):continue
            dest=RAW/'registered'/p.relative_to(ROOT);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dest)
        common.write('manifest.json',dict(cells=cells(),sha256={str(p.relative_to(ROOT)):common.sha(p) for p in paths}))
        rng=random.Random(607093);schedule=[]
        for kind,repeats in [('meter',2),('primary',10)]:
            for k in range(repeats):
                order=list(range(576));rng.shuffle(order);schedule.extend((kind,k,i) for i in order)
        common.write('schedule.json',schedule);entries={}
        for number,(kind,k,i) in enumerate(schedule):
            c=cells()[i];actual=['enumerate' if c[0]=='enumerate' else 'projection',*c[1:]]
            h,rows=common.invoke(binary(kind,c[0]),actual,f'{kind}-{k:02}-{i:03}.json')
            if kind=='meter':
                m=common.memory(rows)
                if c in entries:assert entries[c]==m
                entries[c]=m
                for mode in ['repaired','original','enumerate']:
                    other=(mode,*c[1:])
                    if other in entries:assert entries[other]['consumer']==m['consumer']
            if number%576==575:print(number+1,'/ 6912 terminal processes',flush=True)
        common.write('analysis.json',inspect())
    else:
        assert inspect()==json.loads((RAW/'analysis.json').read_text());print('6912 terminal processes, exact ownership, full phase sums and frozen inputs verified.')
