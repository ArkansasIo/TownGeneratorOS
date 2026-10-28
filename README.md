# World Map Generator OS

A procedural Earth-style world-map and settlement-generation workspace based on the source structure of Watabou's Medieval Fantasy City Generator. The original city generator remains in the Haxe/OpenFL source tree; the new browser-based world-map editor is served as a lightweight static app.

## Open the app

- **GitHub Pages homepage:** once deployment succeeds, use the URL shown in [Settings → Pages](https://github.com/ArkansasIo/TownGeneratorOS/settings/pages).
- **Source page:** [world-map.html](./world-map.html)
- **3D procedural galaxy:** [galaxy.html](./galaxy.html) — four-quadrant galaxy, seeded star systems, orbiting planets, and spherical planet inspection.\n- **Planetary atlas:** [planetary-atlas.html](./planetary-atlas.html) — body catalog, source provenance, ingestion status, and API-backed procedural surface previews.
- **World map data:** [Natural Earth land geometry](./Assets/world-land.geojson) and [country boundaries](./Assets/world-countries.geojson).

## World map features

- Earth-style continent silhouettes loaded from GeoJSON.
- Pan and zoom controls for the 2D canvas, including wheel zoom and drag navigation.
- 3D procedural galaxy preview with quadrant navigation, seeded star systems, orbiting planets, and spherical planet views.
- Seed field for repeatable procedural settlement generation.
- Generate cities, towns, villages, ports, and fortified settlements.
- Add settlements manually by double-clicking land; select, rename, or delete markers.
- Choose terrain and political-border overlays.
- Export world data to JSON and import a previously exported JSON world.
- Responsive layout for desktop and mobile screens.

The current editor is a prototype: continent outlines use a simplified equirectangular display, and settlement placement uses procedural point generation. It is not a navigable globe, and the settlement system is not yet connected to the original Haxe city-layout engine.

## GitHub Pages deployment

The workflow [Deploy World Map Generator OS to GitHub Pages](https://github.com/ArkansasIo/TownGeneratorOS/actions/workflows/pages.yml) publishes `index.html`, `world-map.html`, `galaxy.html`, `planetary-atlas.html`, and the GeoJSON assets. It runs on relevant pushes to `master` or can be started manually from **Actions → Deploy World Map Generator OS to GitHub Pages → Run workflow**.

If this is the first deployment, open [Settings → Pages](https://github.com/ArkansasIo/TownGeneratorOS/settings/pages) and choose **GitHub Actions** as the source. The deployment is not considered live until the workflow's deploy job succeeds.

## Build the original Haxe city generator

Install Haxe and Haxelib, then run:

```sh
haxelib install lime 7.3.0
haxelib install openfl 8.9.0
haxelib install msignal 1.2.5
haxelib run lime build html5 -release
```

## Attribution and licensing

This repository is a fork based on [Watabou's Medieval Fantasy City Generator](https://watabou.itch.io/medieval-fantasy-city-generator/). Natural Earth geometry is included for geographic outlines; consult the dataset's terms and the repository [LICENSE](./LICENSE) before redistributing. Preserve applicable upstream attribution.


## Universe backend prototype

The `server/` folder contains an initial Rust/Axum service scaffold and MySQL schema for universe metadata, star systems, celestial bodies, settlements, players, sessions, and events. See [Multiplayer Architecture](./docs/MULTIPLAYER_ARCHITECTURE.md). The WebSocket is currently a transport prototype only; authenticated multiplayer gameplay is not yet implemented. GitHub Pages hosts the static client, not the Rust/MySQL service.


## Planetary geology and procedural surface pipeline

- [Planetary data and map pipeline](./docs/PLANETARY_DATA_AND_MAP_PIPELINE.md) — source catalogue, provenance, zoom/LOD, historical roads and body-specific geology.
- [Object inspector and detail menus](./docs/OBJECT_INSPECTOR_AND_DETAIL_MENUS.md) — detailed selection/right-click menu design for planets, settlements, roads, creatures and strategy objects.
- [Planetary data source catalogue](./Assets/metadata/planetary-data-sources.json) — Natural Earth, OpenStreetMap, USGS Astrogeology and NASA PDS links with ingestion status.
- [Engine status and roadmap](./docs/ENGINE_STATUS_AND_ROADMAP.md) — capabilities, limitations and deployment/security requirements.

### Rust API prototypes

- `POST /api/v1/planets/generate` — seeded planet profile, class, atmosphere, hydrology, biosphere, continents and biomes.
- `POST /api/v1/planets/surface-tile` — bounded deterministic terrain/climate/biome samples for a procedural tile.\n- `GET /api/v1/planets/catalog` — body-specific data source catalog.\n- `GET /api/v1/planets/sources` — catalog of available data source families.\n- `GET /api/v1/planets/data-status` — explicit ingested-versus-catalogued status.
- `POST /api/v1/world/roads` — deterministic road graph from supplied settlement sites.
- `POST /api/v1/physics/orbit` and `POST /api/v1/physics/surface-gravity` — simplified scientific calculations using SI units.

Example surface request:

```json
{
  "seed": 12345,
  "width": 128,
  "height": 64,
  "class": "terrestrial"
}
```

These endpoints are prototypes. Procedural surface output is not measured geology; real planetary map products have been catalogued but still require dataset-specific ingestion, attribution, validation and tiling. See [the provenance schema](./docs/PLANETARY_DATA_PROVENANCE_SCHEMA.md) and [geospatial tools](./tools/geo/README.md). Raster staging is supported by `tools/geo/import_planetary_raster.py`; it requires GDAL and does not itself certify scientific validity. Earth coastline data is now restricted to the configured homeworld in the 3D client so it is not reused as the geography of every generated planet.

### Backend checks

Run locally with Rust installed:

```sh
cd server
cargo fmt --check
cargo test
cargo check
```

The API currently requires a configured MySQL database at runtime. The Rust API and database are not hosted by GitHub Pages.
