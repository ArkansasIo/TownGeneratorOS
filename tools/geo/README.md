# Geospatial tools

These tools support the planetary and world-map data pipeline. They do not claim that source catalogs have already been downloaded or validated.

## Dependencies

- Python 3.10+ for the validator and tests.
- GDAL command-line tools (ogr2ogr, gdalinfo, gdal_translate) for imports.

## Validate vectors

    python tools/geo/validate_geojson.py Assets/world-land.geojson
    python -m unittest tools/geo/test_validate_geojson.py

The validator checks GeoJSON structure and coordinate ranges. It is not a substitute for topology, CRS or source-license review.

## Import Natural Earth vectors

    python tools/geo/import_natural_earth.py --output-dir generated-data/natural-earth

Requires network access and GDAL. Outputs are outside the repository by default. Inspect the generated manifest and validate each output before publishing.

## Stage a planetary raster

    python tools/geo/import_planetary_raster.py path/to/source.tif \
      --body-id sol-mars --layer-id mars_elevation \
      --source-name "Publisher/product name" --source-url "https://official-catalog.example/" \
      --product-id "exact-product-id" --product-version "exact-version" \
      --license "verified terms" --units metres

Use actual official product metadata. The helper writes a GeoTIFF and JSON manifest under generated-data/planetary. It stages a raster; it does not reproject, tile, or scientifically validate it.

## Output policy

Generated datasets are ignored by Git. Keep large source rasters outside source control unless repository size and license review explicitly approve them. See docs/PLANETARY_DATA_PROVENANCE_SCHEMA.md.
