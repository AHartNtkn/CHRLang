"""Prospective paired confirmation; uses the completed sizing launch contract."""
import hashlib
import itertools
import json
from pathlib import Path
import random
import pulltab_sizing as sizing

ROOT = sizing.ROOT
OUT = ROOT / "docs/experiments/results/s03-pulltab-confirmation"


def main():
    OUT.mkdir(parents=True, exist_ok=False)
    sizing.OUT = OUT
    original = json.loads((ROOT / "docs/experiments/results/s03-pulltab-sizing/freeze.json").read_text())
    for name, digest in original["binaries"].items():
        assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest
    for name, digest in original["sources"].items():
        if name.endswith("S03-pulltab-lifecycle.md"):
            continue  # Confirmation is an explicitly appended prospective registration.
        assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest
    paths = ["docs/experiments/registrations/S03-pulltab-lifecycle.md",
             "research/chr-direct-conditional/experiments/pulltab_confirm.py"]
    freeze = {"sizing_freeze": "../s03-pulltab-sizing/freeze.json", "binaries": original["binaries"],
              "cpu": sizing.CPU, "seed": 7110,
              "sources": {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in paths}}
    (OUT / "freeze.json").write_text(json.dumps(freeze, indent=2) + "\n")
    for name in paths:
        target = OUT / "source" / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((ROOT / name).read_bytes())
    rng = random.Random(7110)
    configs = list(itertools.product(sizing.FAMILIES, [0, 32], [1, 8], [False, True], [False, True]))
    assert len(configs) == 80
    rng.shuffle(configs)
    index = 0
    manifest = []
    for position, config in enumerate(configs):
        for block in range(-1, 7):
            modes = sizing.MODES.copy()
            rng.shuffle(modes)
            for mode in modes:
                kind = "warmup" if block == -1 else "ordinary"
                payload = sizing.run(kind, index, (mode, *config))
                manifest.append({"file": f"{kind}-{index:04}.json", "block": block,
                                 "mode": mode, "config": config})
                index += 1
        print(f"confirmed configurations {position + 1}/80", flush=True)
    (OUT / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    configs = list(itertools.product(sizing.MODES, sizing.FAMILIES, [0], [1], [False, True], [False, True]))
    previous = []
    for repetition in range(2):
        for i, config in enumerate(configs):
            records = sizing.allocation_records(sizing.run("allocation", repetition * len(configs) + i, config))
            if repetition == 0:
                previous.append(records)
            else:
                assert records == previous[i], config
        print(f"cold allocation repetition {repetition + 1}/2", flush=True)
    print("Completed 560 warmups, 3920 confirmation timings and 280 cold allocation runs.", flush=True)


if __name__ == "__main__":
    main()
