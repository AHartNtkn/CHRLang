#!/usr/bin/env python3
"""Prospectively bounded S03 exploratory sizing; no confirmatory statistics."""
import hashlib
import json
import os
from pathlib import Path
import platform
import random
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "docs/experiments/results/s03-demand-sizing"
BIN = Path("/tmp/chr-demand-sizing-26e59f5")
MODES = ["demand", "current", "graph", "conditional", "scan", "indexed", "lowered"]
FAMILIES = ["plain", "opaque", "discriminate"]
CPU = min(os.sched_getaffinity(0))

def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
    os.sched_setaffinity(0, {CPU})

def run(kind, index, mode, family, depth, queries, consuming, cancel=None):
    binary = BIN / ("meter" if kind in ("allocation", "cancel-meter") else "ordinary")
    command = [str(binary), mode, family, str(depth), str(queries), str(int(consuming))]
    if cancel is not None:
        command.append(str(cancel))
    dest = OUT / f"{kind}-{index:03}.json"
    if dest.exists():
        raise RuntimeError(f"Refusing to overwrite an existing run: {dest}")
    start = time.monotonic()
    try:
        p = subprocess.run(command, capture_output=True, text=True, timeout=60, preexec_fn=limits)
        result = {"command": command, "wall_seconds": time.monotonic()-start,
                  "returncode": p.returncode, "stdout": p.stdout, "stderr": p.stderr}
    except subprocess.TimeoutExpired as error:
        result = {"command": command, "wall_seconds": time.monotonic()-start,
                  "timeout": True, "stdout": str(error.stdout), "stderr": str(error.stderr)}
        dest.write_text(json.dumps(result, indent=2)+"\n")
        raise
    dest.write_text(json.dumps(result, indent=2)+"\n")
    if p.returncode:
        raise RuntimeError(f"Failed process: {dest}")
    payload = [json.loads(line) for line in p.stdout.splitlines() if line.startswith('{')][-1]
    assert payload["event"] == "result" and not payload["counters"]
    assert payload["meter"] == (binary.name == "meter")
    assert len(payload["samples"]) == queries
    for i, sample in enumerate(payload["samples"]):
        assert sample["complete"] == (cancel is None or i % 2 == 1)
    return payload

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    sources = [ROOT/"research/chr-direct-conditional/experiments"/f for f in
               ["demand_cost.rs", "demand_source.rs", "demand_sizing.py"]]
    sources += [ROOT/"research/chr-direct-choice/src/demand.rs",
                ROOT/"research/chr-direct-conditional/Cargo.toml",
                ROOT/"Cargo.lock"]
    freeze = {"cpu": CPU, "available_cpus": sorted(os.sched_getaffinity(0)),
              "host": platform.platform(), "base_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
              "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "sha256": {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources},
              "binaries": {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in [BIN/"ordinary", BIN/"meter"]}}
    (OUT/"freeze.json").write_text(json.dumps(freeze, indent=2)+"\n")
    check = subprocess.run([str(BIN/"meter"), "meter-check"], capture_output=True, text=True, timeout=60, preexec_fn=limits)
    (OUT/"meter-check.log").write_text(check.stdout+check.stderr)
    assert check.returncode == 0
    for i, mode in enumerate(MODES):
        run("cancel", i, mode, "opaque", 8, 2, True, 1)
        run("cancel-meter", i, mode, "opaque", 8, 2, True, 1)
    configs = [(m,f,n,q,True) for m in MODES for f in FAMILIES for n in [0,8,32] for q in [1,8]]
    configs += [(m,f,32,8,False) for m in MODES for f in FAMILIES]
    random.Random(7106).shuffle(configs)
    assert len(configs) == 147
    for i, config in enumerate(configs):
        run("ordinary", i, *config)
        if (i+1) % 21 == 0:
            print(f"ordinary {i+1}/147", flush=True)
    diagnostics = [(m,f,32,8,r) for m in MODES for f in FAMILIES for r in [False,True]]
    for repeat in range(2):
        for i, config in enumerate(diagnostics):
            run("allocation", repeat*len(diagnostics)+i, *config)
        print(f"allocation repetition {repeat+1}/2", flush=True)
    print("completed 147 ordinary, 84 allocation, 14 cancellation processes and meter self-check", flush=True)

if __name__ == "__main__":
    main()
