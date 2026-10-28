# Planetary Geography and Biologics Data Model

## Scope
This document defines a shared data contract for the procedural map editor, 3D planet viewer, and Rust generation service. It supports many planets; generated profiles are explicitly marked as fictional estimates, not observations.

## API
`POST /api/v1/planets/generate`

Example request:
```json
{
  "seed": 20261028,
  "id": "planet-001",
  "name": "Aurelia",
  "class": "terrestrial"
}
```
Planet classes use snake_case JSON enum values: `terrestrial`, `ocean_world`, `desert`, `ice_world`, `volcanic`, `gas_giant`, `ice_giant`, `dwarf_planet`, `barren_rock`, `artificial_habitat`, and `unknown`.

## Hierarchical geography
- Planet: class, radius, gravity, climate, atmospheric composition, hydrology, seed, provenance.
- Continent/landmass: spherical center, area share, mean elevation, mountain fraction.
- Region: climate, precipitation, elevation band, biome class.
- Feature: mountain range, river basin, lake, glacier, desert, reef, volcano, impact crater.
- Settlement: city, town, village, port, research station, colony, outpost.
- Network: road, trail, rail, bridge, ferry, trade route.
- Site: building, landmark, resource deposit, habitat, ruin, point of interest.
- Biosphere: biological domains, trophic groups, species, ecological niches, evolution history.

## Determinism and persistence
A generated feature should include `id`, `planet_id`, `seed`, `generator_version`, `feature_type`, geometry, attributes, and provenance. Derive per-feature seeds from planet seed plus stable feature IDs. Do not use random values that depend on UI rendering order. Persist user edits as overlays so regeneration does not erase them.

## Rendering
Use latitude/longitude on a sphere as the canonical geographic coordinate system. Convert to a unit vector:
- x = cos(latitude) * cos(longitude)
- y = sin(latitude)
- z = cos(latitude) * sin(longitude)
Angles are converted to radians before evaluating trigonometric functions. Keep map projection code separate from canonical geography so a planet can be rendered as a 2D equirectangular map or as a 3D globe.

## Road and river generation
The current Rust road graph is an early prototype using Euclidean distances. The production algorithm should use a terrain cost raster or navigation mesh:
- discourage steep slopes, cliffs, deep water and unstable terrain;
- favor valleys, existing passes, bridges, ford sites and settlements;
- connect major hubs first, then regional centers, then local settlements;
- use river-basin and drainage graphs so rivers flow downhill toward lakes or seas;
- keep roads and rivers as line geometries with elevation profiles.

## Biological classification
Separate three dimensions:
1. **Biochemistry**: solvent, carbon chemistry assumption, temperature/pressure envelope, available energy.
2. **Ecological complexity**: sterile/prebiotic, microbial, multicellular, complex ecosystems, technological.
3. **Taxonomic/ecological groups**: primary producers, decomposers, consumers, extremophiles, aquatic, aerial, subterranean.

All generated life should be labeled `procedural_estimate`; never imply life has been detected on real exoplanets without a reliable observation source. The present profile generator is a gameplay prototype, not a validated astrobiology model.

## Current limitations
The profile generator currently emits a small set of broad biome summaries and synthetic continent centers. It does not yet produce coastline polygons, terrain elevation grids, connected river geometry, road routing over terrain, detailed ecosystem populations, or authored high-resolution image assets. These require separate generation and rendering passes.
