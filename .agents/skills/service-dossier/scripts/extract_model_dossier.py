#!/usr/bin/env python3
"""Emit Smithy-derived dossier material for a vendored AWS service model."""

from __future__ import annotations

import argparse
import html
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[4]
MODELS_ROOT = ROOT / "vendor" / "api-models-aws" / "models"


def clean_doc(value: dict[str, Any], max_len: int = 220) -> str:
    raw = value.get("traits", {}).get("smithy.api#documentation", "")
    text = re.sub(r"<[^>]+>", " ", raw)
    text = html.unescape(text)
    text = re.sub(r"\s+", " ", text).strip()
    if len(text) > max_len:
        return text[: max_len - 4].rstrip() + " ..."
    return text or "-"


def short_name(shape_id: str) -> str:
    return shape_id.split("#", 1)[-1]


def resolve_model(slug: str) -> Path:
    direct = MODELS_ROOT / slug / "service"
    if direct.exists():
        files = sorted(direct.glob("*/*.json"))
        if files:
            return files[-1]

    matches = sorted(MODELS_ROOT.glob(f"*{slug}*/service/*/*.json"))
    if len(matches) == 1:
        return matches[0]
    if not matches:
        raise SystemExit(f"No Smithy model found for {slug!r}")

    print("Multiple Smithy models matched; choose one explicitly:")
    for match in matches:
        print(f"- {match.relative_to(ROOT)}")
    raise SystemExit(2)


def protocol_names(traits: dict[str, Any]) -> list[str]:
    protocol_map = {
        "aws.protocols#restJson1": "restJson1",
        "aws.protocols#restXml": "restXml",
        "aws.protocols#awsJson1_0": "awsJson1_0",
        "aws.protocols#awsJson1_1": "awsJson1_1",
        "aws.protocols#awsQuery": "awsQuery",
        "aws.protocols#ec2Query": "ec2Query",
        "aws.protocols#awsQueryCompatible": "awsQueryCompatible",
    }
    return [label for key, label in protocol_map.items() if key in traits]


def normalise_sdk_slug(sdk_id: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", sdk_id.lower())


def required_members(shape: dict[str, Any]) -> list[str]:
    members = shape.get("members", {})
    return [
        name
        for name, member in members.items()
        if "smithy.api#required" in member.get("traits", {})
    ]


def op_traits(op: dict[str, Any], shapes: dict[str, Any]) -> tuple[list[str], list[str]]:
    traits = op.get("traits", {})
    labels: list[str] = []
    tokens: list[str] = []
    if "smithy.api#readonly" in traits:
        labels.append("readonly")
    if "smithy.api#idempotent" in traits:
        labels.append("idempotent")
    if "smithy.api#paginated" in traits:
        labels.append("paginated")

    input_target = op.get("input", {}).get("target")
    if input_target in shapes:
        for name, member in shapes[input_target].get("members", {}).items():
            if "smithy.api#idempotencyToken" in member.get("traits", {}):
                tokens.append(name)
        if tokens and "idempotency-token" not in labels:
            labels.append("idempotency-token")
    return labels, tokens


def http_binding(op: dict[str, Any]) -> str:
    http_trait = op.get("traits", {}).get("smithy.api#http")
    if not http_trait:
        return "-"
    return f"{http_trait.get('method', '-')} {http_trait.get('uri', '-')}"


def family_name(operation_name: str) -> str:
    match = re.match(r"[A-Z][a-z]+", operation_name)
    return match.group(0) if match else operation_name


def shape_members(shape: dict[str, Any], max_members: int = 12) -> str:
    members = list(shape.get("members", {}).keys())
    if not members:
        return "-"
    if len(members) > max_members:
        shown = ", ".join(members[:max_members])
        return f"{shown}, ... (+{len(members) - max_members})"
    return ", ".join(members)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("service", help="Smithy model slug or unambiguous substring")
    parser.add_argument(
        "--existing",
        type=Path,
        help="Existing dossier path to inspect for manual sections. Defaults to docs/services/<model-slug>.md when present.",
    )
    args = parser.parse_args()

    model_path = resolve_model(args.service)
    model_slug = model_path.parts[-4]
    existing_path = args.existing
    if existing_path is None:
        candidate = ROOT / ".agents" / "docs" / "services" / f"{model_slug}.md"
        existing_path = candidate if candidate.exists() else None

    model = json.loads(model_path.read_text())
    shapes: dict[str, Any] = model["shapes"]
    service_id, service = next(
        (shape_id, shape)
        for shape_id, shape in shapes.items()
        if shape.get("type") == "service"
    )
    service_traits = service.get("traits", {})
    aws_service = service_traits.get("aws.api#service", {})
    title = service_traits.get("smithy.api#title", short_name(service_id))
    protocols = protocol_names(service_traits)
    operations = [short_name(ref["target"]) for ref in service.get("operations", [])]
    resources = {
        short_name(shape_id): shape
        for shape_id, shape in shapes.items()
        if shape.get("type") == "resource"
    }

    operation_rows: list[dict[str, Any]] = []
    for operation_name in operations:
        operation = shapes[f"{service_id.rsplit('#', 1)[0]}#{operation_name}"]
        input_target = operation.get("input", {}).get("target")
        output_target = operation.get("output", {}).get("target")
        labels, tokens = op_traits(operation, shapes)
        input_shape = shapes.get(input_target, {}) if input_target else {}
        errors = [short_name(error["target"]) for error in operation.get("errors", [])]
        operation_rows.append(
            {
                "name": operation_name,
                "family": family_name(operation_name),
                "http": http_binding(operation),
                "traits": labels,
                "required": required_members(input_shape),
                "tokens": tokens,
                "output": short_name(output_target) if output_target else "Unit",
                "errors": errors,
                "doc": clean_doc(operation),
            }
        )

    print(f"# {title}")
    print()
    if existing_path is not None:
        if not existing_path.is_absolute():
            existing_path = ROOT / existing_path
        if existing_path.exists():
            section_names = [
                line.removeprefix("## ").strip()
                for line in existing_path.read_text().splitlines()
                if line.startswith("## ")
            ]
            extractor_sections = {
                "Service Overview",
                "Possible Usage Scenarios",
                "Service Identity and Protocol",
                "Behavioural Model Notes",
                "Official AWS Documentation Research",
                "Resource Model",
                "Operation Groups",
                "Operation Detail Matrix",
                "Important Shapes",
                "Research Checklist for Parity Work",
            }
            high_judgement_sections = [
                section
                for section in section_names
                if section
                in {
                    "Research Checklist for Parity Work",
                    "Possible Usage Scenarios",
                    "Behavioural Model Notes",
                    "Official AWS Documentation Research",
                }
            ]
            manual_sections = [
                section for section in section_names if section not in extractor_sections
            ]
            print("## Existing Dossier Notes")
            print()
            print(f"- Existing dossier: `{existing_path.relative_to(ROOT)}`")
            print("- This extractor is read-only and emits Smithy-derived comparison material only.")
            if section_names:
                print(
                    "- Existing top-level sections are merge-sensitive: "
                    + ", ".join(f"`{section}`" for section in section_names)
                    + "."
                )
            if high_judgement_sections:
                print(
                    "- Review these human-judgement sections especially carefully; do not replace them wholesale: "
                    + ", ".join(f"`{section}`" for section in high_judgement_sections)
                    + "."
                )
            if manual_sections:
                print(
                    "- Preserve these non-standard or manual sections during merge: "
                    + ", ".join(f"`{section}`" for section in manual_sections)
                    + "."
                )
            else:
                print("- No non-standard top-level sections were detected.")
            print("- Preserve manual edits inside every existing section unless the current Smithy model or official AWS documentation clearly contradicts them.")
            print()
    print("## Service Identity and Protocol")
    print()
    print(f"- AWS model slug: `{model_slug}`")
    print(f"- AWS SDK for Rust slug: `{normalise_sdk_slug(aws_service.get('sdkId', title))}`")
    print(f"- Model version: `{service.get('version', model_path.parent.name)}`")
    print(f"- Model file: `{model_path.relative_to(ROOT)}`")
    print(f"- SDK ID: `{aws_service.get('sdkId', '-')}`")
    print(f"- Endpoint prefix: `{aws_service.get('endpointPrefix', '-')}`")
    print(f"- ARN namespace: `{aws_service.get('arnNamespace', '-')}`")
    print("- CloudFormation name: `-`")
    print("- CloudTrail event source: `-`")
    print(f"- Protocols: `{', '.join(protocols) or '-'}`")
    print("- Auth schemes: `sigv4`" if "aws.auth#sigv4" in service_traits else "- Auth schemes: `-`")
    endpoint_params = (
        service_traits.get("smithy.rules#endpointRuleSet", {})
        .get("parameters", {})
        .keys()
    )
    print(f"- Endpoint rule parameters: `{', '.join(endpoint_params) or '-'}`")
    print()

    family_counts = Counter(row["family"] for row in operation_rows)
    print("## Behavioural Model Notes")
    print()
    print(
        "- Operation surface is concentrated in these families: "
        + ", ".join(f"`{family}` ({count})" for family, count in family_counts.most_common())
        + "."
    )
    paginated = [row["name"] for row in operation_rows if "paginated" in row["traits"]]
    idempotent = [row["name"] for row in operation_rows if "idempotent" in row["traits"]]
    token_ops = [row["name"] for row in operation_rows if row["tokens"]]
    print(f"- Pagination is modelled for {len(paginated)} operations.")
    print(f"- Idempotency is explicit for {len(idempotent)} operations.")
    if token_ops:
        print(f"- Explicit idempotency tokens appear on: {', '.join(f'`{name}`' for name in token_ops)}.")
    print(f"- {len(operation_rows)} operations declare modelled service errors.")
    if resources:
        print(f"- Smithy declares {len(resources)} resource shapes.")
    else:
        print("- The Smithy model declares no `resource` shapes; derive conceptual resources from docs and operation names.")
    print()

    if resources:
        print("## Resource Model")
        print()
        print("| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |")
        print("|---|---|---|---|---|")
        namespace = service_id.rsplit("#", 1)[0]
        for resource_name, resource in sorted(resources.items()):
            identifiers = ", ".join(resource.get("identifiers", {}).keys()) or "-"
            lifecycle_refs = []
            for key in ("create", "put", "read", "update", "delete"):
                target = resource.get(key, {}).get("target")
                if target:
                    lifecycle_refs.append(short_name(target))
            operation_refs = [short_name(ref["target"]) for ref in resource.get("operations", [])]
            other_ops = [op for op in operation_refs if op not in lifecycle_refs]
            print(
                f"| `{resource_name}` | {identifiers} | {', '.join(lifecycle_refs) or '-'} | "
                f"{', '.join(other_ops) or '-'} | {clean_doc(resource)} |"
            )
        print()

    print("## Operation Groups")
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in operation_rows:
        grouped[row["family"]].append(row)
    for family, rows in sorted(grouped.items(), key=lambda item: (-len(item[1]), item[0])):
        trait_counts = Counter(trait for row in rows for trait in row["traits"])
        required_counts = Counter(name for row in rows for name in row["required"])
        common_required = [name for name, count in required_counts.items() if count > 1]
        print()
        print(f"### {family}")
        print()
        print("- Operations: " + ", ".join(f"`{row['name']}`" for row in rows))
        if trait_counts:
            print(
                "- Traits: "
                + ", ".join(f"`{trait}` ({count})" for trait, count in trait_counts.items())
            )
        print(
            "- Common required input members in this group: "
            + (", ".join(f"`{name}`" for name in common_required) if common_required else "-")
        )
    print()

    print("## Operation Detail Matrix")
    print()
    print("| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |")
    print("|---|---|---|---|---|---|---|---|")
    for row in operation_rows:
        print(
            "| `{name}` | `{http}` | {traits} | {required} | {tokens} | `{output}` | {errors} | {doc} |".format(
                name=row["name"],
                http=row["http"],
                traits=", ".join(f"`{trait}`" for trait in row["traits"]) or "-",
                required=", ".join(f"`{name}`" for name in row["required"]) or "-",
                tokens=", ".join(f"`{name}`" for name in row["tokens"]) or "-",
                output=row["output"],
                errors=", ".join(f"`{name}`" for name in row["errors"]) or "-",
                doc=row["doc"],
            )
        )
    print()

    important: dict[str, dict[str, Any]] = {}
    for shape_id, shape in shapes.items():
        shape_name = short_name(shape_id)
        traits = shape.get("traits", {})
        if "smithy.api#error" in traits:
            important[shape_name] = shape
    for row in operation_rows:
        op = shapes[f"{service_id.rsplit('#', 1)[0]}#{row['name']}"]
        for target in (op.get("input", {}).get("target"), op.get("output", {}).get("target")):
            if target in shapes and len(important) < 40:
                important[short_name(target)] = shapes[target]
    for shape_id, shape in shapes.items():
        shape_name = short_name(shape_id)
        if shape.get("type") in {"enum", "intEnum"} and len(important) < 50:
            important[shape_name] = shape

    print("## Important Shapes")
    print()
    print("| Shape | Type | Members | Documentation cue |")
    print("|---|---|---|---|")
    for shape_name, shape in important.items():
        print(
            f"| `{shape_name}` | `{shape.get('type', '-')}` | "
            f"{shape_members(shape)} | {clean_doc(shape, 160)} |"
        )


if __name__ == "__main__":
    main()
