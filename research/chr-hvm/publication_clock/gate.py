"""Exact semantic/cancellation qualification before instrumentation timing."""
import hashlib,importlib.util,json,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];OUT=ROOT/'docs/experiments/results/s10-publication-clock';BUILD=ROOT/'target/s10-publication-clock'
spec=importlib.util.spec_from_file_location('rust_gate',ROOT/'research/chr-hvm/rust_lifecycle/gate.py');rust=importlib.util.module_from_spec(spec);spec.loader.exec_module(rust)
wire=rust.wire

def fields(result,kind,language):
    lines=[json.loads(l) for l in result['stderr'].splitlines()]
    events=lines[1:-1] if language=='rust' else lines[:-1]
    for e in events:
        key='compute_observe_ns' if language=='rust' else 'compute_traverse_ns'
        assert (e['serialization_ns'] is None)==(kind=='primary')
        if kind=='primary':assert e[key] is None
        else:assert e['service_ns']==e['serialization_ns']+e[key]

def main():
    groups={name:json.loads((ROOT/'docs/experiments/results'/folder/'groups.json').read_text()) for name,folder in [('common','s10-native-common-source'),('substantive','s10-native-substantive')]}
    prior=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-rust-lifecycle/runs.jsonl').read_text().splitlines()]
    reused=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-rust-lifecycle/retained-reuse.jsonl').read_text().splitlines()]
    selection=json.loads((ROOT/'docs/experiments/results/s10-rust-lifecycle/reuse-selection.json').read_text())
    with (OUT/'rust.jsonl').open('w') as log:
        for kind in ['primary','diagnostic']:
            rust.BINARY=BUILD/f'rust-{kind}'
            for old in prior:
                group=groups[old['family']][old['group']]
                result=rust.invoke(old['mode'],rust.joined(group))
                log.write(json.dumps(dict(kind=kind,family=old['family'],group=old['group'],mode=old['mode'],result=result))+'\n');log.flush()
                rust.check(result,group);assert result['stdout_hex']==old['result']['stdout_hex']
                fields(result,kind,'rust')
                old_events=[json.loads(l) for l in old['result']['stderr'].splitlines()][1:-1]
                events=[json.loads(l) for l in result['stderr'].splitlines()][1:-1]
                for a,b in zip(events,old_events):
                    for key in ['calls','exhausted','wire_bytes','wire_capacity']:assert a[key]==b[key]
            print(kind,'338 Rust corpus processes pass',flush=True)
    with (OUT/'reuse.jsonl').open('w') as log:
        for kind in ['primary','diagnostic']:
            rust.BINARY=BUILD/f'rust-{kind}'
            for old in reused:
                s=selection[str(old['mode'])]['source'];group=[s]*5
                result=rust.invoke(old['mode'],rust.joined(group),','.join(map(str,old['budgets'])))
                rust.check(result,group,False);fields(result,kind,'rust')
                assert result['stdout_hex']==old['result']['stdout_hex']
                events=[json.loads(l) for l in result['stderr'].splitlines()][1:-1]
                for e,budget in zip(events,old['budgets']):assert e['calls']<=budget
                assert events[0]['calls']==0 and not events[0]['exhausted'] and not events[3]['exhausted'] and events[-1]['exhausted']
                log.write(json.dumps(dict(kind=kind,mode=old['mode'],result=result))+'\n');log.flush()
    prior=[json.loads(l) for l in (ROOT/'docs/experiments/results/s10-host-frontend/runs.jsonl').read_text().splitlines()]
    with (OUT/'native.jsonl').open('w') as log:
        for kind in ['primary','diagnostic']:
            for old in prior:
                artifact=ROOT/'docs/experiments/results/s10-host-frontend'/f"{old['family']}-{old['group']}.hvm"
                result=wire.invoke([BUILD/f'native-{kind}',artifact],old['validation_snapshot']['protocol'],wire.native_bounds)
                log.write(json.dumps(dict(kind=kind,family=old['family'],group=old['group'],result=result))+'\n');log.flush()
                assert result['code']==0 and result['stdout_hex']==old['result']['stdout_hex']
                fields(result,kind,'native')
                events=[json.loads(l) for l in result['stderr'].splitlines()][:-1]
                old_events=[json.loads(l) for l in old['result']['stderr'].splitlines()][:-1]
                payloads=wire.records(bytes.fromhex(result['stdout_hex']))
                assert len(events)==len(old_events)==len(payloads)
                for i,(a,b) in enumerate(zip(events,old_events)):
                    for key in ['query','calls','pending','unsupported','dynamic_words']:assert a[key]==b[key]
                    assert (a['first_observation_ns'] is not None)==bool(payloads[i])
                    if a['first_observation_ns'] is not None:assert a['first_observation_ns']<=a['service_ns']
            print(kind,'479 native query replays pass',flush=True)
    paths=list(Path(__file__).parent.glob('*.py'))+list(BUILD.glob('*'))+[OUT/n for n in ['rust.jsonl','reuse.jsonl','native.jsonl','build.json','prior-runner.rs','prior-rust-gate.py']]
    paths += [ROOT/p for p in ['research/chr-direct-conditional/examples/native_lifecycle.rs','research/chr-direct-conditional/Cargo.toml','research/chr-hvm/rust_lifecycle/gate.py','docs/experiments/registrations/S10-publication-clock.md']]
    (OUT/'validation.json').write_text(json.dumps(dict(rust_processes=676,rust_checks=4572,rust_unsupported=1174,rust_reuse_queries=130,native_queries=958,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths if p.is_file()}),indent=2)+'\n')
if __name__=='__main__':main()
