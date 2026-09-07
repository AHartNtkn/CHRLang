"""Cross-check the direct transition service with E00 expectations and reference.

Usage: cargo run -q -p chr-symbolic-fixtures --example export_cases | python research/chr-symbolic/check_cases.py
"""
import json
import sys
from machine import Rule,Search


def freeze(x):
    if isinstance(x,list):return tuple(freeze(v) for v in x)
    if isinstance(x,dict):return {k:freeze(v) for k,v in x.items()}
    return x


def equivalent(a,b):
    def match(x,y,forward,backward):
        if isinstance(x,int) or isinstance(y,int):
            if not isinstance(x,int) or not isinstance(y,int):return False
            if x in forward:return forward[x]==y
            if y in backward:return False
            forward[x]=y;backward[y]=x;return True
        return x[0]==y[0] and len(x[1])==len(y[1]) and all(match(c,d,forward,backward) for c,d in zip(x[1],y[1]))
    forward={};backward={}
    if len(a['outputs'])!=len(b['outputs']) or len(a['residual'])!=len(b['residual']):return False
    if not all(match(x,y,forward,backward) for x,y in zip(a['outputs'],b['outputs'])):return False
    def residual(index,remaining,f,r):
        if index==len(a['residual']):return True
        for j in remaining:
            nf,nr=dict(f),dict(r)
            if match(a['residual'][index],b['residual'][j],nf,nr) and residual(index+1,remaining-{j},nf,nr):return True
        return False
    return residual(0,set(range(len(b['residual']))),forward,backward)


def check(case):
    rules=[Rule(r['kept'],r['removed'],r['body'],r['guards']) for r in case['rules']]
    run=Search(rules,case['constraints'],case['outputs'])
    answers=[]
    for _ in range(case['budget']):
        for answer in run.advance(1):
            if not any(equivalent(answer,old) for old in answers):answers.append(answer)
        if run.exhausted or case['limit'] is not None and len(answers)>=case['limit']:break
    assert run.exhausted==case['exhausted'],(case['id'],'exhaustion')
    assert len(run.completed)==case['raw'],(case['id'],'raw',len(run.completed),case['raw'])
    for key in ['expected','reference']:
        assert len(answers)==len(case[key]),(case['id'],key,'count',len(answers),len(case[key]))
        assert all(any(equivalent(a,b) for b in case[key]) for a in answers),(case['id'],key,'answers',answers,case[key])
    return run.steps


if __name__=='__main__':
    count=0;steps=0
    for line in sys.stdin:
        case=freeze(json.loads(line));steps+=check(case);count+=1
    assert count>0,'no fixtures supplied'
    print(f'{count} cases passed; {steps} direct transitions; full residual alpha equivalence and raw multiplicity checked')
