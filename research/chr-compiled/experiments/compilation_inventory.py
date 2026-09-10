"""Recheck existing compilation evidence without changing its archived receipts."""
from pathlib import Path
import hashlib
import importlib.util
import json

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'docs/experiments/results/s06-compilation-inventory'


def main():
    path = ROOT / 'research/chr-compiled/experiments/recursive_lifecycle_pilot.py'
    spec = importlib.util.spec_from_file_location('recursive_audit', path)
    audit = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(audit)
    OUT.mkdir(exist_ok=True)
    audit.write_json = lambda path, value: (OUT / ('r05-' + path.name)).write_text(
        json.dumps(value, indent=2) + '\n')
    archive = ROOT / 'docs/experiments/results/r05-recursive-lifecycle'
    result = audit.audit(archive, archived=True)
    assert result['recorded'] == 324 and result['complete_primary_cells'] == 54
    assert not result['failed_rows'] and not result['failed_builds'] and not result['missing_builds']
    meta = json.loads((archive / 'metadata.json').read_text())
    files = ['src/recursive.rs', 'src/recursive/native.rs', 'src/recursive/session.rs',
             'examples/recursive_session_packages.rs', 'experiments/recursive_lifecycle_pilot.py']
    comparison = []
    for file in files:
        name = 'research/chr-compiled/' + file
        current = hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
        old = meta['frozen'].get(name, meta['frozen'].get(str(ROOT / name)))
        assert old is not None
        comparison.append(dict(path=name, current=current, archived=old, same=current == old))
    (OUT / 'source-comparison.json').write_text(json.dumps(comparison, indent=2) + '\n')
    print(json.dumps(result))
    print('Core compiler/plan/protocol/emitter/auditor unchanged:', all(c['same'] for c in comparison))


if __name__ == '__main__':
    main()
