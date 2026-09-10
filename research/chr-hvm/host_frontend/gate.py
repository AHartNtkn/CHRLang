"""Qualify common-text host preparation against frozen native source sessions."""
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
from time import perf_counter_ns as clock
from frontend import decode
sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s10-host-frontend'
sys.path.insert(0, str(ROOT / 'research/chr-hvm/answer_wire'))
spec = importlib.util.spec_from_file_location('wire_gate', ROOT / 'research/chr-hvm/answer_wire/gate.py')
wire = importlib.util.module_from_spec(spec)
spec.loader.exec_module(wire)


def joined(sources):
    refs = {r['case']: r['result']['input'] for r in json.loads((ROOT / 'docs/experiments/results/s10-native-prepared/symbol-references.json').read_text())}
    texts = [refs.get(s['name'], s['input']) for s in sources]
    return texts[0].rstrip('\n') + ''.join('\nNEXT\n' + '\n'.join(t.splitlines()[:2]) for t in texts[1:]) + '\n'


def main():
    prepared = json.loads((ROOT / 'docs/experiments/results/s10-native-prepared/plans.json').read_text())
    substantive = json.loads((ROOT / 'docs/experiments/results/s10-native-substantive/groups.json').read_text())
    previous = [json.loads(l) for l in (ROOT / 'docs/experiments/results/s10-answer-wire/native-prepared.jsonl').read_text().splitlines()]
    previous = {r['group']: r['result'] for r in previous if r['mode'] == 'ordinary'}
    subs = [json.loads(l) for l in (ROOT / 'docs/experiments/results/s10-answer-wire/native-substantive.jsonl').read_text().splitlines()]
    subs = {r['group']: r['result'] for r in subs if r['mode'] == 'ordinary'}
    specs = []
    for p in prepared:
        specs.append(dict(family='common', group=p['group'], queries=p['queries'], program=p['program'], input=p['input'], previous=previous[p['group']]))
    for i, group in enumerate(substantive):
        specs.append(dict(family='substantive', group=i, queries=[dict(source=s,limit=1048576,keep=True) for s in group], program=f'docs/experiments/results/s10-native-substantive/rules-{i}.hvm', previous=subs[i]))
    total = 0
    with (OUT / 'runs.jsonl').open('w') as log:
        for plan in specs:
            sources = [q['source'] for q in plan['queries']]
            text = joined(sources)
            start = clock()
            rules, queries = decode(text)
            decode_ns = clock() - start
            assert rules == sources[0]['rules']
            assert queries == [dict(query=s['query'], outputs=s['outputs']) for s in sources]
            start = clock()
            program, predicates, atoms = wire.prepared.compile_source(dict(rules=rules,query=[],outputs=[]))
            emission_ns = clock() - start
            assert program == (ROOT / plan['program']).read_text()
            lines = []
            dictionaries = []
            encoding_ns = []
            for query, control in zip(queries, plan['queries']):
                start = clock()
                line, ps, ats = wire.prepared.encode(query, predicates, atoms, control['limit'], control['keep'])
                elapsed = clock() - start
                lines.append(line)
                dictionaries.append((ps, ats))
                encoding_ns.append(elapsed)
                if plan['family'] == 'common':
                    assert ps == control['predicates'] and ats == control['atoms']
            start = clock()
            protocol = str(len(lines)) + '\n' + '\n'.join(lines) + '\n'
            assembly_ns = clock() - start
            if 'input' in plan:
                assert protocol == plan['input']
            # Fresh emitted artifacts are execution inputs, not copies of frozen files.
            artifact = OUT / f"{plan['family']}-{plan['group']}.hvm"
            artifact.write_text(program)
            actual = wire.invoke([wire.BUILD / 'ordinary/run', artifact], protocol, wire.native_bounds)
            old = plan['previous']
            assert actual['code'] == old['code'] == 0
            assert actual['stdout_hex'] == old['stdout_hex']
            events = [json.loads(l) for l in actual['stderr'].splitlines()]
            prior_events = [json.loads(l) for l in old['stderr'].splitlines()]
            assert len(events) == len(prior_events) == len(queries) + 1
            values = wire.records(bytes.fromhex(actual['stdout_hex']))
            assert set(values) == set(range(len(queries)))
            for i, (ps, ats) in enumerate(dictionaries):
                for key in ['query', 'calls', 'pending', 'unsupported', 'dynamic_words']:
                    assert events[i][key] == prior_events[i][key]
                # Decode with freshly reconstructed dictionaries, then compare complete outcomes or exact prior prefix.
                got = wire.decode(values[i], ps, ats)
                expected = wire.decode(wire.records(bytes.fromhex(old['stdout_hex']))[i], ps, ats)
                assert list(map(wire.prepared.normalize, got)) == list(map(wire.prepared.normalize, expected))
                if plan['queries'][i].get('kind', 'complete') == 'complete':
                    assert sorted(map(wire.common.normalize, got)) == sorted(map(wire.common.normalize, sources[i]['expected']))
            snapshot = dict(dictionaries=dictionaries, program_sha256=hashlib.sha256(program.encode()).hexdigest(), protocol=protocol)
            # Serialize validation-only snapshots before release; they must not retain measured owners.
            snapshot_text = json.dumps(snapshot)
            del snapshot, query, ps, ats, line, got, expected
            start = clock()
            del rules, queries, text, program, predicates, atoms, lines, dictionaries, protocol
            release_ns = clock() - start
            record = dict(family=plan['family'],group=plan['group'],queries=len(sources),decode_ns=decode_ns,emission_ns=emission_ns,encoding_ns=encoding_ns,protocol_assembly_ns=assembly_ns,release_ns=release_ns,validation_snapshot=json.loads(snapshot_text),result=actual)
            log.write(json.dumps(record) + '\n');log.flush()
            total += len(sources)
            print(plan['family'],plan['group'],'frontend and native replay pass',flush=True)
    paths = list(Path(__file__).parent.glob('*.py')) + list(OUT.glob('*.hvm')) + [OUT/'runs.jsonl', ROOT/'research/chr-hvm/source_choice/compiler.py',ROOT/'research/chr-hvm/identity/kernel.hvm',ROOT/'target/s10-answer-wire/ordinary/run',ROOT/'docs/experiments/registrations/S10-host-frontend.md']
    paths += [ROOT / p for p in ['research/chr-hvm/prepared/gate.py','research/chr-hvm/identity/build_kernel.py','research/chr-hvm/answer_wire/gate.py','research/chr-hvm/answer_wire/codec.py','docs/experiments/results/s10-native-prepared/plans.json','docs/experiments/results/s10-native-prepared/symbol-references.json','docs/experiments/results/s10-native-substantive/groups.json','docs/experiments/results/s10-answer-wire/native-prepared.jsonl','docs/experiments/results/s10-answer-wire/native-substantive.jsonl']]
    (OUT/'validation.json').write_text(json.dumps(dict(sessions=len(specs),queries=total,comparative_timing=False,hashes={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}),indent=2)+'\n')
    print(total,'host-prepared queries qualify.')

if __name__ == '__main__':
    main()
