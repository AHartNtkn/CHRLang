"""Verify frozen receipts and apply the registered paired lifecycle criterion."""
import collections
import hashlib
import json
from pathlib import Path
import statistics

ROOT = Path(__file__).resolve().parents[3]
BASE = ROOT / "docs/experiments/results"
SIZING = BASE / "s03-pulltab-sizing"
CONFIRM = BASE / "s03-pulltab-confirmation"
PHASES = ["setup", "execute_observe", "engine_drop", "answer_drop"]
CONTRASTS = [("pull", "dependencies"), ("dependencies", "static"), ("dependencies", "sealed"),
             ("prefix", "sealed"), ("prefix", "dependencies")]


def read(directory, name):
    record = json.loads((directory / name).read_text())
    assert record["returncode"] == 0 and not record.get("timeout")
    payload = json.loads(record["stdout"].splitlines()[-1])
    assert payload["event"] == "result" and not payload["counters"]
    assert all(s["complete"] for s in payload["samples"])
    return record, payload


def measures(v):
    result = {"preparation": v["preparation"]["ns"], "prepared_drop": v["prepared_drop"]["ns"]}
    for phase in PHASES:
        result[phase] = sum(s[phase]["ns"] for s in v["samples"])
    result["total"] = sum(result.values())
    result["with_inputs"] = result["total"] + v["source_build"]["ns"] + sum(s["input_build"]["ns"] for s in v["samples"])
    result["first_execution"] = v["samples"][0]["first_answer_ns"]
    return result


def classify(ratios):
    median = statistics.median(ratios)
    return "gain" if median <= .9 and max(ratios) < 1 else "loss" if median >= 1.1 and min(ratios) > 1 else "unresolved"


def main():
    for directory in [SIZING, CONFIRM]:
        freeze = json.loads((directory / "freeze.json").read_text())
        for name, digest in freeze["sources"].items():
            assert hashlib.sha256((directory / "source" / name).read_bytes()).hexdigest() == digest
        for name, digest in freeze["binaries"].items():
            assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest
    manifest = json.loads((CONFIRM / "manifest.json").read_text())
    assert len(manifest) == 4480
    samples = {}
    for item in manifest:
        record, value = read(CONFIRM, item["file"])
        config = tuple(item["config"])
        command = record["command"]
        assert command[1:3] == [item["mode"], config[0]]
        assert list(map(int, command[3:])) == [config[1], config[2], int(config[3]), int(config[4])]
        assert len(value["samples"]) == config[2] and not value["meter"]
        if item["block"] >= 0:
            key = (config, item["mode"], item["block"])
            assert key not in samples
            samples[key] = measures(value)
    assert len(samples) == 3920
    configs = sorted({key[0] for key in samples})
    assert len(configs) == 80
    cells, counts = [], {}
    for a, b in CONTRASTS:
        label = f"{a}/{b}"
        count = collections.Counter()
        for config in configs:
            aa = [samples[(config, a, i)] for i in range(7)]
            bb = [samples[(config, b, i)] for i in range(7)]
            ratios = [x["total"] / y["total"] for x, y in zip(aa, bb)]
            broad = [x["with_inputs"] / y["with_inputs"] for x, y in zip(aa, bb)]
            status = classify(ratios)
            count[status] += 1
            cells.append({"contrast": label, "config": config, "status": status,
                          "median_ratio": statistics.median(ratios), "min_ratio": min(ratios), "max_ratio": max(ratios),
                          "with_inputs_status": classify(broad), "with_inputs_ratio": statistics.median(broad),
                          "candidate_ns": {k: statistics.median([r[k] for r in aa]) for k in aa[0]},
                          "control_ns": {k: statistics.median([r[k] for r in bb]) for k in bb[0]}})
        counts[label] = dict(count)
    allocations = {}
    for directory in [SIZING, CONFIRM]:
        seen = {}
        for file in sorted(directory.glob("allocation-*.json")):
            record, value = read(directory, file.name)
            assert value["meter"]
            command = record["command"][1:]
            key = tuple(command)
            primary = [value["preparation"], value["prepared_drop"]]
            for s in value["samples"]:
                primary.extend(s[k] for k in PHASES)
            all_phases = primary + [value["source_build"]] + [s["input_build"] for s in value["samples"]]
            memory = {"requested_primary": sum(p["memory"]["requested_bytes"] for p in primary),
                      "peak_measured_growth": max(p["memory"]["peak_live"] for p in all_phases) - value["source_build"]["memory"]["live_start"]}
            if key in seen:
                assert seen[key] == memory
            seen[key] = memory
        assert len(seen) == 140
        allocations.update({"/".join(k): v for k, v in seen.items()})
    result = {"counts": counts, "cells": cells, "allocations": allocations,
              "confirmed_processes": len(samples), "warmups": len(manifest) - len(samples)}
    (BASE / "s03-pulltab-lifecycle-summary.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(counts, indent=2))


if __name__ == "__main__":
    main()
