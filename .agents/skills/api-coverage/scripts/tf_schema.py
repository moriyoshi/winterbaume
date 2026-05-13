"""Shared helpers for fetching and reading the Terraform AWS provider schema.

Both `generate_terraform_resource_coverage.py` and
`generate_terraform_converter_coverage.py` need the flat
`{resource_type: schema_dict}` mapping the AWS provider declares. This
module centralises the fetch-or-cache logic so the two reports stay
consistent and the rare `terraform init && terraform providers schema -json`
round-trip happens in one place.
"""

from __future__ import annotations

import json
import subprocess
import tempfile
from pathlib import Path

AWS_PROVIDER_KEY = "registry.terraform.io/hashicorp/aws"

_MAIN_TF = (
    'terraform {\n'
    '  required_providers {\n'
    '    aws = {\n'
    '      source  = "hashicorp/aws"\n'
    '      version = "~> 5.0"\n'
    '    }\n'
    '  }\n'
    '}\n'
)


def get_terraform_schema(cache_path: Path | None = None) -> dict:
    """Return the AWS provider's flat `{resource_type: schema_dict}` mapping.

    Tries `cache_path` first when supplied. The cache file may be in either
    of two formats: the inner `resource_schemas` dict (preferred, smaller)
    or the full `terraform providers schema -json` envelope. On a cache
    miss runs `terraform init` and `terraform providers schema -json` in a
    temp dir and writes the inner form to the cache.
    """
    if cache_path is not None and cache_path.exists():
        with open(cache_path) as f:
            data = json.load(f)
        if "provider_schemas" in data:
            return data["provider_schemas"][AWS_PROVIDER_KEY]["resource_schemas"]
        return data

    with tempfile.TemporaryDirectory() as tmpdir:
        Path(tmpdir, "main.tf").write_text(_MAIN_TF)
        subprocess.run(
            ["terraform", "init", "-no-color"],
            cwd=tmpdir, capture_output=True, check=True,
        )
        result = subprocess.run(
            ["terraform", "providers", "schema", "-json"],
            cwd=tmpdir, capture_output=True, check=True,
        )
        full = json.loads(result.stdout)
        schemas = full["provider_schemas"][AWS_PROVIDER_KEY]["resource_schemas"]

    if cache_path is not None:
        cache_path.parent.mkdir(parents=True, exist_ok=True)
        with open(cache_path, "w") as f:
            json.dump(schemas, f)

    return schemas


def collect_schema_attributes(schema_block: dict) -> set[str]:
    """Recursively collect attribute names from a Terraform schema block.

    Returns top-level attribute names. For `block_types` (nested blocks),
    returns the block name itself, since converters reference them as
    single keys in Terraform state JSON. The synthetic `id` field is
    always present in state but absent from the schema, so it is dropped.
    """
    attrs: set[str] = set()
    if "attributes" in schema_block:
        attrs.update(schema_block["attributes"].keys())
    if "block_types" in schema_block:
        attrs.update(schema_block["block_types"].keys())
    attrs.discard("id")
    return attrs
