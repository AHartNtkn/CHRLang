"""Factored bounded CHR transitions. Experimental fixed committed policy."""
import z3
from heap import Encoding,Heap,select
from matching import applications,View


class Bounded:
    def __init__(self,rules,constraints,outputs,*,transitions,nodes,occurrences,pending,service):
        if min(occurrences,pending)<1 or transitions<0:raise ValueError('invalid machine bounds')
        self.rules=rules;self.constraints=constraints;self.query_outputs=outputs;self.e=Encoding()
        signature=[]
        def scan(t):
            if isinstance(t,int):return
            signature.append((t[0],len(t[1])))
            for c in t[1]:scan(c)
        def scan_goal(g):
            if g[0]=='post':scan(g[1])
            elif g[0]=='eq':scan(g[1]);scan(g[2])
            elif g[0] in ('and','or'):
                for child in g[1:]:scan_goal(child)
        for c in constraints:scan(c)
        for t in outputs:scan(t)
        for r in rules:
            for c in r.kept+r.removed:scan(c)
            for a,b in r.guards:scan(a);scan(b)
            scan_goal(r.body)
        self.h=Heap(self.e,nodes,signature)
        self.goals=[('true',)]
        bindings={}
        initial=[self.goal(('post',c),bindings,True) for c in constraints]
        self.outputs=[self.term(t,bindings,True) for t in outputs]
        self.queue=[z3.IntVal(i) for i in initial[:pending]]+[z3.IntVal(0)]*max(0,pending-len(initial))
        self.length=z3.IntVal(min(len(initial),pending))
        if len(initial)>pending:self.h.cutoff=z3.BoolVal(True)
        self.roots=[z3.IntVal(0)]*occurrences;self.alive=[z3.BoolVal(False)]*occurrences
        self.next_occ=z3.IntVal(0);self.history={};self.done=z3.BoolVal(False)
        self.choices=[];self.events=[];self.states=[]
        self.capture()
        for tick in range(transitions):
            self.step(tick,service);self.capture()

    def capture(self):
        from types import SimpleNamespace
        h=self.h
        heap=SimpleNamespace(e=h.e,n=h.n,signature=h.signature,ids=h.ids,tags=h.tags,
                             children=list(h.children),sub=h.sub,variables=h.variables)
        self.states.append((heap,list(self.queue),self.length,list(self.roots),list(self.alive),
                            dict(self.history),self.next_occ))

    def term(self,t,bindings,guard):
        if isinstance(t,int):
            if t not in bindings:bindings[t]=self.h.variable(guard)
            return bindings[t]
        return self.h.app(t[0],[self.term(c,bindings,guard) for c in t[1]],guard)

    def goal(self,g,bindings,guard):
        kind=g[0]
        if kind=='post':value=(kind,self.term(g[1],bindings,guard))
        elif kind=='eq':value=(kind,self.term(g[1],bindings,guard),self.term(g[2],bindings,guard))
        elif kind in ('and','or'):value=(kind,*(self.goal(c,bindings,guard) for c in g[1:]))
        elif kind in ('true','fail'):value=g
        else:raise ValueError('unknown goal')
        self.goals.append(value)
        return len(self.goals)-1

    def replace_front(self,children,guard,pop=True):
        count=len(children);base=self.length-(1 if pop else 0)
        fits=base+count<=len(self.queue)
        self.h.cutoff=self.e.bind(z3.Or(self.h.cutoff,z3.And(guard,z3.Not(fits))))
        write=z3.And(guard,fits)
        old=self.queue
        self.queue=[self.e.bind(z3.If(write,children[i] if i<count else old[i-count+(1 if pop else 0)] if i-count+(1 if pop else 0)<len(old) else 0,old[i])) for i in range(len(old))]
        self.length=self.e.bind(z3.If(write,base+count,self.length))

    def step(self,tick,service):
        events=[]
        active=self.e.bind(z3.And(self.h.active(),z3.Not(self.done)))
        if z3.is_false(z3.simplify(active)):
            self.events.append([])
            return
        old_length=self.length;front=self.queue[0]
        pending_guard=self.e.bind(z3.And(active,old_length>0))
        rule_guard=self.e.bind(z3.And(active,old_length==0))
        # Read enabling conditions from the state before this transition.
        candidates=applications(self.h,self.rules,self.roots,self.alive,self.history)
        goals=list(self.goals)
        for index,g in enumerate(goals):
            guard=self.e.bind(z3.And(pending_guard,front==index))
            kind=g[0]
            if kind!='or':events.append((guard,(kind,)))
            if kind=='and':self.replace_front(list(g[1:]),guard)
            elif kind=='or':
                choice=z3.Bool(f'choice_{tick}_{index}')
                self.choices.append((guard,choice))
                events.append((guard,('or',choice)))
                self.e.solver.add(z3.Implies(z3.Not(guard),z3.Not(choice)))
                self.replace_front([z3.If(choice,g[2],g[1])],guard)
            else:
                self.replace_front([],guard)
                if kind=='post':
                    fits=self.next_occ<len(self.roots)
                    write=z3.And(guard,fits)
                    self.roots=[self.e.bind(z3.If(z3.And(write,self.next_occ==i),g[1],v)) for i,v in enumerate(self.roots)]
                    self.alive=[self.e.bind(z3.Or(v,z3.And(write,self.next_occ==i))) for i,v in enumerate(self.alive)]
                    self.next_occ=self.e.bind(z3.If(write,self.next_occ+1,self.next_occ))
                    self.h.cutoff=self.e.bind(z3.Or(self.h.cutoff,z3.And(guard,z3.Not(fits))))
                elif kind=='eq':self.h.unify(g[1],g[2],service,guard)
                elif kind=='fail':self.h.failed=self.e.bind(z3.Or(self.h.failed,guard))
        selected=[]
        for application in candidates:
            guard=self.e.bind(z3.And(rule_guard,application.selected))
            selected.append(application.selected)
            events.append((guard,('apply',application.rule,application.occurrences)))
            rule=self.rules[application.rule]
            bindings=dict(application.bindings)
            # Guard-local variables become fresh only for the committed rule.
            def guard_vars(t):
                if isinstance(t,int):
                    if t not in bindings:bindings[t]=self.h.variable(guard)
                else:
                    for c in t[1]:guard_vars(c)
            for a,b in rule.guards:guard_vars(a);guard_vars(b)
            body=self.goal(rule.body,bindings,guard)
            self.replace_front([z3.IntVal(body)],guard,pop=False)
            removed=set(application.occurrences[len(rule.kept):])
            self.alive=[self.e.bind(z3.And(v,z3.Not(guard))) if i in removed else v for i,v in enumerate(self.alive)]
            token=(application.rule,application.occurrences)
            self.history[token]=self.e.bind(z3.Or(self.history.get(token,z3.BoolVal(False)),guard))
        endpoint=self.e.bind(z3.And(rule_guard,z3.Not(z3.Or(*selected))))
        events.append((endpoint,('answer',)))
        self.events.append(events)
        self.done=self.e.bind(z3.Or(self.done,endpoint))

    def answers(self,limit=None):
        from check_cases import equivalent
        view=View(self.h)
        state_views=[View(state[0]) for state in self.states]
        solver=self.e.solver
        solver.push();solver.add(self.done,z3.Not(self.h.failed),z3.Not(self.h.cutoff))
        answers=[];self.models=0
        def decode(value):
            index=next(i for i in range(view.sort.num_constructors()) if value.decl()==view.sort.constructor(i))
            if index==0:return value.arg(0).as_long()
            return (self.h.signature[index-1][0],tuple(decode(c) for c in value.children()))
        while True:
            status=solver.check()
            if status==z3.unsat:break
            if status!=z3.sat:raise RuntimeError(f'bounded solver returned {status}: {solver.reason_unknown()}')
            model=solver.model();self.models+=1
            answer={'outputs':[decode(model.eval(select(view.values,h))) for h in self.outputs],
                    'residual':[decode(model.eval(select(view.values,h))) for h,a in zip(self.roots,self.alive) if z3.is_true(model.eval(a))]}
            self.verify(model,state_views,decode,answer)
            if not any(equivalent(answer,a) for a in answers):answers.append(answer)
            if limit is not None and len(answers)>=limit:break
            bits=[bit for _,bit in self.choices]
            solver.add(z3.Or(*[bit!=model.eval(bit) for bit in bits]))
        solver.pop()
        return answers


    def boundary_status(self):
        """Existence of resource-cutoff or still-active paths at these bounds."""
        result={}
        for name,predicate in [('resource_cutoff',self.h.cutoff),
                               ('transition_cutoff',z3.And(z3.Not(self.done),z3.Not(self.h.failed),z3.Not(self.h.cutoff)))]:
            self.e.solver.push();self.e.solver.add(predicate)
            status=self.e.solver.check()
            if status==z3.unknown:raise RuntimeError(self.e.solver.reason_unknown())
            result[name]=status==z3.sat
            self.e.solver.pop()
        return result

    def verify(self,model,views,decode,answer):
        from machine import verify_witness
        states=[];actions=[]
        for tick,(state,view) in enumerate(zip(self.states,views)):
            heap,queue,length,roots,alive,history,next_occ=state
            def term(handle):return decode(model.eval(select(view.values,handle)))
            def goal(index):
                g=self.goals[index];kind=g[0]
                if kind=='post':return (kind,term(g[1]))
                if kind=='eq':return (kind,term(g[1]),term(g[2]))
                if kind in ('and','or'):return (kind,*(goal(i) for i in g[1:]))
                return g
            substitution={}
            for i,tag in enumerate(heap.tags):
                if model.eval(tag).as_long()==1:
                    var=model.eval(heap.ids[i]).as_long();value=term(i)
                    if value!=var:substitution[var]=value
            states.append({'pending':[goal(model.eval(g).as_long()) for g in queue[:model.eval(length).as_long()]],
                           'store':[(i,term(root)) for i,(root,a) in enumerate(zip(roots,alive)) if z3.is_true(model.eval(a))],
                           'substitution':substitution,'history':sorted(k for k,v in history.items() if z3.is_true(model.eval(v))),
                           'next_var':model.eval(heap.variables).as_long(),'next_occurrence':model.eval(next_occ).as_long(),
                           'outputs':[term(h) for h in self.outputs]})
            if tick==len(self.events):break
            chosen=[action for guard,action in self.events[tick] if z3.is_true(model.eval(guard))]
            if not chosen:break
            if len(chosen)!=1:raise ValueError('symbolic transition is not uniquely selected')
            action=chosen[0]
            if action[0]=='or':action=('or',z3.is_true(model.eval(action[1])))
            actions.append(action)
        checked=verify_witness(self.rules,self.constraints,self.query_outputs,states,actions)
        if checked!=answer:raise ValueError('decoded endpoint differs from independent witness')
