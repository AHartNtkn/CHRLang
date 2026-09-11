"""Post-campaign phase attribution; no additional timing verdicts."""
import json,statistics,sys
sys.dont_write_bytecode=True
import call_compiled_cost as cost

def diagnose():
    groups={}
    for x in cost.audit()['comparisons']:
        groups.setdefault(tuple(x['scenario']),{})[x['control']]=x
    selected={c for c,v in groups.items()if all(x['verdict']=='gain'for x in v.values())}
    old=json.loads((cost.ROOT/'docs/experiments/results/s05-call-trace-timing/analysis.json').read_text())['comparisons']
    prior=[groups[tuple(x['scenario'])]['direct']for x in old if x['control']=='direct'and x['verdict']=='gain']
    modes={}
    for mode in cost.MODES:
        shares=[];later=[]
        for i,c in enumerate(cost.CELLS):
            if c[0]!=mode or tuple(c[1:])not in selected:continue
            for k in range(10):
                r=json.loads((cost.RAW/f'primary-{k:02}-{i:03}.json').read_text())
                rows=[json.loads(x)for x in r['stdout'].splitlines()][1:]
                perquery=[sum(x['ns']for x in rows if x['phase']=='service_observe'and x['query']==q)for q in range(4)]
                shares.append(sum(perquery)/sum(x['ns']for x in rows))
                later.append(statistics.median(perquery[1:])/perquery[0])
        modes[mode]=dict(service_share_median=statistics.median(shares),later_first_median=statistics.median(later))
    return dict(all_control_gains=[dict(scenario=list(c),comparisons=groups[c])for c in sorted(selected)],prior_direct_gains_now=prior,selected_phase_diagnostics=modes)

if __name__=='__main__':
    path=cost.RAW/'diagnosis.json'
    if sys.argv[1]=='run':
        assert not path.exists();path.write_text(json.dumps(diagnose(),indent=2)+'\n')
    else:
        assert diagnose()==json.loads(path.read_text());print('Eight joint gains, prior-campaign comparison and phase attribution verified.')
