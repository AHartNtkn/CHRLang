"""Prospectively selected, bounded primary/diagnostic instrumentation pilot."""
import json,os,random,subprocess
from pathlib import Path
from gate import ROOT,OUT,BUILD,rust,wire,fields

def main():
    assert {0,1}<=os.sched_getaffinity(0)
    groups={name:json.loads((ROOT/'docs/experiments/results'/folder/'groups.json').read_text()) for name,folder in [('common','s10-native-common-source'),('substantive','s10-native-substantive')]}
    choices={}
    prior=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-rust-lifecycle/runs.jsonl').read_text().splitlines()]
    for row in prior:
        data=bytes.fromhex(row['result']['stdout_hex'])
        if data.startswith(b'UNSUPPORTED'):continue
        for source,value in zip(groups[row['family']][row['group']],wire.rust_records(data)):
            if source['ongoing'] or not value['answers']:continue
            entries=choices.setdefault(row['mode'],{})
            if 'many' not in entries or len(value['answers'])>entries['many']['answers']:
                entries['many']=dict(source=source,answers=len(value['answers']))
            if len(value['answers'])==1 and 'one' not in entries:entries['one']=dict(source=source,answers=1)
    assert set(choices)==set(range(13))
    candidates=[]
    for mode,entries in choices.items():
        assert set(entries)==({'many'} if mode==11 else {'many','one'})
        seen=set()
        for role,entry in entries.items():
            if entry['source']['input'] in seen:continue
            seen.add(entry['source']['input']);candidates.append(dict(language='rust',mode=mode,role=role,**entry))
    finite=[s for group in [g for family in groups.values() for g in family] for s in group if not s['ongoing'] and s['expected']]
    many=max(finite,key=lambda s:len(s['expected']));one=next(s for s in finite if len(s['expected'])==1)
    for role,source in [('many',many),('one',one)]:
        if any(c['language']=='native' and c['source']['input']==source['input'] for c in candidates):continue
        program,ps,ats=wire.prepared.compile_source(dict(rules=source['rules'],query=[],outputs=[]))
        path=OUT/f'pilot-{role}.hvm';path.write_text(program)
        line,ps,ats=wire.prepared.encode(source,ps,ats,1048576,True)
        candidates.append(dict(language='native',mode=None,role=role,source=source,answers=len(source['expected']),program=str(path.relative_to(ROOT)),input='1\n'+line+'\n',predicates=ps,atoms=ats))
    (OUT/'selection.json').write_text(json.dumps(candidates,indent=2)+'\n')
    jobs=[(i,kind,cpu,rep) for i in range(len(candidates)) for kind in ['primary','diagnostic'] for cpu in [0,1] for rep in range(5)]
    random.Random(20260911).shuffle(jobs)
    warm=[(i,kind,cpu,-1) for i in range(len(candidates)) for kind in ['primary','diagnostic'] for cpu in [0,1]]
    with (OUT/'pilot.jsonl').open('w') as log:
        for i,kind,cpu,rep in warm+jobs:
            candidate=candidates[i];lang=candidate['language'];s=candidate['source']
            def bounds():
                os.sched_setaffinity(0,{cpu});assert os.sched_getaffinity(0)=={cpu}
                (rust.common.bounds if lang=='rust' else wire.native_bounds)()
            command=([BUILD/f'rust-{kind}',candidate['mode'],200000] if lang=='rust' else [BUILD/f'native-{kind}',ROOT/candidate['program']])
            text=s['input'] if lang=='rust' else candidate['input']
            p=subprocess.run(list(map(str,command)),input=text.encode(),capture_output=True,timeout=15,preexec_fn=bounds)
            result=dict(code=p.returncode,stdout_hex=p.stdout.hex(),stderr=p.stderr.decode())
            assert p.returncode==0,result
            fields(result,kind,lang)
            if lang=='rust':
                rust.check(result,[s]);event=json.loads(result['stderr'].splitlines()[1])
            else:
                records=wire.records(p.stdout);assert set(records)=={0}
                got=wire.decode(records[0],candidate['predicates'],candidate['atoms'])
                assert sorted(map(wire.common.normalize,got))==sorted(map(wire.common.normalize,s['expected']))
                event=json.loads(result['stderr'].splitlines()[0]);assert not event['pending'] and not event['unsupported']
            log.write(json.dumps(dict(candidate=i,kind=kind,cpu=cpu,repetition=rep,service_ns=event['service_ns'],result=result))+'\n');log.flush()
    print(len(candidates),'configurations;',len(jobs),'pilot processes and',len(warm),'excluded warmups validate.')
if __name__=='__main__':main()
