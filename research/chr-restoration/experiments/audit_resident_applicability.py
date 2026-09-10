"""Audit archived restoration receipts against their exact committed source snapshot."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / 'docs/experiments/results/s04-resident-cost'
REV = '648b2e20a'

def sha(data):
    return hashlib.sha256(data).hexdigest()

freeze = json.loads((BASE / 'freeze.json').read_text())
with tempfile.TemporaryDirectory(prefix='chr-resident-audit-') as temporary:
    snapshot = Path(temporary)
    for name, expected in freeze['sources'].items():
        content = subprocess.check_output(['git', 'show', f'{REV}:{name}'], cwd=ROOT)
        assert sha(content) == expected, name
        destination = snapshot / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(content)
    target = snapshot / BASE.relative_to(ROOT)
    shutil.copytree(BASE, target)
    auditor = snapshot / 'research/chr-restoration/experiments/summarize_resident_cost.py'
    result = subprocess.run(['python', str(auditor)], capture_output=True, text=True, timeout=60)
    assert result.returncode == 0, result.stderr
    for name in ['audit.json', 'summary.json', 'phases.csv']:
        assert (target / name).read_bytes() == (BASE / name).read_bytes(), name
    # Current basic restoration behavior differs only by the new module declaration.
    old = (snapshot / 'research/chr-restoration/src/lib.rs').read_text()
    current = (ROOT / 'research/chr-restoration/src/lib.rs').read_text()
    assert current == old + '\npub mod reunion;\n'
    assert (ROOT / 'research/chr-restoration/examples/lifecycle.rs').read_bytes() == (snapshot / 'research/chr-restoration/examples/lifecycle.rs').read_bytes()
    print(json.dumps({
        'revision': REV,
        'frozen_sources_verified': len(freeze['sources']),
        'binaries_verified_by_original_auditor': len(freeze['binaries']),
        'original_audit': json.loads((target / 'audit.json').read_text()),
        'reconstructed_artifacts_equal': ['audit.json', 'summary.json', 'phases.csv'],
        'current_restoration_change': 'module declaration only',
        'current_lifecycle_runner_equal': True,
        'scope': 'Archived measurements and restoration source applicability; not a new timing result or unchanged host/dependency claim.'
    }, indent=2))
