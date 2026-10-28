# Planetary map provenance and ingestion contract

A source catalog is not an ingested dataset, and an ingested raster is not automatically validated. Every map layer must identify the celestial body it belongs to.

## Required metadata

| Field | Meaning |
|---|---|
| body_id | Stable identity such as sol-earth, sol-mars, or sol-moon. |
| layer_id | Stable ID unique within the body. |
| source_name, source_url | Publisher and official product/catalog URL. |
| product_id, product_version | Exact mission product, release, or dataset version. |
| license | License or usage terms copied from source metadata. |
| coordinate_reference_system | Original projection and datum; do not assume latitude/longitude. |
| resolution_m | Ground sampling distance when known; otherwise null. |
| units, no_data_value | Measurement units and sentinel value. |
| ingestion_status | catalogued, downloaded, staged_needs_domain_validation, validated, or rejected. |
| sha256 | Checksum of the staged source artifact. |

## Rules

1. Earth data may only be rendered on Earth. Mars, the Moon, Venus, Mercury, Ceres and Titan require body-specific products.
2. Procedural layers use provenance procedural_estimate; generated values must never be labelled measured.
3. Preserve original source files and metadata; normalized products are derivatives.
4. Record product version, CRS, resolution, units, no-data handling, body ID and checksum before publishing tiles.
5. Validate vector coordinate bounds. For rasters, validate CRS, dimensions, pixel resolution, no-data values, numeric range, body identity and known-location spot checks.
6. Do not commit large source rasters to Git. Store them externally and commit small manifests and scripts.
7. Review dataset-specific terms before redistribution. OpenStreetMap requires ODbL attribution; individual scientific products may have additional caveats.

## Tooling

- tools/geo/validate_geojson.py: vector coordinate sanity checks.
- tools/geo/import_natural_earth.py: Natural Earth vector import; requires GDAL/ogr2ogr.
- tools/geo/import_planetary_raster.py: metadata-first raster staging; requires GDAL.
- Assets/metadata/planetary-body-catalog.json: discovery catalog, not proof of ingestion.
- POST /api/v1/planets/catalog: source body catalog API.
- GET /api/v1/planets/data-status: explicit ingestion/fallback status.

## Staging example

    python tools/geo/import_planetary_raster.py downloaded_mola.tif \
      --body-id sol-mars --layer-id mars_mola_elevation \
      --source-name "NASA MOLA gridded topography" \
      --source-url "https://pds.nasa.gov/" \
      --product-id "REPLACE_WITH_EXACT_PRODUCT_ID" \
      --product-version "REPLACE_WITH_RELEASE" \
      --license "REVIEW_PRODUCT_TERMS" --units metres

The example values are placeholders. Use exact product metadata from the selected source.
