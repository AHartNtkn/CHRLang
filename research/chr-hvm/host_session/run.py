"""Combined batch endpoint; native diagnostic intervals are nested, never added twice."""
import importlib.util
import json
import resource
import subprocess
import sys
import tempfile
from pathlib import Path
from time import perf_counter_ns as clock
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'research/chr-hvm/host_frontend'))
from frontend import decode
sys.path.insert(0,str(ROOT/'research/chr-hvm/answer_wire'))
from codec import records
spec=importlib.util.spec_from_file_location('prepared_gate',ROOT/'research/chr-hvm/prepared/gate.py')
prepared=importlib.util.module_from_spec(spec);spec.loader.exec_module(prepared)


def bounds():
    resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30))
    resource.setrlimit(resource.RLIMIT_CPU,(10,10))


def main():
    if len(sys.argv)!=4:
        raise ValueError('expected budgets, temporary root and native primary executable')
    binary=Path(sys.argv[3]).resolve(strict=True)
    budgets=[int(x) for x in sys.argv[1].split(',')]
    if not budgets or any(n<0 or n>1048576 for n in budgets):
        raise ValueError('invalid service budgets')
    temporary_root=sys.argv[2] if len(sys.argv)>2 else None
    phases={}
    begin=clock()
    start=clock();text=sys.stdin.read();phases['input_load_ns']=clock()-start
    start=clock();rules,queries=decode(text);phases['decode_ns']=clock()-start
    if len(budgets) not in [1,len(queries)]:
        raise ValueError('one budget or one per query required')
    start=clock();del text;phases['input_drop_ns']=clock()-start
    start=clock()
    program,predicates,atoms=prepared.compile_source(dict(rules=rules,query=[],outputs=[]))
    phases['emission_ns']=clock()-start
    start=clock();temporary=tempfile.TemporaryDirectory(prefix='chr-native-',dir=temporary_root)
    artifact=Path(temporary.name)/'rules.hvm'
    try:
        artifact.write_text(program)
        phases['artifact_write_ns']=clock()-start
        start=clock()
        lines=[];dictionaries=[]
        for i,query in enumerate(queries):
            line,ps,ats=prepared.encode(query,predicates,atoms,budgets[0 if len(budgets)==1 else i],True)
            lines.append(line);dictionaries.append((ps,ats))
        protocol=(str(len(lines))+'\n'+'\n'.join(lines)+'\n').encode()
        del line,ps,ats,query
        phases['query_encoding_ns']=clock()-start
        start=clock()
        child=subprocess.run([str(binary),str(artifact)],input=protocol,capture_output=True,timeout=15,preexec_fn=bounds)
        phases['native_process_ns']=clock()-start
        if child.returncode:
            raise RuntimeError(f'native child exit {child.returncode}: {child.stderr.decode()}')
        start=clock()
        native=[json.loads(line) for line in child.stderr.splitlines()]
        payloads=records(child.stdout)
        if len(native)!=len(queries)+1 or set(payloads)!=set(range(len(queries))):
            raise ValueError('native endpoint shape')
        consumers=[]
        for i,(ps,ats) in enumerate(dictionaries):
            event=native[i]
            if event['serialization_ns'] is not None or event['compute_traverse_ns'] is not None:
                raise ValueError('primary host requires native serialization clocks disabled')
            if event['query']!=i or event['unsupported']:
                raise ValueError('native unsupported or mismatched query')
            payload=payloads[i]
            exhausted='false' if event['pending'] else 'true'
            header=f'WIRE {i} {exhausted} {len(payload)}\nPRED {",".join(ps)}\nATOM {",".join(ats)}\n'.encode()
            consumers.append(header+payload)
        del header,payload,ps,ats
        phases['consumer_assembly_ns']=clock()-start
        start=clock()
        del rules,queries,program,predicates,atoms,lines,protocol,payloads,child,dictionaries
        temporary.cleanup()
        phases['prepared_artifact_drop_ns']=clock()-start
        start=clock()
        for consumer in consumers:
            sys.stdout.buffer.write(consumer)
        sys.stdout.buffer.flush()
        phases['publication_ns']=clock()-start
        start=clock();del consumer,consumers;phases['consumer_drop_ns']=clock()-start
        elapsed=clock()-begin
        print(json.dumps(dict(endpoint='batch-owned-wire',native_binary=str(binary),phases=phases,named_ns=sum(phases.values()),session_elapsed_ns=elapsed,native=native)),file=sys.stderr)
    finally:
        temporary.cleanup()


if __name__=='__main__':main()
