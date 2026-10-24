#!/usr/bin/env python3
"""Show all sw-checklist results and reject new or enlarged inherited failures."""
import json
import re
import subprocess
from pathlib import Path


def validate(report, limits):
    failures = [line for line in report.splitlines() if line.startswith("[FAIL]")]
    for line in failures:
        metric = re.search(r" has (\d+)", line)
        key = line[:metric.start()] if metric else line
        if not metric or key not in limits or int(metric.group(1)) > limits[key]:
            raise SystemExit(f"New or enlarged checklist failure: {line}")
    return len(failures)


def main():
    root = Path(__file__).resolve().parent.parent
    baseline = json.loads((root / "scripts/checklist-baseline.json").read_text())
    result = subprocess.run(["sw-checklist", "-v", str(root)], capture_output=True, text=True)
    report = re.sub(r"\x1b\[[0-9;]*m", "", result.stdout)
    print(report, end="")
    if result.stderr or result.returncode not in (0, 1) or "Summary:" not in report:
        raise SystemExit(f"sw-checklist failed to run correctly: {result.stderr}")
    count = validate(report, baseline["limits"])
    if result.returncode and count == 0:
        raise SystemExit("sw-checklist failed without reporting a recognized failure")
    print(f"Checklist regression gate passed; {count} inherited failures remain (not a clean checklist).")


if __name__ == "__main__":
    main()
