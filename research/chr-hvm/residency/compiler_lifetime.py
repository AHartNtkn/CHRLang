"""Check that source compilation does not require later cyclic GC for local emitters."""
import gc,importlib.util,json,sys,types
from pathlib import Path
root=Path(__file__).resolve().parents[3];sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('compiler',root/'research/chr-hvm/source_choice/compiler.py')
c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
source=json.loads((root/'docs/experiments/results/s10-native-substantive/groups.json').read_text())[0][0]
gc.collect();gc.disable()
def retained():
 return sum(isinstance(x,types.FunctionType) and x.__code__.co_filename==c.__file__ and '<locals>' in x.__qualname__ for x in gc.get_objects())
before=retained()
program,predicates,atoms=c.compile_source({k:source[k] for k in ['rules','query','outputs']})
del program,predicates,atoms
after=retained();collected=gc.collect();final=retained();gc.enable()
print(json.dumps(dict(before=before,after_return=after,collected_objects=collected,after_collection=final)))
assert after==before, 'local emitter functions survive until cyclic collection'

spec=importlib.util.spec_from_file_location('prior_compiler',root/'docs/experiments/results/s10-residency/compiler-before.py')
prior=importlib.util.module_from_spec(spec)
# Its relative kernel/emitter paths still refer to the authoritative compiler directory.
prior.__file__=c.__file__
exec(compile((root/'docs/experiments/results/s10-residency/compiler-before.py').read_text(),c.__file__,'exec'),prior.__dict__)
plans=json.loads((root/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
groups=json.loads((root/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
sources=[p['queries'][0]['source'] for p in plans]+[g[0] for g in groups]
for source in sources:
    program=dict(rules=source['rules'],query=[],outputs=[])
    assert c.compile_source(program)==prior.compile_source(program)
print('All26 source groups preserve exact emitted program and dictionaries')
