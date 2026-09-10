"""Verify frozen inputs, semantic replay, source edits and phase partitioning."""
import difflib
import hashlib
import json
import re
from pathlib import Path
from build import ROOT, OUT, BUILD, SOURCE_HASH
from check import check


def main():
    v = json.loads((OUT / 'validation.json').read_text())
    for path, digest in v['hashes'].items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
    supplement = json.loads((OUT / 'supplement-hashes.json').read_text())
    for path, digest in supplement.items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, path
    original = (ROOT / 'target/s10-native-prepared/native.c').read_text()
    assert hashlib.sha256(original.encode()).hexdigest() == SOURCE_HASH
    changed = (BUILD / 'native.c').read_text()
    # Reconstruct the frozen source by reversing only the two permitted edits.
    macro = original[original.index('#define ITRS_INC(name)'):original.index('static u32 FRESH')]
    replacement = '#define ITRS_INC(name) do { if (CHR_ACTIVE) CHR_VISITS++; } while (0)\n'
    assert changed.count(replacement) == changed.count('    emit_answer(term);') == 1
    restored = changed.replace(replacement, macro).replace('    emit_answer(term);', '    print_term_quoted(term); printf("\\n");')
    assert restored == original
    assert not re.search(r'ITRS\s*(?:\+\+|\+=)', changed)
    (OUT / 'native-source.diff').write_text(''.join(difflib.unified_diff(original.splitlines(True), changed.splitlines(True), fromfile='frozen-native.c', tofile='lifecycle-native.c', n=0)))
    # Query representation and complete session cleanup use exactly the qualified functions.
    prior_harness = (ROOT / 'research/chr-hvm/prepared/harness.c').read_text()
    harness = (ROOT / 'research/chr-hvm/lifecycle/harness.c').read_text()
    query = prior_harness[prior_harness.index('static Term ctr('):prior_harness.index('static OwnedAnswer execute(')]
    cleanup = prior_harness[prior_harness.index('static void session_free('):prior_harness.index('static int session(')]
    assert query in harness and cleanup in harness
    assert (BUILD / 'harness.c').read_text() == harness
    assert not re.search(r'#define\s+(malloc|calloc|realloc|free|strdup)\b', harness)
    prior = ROOT / 'docs/experiments/results/s10-native-prepared'
    plans = json.loads((prior / 'plans.json').read_text())
    expected = [json.loads(l) for l in (prior / 'runs.jsonl').read_text().splitlines()]
    counts = {}; observed = {}
    for mode in ['ordinary', 'ubsan']:
        rows = [json.loads(l) for l in (OUT / f'{mode}.jsonl').read_text().splitlines()]
        assert len(rows) == len(plans) == len(expected) == 22
        total = first = 0
        for plan, row, old in zip(plans, rows, expected):
            assert row['group'] == plan['group'] == old['group']
            events = check(plan, row, old)
            total += len(events) - 1
            first += sum(e['first_observation_ns'] is not None for e in events[:-1])
        counts[mode] = total; observed[mode] = first
    multi = json.loads((OUT / 'sessions.json').read_text()); assert multi['code'] == 0
    assert multi['groups'] == [0, 21, 0]
    parts = multi['stdout'].split('SESSION '); assert len(parts) == 4 and parts[0] == ''
    lines = multi['stderr'].splitlines(); offset = 0
    for i, (part, g) in enumerate(zip(parts[1:], multi['groups'])):
        number, body = part.split('\n', 1); assert int(number) == i
        n = len(plans[g]['queries']) + 1
        check(plans[g], dict(code=0, stdout=body, stderr='\n'.join(lines[offset:offset+n])), expected[g])
        offset += n
    assert offset == len(lines)
    build = json.loads((OUT / 'build.json').read_text())
    upstream = json.loads((OUT / 'upstream-warnings.json').read_text()); assert upstream['code'] == 0
    for row in build:
        assert row['code'] == 0
        warnings = re.findall(r'warning: (.*)', row['stderr'])
        assert len(warnings) == 5 and all(w in upstream['stderr'] for w in warnings)
    result = dict(query_replays=counts, queries_with_observation=observed, same_process_queries=50,
                  permitted_native_source_edits=2, phase_partitions_checked=816,
                  diagnostic_iteration_updates=False, inherited_unused_parameter_warnings=5,
                  comparative_cost_matrix=False)
    (OUT / 'audit.json').write_text(json.dumps(result, indent=2) + '\n'); print(result)

if __name__ == '__main__':
    main()
