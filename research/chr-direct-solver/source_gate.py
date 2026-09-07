"""Registered R04 source correspondence; no performance measurements."""
import argparse
import hashlib
import json
from itertools import combinations, permutations, product
from pathlib import Path
import subprocess

from oracle import evaluate

ROOT = Path(__file__).resolve().parents[2]


def cases():
    graphs = [('path', [(0, 1), (1, 2), (2, 3)]),
              ('self-loop', [(0, 0), (1, 2), (2, 3)]),
              ('dense-sat', [e for e in combinations(range(4), 2) if e != (0, 1)]),
              ('dense-unsat', list(combinations(range(4), 2)))]
    for name, edges in graphs:
        base = dict(forbid=tuple((f'x{i}', f'x{j}', a, a)
                                for i, j in edges for a in range(3)),
                    outputs=tuple((f'o{i}', f'x{i}') for i in range(4)))
        yield name + '-choice', dict(base, choose=tuple(f'x{i}' for i in range(4)))
        for values in product(range(3), repeat=4):
            yield name + '-given-' + ''.join(map(str, values)), dict(
                base, given=tuple((f'x{i}', a) for i, a in enumerate(values)))
    yield 'repeated-chooser', dict(choose=('x', 'x'), outputs=(('out', 'x'),))
    yield 'repeated-residual', dict(given=(('x', 0), ('y', 1)),
                                  forbid=(('x', 'y', 0, 0),) * 2)
    yield 'off-diagonal-self', dict(choose=('x',), forbid=(('x', 'x', 0, 1),))
    yield 'output-aliases', dict(choose=('x',), outputs=(('left', 'x'), ('right', 'x')))
    yield 'hidden-choice', dict(choose=('x',))
    yield 'conflicting-givens', dict(given=(('x', 0), ('x', 1)))
    yield 'empty', {}
    yield 'ground-chooser', dict(choose=(1, 1))
    yield 'ground-forbidden', dict(forbid=((0, 1, 0, 1),))
    yield 'ground-allowed', dict(forbid=((0, 1, 0, 0),))


def encode(query, variables):
    ids = {name: i for i, name in enumerate(variables)}

    def term(value):
        return f'v{ids[value]}' if isinstance(value, str) else f'a{value}'

    lines = ['choose ' + term(t) for t in query.get('choose', ())]
    for name in ('given', 'forbid'):
        lines.extend(name + ' ' + ' '.join(map(term, args)) for args in query.get(name, ()))
    lines.extend('output ' + name + ' ' + term(t) for name, t in query.get('outputs', ()))
    return '\n'.join(lines + ['end']) + '\n'


def observed(result):
    return tuple(sorted(set((tuple(tuple(pair) for pair in a['outputs']),
                             tuple(sorted(tuple(row) for row in a['residual'])))
                            for a in result['answers'])))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, default=ROOT / 'target/debug/chr-direct-solver')
    parser.add_argument('--receipt', type=Path, required=True)
    args = parser.parse_args()
    registry = [(name, q, evaluate(q)) for name, q in cases()]
    payload = ''.join(encode(q, oracle['variables']) for _, q, oracle in registry)
    receipt = dict(status='incomplete', queries=len(registry), orders=[],
                   source_hashes={}, failures=[])
    source_paths = [Path(__file__), Path(__file__).with_name('oracle.py'),
                    Path(__file__).with_name('src') / 'main.rs']
    for crate in ('chr-reference', 'chr-syntax'):
        source_paths.extend(sorted((ROOT / 'crates' / crate / 'src').glob('*.rs')))
    for path in source_paths:
        receipt['source_hashes'][str(path.relative_to(ROOT))] = hashlib.sha256(path.read_bytes()).hexdigest()
    receipt['revision'] = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    receipt['binary_sha256'] = hashlib.sha256(args.binary.read_bytes()).hexdigest()
    receipt['source_input_sha256'] = hashlib.sha256(payload.encode()).hexdigest()
    try:
        for order, permutation in enumerate(permutations(('choose', 'given', 'forbid'))):
            process = subprocess.run([str(args.binary.resolve()), str(order)], input=payload,
                                     text=True, capture_output=True, timeout=60, check=True)
            results = [json.loads(line) for line in process.stdout.splitlines()]
            if len(results) != len(registry):
                raise AssertionError('Source bridge result count differs from query count')
            for (name, query, oracle), result in zip(registry, results):
                if (not result['exhausted'] or result['raw'] != oracle['raw']
                        or observed(result) != oracle['answers']
                        or (not query.get('choose') and result['splits'] != 0)):
                    receipt['failures'].append(dict(case=name, order=order, expected=oracle, actual=result))
            receipt['orders'].append(dict(order=permutation, checked=len(results)))
        if receipt['failures']:
            raise AssertionError('Source correspondence mismatch; see receipt')
        receipt['status'] = 'passed'
    except Exception as error:
        receipt['error'] = repr(error)
        raise
    finally:
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2) + '\n')
    print(f"{len(registry)} queries × {len(receipt['orders'])} rule orders: full observations, raw counts and exhaustion agree")


if __name__ == '__main__':
    main()
