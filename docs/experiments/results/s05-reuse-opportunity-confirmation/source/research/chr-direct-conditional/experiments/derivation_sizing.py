"""Registered, nonconfirmatory S03 lifecycle sizing with separate diagnostics."""
import hashlib
import itertools
import json
import os
from pathlib import Path
import platform
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "docs/experiments/results/s03-derivation-sizing"
BIN = Path("/tmp/chr-derivation-lifecycle-b5d235e3")
MODES = ["dependencies", "templates", "scan", "indexed", "sealed", "lowered"]
FAMILIES = ["single", "repeat", "distinct", "choice", "grow"]
CPU = min(os.sched_getaffinity(0))


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    os.sched_setaffinity(0, {CPU})


def run(kind, index, config, cancel=None):
    mode, family, depth, queries, consuming, reverse = config
    binary = BIN / ("meter" if kind in ("allocation", "cancel-meter") else "ordinary")
    command = [str(binary), mode, family, str(depth), str(queries), str(int(consuming)), str(int(reverse))]
    if cancel is not None:
        command.append(str(cancel))
    dest = OUT / f"{kind}-{index:04}.json"
    assert not dest.exists(), dest
    start = time.monotonic()
    try:
        p = subprocess.run(command, capture_output=True, text=True, timeout=60, preexec_fn=limits)
        result = {"command": command, "wall_seconds": time.monotonic() - start,
                  "returncode": p.returncode, "stdout": p.stdout, "stderr": p.stderr}
    except subprocess.TimeoutExpired as error:
        result = {"command": command, "wall_seconds": time.monotonic() - start,
                  "timeout": True, "stdout": str(error.stdout), "stderr": str(error.stderr)}
        dest.write_text(json.dumps(result, indent=2) + "\n")
        raise
    dest.write_text(json.dumps(result, indent=2) + "\n")
    assert p.returncode == 0, dest
    payload = [json.loads(line) for line in p.stdout.splitlines() if line.startswith('{')][-1]
    assert payload["event"] == "result" and not payload["counters"]
    assert payload["meter"] == (binary.name == "meter")
    assert len(payload["samples"]) == queries
    for i, sample in enumerate(payload["samples"]):
        assert sample["complete"] == (cancel is None or i % 2 == 1), dest
    return payload


def allocation_records(payload):
    def extract(value):
        if isinstance(value, dict):
            if "memory" in value:
                return value["memory"]
            return {k: extract(v) for k, v in value.items() if k not in ["first_answer_ns"]}
        if isinstance(value, list):
            return [extract(v) for v in value]
        return value
    return extract(payload)


def main():
    OUT.mkdir(parents=True, exist_ok=False)
    paths = ["research/chr-direct-conditional/experiments/" + f for f in
             ["derivation_cost.rs", "derivation_source.rs", "derivation_sizing.py"]]
    paths += ["research/chr-direct-choice/src/demand.rs", "research/chr-direct-choice/src/demand/templates.rs", "research/chr-direct-choice/Cargo.toml",
              "research/chr-direct-conditional/Cargo.toml", "research/chr-compiled/src/pure_prefix.rs",
              "research/chr-compiled/experiments/meter.rs", "research/chr-direct-conditional/tests/runtime_support/mod.rs",
              "docs/experiments/registrations/S03-derivation-lifecycle.md", "Cargo.lock"]
    freeze = {"cpu": CPU, "available_cpus": sorted(os.sched_getaffinity(0)), "host": platform.platform(),
              "base_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
              "sources": {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in paths},
              "binaries": {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in [BIN / "ordinary", BIN / "meter"]}}
    (OUT / "freeze.json").write_text(json.dumps(freeze, indent=2) + "\n")
    for p in paths:
        target = OUT / "source" / p
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes((ROOT / p).read_bytes())
    check = subprocess.run([str(BIN / "meter"), "meter-check"], capture_output=True, text=True, timeout=60, preexec_fn=limits)
    (OUT / "meter-check.log").write_text(check.stdout + check.stderr)
    assert check.returncode == 0
    for index, (mode, cancel) in enumerate(itertools.product(MODES, [0, 1])):
        config = (mode, "choice", 8, 2, True, False)
        run("cancel", index, config, cancel)
        run("cancel-meter", index, config, cancel)
    configs = [(m, f, n, q, r, o) for m, f, q, r, o in itertools.product(MODES, FAMILIES, [1, 8], [False, True], [False, True]) for n in [0, 8, 12 if f == "grow" else 32]]
    assert len(configs) == 720
    random.Random(7111).shuffle(configs)
    for i, config in enumerate(configs):
        run("ordinary", i, config)
        if (i + 1) % 40 == 0:
            print(f"ordinary {i + 1}/720", flush=True)
    configs = [(m, f, n, q, r, o) for m, f, r, o in itertools.product(MODES, FAMILIES, [False, True], [False, True]) for n, q in [(0, 1), (12 if f == "grow" else 32, 8)]]
    previous = []
    for repetition in range(2):
        for i, config in enumerate(configs):
            records = allocation_records(run("allocation", repetition * len(configs) + i, config))
            if repetition == 0:
                previous.append(records)
            else:
                assert records == previous[i], ("allocation replay differs", config)
        print(f"allocation {repetition + 1}/2", flush=True)
    print("Completed 720 exploratory timings, 480 exact allocation replays, 24 cancellation runs.", flush=True)


if __name__ == "__main__":
    main()
