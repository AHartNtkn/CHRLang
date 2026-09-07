"""Independent exhaustive prefix oracle for the finite supported matcher gate."""
from source import drain, prefix_matches


def expected(rules, state, rule):
    heads=rules[rule].kept+rules[rule].removed
    values=[]
    # Deliberately no candidate predicate index: every occurrence is considered.
    for entry in prefix_matches(heads,[state.store]*len(heads),dict(state.sub)):
        if not isinstance(entry,str):
            selected,bindings=entry
            values.append((tuple(oid for oid,_ in selected),tuple(sorted(bindings.items()))))
    return values


def validate(index, states):
    wanted={}
    for context,state in states.items():
        drain(index.sync(context,state))
        for rule in range(len(index.rules)):
            actual=drain(index.complete(context,rule))
            oracle=expected(index.rules,state,rule)
            assert actual==oracle,(context,rule,actual,oracle)
            for ids,bindings in oracle:
                key=(rule,ids,bindings)
                wanted[key]=wanted.get(key,0)|(1<<context)
    for key,mask in index.rows.items():
        heads=index.rules[key[0]].kept+index.rules[key[0]].removed
        if len(key[1])==len(heads):
            assert wanted.get(key,0)==mask,(key,mask,wanted.get(key,0))
    return True
