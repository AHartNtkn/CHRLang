"""Run the registered diagnostic binary twice with bounded resources; no timings."""
import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("binary", type=Path)
parser.add_argument("output", type=Path)
parser.add_argument("--policy", choices=["CurrentContext", "StaticBirth", "MatchDependencies"], default="StaticBirth")
args = parser.parse_args()
binary = args.binary.resolve()
args.output.mkdir(parents=True, exist_ok=False)
source_paths = [
    "research/chr-direct-choice/src/demand.rs",
    "research/chr-direct-choice/Cargo.toml",
    "research/chr-direct-conditional/Cargo.toml",
    "research/chr-direct-conditional/tests/pulltab_work.rs",
    "research/chr-direct-conditional/tests/runtime_support/mod.rs",
    "research/chr-direct-conditional/experiments/pulltab_work.py",
    "docs/experiments/registrations/S03-pulltab-work.md",
    "docs/experiments/registrations/S03-match-dependencies.md",
    "Cargo.lock",
]
metadata = {
    "policy": args.policy,
    "base_commit": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
    "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
    "binary": str(binary),
    "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
    "sources": {p: hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in source_paths},
    "argv": [str(binary), "--nocapture", "--test-threads=1"],
    "timeout_seconds": 60,
    "address_space_bytes": 1 << 30,
}
(args.output / "freeze.json").write_text(json.dumps(metadata, indent=2) + "\n")
for name in source_paths:
    dest = args.output / "source" / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(Path(name).read_bytes())
environment = os.environ.copy()
environment["PULL_POLICY"] = args.policy


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))


previous = None
for repetition in [1, 2]:
    result = subprocess.run(metadata["argv"], text=True, capture_output=True,
                            timeout=60, preexec_fn=limits, env=environment)
    (args.output / f"run{repetition}.log").write_text(result.stdout + result.stderr)
    assert result.returncode == 0, result.stderr + result.stdout
    lines = [line for line in result.stdout.splitlines() if line.startswith("PULL,")]
    assert len(lines) == 144, len(lines)
    if previous is not None:
        assert lines == previous, "diagnostic repetitions differ"
    previous = lines
    header = next(line for line in result.stdout.splitlines() if "PULL_HEADER," in line)
    header = header[header.index("PULL_HEADER,"):].split(",")[1:]
    with (args.output / f"run{repetition}.csv").open("w") as out:
        writer = csv.writer(out)
        writer.writerow(header)
        writer.writerows(line.split(",")[1:] for line in lines)
print("Two identical 144-row diagnostic runs; all independent answer checks passed.")
