"""Replay full caller semantics and qualify checked kept-read source admission."""
import hashlib
import importlib.util
import json
import resource
import subprocess
import sys
from pathlib import Path
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s10-finite-kept-read'
spec = importlib.util.spec_from_file_location('common_gate', ROOT/'research/chr-hvm/common_source/gate.py')
common = importlib.util.module_from_spec(spec); spec.loader.exec_module(common)
BINARY = ROOT/'target/debug/examples/native_common_source'

def invoke(mode,text):
    p=subprocess.run([str(BINARY),str(mode)],input=text,capture_output=True,text=True,timeout=15,preexec_fn=common.bounds)
    return dict(code=p.returncode,stdout=p.stdout,stderr=p.stderr)

def joined(group):
    text=group[0]['input'].rstrip('\n')
    for s in group[1:]:text+='\nNEXT\n'+'\n'.join(s['input'].splitlines()[:2])
    return text+'\n'

def verify(group,result):
    assert result['code']==0,result
    answers=common.parse(result['stdout']);assert len(answers)==len(group)
    for i,(s,a) in enumerate(zip(group,answers)):
        assert a['index']==i and a['exhausted']==(not s['ongoing'])
        assert sorted(map(common.normalize,a['answers']))==sorted(map(common.normalize,s['expected'])),s['name']

def main():
    prior=ROOT/'docs/experiments/results/s10-native-common-source'
    groups=json.loads((prior/'groups.json').read_text())
    old={(r['group'],r['mode']):r for r in map(json.loads,(prior/'runs.jsonl').read_text().splitlines())}
    with (OUT/'common-replays.jsonl').open('w') as log:
        for g,group in enumerate(groups):
            for mode,label in enumerate(common.MODES):
                r=invoke(mode,joined(group));log.write(json.dumps(dict(group=g,mode=label,result=r))+'\n');log.flush()
                assert r['code']==0 and r['stdout']==old[g,label]['stdout'] and r['stderr']==old[g,label]['stderr'],(g,label,r)
    groups=json.loads((ROOT/'docs/experiments/results/s10-native-substantive/groups.json').read_text())
    admitted=unsupported=0
    with (OUT/'substantive.jsonl').open('w') as log:
        for g,group in enumerate(groups):
            for s in group:
                r=invoke(12,s['input']);log.write(json.dumps(dict(group=g,case=s['name'],result=r))+'\n');log.flush()
                assert r['code']==0,r
                if s['parameters']['late']:
                    assert r['stdout']=='UNSUPPORTED Source("initial kept-read occurrences are missing")\n';unsupported+=1
                else:
                    verify([s],r);admitted+=1
    with (OUT/'reuse.jsonl').open('w') as log:
        for g,group in enumerate(groups):
            if group[0]['parameters']['late']:continue
            r=invoke(12,joined(group));log.write(json.dumps(dict(group=g,result=r))+'\n');log.flush();verify(group,r)
    probe=json.loads((OUT/'late-private-probe.json').read_text())[0]['input']
    probes=[('ordinary-later-private',11,probe),
            ('checked-later-private',12,'go:v10,ready\n10\nready;pick:v0;(=:v0:a|=:v0:b)\n-;go:v0;pick:v0\n'),
            ('checked-consumed-readiness',12,'go:v10,ready\n10\nready;pick:v0;(=:v0:a|=:v0:b)\n-;go:v0,ready;pick:v0\n')]
    records=[]
    for name,mode,text in probes:
        r=invoke(mode,text);scan=invoke(0,text)
        p=subprocess.run([str(ROOT/'target/debug/examples/native_choice_reference')],input=text,capture_output=True,text=True,timeout=3)
        assert p.returncode==0
        header,body=p.stdout.split('\n',1)
        ref='QUERY 0 '+header+'\n'+body
        assert r['code']==scan['code']==0 and r['stdout']==scan['stdout']==ref,(name,r,scan,ref)
        records.append(dict(name=name,mode=mode,input=text,result=r,scan=scan,reference=dict(code=p.returncode,stdout=p.stdout,stderr=p.stderr)))
    (OUT/'caller-probes.json').write_text(json.dumps(records,indent=2)+'\n')
    paths=[Path(__file__),BINARY,ROOT/'target/debug/examples/native_choice_reference',ROOT/'research/chr-direct-conditional/examples/native_common_source.rs',ROOT/'research/chr-direct-conditional/examples/support/finite_kept_read.rs',ROOT/'research/chr-direct-conditional/examples/support/finite_bridge.rs',ROOT/'research/chr-compiled/experiments/finite_phase.rs',ROOT/'research/chr-direct-conditional/tests/finite_kept_read_gate.rs',ROOT/'docs/experiments/registrations/S10-finite-kept-read.md']
    paths += [OUT/n for n in ['common-replays.jsonl','substantive.jsonl','reuse.jsonl','caller-probes.json','late-private-probe.json','initial-common-source.rs']]
    paths += [ROOT/'target/s10-finite-kept-read/initial-common-source']
    (OUT/'validation.json').write_text(json.dumps(dict(common_replays=264,admitted=admitted,unsupported=unsupported,reuse_queries=48,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
    print('264 exact common-source replays;',admitted,'substantive admissions;',unsupported,'explicit exclusions; 48 reused queries; three independent caller probes pass.')

if __name__=='__main__':main()
