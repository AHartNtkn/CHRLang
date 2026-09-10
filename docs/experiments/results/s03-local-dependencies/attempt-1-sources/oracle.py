"""Independent atomic effects, with no claim protocol or imported eligibility code."""
def outcomes(case,cancel,priority):
    count,initial_binding,apps=case
    initial=(frozenset(range(count)),initial_binding,frozenset(),frozenset())
    seen={initial};todo=[initial];result=set()
    while todo:
        resources,binding,done,aborted=todo.pop();following=[]
        candidates=[]
        for i,app in enumerate(apps):
            if i not in done|aborted and set(app.take+app.keep)<=resources and (not app.needs_binding or binding):candidates.append(i)
        for i in candidates[:1] if priority else candidates:
            app=apps[i]
            following.append((resources-set(app.take),binding or app.binds,done|{i},aborted))
        if cancel is not None and cancel not in done|aborted:
            following.append((resources,binding,done,aborted|{cancel}))
        if not following:
            result.add((tuple(int(r in resources) for r in range(count)),binding,tuple(sorted(done)),tuple(sorted(aborted))))
        for s in following:
            s=tuple(frozenset(x) if isinstance(x,set) else x for x in s)
            if s not in seen:seen.add(s);todo.append(s)
    return result
