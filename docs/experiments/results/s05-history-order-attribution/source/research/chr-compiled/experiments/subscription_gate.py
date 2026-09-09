from pathlib import Path
import subprocess,json,hashlib
out=Path('docs/experiments/results/s01-subscription-kernel')
target=Path('research/chr-compiled/experiments/subscription_join.rs')
original=target.read_text()
def run(name,command,expected=0):
 p=subprocess.run(command,text=True,capture_output=True,timeout=120)
 (out/(name+'.json')).write_text(json.dumps(dict(command=command,exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr),indent=2)+'\n')
 if expected==0:assert p.returncode==0,(name,p.stderr)
 else: assert p.returncode!=0 and 'panicked at' in p.stdout,(name,p.stdout,p.stderr)
 print(name,p.returncode,flush=True)
base=['cargo','test','-p','chr-compiled','--release','--test','subscription_join_gate','--test','subscription_source_gate']
run('release-metrics',base)
run('release-counter-free',base+['--no-default-features'])
run('clippy',['cargo','clippy','-p','chr-compiled','--no-default-features','--test','subscription_join_gate','--test','subscription_source_gate','--','-D','warnings'])
variants={
 'fault-retirement':('                for t in tuples {\n                    self.forget(t);\n                }','                for _t in tuples {}'),
 'fault-occurrence-collapse':('        out.sort_unstable();','        out.sort_unstable();\n        out.dedup_by_key(|t| self.key(*t));'),
 'fault-invalidation':('        for t in tuples {\n            self.forget(t);\n        }\n        self.relations[relation].remove(id);','        for _t in tuples {}\n        self.relations[relation].remove(id);'),
}
try:
 for name,(old,new) in variants.items():
  assert original.count(old)==1,(name,original.count(old))
  target.write_text(original.replace(old,new))
  (out/(name+'.patch.txt')).write_text('Replace exactly:\n'+old+'\nWith:\n'+new+'\n')
  run(name,['cargo','test','-p','chr-compiled','--release','--test','subscription_join_gate'],1)
finally:target.write_text(original)
run('release-restored',base)
files=[target,Path('research/chr-compiled/experiments/subscription_source.rs'),Path('research/chr-compiled/tests/subscription_join_gate.rs'),Path('research/chr-compiled/tests/subscription_source_gate.rs')]
(out/'source-hashes.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files},indent=2)+'\n')
