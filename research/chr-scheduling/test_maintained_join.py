import unittest
from dataclasses import replace
from source import Rule, State, StepJob, drain
from maintained_join import MaintainedJoin, Selector
from test_source import finish


def t(name, *args): return (name, args)


class MaintainedGate(unittest.TestCase):
    def matches(self, index, context, state, rule=0):
        drain(index.sync(context, state))
        return drain(index.complete(context, rule))

    def test_middle_insert_and_consumption_preserve_prefix(self):
        rules = (Rule((t('left', 0), t('edge', 0, 1)), (t('right', 1),), ('true',)),)
        index = MaintainedJoin(rules, 'selective')
        state = State((), (), 0, ((0,t('left',t('a'))),(2,t('right',t('b')))), next_occurrence=3)
        self.assertEqual(self.matches(index, 0, state), [])
        state = replace(state, store=state.store+((1,t('edge',t('a'),t('b'))),))
        self.assertEqual([x[0] for x in self.matches(index,0,state)], [(0,1,2)])
        state = replace(state, store=state.store[:-2]+((1,t('edge',t('a'),t('b'))),))
        self.assertEqual(self.matches(index,0,state), [])
        self.assertTrue(any(key[1] == (0,1) for key in index.rows))

    def test_alias_dependencies_and_context_locality(self):
        rules = (Rule((), (t('p',t('a')),), ('true',)),)
        index = MaintainedJoin(rules, 'selective')
        a = State((), (0,), 2, ((0,t('p',0)),), ((0,1),), next_occurrence=1)
        self.assertEqual(self.matches(index,0,a), [])
        self.assertEqual(self.matches(index,1,a), [])
        b = replace(a, sub=((0,1),(1,t('a'))))
        self.assertEqual([x[0] for x in self.matches(index,0,b)], [(0,)])
        self.assertEqual(self.matches(index,1,a), [])
        self.assertEqual(index.rows[next(k for k in index.rows if k[1] == (0,))], 1)

    def test_repeated_variables_distinct_occurrences_and_bindings(self):
        rules = (Rule((), (t('p',0,0),t('p',1,1)), ('true',)),)
        index = MaintainedJoin(rules,'selective')
        a = State((),(),1,((0,t('p',0,0)),(1,t('p',0,0))), next_occurrence=2)
        self.assertEqual([x[0] for x in self.matches(index,0,a)],[(0,1),(1,0)])
        b=replace(a,sub=((0,t('a')),))
        self.matches(index,1,b)
        self.assertTrue(any(k[1] == (0,1) and dict(k[2])[0] == 0 for k in index.rows))
        self.assertTrue(any(k[1] == (0,1) and dict(k[2])[0] == t('a') for k in index.rows))

    def test_source_guard_history_and_earlier_selection(self):
        rules=(Rule((t('p',0),),(),('post',t('seen',0)),((0,t('a')),)),
               Rule((),(t('q'),),('true',)))
        for mode in ('full','selective'):
            index=MaintainedJoin(rules,mode)
            state=State((),(),1,((0,t('p',0)),(1,t('q'))),next_occurrence=2)
            for s in (state,replace(state,sub=((0,t('a')),)),
                      replace(state,sub=((0,t('a')),),history=frozenset({(0,(0,))}))):
                expected=StepJob(s,rules,'prefix'); actual=StepJob(s,rules,Selector(index,0))
                self.assertEqual(finish(actual),finish(expected))
                self.assertEqual(actual.action,expected.action)


class SourceIntegration(unittest.TestCase):
    def test_real_simpagation_and_alias_updates(self):
        from maintained_cases import case
        from check_maintained_source import check
        for consuming in (False,True):
            fixture=case(4,2,1,consuming)
            runs=[check(fixture,mode) for mode in ('prefix','full','selective')]
            self.assertEqual(runs[0]['raw'],2)
            for run in runs[1:]:
                self.assertEqual((run['steps'],run['answers'],run['raw']),
                                 (runs[0]['steps'],runs[0]['answers'],runs[0]['raw']))
            self.assertGreater(runs[2]['counts']['maintain.test_reuse'],0)
            self.assertLess(runs[2]['counts']['maintain.test_create'],runs[1]['counts']['maintain.test_create'])

class SupportedProjection(unittest.TestCase):
    def test_eight_context_deltas_match_exhaustive_prefix_oracle(self):
        from check_maintained_store import validate
        rules=(Rule((t('left',0),t('edge',0,1)),(t('right',1),),('true',)),)
        for mode in ('full','selective'):
            index=MaintainedJoin(rules,mode)
            states={i:State((),(),2,((0,t('left',t('a'))),(1,t('edge',t('a'),0))),
                                  ((0,1),),next_occurrence=2) for i in range(8)}
            self.assertTrue(validate(index,states))
            states={i:replace(s,store=s.store+((2,t('right',t('b'))),),next_occurrence=3)
                    for i,s in states.items()}
            self.assertTrue(validate(index,states))
            states={i:replace(s,sub=((0,1),(1,t('b' if i%2 else 'c')))) for i,s in states.items()}
            self.assertTrue(validate(index,states))
            states[1]=replace(states[1],store=states[1].store[:2])
            self.assertTrue(validate(index,states))
            self.assertEqual(index.rows[(0,(0,1,2),((0,t('a')),(1,t('b'))))],0b10101000)

    def test_skipped_root_and_nonbinding_guard_local(self):
        from check_maintained_store import validate
        rules=(Rule((),(t('p',0),),('true',),((0,9),)),)
        for mode in ('full','selective'):
            index=MaintainedJoin(rules,mode)
            a=State((),(0,),2,((0,0),),((0,1),),next_occurrence=1)
            validate(index,{0:a})
            b=replace(a,sub=((0,1),(1,t('p',t('a')))))
            validate(index,{0:b})
            expected=StepJob(b,rules,'prefix'); actual=StepJob(b,rules,Selector(index,0))
            self.assertEqual(finish(actual),finish(expected))
            self.assertEqual(actual.action,('answer',))
            self.assertEqual(b.next_var,2)

class RegisteredEntry(unittest.TestCase):
    def test_supported_update_fixtures(self):
        from maintained_store_cases import cases
        from check_maintained_store import validate
        for fixture in cases():
            for mode in ('full','selective'):
                index=MaintainedJoin(fixture['rules'],mode)
                for _,states in fixture['updates']:
                    validate(index,states)

    def test_four_round_analytic_observations(self):
        from maintained_cases import case
        from check_maintained_source import check
        for consuming in (False,True):
            run=check(case(4,1,4,consuming),'prefix',transitions=False)
            self.assertEqual(run['raw'],1)

class StorageAccounting(unittest.TestCase):
    def test_fork_peak_includes_unchanged_snapshot(self):
        rules=(Rule((),(t('p',0),),('true',)),)
        index=MaintainedJoin(rules)
        state=State((),(),1,((0,t('p',0)),),next_occurrence=1)
        drain(index.sync(0,state))
        before=(index.peak_tests,index.peak_dependencies,index.peak_index_edges)
        drain(index.fork(0,1)); drain(index.sync(1,state))
        self.assertEqual((index.peak_tests,index.peak_dependencies,index.peak_index_edges),
                         tuple(2*x for x in before))

class DeltaAndLifetime(unittest.TestCase):
    def test_fork_alias_invalidates_descendants_and_failed_tests_locally(self):
        from check_maintained_store import validate
        rules=(Rule((t('p',0),t('q',0,1)),(t('r',1),),('true',)),)
        for mode in ('full','selective'):
            index=MaintainedJoin(rules,mode)
            a=State((),(),3,((0,t('p',0)),(1,t('q',0,1)),
                            (2,t('r',1)),(3,t('r',t('a')))),((0,2),),next_occurrence=4)
            drain(index.sync(0,a)); drain(index.fork(0,1))
            b=replace(a,sub=((0,2),(2,t('b')),(1,t('a'))))
            validate(index,{0:b,1:a})
            self.assertEqual(len(drain(index.complete(0,0))),2)
            self.assertEqual(len(drain(index.complete(1,0))),1)

    def test_abandoned_update_cannot_return_cached_success(self):
        index=MaintainedJoin((Rule((),(t('p'),),('true',)),))
        a=State((),(),0,((0,t('p')),),next_occurrence=1)
        drain(index.sync(0,a))
        update=index.sync(0,replace(a,store=()))
        next(update); update.close()
        with self.assertRaises(ValueError):drain(index.sync(0,a))
        with self.assertRaises(ValueError):drain(index.complete(0,0))

if __name__ == '__main__': unittest.main()
