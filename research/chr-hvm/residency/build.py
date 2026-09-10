from pathlib import Path
import subprocess,json,hashlib
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-residency';build=root/'target/s10-residency';prior=root/'target/s10-publication-clock'
known=json.loads((root/'docs/experiments/results/s10-publication-clock/validation.json').read_text())['hashes']
assert hashlib.sha256((prior/'native-primary').read_bytes()).hexdigest()==known['target/s10-publication-clock/native-primary']
for name in ['native.c','wire.h']:(build/name).write_bytes((prior/name).read_bytes())
(build/'residency.h').write_bytes(Path(__file__).with_name('residency.h').read_bytes())
s=(prior/'harness.c').read_text()
def replace(a,b):
 global s
 assert s.count(a)==1,(a,s.count(a));s=s.replace(a,b)
replace('static void emit_answer(uint64_t term);','#include "residency.h"\nstatic void emit_answer(uint64_t term);')
replace('static int session(const char *path) {','static int session(const char *path) {\n  nr_emit("entry",0);')
replace('  runtime_init(0, 0, 0);','  runtime_init(0, 0, 0); nr_emit("runtime_init",0);')
replace('  u64 seal = HEAP_NEXT;','  u64 seal = HEAP_NEXT; nr_emit("prepared",0);')
replace('    times.service = now_ns() - service_start;','    times.service = now_ns() - service_start; nr_emit("query_service_end",i);')
replace('    times.drop = now_ns() - start;','    times.drop = now_ns() - start; nr_emit("query_reset",i);')
replace('  uint64_t prepared_drop_ns = now_ns() - start;','  uint64_t prepared_drop_ns = now_ns() - start; nr_emit("prepared_drop",0);')
replace('  uint64_t lifecycle = runtime_init_ns','  nr_emit("consumer_drop",0);\n  uint64_t lifecycle = runtime_init_ns')
(build/'harness.c').write_text(s)
commands=[]
for name,source,flags in [('check',Path(__file__).with_name('check.c'),['-Wextra']),('native',build/'harness.c',['-DSERIALIZATION_CLOCK=0'])]:
 cmd=['clang','-O2','-Wall','-Werror',*flags,str(source),'-o',str(build/name)]
 p=subprocess.run(cmd,capture_output=True,text=True,timeout=60);commands.append(dict(command=cmd,code=p.returncode,stderr=p.stderr));assert p.returncode==0,p.stderr
(out/'build.json').write_text(json.dumps(commands,indent=2)+'\n')
p=subprocess.run([build/'check'],capture_output=True,text=True,timeout=10);(out/'check.log').write_text(p.stderr);assert p.returncode==0,p.stderr
print('Ordinary-allocator residency build and touched-page calibration pass')
