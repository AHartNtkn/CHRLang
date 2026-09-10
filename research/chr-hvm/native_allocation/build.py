"""Build a separately instrumented copy; preserve the qualified primary executable."""
from pathlib import Path
import subprocess,json,hashlib
root=Path(__file__).resolve().parents[3];out=root/'docs/experiments/results/s10-native-allocation';build=root/'target/s10-native-allocation';build.mkdir(exist_ok=True)
prior=root/'target/s10-publication-clock'
known=json.loads((root/'docs/experiments/results/s10-publication-clock/validation.json').read_text())['hashes']
assert hashlib.sha256((prior/'native-primary').read_bytes()).hexdigest()==known['target/s10-publication-clock/native-primary']
for name in ['native.c','wire.h']:(build/name).write_bytes((prior/name).read_bytes())
(build/'meter.h').write_bytes(Path(__file__).with_name('meter.h').read_bytes())
s=(prior/'harness.c').read_text()
def replace(a,b):
    global s
    assert s.count(a)==1,(a,s.count(a));s=s.replace(a,b)
replace('static void emit_answer(uint64_t term);','''#include "meter.h"
#define malloc nm_malloc
#define calloc nm_calloc
#define realloc nm_realloc
#define free nm_free
#define strdup nm_strdup
#define mmap nm_mmap
#define munmap nm_munmap
static void emit_answer(uint64_t term);''')
replace('  runtime_init(0, 0, 0);','  runtime_init(0, 0, 0); nm_emit("runtime_init",0);')
replace('  u64 seal = HEAP_NEXT;','  u64 seal = HEAP_NEXT; nm_emit("prepared",0);')
replace('  uint64_t source_drop_ns = now_ns() - start;','  uint64_t source_drop_ns = now_ns() - start; nm_emit("source_drop",0);')
replace('    times.drop = now_ns() - start;','    times.drop = now_ns() - start; nm_emit("query_reset",i);')
replace('  uint64_t prepared_drop_ns = now_ns() - start;','  uint64_t prepared_drop_ns = now_ns() - start; nm_emit("prepared_drop",0);')
replace('  uint64_t lifecycle = runtime_init_ns','  nm_emit("consumer_drop",0); assert(nm_live==0 && nm_mapped==0);\n  uint64_t lifecycle = runtime_init_ns')
assert s.count('\\"allocator\\":\\"ordinary\\"')==2
s=s.replace('\\"allocator\\":\\"ordinary\\"','\\"allocator\\":\\"direct-call-meter\\"')
(build/'harness.c').write_text(s)
commands=[]
for mode,flags in [('check',[]),('check-ubsan',['-fsanitize=undefined','-fno-sanitize-recover=all'])]:
    cmd=['clang','-O2','-Wall','-Wextra','-Werror',*flags,str(Path(__file__).with_name('check.c')),'-o',str(build/mode)]
    p=subprocess.run(cmd,capture_output=True,text=True);commands.append(dict(command=cmd,code=p.returncode,stderr=p.stderr));assert p.returncode==0,p.stderr
    p=subprocess.run([build/mode],capture_output=True,text=True,timeout=10);assert p.returncode==0,p.stderr;(out/f'{mode}.log').write_text(p.stderr)
cmd=['clang','-O2','-Wall','-Werror','-DSERIALIZATION_CLOCK=0',str(build/'harness.c'),'-o',str(build/'native')]
p=subprocess.run(cmd,capture_output=True,text=True);commands.append(dict(command=cmd,code=p.returncode,stderr=p.stderr));(out/'build.json').write_text(json.dumps(commands,indent=2)+'\n');assert p.returncode==0,p.stderr
print('Diagnostic native and both allocator self-checks built and passed')
