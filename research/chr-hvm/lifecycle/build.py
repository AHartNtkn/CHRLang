"""Adapt only diagnostic counters and complete-root publication in the frozen reducer."""
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s10-native-timing-entry'
BUILD = ROOT / 'target/s10-native-timing-entry'
SOURCE_HASH = 'ab68284da6656493c0bde1f021e3f1104c233c5c349a435161893666ce45d13a'

def adapt(source):
    start = source.index('#define ITRS_INC(name)')
    end = source.index('static u32 FRESH', start)
    old = source[start:end]
    assert old.count('ITRS++') == 1 and old.count('CHR_VISITS++') == 1
    source = source[:start] + '#define ITRS_INC(name) do { if (CHR_ACTIVE) CHR_VISITS++; } while (0)\n' + source[end:]
    publication = '    print_term_quoted(term); printf("\\n");'
    assert source.count(publication) == 1
    return source.replace(publication, '    emit_answer(term);')

def main():
    BUILD.mkdir(exist_ok=True)
    source = (ROOT / 'target/s10-native-prepared/native.c').read_bytes()
    assert hashlib.sha256(source).hexdigest() == SOURCE_HASH
    (BUILD / 'native.c').write_text(adapt(source.decode()))
    (BUILD / 'harness.c').write_bytes(Path(__file__).with_name('harness.c').read_bytes())
    rows = []
    for name, flags in [('ordinary', ['-O2']), ('ubsan', ['-O1', '-fsanitize=undefined', '-fno-sanitize-recover=all'])]:
        command = ['clang', *flags, '-Wall', '-Wextra', str(BUILD / 'harness.c'), '-o', str(BUILD / name)]
        p = subprocess.run(command, capture_output=True, text=True, timeout=60)
        rows.append(dict(command=command, code=p.returncode, stdout=p.stdout, stderr=p.stderr))
        (OUT / 'build.json').write_text(json.dumps(rows, indent=2) + '\n')
        assert p.returncode == 0, p.stderr
    print('Ordinary and sanitized lifecycle builds pass.')

if __name__ == '__main__':
    main()
