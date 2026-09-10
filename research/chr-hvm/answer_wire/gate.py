"""Validate the shared owned-byte endpoint against independent source observations."""
import hashlib,importlib.util,json,resource,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
from build import ROOT,OUT,BUILD
from codec import decode,records

def load(name,path):
    s=importlib.util.spec_from_file_location(name,ROOT/path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
prepared=load('prepared_gate','research/chr-hvm/prepared/gate.py')
common=load('common_gate','research/chr-hvm/common_source/gate.py')

def native_bounds():
    resource.setrlimit(resource.RLIMIT_AS,(96<<30,96<<30));resource.setrlimit(resource.RLIMIT_CPU,(10,10))
def invoke(command,text,bounds):
    p=subprocess.run(list(map(str,command)),input=text.encode(),capture_output=True,timeout=15,preexec_fn=bounds)
    return dict(code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode())
def rust_records(data):
    at=0;out=[]
    def line():
        nonlocal at
        end=data.index(b'\n',at);s=data[at:end].decode();at=end+1;return s
    while at<len(data):
        tag,index,exhausted,size=line().split();assert tag=='WIRE'
        preds=line();atoms=line();assert preds.startswith('PRED ') and atoms.startswith('ATOM ')
        ps=preds[5:].split(',') if preds[5:] else [];ats=atoms[5:].split(',') if atoms[5:] else []
        size=int(size);payload=data[at:at+size];assert len(payload)==size;at+=size
        out.append(dict(index=int(index),exhausted=exhausted=='true',predicates=ps,atoms=ats,answers=decode(payload,ps,ats),bytes=size))
    return out

def native_check(plan,row,old):
    assert row['code']==0,row['stderr'][-1000:]
    values=records(bytes.fromhex(row['stdout_hex']));prior=prepared.output_records(old['stdout'])
    events=[json.loads(l) for l in row['stderr'].splitlines()];previous=[json.loads(l) for l in old['stderr'].splitlines()]
    assert set(values)==set(prior) and len(events)==len(previous)
    for i,q in enumerate(plan['queries']):
        actual=decode(values[i],q['predicates'],q['atoms'])
        expected=[prepared.parse(l,q['predicates'],q['atoms']) for l in prior[i].splitlines()]
        assert list(map(prepared.normalize,actual))==list(map(prepared.normalize,expected)),q['source']['name']
        for k in ['query','calls','pending','unsupported','dynamic_words']:assert events[i][k]==previous[i][k],k
        assert events[i]['retained_bytes']>=len(values[i])
        if 'prepared_unchanged' in events[i]:assert events[i]['prepared_unchanged'] and events[i]['query_restored']
    if 'tracked_live' in events[-1]:assert events[-1]['tracked_live']==0 and events[-1]['consumer_survives_prepared_drop']

def main():
    plans=json.loads((ROOT/'docs/experiments/results/s10-native-prepared/plans.json').read_text())
    old=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-native-prepared/runs.jsonl').read_text().splitlines()]
    with (OUT/'native-prepared.jsonl').open('w') as log:
        for mode in ['ownership','ordinary']:
            for plan,prior in zip(plans,old):
                row=invoke([BUILD/mode/'run',ROOT/plan['program']],plan['input'],native_bounds)
                log.write(json.dumps(dict(mode=mode,group=plan['group'],result=row))+'\n');log.flush();native_check(plan,row,prior)
            print(mode,'383 prepared schedules pass',flush=True)
    root=ROOT/'docs/experiments/results/s10-native-substantive';groups=json.loads((root/'groups.json').read_text())
    old={(r['group'],r['mode']):r['result'] for r in map(json.loads,(root/'reuse.jsonl').read_text().splitlines())}
    with (OUT/'native-substantive.jsonl').open('w') as log:
        for g,group in enumerate(groups):
            _,preds,atoms=prepared.compile_source(dict(rules=group[0]['rules'],query=[],outputs=[]))
            queries=[];lines=[]
            for s in group:
                line,ps,ats=prepared.encode(s,preds,atoms,1048576,True);lines.append(line);queries.append(dict(source=s,predicates=ps,atoms=ats))
            plan=dict(queries=queries)
            for mode in ['ownership','ordinary']:
                row=invoke([BUILD/mode/'run',root/f'rules-{g}.hvm'],str(len(lines))+'\n'+'\n'.join(lines)+'\n',native_bounds)
                log.write(json.dumps(dict(group=g,mode=mode,result=row))+'\n');log.flush();native_check(plan,row,old[g,mode])
    counts={}
    with (OUT/'rust.jsonl').open('w') as log:
        for family,groups in [('common',json.loads((ROOT/'docs/experiments/results/s10-native-common-source/groups.json').read_text())),('substantive',groups)]:
            for g,group in enumerate(groups):
                text=group[0]['input'].rstrip('\n')
                for s in group[1:]:text+='\nNEXT\n'+'\n'.join(s['input'].splitlines()[:2])
                for mode in range(13):
                    row=invoke([common.BINARY,mode,'--wire'],text+'\n',common.bounds)
                    log.write(json.dumps(dict(family=family,group=g,mode=mode,result=row))+'\n');log.flush();assert row['code']==0,row
                    data=bytes.fromhex(row['stdout_hex'])
                    if data.startswith(b'UNSUPPORTED '):
                        assert mode in [10,11,12];counts['unsupported']=counts.get('unsupported',0)+len(group);continue
                    values=rust_records(data);assert len(values)==len(group),(family,g,mode)
                    for i,(s,a) in enumerate(zip(group,values)):
                        _,ps,ats=prepared.compile_source(dict(rules=s['rules'],query=[],outputs=[]))
                        _,ps,ats=prepared.encode(s,ps,ats,65536,True)
                        assert a['predicates']==ps and a['atoms']==ats
                        assert a['index']==i and a['exhausted']==(not s['ongoing'])
                        assert sorted(map(common.normalize,a['answers']))==sorted(map(common.normalize,s['expected'])),(family,g,mode,s['name'])
                    counts['passed']=counts.get('passed',0)+len(group)
            print(family,'Rust wire observations pass',flush=True)
    paths=list(Path(__file__).parent.glob('*.py'))+[Path(__file__).with_name('wire.h')]
    paths += [ROOT/p for p in ['research/chr-direct-conditional/examples/native_common_source.rs','research/chr-direct-conditional/examples/support/answer_wire.rs','research/chr-direct-conditional/tests/answer_wire_gate.rs','target/debug/examples/native_common_source','docs/experiments/registrations/S10-answer-wire.md']]
    paths += [p for p in BUILD.rglob('*') if p.is_file()]
    paths += [OUT/n for n in ['native-prepared.jsonl','native-substantive.jsonl','rust.jsonl','build.json','initial-common-source.rs']]
    (OUT/'validation.json').write_text(json.dumps(dict(native_prepared_queries=766,native_substantive_queries=192,rust=counts,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n');print(counts)
if __name__=='__main__':main()
