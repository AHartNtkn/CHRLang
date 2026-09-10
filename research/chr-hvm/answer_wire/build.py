"""Adapt frozen harness publication and consumer capacity accounting only."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-answer-wire';BUILD=ROOT/'target/s10-answer-wire'
def replace(s,a,b):
    assert s.count(a)==1,(a,s.count(a));return s.replace(a,b)
def adapt(mode,source):
    source=replace(source,'char *text; size_t len;','char *text; size_t len; size_t capacity; unsigned present;')
    source=source.replace('if(held[i].text)','if(held[i].present)').replace('if (held[i].text)','if (held[i].present)')
    source=source.replace('assert(fwrite(a->text,1,a->len,stdout)==a->len);','if(a->len)assert(fwrite(a->text,1,a->len,stdout)==a->len);').replace('assert(fwrite(a->text, 1, a->len, stdout) == a->len);','if(a->len)assert(fwrite(a->text, 1, a->len, stdout) == a->len);')
    if mode=='ownership':
        source=replace(source,'static void stream_bytes_free(void *p) { free(p); }\n','')
        source=replace(source,'#define HVM_NO_MAIN','static void wire_publish(uint64_t term);\n#define HVM_NO_MAIN')
        source=replace(source,'#include "native.c"','#include "native.c"\n#include "wire.h"\nstatic WireBuffer wire;\nstatic void wire_publish(uint64_t t) { wire_emit(&wire, t); }')
        source=replace(source,' FILE *saved=stdout;char *bytes=NULL;size_t len=0;FILE *stream=open_memstream(&bytes,&len);assert(stream);stdout=stream;',' wire=(WireBuffer){0};')
        source=replace(source,' assert(fclose(stream)==0);stdout=saved;','')
        source=replace(source,' OwnedAnswer answer={.text=malloc(len+1),.len=len,.query=query};memcpy(answer.text,bytes,len+1);stream_bytes_free(bytes);answer.hash=bytes_hash(answer.text,len);',' OwnedAnswer answer={.text=(char*)wire.data,.len=wire.len,.capacity=wire.capacity,.present=1,.query=query};wire=(WireBuffer){0};answer.hash=bytes_hash(answer.text,answer.len);')
        source=source.replace('answer.len+1','answer.capacity').replace('held[i].len+1','held[i].capacity')
    else:
        source=replace(source,'#include "native.c"','#include "native.c"\n#include "wire.h"')
        source=replace(source,'static FILE *answer_stream;','static WireBuffer wire;')
        source=replace(source,'  print_term_quoted(term);\n  printf("\\n");\n  // Publish complete bytes to the stream owner before claiming first observation.\n  assert(fflush(answer_stream) == 0);','  wire_emit(&wire, term);')
        source=replace(source,'    FILE *saved = stdout; char *bytes = NULL; size_t len = 0;\n    answer_stream = open_memstream(&bytes, &len); assert(answer_stream);\n    stdout = answer_stream;','    wire = (WireBuffer){0};')
        source=replace(source,'    assert(fclose(answer_stream) == 0); stdout = saved; answer_stream = NULL;\n    OwnedAnswer answer = {.text = malloc(len + 1), .len = len, .query = i};\n    assert(answer.text); memcpy(answer.text, bytes, len + 1); free(bytes);','    size_t len = wire.len;\n    OwnedAnswer answer = {.text = (char*)wire.data, .len = len, .capacity = wire.capacity, .present = 1, .query = i};\n    wire = (WireBuffer){0};')
        source=replace(source,'(unsigned long long)used,len+1,','(unsigned long long)used,answer.capacity,')
    return source

def main():
    records=[]
    for mode in ['ownership','ordinary']:
        prior=ROOT/'target/s10-native-substantive'/mode
        dest=BUILD/mode;dest.mkdir(parents=True,exist_ok=True)
        source=(prior/'harness.c').read_text();(dest/'harness.c').write_text(adapt(mode,source))
        native=(prior/'native.c').read_text()
        if mode=='ownership':native=replace(native,'    print_term_quoted(term); printf("\\n");','    wire_publish(term);')
        (dest/'native.c').write_text(native);(dest/'wire.h').write_bytes(Path(__file__).with_name('wire.h').read_bytes())
        command=['clang','-O2','-Wall',str(dest/'harness.c'),'-o',str(dest/'run')]
        p=subprocess.run(command,capture_output=True,text=True,timeout=60)
        records.append(dict(mode=mode,command=command,code=p.returncode,stdout=p.stdout,stderr=p.stderr,prior_harness_sha256=hashlib.sha256(source.encode()).hexdigest()))
        (OUT/'build.json').write_text(json.dumps(records,indent=2)+'\n');assert p.returncode==0,p.stderr
    print('Native ownership and ordinary wire encoders build.')
if __name__=='__main__':main()
