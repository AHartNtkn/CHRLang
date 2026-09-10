"""Registered bounded producer-lead diagnostic; no timing claim."""
from pathlib import Path
import hashlib, json, resource, subprocess
ROOT = Path(__file__).resolve().parents[3]
RAW = ROOT / "docs/experiments/results/s08-equality-overlap"
def digest(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()
def limits():
    resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
    resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
if __name__ == "__main__":
    for mode in ["forward", "reverse"]:
        binary = ROOT / f"target/s08-lead-{mode}/release/examples/equality_backlog"
        files = [Path(__file__), binary, ROOT / "research/chr-direct-conditional/examples/equality_backlog.rs", ROOT / "research/chr-direct-conditional/Cargo.toml", ROOT / "docs/experiments/registrations/S08-bounded-producer-lead.md"] + list((ROOT / "research/chr-direct-conditional/src").glob("*.rs"))
        with (RAW / f"lead-{mode}-manifest.json").open("x") as f:
            json.dump({str(p.relative_to(ROOT)): digest(p) for p in files}, f, indent=2)
        outcomes = []
        for rep in range(2):
            cmd = [str(binary)]
            p = subprocess.run(cmd, capture_output=True, text=True, preexec_fn=limits, timeout=60)
            with (RAW / f"lead-{mode}-{rep}.json").open("x") as f:
                json.dump(dict(command=cmd, returncode=p.returncode, stdout=p.stdout, stderr=p.stderr), f, indent=2)
            assert p.returncode == 0, p.stderr
            assert not p.stderr
            rows = [json.loads(l) for l in p.stdout.splitlines() if l.startswith("{")]
            assert [r["answers"] for r in rows] == [1, 8, 32, 64, 128]
            for l in p.stdout.splitlines():
                if l.startswith("backlog"):
                    assert int(l.split("pending=")[1]) <= 2
            outcomes.append(p.stdout)
        assert outcomes[0] == outcomes[1]
        print(mode, outcomes[0])
