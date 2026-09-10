"""Source-derived native compiler for ground, consuming, guard-free CHR.

Counts are a quotient of indistinguishable ground occurrences, not a general
resource identity model. Propagation and non-ground source are outside admission.
"""
from collections import Counter
import re

def linear(body,variables):
    prefix=''
    label=9000000
    for variable in variables:
        pattern=r'\b'+variable+r'\b'
        count=len(re.findall(pattern,body))
        current=variable
        for i in range(count-1):
            name=f'd{label}'
            prefix+=f'!{name}&({label})={current};'
            body=re.sub(pattern,name+'₀',body,count=1)
            current=name+'₁';label+=1
        if count:body=re.sub(pattern,current,body,count=1)
    return prefix+body

def compile_source(source):
    if set(source)-{'name','predicates','rules','query'}:raise ValueError('source fields outside admission')
    predicates=source['predicates'];rules=source['rules']
    if not (0 < len(predicates)<=12 and len(set(predicates))==len(predicates) and all(isinstance(p,str) and re.fullmatch('[a-z][a-z0-9]*',p) for p in predicates)):
        raise ValueError('predicate admission')
    for rule in rules:
        if set(rule)!={'kept','removed','alternatives'}:raise ValueError('rule fields outside admission')
        if not rule['removed'] or len(rule['alternatives']) not in [1,2]:raise ValueError('consuming binary source required')
        if not all(p in predicates for p in rule['kept']+rule['removed']):raise ValueError('ground head admission')
        for arm in rule['alternatives']:
            if arm is not None and (not all(p in predicates for p in arm) or len(arm)>len(rule['removed'])):raise ValueError('ground non-growing body required')
    if len(source['query'])>64 or not all(p in predicates for p in source['query']):raise ValueError('query admission')
    variables=[f'x{i}' for i in range(len(predicates))]+['label']
    def call(i,counts,label):return f'@r{i}('+','.join(counts+[label])+')'
    chunks=['@choose = λ{0: λa.λb.b; λn.λa.λb.a}']
    for i,rule in enumerate(rules):
        need=Counter(rule['kept']+rule['removed']);removed=Counter(rule['removed'])
        checks=[f'(x{j}>={need[p]})' for j,p in enumerate(predicates) if need[p]]
        condition=' && '.join(checks)
        arms=[]
        for arm_index,arm in enumerate(rule['alternatives']):
            if arm is None:arms.append('&{}');continue
            added=Counter(arm)
            counts=[f'(x{j}-{removed[p]}+{added[p]})' for j,p in enumerate(predicates)]
            new_label=f'(label*2+{arm_index})' if len(rule['alternatives'])==2 else 'label'
            arms.append(call(0,counts,new_label))
        action=arms[0] if len(arms)==1 else '@choose((label<8388608),&(label){'+','.join(arms)+'},#ChoiceLimit)'
        body=f'@choose(({condition}),{action},{call(i+1,variables[:-1],"label")})'
        chunks.append(f'@r{i} = '+''.join('λ'+v+'.' for v in variables)+linear(body,variables))
    answer='#Nil'
    for v in reversed(variables[:-1]):answer='#Cons{'+v+','+answer+'}'
    chunks.append(f'@r{len(rules)} = '+''.join('λ'+v+'.' for v in variables)+'#Answer{'+answer+'}')
    counts=Counter(source['query']);chunks.append('@main = '+call(0,[str(counts[p]) for p in predicates],'1'))
    return '\n'.join(chunks)+'\n'
