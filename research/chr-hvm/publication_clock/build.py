"""Build primary and diagnostic clocks from one runner body per language."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-publication-clock';BUILD=ROOT/'target/s10-publication-clock'

def replace(s,a,b):
    assert s.count(a)==1,(a,s.count(a))
    return s.replace(a,b)

def main():
    commands=[]
    for mode,flags in [('primary',[]),('diagnostic',['--features','serialization-clock'])]:
        command=['cargo','build','-p','chr-direct-conditional','--no-default-features',*flags,'--release','--example','native_lifecycle']
        p=subprocess.run(command,capture_output=True,text=True,timeout=180)
        commands.append(dict(command=command,code=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stderr
        dest=BUILD/f'rust-{mode}';dest.write_bytes((ROOT/'target/release/examples/native_lifecycle').read_bytes());dest.chmod(0o755)
    source=(ROOT/'target/s10-answer-wire/ordinary/harness.c').read_text()
    assert hashlib.sha256(source.encode()).hexdigest()=='1310f1f66d5eb11fabd82267990724d5cf7594500b8eae6f4305bf33e1a6b604'
    source=replace(source,'  uint64_t start = now_ns();\n  wire_emit(&wire, term);\n  uint64_t end = now_ns();\n  active_times->serialization += end - start;\n  if (!active_times->observed) {\n    active_times->observed = 1;\n    active_times->first = end - service_start;\n  }', '''#if SERIALIZATION_CLOCK
  uint64_t start = now_ns();
#endif
  wire_emit(&wire, term);
#if SERIALIZATION_CLOCK
  uint64_t end = now_ns();
  active_times->serialization += end - start;
#endif
  if (!active_times->observed) {
    active_times->observed = 1;
#if SERIALIZATION_CLOCK
    active_times->first = end - service_start;
#else
    active_times->first = now_ns() - service_start;
#endif
  }''')
    source=replace(source,'    assert(times.serialization <= times.service);','''    assert(times.serialization <= times.service);
    char serialization[32], compute[32];
#if SERIALIZATION_CLOCK
    snprintf(serialization,sizeof(serialization),"%llu",(unsigned long long)times.serialization);
    snprintf(compute,sizeof(compute),"%llu",(unsigned long long)(times.service-times.serialization));
#else
    strcpy(serialization,"null"); strcpy(compute,"null");
#endif''')
    source=replace(source,'\\"serialization_ns\\":%llu','\\"serialization_ns\\":%s')
    source=replace(source,'\\"compute_traverse_ns\\":%llu','\\"compute_traverse_ns\\":%s')
    source=replace(source,'(unsigned long long)times.service,(unsigned long long)times.serialization,\n      (unsigned long long)(times.service-times.serialization),','(unsigned long long)times.service,serialization,\n      compute,')
    (BUILD/'harness.c').write_text(source)
    for name in ['native.c','wire.h']:(BUILD/name).write_bytes((ROOT/'target/s10-answer-wire/ordinary'/name).read_bytes())
    for mode,value in [('primary',0),('diagnostic',1)]:
        command=['clang','-O2','-Wall','-Werror',f'-DSERIALIZATION_CLOCK={value}',str(BUILD/'harness.c'),'-o',str(BUILD/f'native-{mode}')]
        p=subprocess.run(command,capture_output=True,text=True,timeout=60)
        commands.append(dict(command=command,code=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stderr
    (OUT/'build.json').write_text(json.dumps(commands,indent=2)+'\n')
    print('Both languages build with primary and diagnostic clocks.')
if __name__=='__main__':main()
