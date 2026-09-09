"""Registered cross-crate optimization feasibility gate; no runtime ranking."""
import json
import os
from pathlib import Path
import re
import resource
import subprocess
import artifact_boundary as gate
from artifact_verify import validate_run

ROOT = gate.ROOT
OUT = ROOT / 'docs/experiments/results/s01-link-optimization'
TARGET = ROOT / 'target/s01-link-optimization'


def run(name, command, limit, env=None, expected=0):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    result = gate.run(name, command, limit, env, expected)
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    result['child_cpu_seconds'] = after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime
    (OUT / (name + '.json')).write_text(json.dumps(result, indent=2) + '\n')
    return result


def inspect(binary):
    assembly = subprocess.check_output(['objdump', '-dC', binary], text=True)
    symbols = {}
    for line in subprocess.check_output(['nm', '-nC', binary], text=True).splitlines():
        match = re.match(r'([0-9a-f]+) \w (.+)', line)
        if match:
            symbols[int(match[1], 16)] = match[2]
    relocations = {}
    for line in subprocess.check_output(['readelf', '-rW', binary], text=True).splitlines():
        match = re.match(r'([0-9a-f]+)\s+\S+\s+R_X86_64_RELATIVE\s+([0-9a-f]+)', line)
        if match:
            relocations[int(match[1], 16)] = int(match[2], 16)
    blocks = [block for block in assembly.split('\n\n')
              if 'Continuation>::tick' in block.split('\n')[0]]
    calls = [line.strip() for block in blocks for line in block.splitlines() if re.search(r'\bcall\b', line)]
    targets = set()
    for line in calls:
        indirect = re.search(r'# ([0-9a-f]+)', line)
        direct = re.search(r'\bcall\s+([0-9a-f]+)', line)
        if indirect:
            targets.add(symbols.get(relocations.get(int(indirect[1], 16)), 'unresolved indirect'))
        elif direct:
            targets.add(symbols.get(int(direct[1], 16), 'unresolved direct'))
        else:
            targets.add('register-indirect')
    return dict(tick_bodies=len(blocks), static_call_sites=len(calls), targets=sorted(targets)), '\n\n'.join(blocks)


def main():
    assert not OUT.exists(), 'Existing receipts must not be overwritten'
    OUT.mkdir(parents=True)
    existed = TARGET.exists()
    TARGET.mkdir(parents=True, exist_ok=True)
    gate.OUT = OUT
    env = os.environ.copy()
    env['CARGO_TARGET_DIR'] = str(TARGET / 'runtime')
    env['RUSTFLAGS'] = '-Cembed-bitcode=yes'
    run('runtime-build', ['cargo', 'build', '--offline', '--release', '-p', 'chr-compiled',
                         '--no-default-features', '--features', 'experiment', '--lib'], 4 << 30, env)
    release = TARGET / 'runtime/release'
    rlib = release / 'libchr_compiled.rlib'
    shared = {str(p.relative_to(ROOT)): gate.digest(p) for p in sorted((release / 'deps').glob('*.rlib'))}
    shared[str(rlib.relative_to(ROOT))] = gate.digest(rlib)
    previous = json.loads((ROOT / 'docs/experiments/results/s01-artifact-boundary/artifacts.json').read_text())
    artifacts = {}
    checked = 0
    for optimization in ['off', 'thin']:
        for family in ['generic', 'chain', 'payload', 'subscription', 'dispatch16']:
            source = ROOT / 'target/s01-artifact-boundary' / (family + '.rs')
            assert gate.digest(source) == previous['artifacts'][family]['source_sha256']
            binary = TARGET / (family + '-' + optimization)
            run('compile-' + binary.name, ['rustc', '--edition', '2024', '-C', 'opt-level=3',
                '-C', 'codegen-units=1', '-C', 'lto=' + optimization, source,
                '--extern', 'chr_compiled=' + str(rlib), '-L', 'dependency=' + str(release / 'deps'), '-o', binary], 4 << 30)
            assert all(gate.digest(ROOT / p) == h for p, h in shared.items())
            artifacts[binary.name] = dict(binary_sha256=gate.digest(binary), binary_bytes=binary.stat().st_size,
                                         source_sha256=gate.digest(source))
            print('compiled', binary.name, flush=True)
        for family in ['chain', 'payload', 'subscription', 'dispatch16']:
            for mode in ['generic', 'planned', 'specialized', 'native', 'native-generic-repair', 'native-planned', 'native-specialized']:
                for size in [0, 5]:
                    binary = TARGET / ((family if mode.startswith('native') else 'generic') + '-' + optimization)
                    result = run(f'run-{optimization}-{family}-{mode}-{size}', [binary, mode, family, size, 3], 1 << 30)
                    validate_run(result, family, mode, size)
                    checked += 3
        mismatch = run('mismatch-' + optimization, [TARGET / ('payload-' + optimization), 'native', 'chain', 0, 1], 1 << 30, expected=2)
        assert not mismatch['stdout'] and 'does not match supplied rules' in mismatch['stderr']
        inspection, assembly = inspect(TARGET / ('chain-' + optimization))
        (OUT / ('inspection-' + optimization + '.json')).write_text(json.dumps(inspection, indent=2) + '\n')
        (OUT / ('chain-' + optimization + '.asm')).write_text(assembly)
        print('validated', optimization, inspection['tick_bodies'], 'bodies', flush=True)
    assert checked == 336
    result = dict(parent_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                  rustc=subprocess.check_output(['rustc', '-Vv'], text=True), target_existed=existed,
                  shared_rlibs=shared, artifacts=artifacts, complete_queries=checked,
                  interpretation='Feasibility and static inspection; costs exploratory, no runtime ranking')
    (OUT / 'summary.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
