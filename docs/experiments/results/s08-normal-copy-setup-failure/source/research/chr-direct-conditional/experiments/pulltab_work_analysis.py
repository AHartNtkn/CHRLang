"""Verify frozen diagnostic sources/repetitions and summarize paired work counts."""
import csv
import hashlib
import json
from pathlib import Path
import sys

summaries = {}
for argument in sys.argv[1:]:
    directory = Path(argument)
    freeze = json.loads((directory / "freeze.json").read_text())
    for source, digest in freeze["sources"].items():
        assert hashlib.sha256((directory / "source" / source).read_bytes()).hexdigest() == digest
    assert (directory / "run1.csv").read_bytes() == (directory / "run2.csv").read_bytes()
    with (directory / "run1.csv").open() as file:
        rows = list(csv.DictReader(file))
    assert len(rows) == 144
    summary = {}
    for family in ["direct", "opaque", "nested"]:
        pairs = [(a, b) for a, b in zip(rows[::2], rows[1::2]) if a["family"] == family]
        assert len(pairs) == 24
        for a, b in pairs:
            assert a["pull"] == "false" and b["pull"] == "true"
            for key in ["family", "consumers", "independent", "depth", "reverse", "answers"]:
                assert a[key] == b[key]
        summary[family] = {}
        metrics = ["expansions", "force_entries", "nodes", "calls", "lifts", "lift_walk_entries"]
        if "dependency_entries" in rows[0]:
            metrics.append("dependency_entries")
        for metric in metrics:
            differences = [int(b[metric]) - int(a[metric]) for a, b in pairs]
            summary[family][metric] = {
                "min_delta": min(differences), "max_delta": max(differences),
                "lower": sum(d < 0 for d in differences),
                "equal": sum(d == 0 for d in differences),
                "higher": sum(d > 0 for d in differences),
            }
    summaries[directory.name] = summary
print(json.dumps(summaries, indent=2))
