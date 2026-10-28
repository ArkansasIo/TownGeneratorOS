# Planetary Data Ingestion and Map Pipeline

## Purpose

Build one map stack that supports a real Earth basemap, medieval-inspired historical/fantasy overlays, and separate maps for every procedural or scientifically mapped planet. The renderer must distinguish measured source data from generated content.

## Verified source catalogue

- **Natural Earth** — global coastlines, land, rivers, lakes, cities, countries and selected roads. The 1:10m vector collection is the most detailed of its three global scales, but it is still not street-level data. Source: https://www.naturalearthdata.com/downloads/10m-physical-vectors/ and https://www.naturalearthdata.com/downloads/10m-cultural-vectors/
- **OpenStreetMap** — modern detailed roads, buildings, waterways, land use and settlement geometry. Observe ODbL attribution and database obligations. Source: https://www.openstreetmap.org/
- **USGS Astrogeology** — peer-reviewed planetary geologic maps, feature maps, topography and remote-sensing products for specific mapped bodies. Source: https://www.usgs.gov/special-topics/planetary-geologic-mapping
- **NASA Planetary Data System** — mission-derived planetary image, cartography and geoscience products. Select datasets by body and instrument, then retain product metadata. Source: https://pds.nasa.gov/ and https://planetary.data.nasa.gov/find-data

The source catalogue lives at `Assets/metadata/planetary-data-sources.json`. Entries are explicitly marked `source_catalogued_not_yet_ingested`: cataloguing a source is not the same as importing or displaying its data.

## Map layer stack

1. **Base physical geography**: coastlines, land/water mask, bathymetry and elevation.
2. **Geology**: lithology, geologic age, faults, volcanic provinces, impact craters and tectonic boundaries where supported by a body-specific dataset.
3. **Hydrology**: rivers, lakes, watersheds, springs and floodplains on Earth-like worlds; planetary channels/valleys only where the source supports them.
4. **Climate and biomes**: temperature, precipitation, prevailing winds, snow/ice, aridity and elevation bands.
5. **Human or fictional geography**: countries, kingdoms, duchies, borders, settlements, roads, ports, castles, farms and trade routes.
6. **Gameplay simulation overlays**: population, resources, armies, hazards, weather, ownership and turn status.
7. **Labels and detail layers**: ocean names, mountain ranges, regions, city names, roads and points of interest.

Each layer needs its own visibility toggle, opacity, source/provenance, scale range, and legend.

## Zoom/LOD strategy

Use a hierarchical spatial index and vector tiles. Do not regenerate map topology on every zoom:
- Universe / planet view: coarse raster or vector overview.
- Continent view: coastlines, mountain systems, major rivers, borders and regional capitals.
- Region view: towns, villages, road corridors, watersheds and terrain shading.
- City view: street graph, walls, districts, markets, buildings and local landmarks.
- Street/building view: building footprints and generated/authored 3D assets.

A world seed and generator-version ID must make procedural features reproducible. Save user edits as overrides instead of silently rewriting them when the generator changes.

## Medieval road and settlement generation

Roads are a graph, not just painted lines. The current Rust road module builds a deterministic Euclidean connectivity backbone from supplied sites. It does **not yet** account for terrain or hydrology. The next road-routing pass should:
- rasterize or sample elevation and slope;
- assign high traversal cost to steep terrain, lakes, sea, glaciers and impassable cliffs;
- use bridges/fords for river crossings and passes through mountain ranges;
- lower cost along valleys and historical corridors;
- connect ports, capital cities, castles, market towns, villages, farms, mines and monasteries;
- build local street networks around terrain-aware town centers, walls, gates, squares and docks;
- label each edge with road class, travel time, surface, seasonality, toll, ownership and maintenance.

Historical accuracy requires historical source data. Modern road data can be a starting reference, but must not be described as a medieval road network. If no historical route source exists, mark it as a generated reconstruction.

## 3D planet map projection

For a globe, convert longitude/latitude to a sphere using a consistent coordinate convention and wrap longitude at ±180°. Apply elevation to a multiresolution displacement mesh or terrain tiles; apply coastlines and geology as surface textures/vector overlays. Keep map coordinates in geographic space and transform to world coordinates only in the rendering layer.

**Earth-specific data must only be applied to Earth.** The current homeworld can use the existing Earth-like GeoJSON as a base if its identity is explicitly configured as Earth. Other planets must use their own catalogued USGS/NASA datasets when available; otherwise use seeded procedural terrain and label it as fictional. Never drape Earth's real continents onto Mars or a randomly generated exoplanet.

## API shape

The Rust endpoint `POST /api/v1/planets/surface-tile` accepts:
```json
{
  "seed": 12345,
  "width": 128,
  "height": 64,
  "class": "terrestrial"
}
```

It returns latitude/longitude samples with generated elevation, temperature, precipitation and biome. This is a deterministic prototype sampler, not a substitute for measured DEM or geological rasters. Dimensions are bounded to control server cost.

Planet profile endpoint: `POST /api/v1/planets/generate`. Road graph endpoint: `POST /api/v1/world/roads`. Orbit/gravity endpoints use SI units and simplified Newtonian/Keplerian models.

## Ingestion checklist

- Download only from the official source/catalog page.
- Record dataset/product ID, version, body, instrument, CRS/projection, datum, resolution, units, no-data value, timestamp/epoch, attribution and license.
- Validate geometry, coordinate ranges, bounds, projection and missing-value handling.
- Convert to a documented internal CRS (retain original coordinates and metadata).
- Generate multiresolution tiles and checksums.
- Store raw source products separately from normalized data and generated overlays.
- Add a provenance label to every rendered layer and API response.
- Add golden tests for known locations and stable-seed tests for generated maps.
- Do not ship large raw datasets in Git unless their size/license and repository policy justify it; prefer a reproducible download/import script and cached build artifacts.

## Known gaps

- Real planetary products are catalogued but not yet imported by an automated ingestion pipeline.
- High-resolution terrain, city buildings and historical medieval routes are not complete merely because a texture or road graph exists.
- Surface sampling is procedural and should be exposed as such in UI and API metadata.
- Production needs tile caching, authentication for edits, rate limiting, and a production CORS allowlist.


## Reproducible Earth-data import

The repository includes `tools/geo/import_natural_earth.py`, which downloads selected Natural Earth 1:10m archives, converts them to GeoJSON with GDAL's `ogr2ogr`, and writes a per-layer manifest with checksums.

```sh
python tools/geo/import_natural_earth.py --output-dir ./generated-data/natural-earth
python tools/geo/validate_geojson.py ./generated-data/natural-earth/land.geojson
```

Install GDAL/ogr2ogr first. Downloaded archives and generated datasets are excluded from Git to avoid committing large derived files. Review the generated manifest and the current source terms before distributing the output. This importer covers Earth layers only; planetary body products need a separate body-aware importer because formats, map projections, resolutions and licenses differ by product.
