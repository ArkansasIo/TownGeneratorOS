# TownGeneratorOS: Procedural World and Universe Engine

## Project scope
TownGeneratorOS combines a browser-based 2D settlement/map editor, a Three.js 3D galaxy explorer, and a Rust service intended to become the authoritative simulation backend.

## Current capabilities
- Procedural 3D star systems, planets, moons, seeded planet textures, and a browser-side planetary-life prototype.
- Turn-based strategy controls in the galaxy interface.
- Medieval map and coastline/country GeoJSON assets.
- Rust/Axum API scaffold and MySQL schema.
- Rust Newtonian two-body orbit and spherical-body surface-gravity calculations.

## Architecture
- `galaxy.html`: visual 3D client; should not be treated as the authoritative state store.
- `world-map.html` / `index.html`: 2D map editor.
- `Assets/`: static map data and visual assets.
- `server/src/`: Rust API and scientific calculations.
- `server/migrations/`: MySQL schema migrations.
- `docs/`: design, data contracts, simulation and operations documentation.

## Simulation data policy
Every world/system object must carry an origin marker:
- `catalog`: measured/observed astronomical data.
- `procedural`: seeded generated content.
- `user`: user-created or edited content.

Never label procedural values as observed scientific facts. Store units explicitly; document reference epoch, source URL/catalog identifier, uncertainty, and retrieval date for imported observations. All physics APIs should validate finite inputs and return documented SI units.

## Road and settlement generation
The detailed road-network generator is a design target, not yet a complete feature. It should use hierarchical graph generation:
1. Continental routes connect trade centers, river crossings, mountain passes, ports, and regional capitals.
2. Regional roads connect villages, farms, mills, mines, castles, monasteries, and market towns.
3. Town streets form walkable graphs that follow terrain, walls, gates, market squares, and landmarks.
4. Local paths connect buildings, fields, docks, and points of interest.

Generate from a world seed plus a versioned algorithm identifier so the same seed can be regenerated. Persist edits separately from generated baselines.

## 3D graphics and assets
Use procedural meshes/materials for terrain and roads, instanced vegetation and props, level-of-detail, texture atlases, and compressed texture formats where supported. Keep a clear separation between code-generated geometry and authored texture/model assets. Include licenses and attribution with all third-party assets. A high-resolution texture alone does not guarantee high geometric detail; use multiresolution terrain and LOD.

## Security and deployment requirements
- Use environment variables for secrets; never commit production credentials.
- Replace permissive production CORS with an explicit allowlist.
- Add authentication, authorization, rate limiting, request-size limits, structured validation, and audit logs before public multiplayer deployment.
- GitHub Pages can host the static frontend; it cannot run the Rust service or MySQL database.
- A passing Pages workflow verifies static publication only; it does not verify Rust compilation or live backend connectivity.

## Validation status
The existing Pages workflow has passed for recent frontend commits. The newly added Rust physics module and routes still require a Rust build/test run and integration testing before being described as verified.
