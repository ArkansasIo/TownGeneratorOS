#!/usr/bin/env python3
"""Validate GeoJSON coordinates and emit a compact ingestion report.

This intentionally validates geometry structure and coordinate ranges only. It does
not reproject coordinates, infer licenses, or claim that a dataset is scientifically
valid. Run: python tools/geo/validate_geojson.py Assets/world-land.geojson
"""
from __future__ import annotations
import argparse
import json
import math
from pathlib import Path
from typing import Any

def coordinates_in_geometry(geometry: dict[str, Any]):
    kind = geometry.get("type")
    coords = geometry.get("coordinates")
    if kind == "GeometryCollection":
        for item in geometry.get("geometries", []):
            yield from coordinates_in_geometry(item)
        return
    def walk(node):
        if isinstance(node, (list, tuple)):
            if len(node) >= 2 and isinstance(node[0], (int, float)) and isinstance(node[1], (int, float)):
                yield node
            else:
                for child in node:
                    yield from walk(child)
    if kind and coords is not None:
        yield from walk(coords)

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=Path)
    parser.add_argument("--max-invalid", type=int, default=0)
    args = parser.parse_args()
    try:
        data = json.loads(args.file.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(json.dumps({"ok": False, "error": str(exc)}))
        return 2
    if data.get("type") != "FeatureCollection" or not isinstance(data.get("features"), list):
        print(json.dumps({"ok": False, "error": "Expected GeoJSON FeatureCollection with features array"}))
        return 2
    count = 0
    invalid = []
    min_lon = min_lat = math.inf
    max_lon = max_lat = -math.inf
    for fi, feature in enumerate(data["features"]):
        geometry = feature.get("geometry") or {}
        for coord in coordinates_in_geometry(geometry):
            count += 1
            lon, lat = float(coord[0]), float(coord[1])
            if not math.isfinite(lon) or not math.isfinite(lat) or not -180 <= lon <= 180 or not -90 <= lat <= 90:
                if len(invalid) < args.max_invalid + 1:
                    invalid.append({"feature": fi, "coordinate": coord[:2]})
                continue
            min_lon, max_lon = min(min_lon, lon), max(max_lon, lon)
            min_lat, max_lat = min(min_lat, lat), max(max_lat, lat)
    report = {
        "ok": not invalid,
        "file": str(args.file),
        "feature_count": len(data["features"]),
        "coordinate_count": count,
        "bounds_lon_lat": None if count == 0 else [min_lon, min_lat, max_lon, max_lat],
        "invalid_coordinate_examples": invalid[:args.max_invalid],
        "note": "Structural check only; verify CRS, source, epoch, units, geometry validity and license separately."
    }
    print(json.dumps(report, indent=2))
    return 0 if report["ok"] else 1

if __name__ == "__main__":
    raise SystemExit(main())
