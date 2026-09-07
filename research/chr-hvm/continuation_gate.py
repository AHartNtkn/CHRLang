"""Pinned-backend continuation boundary checks; not a general CHR compiler."""
import argparse
import hashlib
import json
import re
from pathlib import Path
import subprocess
import tempfile

PIN = '6defdfc7dae2a3cca5dd6e74ed0612385b5646a8'
PRELUDE = '''
@step = λ{
  #Loop: #Loop;
  #Emit: λx. #Bind{x};
  #Bind: λx. #Done{x};
  #Done: λx. #Done{x};
  #Choice: λa. λb. #Choice{a,b}
}
@advance = λ{
  #ZER: λs. #More{s};
  #SUC: λq. λs. @advance(q,@step(s))
}
@spin = λs. @spin(s)
'''


def cases():
    for budget in (0, 1, 2, 4, 16):
        yield f'loop-{budget}', f'@advance({budget}n,#Loop)', '#More{#Loop}', False
        state = '#Emit{#A}' if budget == 0 else '#Bind{#A}' if budget == 1 else '#Done{#A}'
        yield f'emit-{budget}', f'@advance({budget}n,#Emit{{#A}})', '#More{' + state + '}', False
        yield f'choice-{budget}', f'@advance({budget}n,#Choice{{#Loop,#Emit{{#A}}}})', '#More{#Choice{#Loop,#Emit{#A}}}', False
    yield 'sibling-round', '#Round{@advance(4n,#Loop),@advance(4n,#Emit{#A})}', '#Round{#More{#Loop},#More{#Done{#A}}}', False
    yield 'unsafe-field', '#More{@spin(#Loop)}', None, True
    yield 'unsafe-thunk', '#More{λu. @spin(#Loop)}', None, True


def parse_data(text, annotation=False):
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    position = 0

    def node():
        nonlocal position
        match = re.match(r'\s*#([A-Za-z][A-Za-z0-9_]*)', text[position:])
        if not match:
            raise ValueError('expected data constructor')
        tag = match.group(1)
        position += match.end()
        while position < len(text) and text[position].isspace():
            position += 1
        children = []
        if position < len(text) and text[position] == '{':
            position += 1
            if position < len(text) and text[position] != '}':
                children.append(node())
                while position < len(text) and text[position] == ',':
                    position += 1
                    children.append(node())
            if position >= len(text) or text[position] != '}':
                raise ValueError('missing constructor close')
            position += 1
        return tag, tuple(children)

    result = node()
    remainder = text[position:].strip()
    if remainder and not (annotation and re.fullmatch(r'#[0-9]+', remainder)):
        raise ValueError('unexpected trailing result')
    return result


def run(binary, output):
    rows = []
    with tempfile.TemporaryDirectory(prefix='chr-hvm-boundary-') as scratch:
        for name, expression, expected, adverse in cases():
            program = PRELUDE + '\n@main = ' + expression + '\n'
            path = Path(scratch) / (name + '.hvm')
            path.write_text(program)
            for collapse in (False, True):
                row = dict(case=name, collapse=collapse, source=program, expected=expected,
                           source_sha256=hashlib.sha256(program.encode()).hexdigest())
                try:
                    result = subprocess.run([str(binary), str(path), '-s'] + (['-C1'] if collapse else []),
                                            text=True, capture_output=True, timeout=0.5 if adverse else 5)
                    row.update(exit_code=result.returncode, stdout=result.stdout, stderr=result.stderr)
                    # Statistics are retained; only the first nonempty result line
                    # is compared with independently derived data state.
                    actual = next((line for line in result.stdout.splitlines() if line.strip()), '')
                    try:
                        parsed = parse_data(actual, annotation=collapse)
                        row['parsed'] = parsed
                        row['passed'] = not adverse and result.returncode == 0 and parsed == parse_data(expected)
                    except (ValueError, TypeError):
                        row['passed'] = False
                except subprocess.TimeoutExpired:
                    row.update(timeout_seconds=0.5 if adverse else 5, passed=adverse)
                rows.append(row)
    output.write_text(''.join(json.dumps(r, sort_keys=True) + '\n' for r in rows))
    return all(r['passed'] for r in rows)


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('binary', type=Path)
    p.add_argument('source_checkout', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    revision = subprocess.check_output(['git', '-C', str(a.source_checkout), 'rev-parse', 'HEAD'], text=True).strip()
    if revision != PIN:
        raise SystemExit('backend revision does not match registered pin')
    if not run(a.binary.resolve(), a.output):
        raise SystemExit('continuation gate mismatch; inspect retained rows')
