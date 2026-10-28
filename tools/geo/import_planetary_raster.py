#!/usr/bin/env python3
"""Inspect and stage planetary rasters with explicit provenance.

Requires GDAL command-line tools. Does not download data or claim scientific validation.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import shutil
import subprocess
from datetime import datetime, timezone
from pathlib import Path


def run_json(command: list[str]) -> dict:
    result = subprocess.run(command, check=True, capture_output=True, text=True)
    return json.loads(result.stdout)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path, help="Downloaded source raster readable by GDAL")
    parser.add_argument("--body-id", required=True)
    parser.add_argument("--layer-id", required=True)
    parser.add_argument("--source-name", required=True)
    parser.add_argument("--source-url", required=True)
    parser.add_argument("--product-id", required=True)
    parser.add_argument("--product-version", default="unspecified")
    parser.add_argument("--license", required=True, dest="license_name")
    parser.add_argument("--units", required=True)
    parser.add_argument("--output-dir", type=Path, default=Path("generated-data/planetary"))
    parser.add_argument("--allow-unknown-crs", action="store_true")
    args = parser.parse_args()
    if not args.source.is_file():
        parser.error(f"source file does not exist: {args.source}")
    if not shutil.which("gdalinfo") or not shutil.which("gdal_translate"):
        parser.error("GDAL is required: install gdalinfo and gdal_translate first")
    info = run_json(["gdalinfo", "-json", str(args.source)])
    crs = info.get("coordinateSystem", {}).get("wkt")
    if not crs and not args.allow_unknown_crs:
        parser.error("source has no declared CRS; verify product documentation before allowing unknown CRS")
    args.output_dir.mkdir(parents=True, exist_ok=True)
    safe_name = "".join(ch if ch.isalnum() or ch in "-_" else "_" for ch in args.layer_id)
    output = args.output_dir / f"{safe_name}.tif"
    subprocess.run(["gdal_translate", "-of", "GTiff", "-co", "TILED=YES", "-co", "COMPRESS=DEFLATE", str(args.source), str(output)], check=True)
    manifest = {
        "schema_version": "planetary-raster-manifest.v1",
        "body_id": args.body_id, "layer_id": args.layer_id,
        "source_name": args.source_name, "source_url": args.source_url,
        "product_id": args.product_id, "product_version": args.product_version,
        "license": args.license_name, "units": args.units,
        "source_file": str(args.source), "source_sha256": sha256(args.source),
        "staged_file": str(output), "staged_sha256": sha256(output),
        "coordinate_reference_system_wkt": crs,
        "geo_transform": info.get("geoTransform"), "size_pixels": info.get("size"),
        "bands": [{"type": b.get("type"), "description": b.get("description"), "no_data_value": b.get("noDataValue")} for b in info.get("bands", [])],
        "ingestion_status": "staged_needs_domain_validation",
        "ingested_at_utc": datetime.now(timezone.utc).isoformat(),
        "validation_required": ["body identity", "projection and datum", "units", "no-data values", "pixel resolution", "source license", "known landmark spot checks"]
    }
    manifest_path = args.output_dir / f"{safe_name}.manifest.json"
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(f"Staged raster: {output}\nManifest: {manifest_path}\nStatus: staged_needs_domain_validation")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
