"""Cancel substantial live work, retain consumers, then reuse the same preparation."""
import json
from gate import ROOT,OUT,invoke,check,joined,common

def main():
    families={'common':json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text()),'substantive':json.loads((ROOT/'docs/experiments/results/s10-native-substantive/groups.json').read_text())}
    selected={}
    for line in (OUT/'runs.jsonl').read_text().splitlines():
        r=json.loads(line)
        if bytes.fromhex(r['result']['stdout_hex']).startswith(b'UNSUPPORTED'):continue
        group=families[r['family']][r['group']];events=[json.loads(l) for l in r['result']['stderr'].splitlines()][1:-1]
        for s,e in zip(group,events):
            if not e['exhausted']:continue
            if r['mode'] not in selected or e['calls']>selected[r['mode']]['calls']:selected[r['mode']]=dict(source=s,calls=e['calls'])
    assert set(selected)==set(range(13))
    (OUT/'reuse-selection.json').write_text(json.dumps(selected,indent=2)+'\n')
    with (OUT/'retained-reuse.jsonl').open('w') as log:
        for mode,choice in selected.items():
            s=choice['source'];budgets=[0,1,64,choice['calls']//2,200000];standalone=[]
            for budget in budgets:
                r=invoke(mode,s['input'],budget);a=check(r,[s],budget==200000);assert a is not None
                standalone.append(dict(budget=budget,result=r))
            r=invoke(mode,joined([s]*5),','.join(map(str,budgets)));a=check(r,[s]*5,False);assert a is not None
            record=dict(mode=mode,budgets=budgets,standalone=standalone,result=r)
            log.write(json.dumps(record)+'\n');log.flush()
            for i,(actual,alone) in enumerate(zip(a,standalone)):
                expected=check(alone['result'],[s],False)[0]
                assert actual['answers']==expected['answers'] and actual['exhausted']==expected['exhausted']
                e=json.loads(r['stderr'].splitlines()[i+1]);assert e['calls']<=budgets[i]
            assert a[-1]['exhausted'] and sorted(map(common.normalize,a[-1]['answers']))==sorted(map(common.normalize,s['expected']))
    print('65 retained-consumer queries agree with 65 standalone budget controls; all13 final queries complete after cancellation.')
if __name__=='__main__':main()
