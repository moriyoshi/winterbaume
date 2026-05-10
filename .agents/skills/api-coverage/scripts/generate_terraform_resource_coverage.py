#!/usr/bin/env python3
"""Per-service Terraform RESOURCE-TYPE coverage report for winterbaume.

Reads the spec files at `crates/winterbaume-terraform/specs/*.toml` to
determine which Terraform resource types each winterbaume service handles,
then compares against the AWS provider schema's full resource catalogue.

This is a complementary signal to
`generate_terraform_converter_coverage.py`, which measures *attribute*
coverage within each handled resource. This script measures *resource-
type* coverage within each service: how many of the service's
Terraform resource types we handle at all.

For each service, computes:
  - The set of TF resource types handled (from `[[resource]] type = "..."`
    entries in the spec).
  - The "service prefix" — either the longest common prefix of the handled
    types, or a manual override for heterogeneous-named services like
    `ec2` (mixes `aws_vpc`, `aws_subnet`, etc.).
  - The set of TF resource types in the schema matching that prefix.
  - Coverage % = handled / candidates.

Output: a Markdown table sorted by absolute gap (descending), followed by
per-service "missing resources" sections listing the unhandled
`aws_<svc>_<resource>` types.

Usage:
    python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py [--output PATH] [--cache PATH]

Default output: .agents/docs/TERRAFORM_RESOURCE_COVERAGE.md
Default cache:  .agents-workspace/tmp/tf-schema/aws_provider_schema.json
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
SPECS_DIR = ROOT / "crates" / "winterbaume-terraform" / "specs"
DEFAULT_CACHE = ROOT / ".agents-workspace" / "tmp" / "tf-schema" / "aws_provider_schema.json"
DEFAULT_OUTPUT = ROOT / ".agents" / "docs" / "TERRAFORM_RESOURCE_COVERAGE.md"


# Manual prefix overrides for services where the longest-common-prefix
# heuristic doesn't produce a useful "service" prefix — heterogeneous-named
# resources (ec2 mixes `aws_vpc`, `aws_subnet`, `aws_security_group`, etc.)
# or services with only one handled type that produces an over-narrow prefix.
PREFIX_OVERRIDES: dict[str, list[str]] = {
    "ec2": [
        "aws_vpc", "aws_subnet", "aws_internet_gateway", "aws_egress_only_internet_gateway",
        "aws_security_group", "aws_route_table", "aws_route", "aws_main_route_table_association",
        "aws_nat_gateway", "aws_eip", "aws_network_acl", "aws_network_interface",
        "aws_key_pair", "aws_placement_group", "aws_customer_gateway", "aws_ec2_",
        "aws_default_vpc", "aws_default_subnet", "aws_default_security_group",
        "aws_default_route_table", "aws_default_network_acl", "aws_volume_attachment",
        "aws_instance", "aws_ami", "aws_spot_", "aws_launch_template",
        "aws_vpc_peering_", "aws_vpc_endpoint", "aws_vpc_dhcp_options",
        "aws_vpc_ipam", "aws_vpc_security_group_", "aws_vpc_network_performance_",
        "aws_vpn_gateway", "aws_vpn_connection", "aws_transit_gateway",
        "aws_flow_log", "aws_verifiedaccess_",
    ],
    "rds": ["aws_db_", "aws_rds_"],
    "elbv2": ["aws_lb", "aws_alb"],
    "autoscaling": ["aws_autoscaling_", "aws_launch_configuration", "aws_launch_template"],
    "cloudtrail": ["aws_cloudtrail"],
    "codepipeline": ["aws_codepipeline"],
    "elasticloadbalancing": ["aws_elb"],
    "cloudwatch": ["aws_cloudwatch_metric_", "aws_cloudwatch_dashboard", "aws_cloudwatch_composite_alarm"],
    "appfabric": ["aws_appfabric_"],
    "ram": ["aws_ram_"],
    "signer": ["aws_signer_"],
    "comprehend": ["aws_comprehend_"],
    "costexplorer": ["aws_ce_"],
    "acmpca": ["aws_acmpca_"],
    "efs": ["aws_efs_"],
    "ec2instanceconnect": ["aws_ec2_instance_connect_"],
    "applicationcostprofiler": ["aws_applicationcostprofiler_"],
    "appsync": ["aws_appsync_"],
    "account": ["aws_account_"],
    "logs": ["aws_cloudwatch_log_"],
    "events": ["aws_cloudwatch_event_"],
    "firehose": ["aws_kinesis_firehose_"],
    "kinesisvideo": ["aws_kinesis_video_"],
    "stepfunctions": ["aws_sfn_"],
    "kafka": ["aws_msk_"],
    "directconnect": ["aws_dx_"],
    "directory": ["aws_directory_service_"],
    "outposts": ["aws_outposts_"],
    "mediastore": ["aws_media_store_", "aws_mediastore_"],
    "mediapackage": ["aws_media_package_"],
    "mediapackagev2": ["aws_media_packagev2_"],
    "ses": ["aws_sesv2_"],
    "sesv1": ["aws_ses_"],
    "lexmodelsv2": ["aws_lexv2models_"],
    "amp": ["aws_prometheus_"],
    "applicationautoscaling": ["aws_appautoscaling_"],
    "fis": ["aws_fis_"],
    "emrcontainers": ["aws_emrcontainers_"],
    "resiliencehub": ["aws_resiliencehub_"],
    "route53domains": ["aws_route53domains_"],
    "servicequotas": ["aws_servicequotas_"],
    "servicediscovery": ["aws_service_discovery_"],
    "servicecatalogappregistry": ["aws_servicecatalogappregistry_"],
    "cognitoidp": [
        "aws_cognito_user_", "aws_cognito_managed_user_pool_client",
        "aws_cognito_resource_server", "aws_cognito_identity_provider",
        "aws_cognito_log_delivery_configuration",
    ],
    "cognitoidentity": ["aws_cognito_identity_"],
    "timestreaminfluxdb": ["aws_timestreaminfluxdb_"],
    "timestreamquery": ["aws_timestreamquery_"],
    "route53resolver": ["aws_route53_resolver_"],
}


# ---------------------------------------------------------------------------
# Schema loading
# ---------------------------------------------------------------------------


def get_terraform_schema(cache_path: Path) -> dict:
    """Return the AWS provider's flat `{resource_type: schema_dict}` mapping.

    Tries `cache_path` first. The cache file may be in either of two
    formats:
      - the inner `resource_schemas` dict (preferred, smaller)
      - the full `terraform providers schema -json` output

    On a cache miss runs `terraform providers schema -json` in a temp dir
    and writes the inner form to the cache.
    """
    if cache_path.exists():
        with open(cache_path) as f:
            data = json.load(f)
        if "provider_schemas" in data:
            return data["provider_schemas"]["registry.terraform.io/hashicorp/aws"]["resource_schemas"]
        return data  # already the inner form

    with tempfile.TemporaryDirectory() as tmpdir:
        Path(tmpdir, "main.tf").write_text(
            'terraform {\n'
            '  required_providers {\n'
            '    aws = {\n'
            '      source  = "hashicorp/aws"\n'
            '      version = "~> 5.0"\n'
            '    }\n'
            '  }\n'
            '}\n'
        )
        subprocess.run(
            ["terraform", "init", "-no-color"], cwd=tmpdir,
            capture_output=True, check=True,
        )
        result = subprocess.run(
            ["terraform", "providers", "schema", "-json"], cwd=tmpdir,
            capture_output=True, check=True,
        )
        full = json.loads(result.stdout)
        schemas = full["provider_schemas"]["registry.terraform.io/hashicorp/aws"]["resource_schemas"]

    cache_path.parent.mkdir(parents=True, exist_ok=True)
    with open(cache_path, "w") as f:
        json.dump(schemas, f)
    return schemas


# ---------------------------------------------------------------------------
# Spec parsing
# ---------------------------------------------------------------------------


def parse_spec_resource_types(spec_path: Path) -> list[str]:
    """Extract the `type = "..."` values from `[[resource]]` blocks in a spec."""
    out: list[str] = []
    in_resource = False
    for line in spec_path.read_text().splitlines():
        s = line.strip()
        if s.startswith("[[resource]]"):
            in_resource = True
            continue
        if s.startswith("[service]"):
            in_resource = False
            continue
        if in_resource and s.startswith("type"):
            m = re.match(r'^type\s*=\s*"([^"]+)"', s)
            if m:
                out.append(m.group(1))
                in_resource = False  # only the first `type` after `[[resource]]`
    return out


# ---------------------------------------------------------------------------
# Prefix derivation
# ---------------------------------------------------------------------------


def common_prefix(strings: list[str]) -> str:
    if not strings:
        return ""
    p = strings[0]
    for s in strings[1:]:
        n = min(len(p), len(s))
        i = 0
        while i < n and p[i] == s[i]:
            i += 1
        p = p[:i]
    return p


def resource_prefix(spec_name: str, handled: list[str]) -> str | list[str]:
    """Resolve the service-level resource prefix.

    Returns either a single string prefix or a list of prefixes for
    services with `PREFIX_OVERRIDES` entries. The single-string form
    falls back to the longest-common-prefix heuristic, capped at two
    underscore-separated segments after `aws_` to avoid the over-narrow
    case where a 1-handled service produces `aws_<word>_<word>_<word>_`.
    """
    if spec_name in PREFIX_OVERRIDES:
        return PREFIX_OVERRIDES[spec_name]
    p = common_prefix(handled)
    if "_" in p:
        p = p[: p.rfind("_") + 1]
    if p.startswith("aws_"):
        rest = p[4:]
        parts = [seg for seg in rest.split("_") if seg]
        if len(parts) >= 2:
            p = "aws_" + "_".join(parts[:2]) + "_"
    return p


def matches_any(name: str, prefixes: list[str]) -> bool:
    """Match a resource name against any of the prefix patterns."""
    for p in prefixes:
        if p.endswith("_"):
            if name.startswith(p):
                return True
        else:
            if name == p or name.startswith(p + "_"):
                return True
    return False


# ---------------------------------------------------------------------------
# Report rendering
# ---------------------------------------------------------------------------


def build_rows(schema_resources: set[str]) -> list[dict]:
    rows = []
    for spec in sorted(SPECS_DIR.glob("*.toml")):
        name = spec.stem
        handled = sorted(set(parse_spec_resource_types(spec)))
        prefix = resource_prefix(name, handled)
        note = ""
        if isinstance(prefix, list):
            candidates = {r for r in schema_resources if matches_any(r, prefix)}
            prefix_display = (
                f"override ({len(prefix)} patterns)" if len(prefix) > 1
                else f"`{prefix[0]}`"
            )
        elif prefix in {"", "aws_"}:
            candidates = set()
            prefix_display = prefix
            note = "heterogeneous prefix; manual review needed"
        else:
            candidates = {r for r in schema_resources if r.startswith(prefix)}
            prefix_display = f"`{prefix}`"
        verified = sorted(set(handled) & schema_resources)
        unknown = sorted(set(handled) - schema_resources)
        missing = sorted(candidates - set(handled))
        denominator = len(candidates) if candidates else len(handled) or 1
        coverage = (len(verified) / denominator) if candidates else (1.0 if handled else 0.0)
        rows.append({
            "service": name,
            "prefix_display": prefix_display,
            "handled": handled,
            "handled_count": len(handled),
            "verified_count": len(verified),
            "schema_count": len(candidates),
            "missing_count": len(missing),
            "coverage": coverage,
            "missing": missing,
            "unknown": unknown,
            "note": note,
        })
    rows.sort(key=lambda r: (-r["missing_count"], r["service"]))
    return rows


def render_report(schema_resources: set[str], rows: list[dict]) -> str:
    out: list[str] = []
    classified = sum(r["schema_count"] for r in rows)
    handled = sum(r["handled_count"] for r in rows)
    unknown = sum(len(r["unknown"]) for r in rows)
    missing = sum(r["missing_count"] for r in rows)

    out.append("# Per-service Terraform resource coverage")
    out.append("")
    out.append(f"Schema resources total: **{len(schema_resources)}**")
    out.append(f"Resources classified to a service via prefix: **{classified}**")
    out.append(f"Currently handled by winterbaume: **{handled}** "
               f"({handled - unknown} verified against schema)")
    out.append(f"Missing within classified prefixes: **{missing}**")
    out.append("")
    out.append("Sorted by missing-count desc.")
    out.append("")
    out.append("| Service | Prefix | Handled | Schema | Missing | Coverage | Note |")
    out.append("|---------|--------|---------|--------|---------|----------|------|")
    for r in rows:
        cov = f"{r['coverage'] * 100:.0f}%" if r["schema_count"] else "—"
        out.append(
            f"| {r['service']} | {r['prefix_display']} | {r['handled_count']} "
            f"| {r['schema_count']} | {r['missing_count']} | {cov} | {r['note']} |"
        )

    out.append("")
    out.append("## Per-service missing resources")
    out.append("")
    for r in rows:
        if not r["missing"]:
            continue
        out.append(f"### {r['service']} — {r['missing_count']} missing")
        out.append("")
        for m in r["missing"]:
            out.append(f"- `{m}`")
        out.append("")

    if unknown:
        out.append("## Resources declared in specs but absent from the AWS provider schema")
        out.append("")
        for r in rows:
            if not r["unknown"]:
                continue
            out.append(f"### {r['service']}")
            out.append("")
            for u in r["unknown"]:
                out.append(f"- `{u}` (typo or removed in schema)")
            out.append("")

    return "\n".join(out) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT,
                        help=f"Output Markdown path (default: {DEFAULT_OUTPUT})")
    parser.add_argument("--cache", type=Path, default=DEFAULT_CACHE,
                        help=f"Schema cache path (default: {DEFAULT_CACHE})")
    args = parser.parse_args()

    schemas = get_terraform_schema(args.cache)
    schema_resources = set(schemas.keys())
    rows = build_rows(schema_resources)
    report = render_report(schema_resources, rows)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(report)
    print(f"wrote {args.output}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
