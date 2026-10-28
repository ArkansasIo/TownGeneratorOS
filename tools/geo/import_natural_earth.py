#!/usr/bin/env python3
"""Download and convert selected Natural Earth 1:10m layers to GeoJSON.

Requirements: Python 3.10+ and GDAL/ogr2ogr on PATH.
Example: python tools/geo/import_natural_earth.py --output-dir ./generated-data/natural-earth
Raw downloads and converted output are intentionally kept outside Git by default.
"""
from __future__ import annotations
import argparse
import json
import shutil
import subprocess
import urllib.request
import zipfile
from pathlib import Path

BASE = "https://naturalearth.s3.amazonaws.com/10m_{group}/ne_10m_{layer}.zip"
LAYERS = [
    ("physical", "land", "land"),
    ("physical", "coastline", "coastline"),
    ("physical", "lakes", "lakes"),
    ("physical", "rivers_lake_centerlines", "rivers"),
    ("cultural", "populated_places", "populated_places"),
    ("cultural", "roads", "roads"),
    ("cultural", "admin_0_countries", "countries"),
]

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--only", nargs="*", choices=[item[2] for item in LAYERS])
    args = parser.parse_args()
    ogr = shutil.which("ogr2ogr")
    if not ogr:
        raise SystemExit("GDAL ogr2ogr is required. Install GDAL, then retry.")
    out = args.output_dir.resolve()
    raw = out / "raw"
    raw.mkdir(parents=True, exist_ok=True)
    selected = [item for item in LAYERS if not args.only or item[2] in args.only]
    manifest = {
        "schema_version": "map-dataset-manifest.v1",
        "source": "Natural Earth",
        "source_home": "https://www.naturalearthdata.com/downloads/",
        "scale": "1:10m",
        "license": "Public domain; retain dataset attribution and verify current terms",
        "status": "downloaded_by_this_run",
        "layers": []
    }
    for group, layer, name in selected:
        url = BASE.format(group=group, layer=layer)
        archive = raw / f"ne_10m_{layer}.zip"
        print(f"Downloading {name}: {url}")
        request = urllib.request.Request(url, headers={"User-Agent": "TownGeneratorOS-GeoImporter/1.0"})
        with urllib.request.urlopen(request, timeout=90) as response, archive.open("wb") as target:
            shutil.copyfileobj(response, target)
        extract = raw / f"ne_10m_{layer}"
        extract.mkdir(parents=True, exist_ok=True)
        with zipfile.ZipFile(archive) as zf:
            zf.extractall(extract)
        shapefiles = list(extract.glob("*.shp"))
        if len(shapefiles) != 1:
            raise SystemExit(f"Expected one shapefile in {archive.name}, found {len(shapefiles)}")
        destination = out / f"{name}.geojson"
        subprocess.run([ogr, "-f", "GeoJSON", str(destination), str(shapefiles[0]), "-t_srs", "EPSG:4326"], check=True)
        manifest["layers"].append({
            "id": f"natural-earth-10m-{name}",
            "name": name,
            "source_url": url,
            "download_archive": str(archive.relative_to(out)),
            "output": destination.name,
            "crs": "EPSG:4326",
            "provenance": "measured_or_compiled_earth_geography",
            "sha256": __import__("hashlib").sha256(destination.read_bytes()).hexdigest()
        })
    (out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(f"Converted {len(manifest['layers'])} layers to {out}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
