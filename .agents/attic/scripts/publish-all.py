#!/usr/bin/env python3
"""Publish all workspace crates to crates.io in dependency order.

Usage:
    ./scripts/publish-all.py              # dry-run (default)
    ./scripts/publish-all.py --execute    # actually publish

Prerequisites:
    - cargo login (authenticated with crates.io token)
    - All changes committed (cargo publish rejects dirty trees)
"""

import json
import subprocess
import sys
import time
from collections import defaultdict, deque


def get_publish_plan() -> list[list[str]]:
    """Compute layered publish order from cargo metadata."""
    raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version=1", "--no-deps"],
        stderr=subprocess.DEVNULL,
    )
    meta = json.loads(raw)
    workspace_members = set(meta["workspace_members"])

    pkgs = {}
    for p in meta["packages"]:
        if p["id"] in workspace_members:
            pkgs[p["name"]] = p

    publishable = {
        name
        for name, p in pkgs.items()
        if p.get("publish") is None or len(p["publish"]) > 0
    }

    # Internal dependency graph
    deps: dict[str, set[str]] = defaultdict(set)
    for name in publishable:
        for dep in pkgs[name]["dependencies"]:
            if dep["name"] in publishable:
                deps[name].add(dep["name"])

    # Topological sort (Kahn's algorithm)
    in_degree = {name: len(deps[name]) for name in publishable}
    rdeps: dict[str, set[str]] = defaultdict(set)
    for name in publishable:
        for dep in deps[name]:
            rdeps[dep].add(name)

    queue = deque(sorted(n for n in publishable if in_degree[n] == 0))
    order: list[str] = []
    while queue:
        node = queue.popleft()
        order.append(node)
        for dependent in sorted(rdeps[node]):
            in_degree[dependent] -= 1
            if in_degree[dependent] == 0:
                queue.append(dependent)

    if len(order) != len(publishable):
        print("ERROR: dependency cycle detected", file=sys.stderr)
        sys.exit(1)

    # Assign layers
    layer: dict[str, int] = {}
    for name in order:
        if not deps[name]:
            layer[name] = 0
        else:
            layer[name] = max(layer[d] for d in deps[name]) + 1

    # Within each layer, publish the most-depended-on crates first so that
    # downstream crates can find them on crates.io sooner.
    dep_count: dict[str, int] = {name: len(rdeps.get(name, set())) for name in publishable}
    max_layer = max(layer.values())
    return [
        sorted(
            (n for n in order if layer[n] == l),
            key=lambda n: (-dep_count[n], n),
        )
        for l in range(max_layer + 1)
    ]


def publish_crate(name: str) -> bool:
    result = subprocess.run(
        ["cargo", "publish", "-p", name, "--no-verify"],
        capture_output=True,
    )
    return result.returncode == 0


def main() -> None:
    execute = "--execute" in sys.argv

    layers = get_publish_plan()
    total = sum(len(l) for l in layers)

    print("===========================================")
    print(" winterbaume bulk publish")
    print("===========================================")
    print(f"Mode:   {'LIVE' if execute else 'DRY RUN (pass --execute to publish)'}")
    print(f"Crates: {total} across {len(layers)} layers")
    print("===========================================")
    print()

    failures: list[str] = []
    seq = 0

    for layer_idx, crates in enumerate(layers):
        print(f"--- Layer {layer_idx} ({len(crates)} crates) ---")

        for name in crates:
            seq += 1
            if not execute:
                print(f"  [{seq}/{total}] {name}")
                continue

            print(f"  [{seq}/{total}] {name} ... ", end="", flush=True)
            if publish_crate(name):
                print("ok")
            else:
                print("FAILED")
                failures.append(name)

        # Wait for the crates.io index between layers (not after the last one).
        if execute and layer_idx < len(layers) - 1:
            print()
            print("  Waiting 30s for crates.io index to update...")
            time.sleep(30)

        print()

    print("===========================================")
    if failures:
        print(f" {len(failures)} crate(s) failed:")
        for f in failures:
            print(f"   - {f}")
        print()
        print(" Re-run failed crates with: cargo publish -p <crate>")
        print("===========================================")
        sys.exit(1)
    else:
        print(f" All {total} crates processed successfully.")
        print("===========================================")


if __name__ == "__main__":
    main()
