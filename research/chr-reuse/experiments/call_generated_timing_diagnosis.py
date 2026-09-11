"""Registered overlap and phase diagnostics, without new timing verdicts."""
import json,statistics,sys
sys.dont_write_bytecode=True
import call_generated_timing as timing
ROOT=timing.ROOT
RAW=timing.RAW

def diagnose():
    comparisons=timing.audit()['comparisons'];groups={}
    for x in comparisons:
        if x['candidate']=='trace':groups.setdefault(tuple(x['scenario']),{})[x['control']]=x
    prior=json.loads((ROOT/'docs/experiments/results/s05-call-compiled-cost/diagnosis.json').read_text())['all_control_gains']
    old=[dict(scenario=x['scenario'],now=groups[tuple(x['scenario'])])for x in prior]
    all_gains=[dict(scenario=list(c),comparisons=v)for c,v in groups.items()if all(x['verdict']=='gain'for x in v.values())]
    selected=[x for x in comparisons if x['candidate']=='generated'and x['verdict']=='gain']
    phases=[]
    for x in selected:
        cell=x['scenario'];entry=dict(scenario=cell,comparison=x,phases={})
        for mode in ['generated','indexed']:
            i=timing.CELLS.index((mode,*cell));blocks=[]
            for k in range(10):
                r=json.loads((RAW/f'{k:02}-{i:04}.json').read_text());rows=[json.loads(line)for line in r['stdout'].splitlines()][1:];sums={}
                for row in rows:sums[row['phase']]=sums.get(row['phase'],0)+row['ns']
                blocks.append(sums)
            entry['phases'][mode]={key:statistics.median(b[key]for b in blocks)for key in blocks[0]}
        phases.append(entry)
    # Exploratory stronger-control screen: medians are hypotheses, not new verdicts.
    values={}
    for k,i in json.loads((RAW/'schedule.json').read_text()):
        r=json.loads((RAW/f'{k:02}-{i:04}.json').read_text())
        values[timing.CELLS[i],k]=sum(json.loads(line)['ns']for line in r['stdout'].splitlines()[1:])
    screen=[]
    for c in timing.CELLS:
        if not c[0].startswith('generated'):continue
        others={}
        for b in ['direct','scan','indexed','sealed']:
            rs=[values[c,k]/values[(b,*c[1:]),k]for k in range(10)]
            others[b]=dict(median=statistics.median(rs),min=min(rs),max=max(rs))
        screen.append(dict(cell=list(c),controls=others))
    blocks=[]
    for k in range(10):
        rs=[values[c,k]/values[('sealed',*c[1:]),k]for c in timing.CELLS if c[0]=='generated-sealed']
        blocks.append(dict(block=k,median=statistics.median(rs),below_one=sum(r<1 for r in rs),above_one=sum(r>1 for r in rs)))
    return dict(prior_eight_now=old,all_six_control_gains=all_gains,generated_gain_phases=phases,exploratory_stronger_controls=screen,inferred_block_diagnostics=blocks)
if __name__=='__main__':
    path=RAW/'diagnosis.json'
    if sys.argv[1]=='run':
        assert not path.exists();path.write_text(json.dumps(diagnose(),indent=2)+'\n')
    else:
        assert diagnose()==json.loads(path.read_text());print('Prior-eight overlap, six-control gains and generated phase attribution verified.')
