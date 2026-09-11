"""Audit the shared-control qualification and compare exact per-step work."""
import itertools,json,re,sys
sys.dont_write_bytecode=True
from audit_probe_projection import ROOT,read,audit

def main():
    out=ROOT/'docs/experiments/results/s01-probe-strong-control'
    names=[f'{f}-{m}-probe_projection-{r}' for f,m in itertools.product([False,True],repeat=2) for r in range(2 if m else 1)]
    reports=audit(out,names)
    for f in [False,True]:assert reports[f'{f}-True-probe_projection-0']==reports[f'{f}-True-probe_projection-1']
    for feature,metrics in itertools.product([False,True],repeat=2):
        raw=read(out/f'{feature}-{metrics}-probe_projection-0.json');assert '2 passed;' in raw['stdout']
        for test in ['partner_order','partner_order_bound','selective_probe']:
            previous=None
            for rep in range(2 if metrics else 1):
                raw=read(out/f'{feature}-{metrics}-{test}-{rep}.json');assert raw['exit_code']==0 and '0 failed;' in raw['stdout']
                if test=='selective_probe':
                    assert 'PROBE_GROUND,cases=2048' in raw['stdout'] and 'PROBE_OPEN,cases=32' in raw['stdout'];lines=re.findall('PROBE_BROAD,[^\n]+',raw['stdout']);assert len(lines)==6
                else:
                    lines=re.findall('PARTNER,id=[^\n]+',raw['stdout']);assert len(lines)==384 and 'PARTNER_SENTINEL,validated=4' in raw['stdout']
                if previous is not None:assert previous==lines
                previous=lines
    control=reports['False-True-probe_projection-0']['summary'];probe=reports['True-True-probe_projection-0']['summary']
    for a,b in zip(control,probe):
        assert (a['n'],a['duplicates'],a['query'])==(b['n'],b['duplicates'],b['query'])
        assert a['steps']==b['steps'] and a['total']['empty']==b['total']['empty']
        assert b['maximum']['bucket']<=2*b['n']
        if a['query']==0:print(a['n'],a['duplicates'],'steps',a['steps'],'control/probe max buckets',a['maximum']['bucket'],b['maximum']['bucket'],'max probes',b['maximum']['probe'])
    (out/'analysis.json').write_text(json.dumps(dict(processes=24,control=control,probe=probe,complete_semantic_gates=True,exact_repeats=True),indent=2)+'\n')
    print('Shared control and probe verified in 24 processes')
if __name__=='__main__':main()
