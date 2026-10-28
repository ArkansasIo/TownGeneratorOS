# World Map Generator OS — Architecture

Project ID: WMGO-001  
Build name: World Map Generator OS  
Build ID: WMGO-WORLD-SIM-0.2.0  
Developer: ArkansasIo  
Status: Prototype architecture; modules are being introduced incrementally.

## Product goal

A continuous, zoomable world atlas combining real geographic outlines, current map features where licensed data is available, historical map layers, procedural medieval settlements, and a Risk-inspired turn-based strategy simulator.

## Coordinate and map model

- Store authoritative features in geographic coordinates: longitude/latitude (WGS84 / EPSG:4326).
- Keep display projection separate from source data. The current canvas prototype uses an equirectangular-style projection; it is not a surveying-grade map.
- Model the world hierarchically: planet → continent → country/realm → region → settlement → street/building detail.
- Geographic features and generated game features must carry provenance metadata: source, license, sourceDate, generated, and seed.
- Never imply that procedural roads or buildings are real streets. Label them as generated.

## Proposed modules

| Module | Responsibility |
|---|---|
| src/map/geometry | Projection, viewport transforms, coastline and polygon operations |
| src/map/layers | Land, rivers, roads, borders, terrain, historical overlays |
| src/map/data-adapters | Normalize GeoJSON and imported datasets |
| src/generation/settlements | Seeded settlement placement, naming, population, economy |
| src/simulation | Deterministic tick engine for weather, population, food, economy and conflict |
| src/strategy/factions | Factions, diplomacy, claims, alliances and AI policy |
| src/strategy/combat | Risk-inspired dice resolution, army movement, occupation and attrition |
| src/storage | Versioned JSON saves; future IndexedDB/server adapter |
| docs | Architecture, data sources, simulation rules and build notes |

The existing Haxe source is preserved. New browser-facing systems are separate JavaScript modules rather than renamed Haxe files that would break the original generator.

## Simulation loop

1. Accept player orders.
2. Validate resources, movement and ownership.
3. Resolve movement and battles.
4. Update weather and terrain modifiers.
5. Apply food production/consumption and population changes.
6. Update faction AI, diplomacy and unrest.
7. Emit an event log and saveable state snapshot.

Simulation time is distinct from wall-clock time. The prototype advances only when explicitly ticked; a future real-time mode can use a configurable timer and must support pause/resume.

## Strategy rules (initial design)

- Continents contain regions; regions contain settlements and can be claimed by factions.
- Factions receive income from controlled settlements and pay army upkeep.
- Movement orders must connect to adjacent regions; sea movement requires ports and transport rules.
- Battles use Risk-inspired dice comparisons, with the defender winning ties. Army size limits dice; terrain, fortification, weather and supply can modify results.
- Conquest changes control only after battle resolution. Occupation can cause unrest and supply costs.
- AI uses a bounded decision budget per tick and logs its selected order.

## Data and quality requirements

- Validate imported JSON and GeoJSON before rendering or simulation.
- Preserve source attribution and license requirements with each layer.
- Test deterministic seeds, resource accounting, ownership validity, battle outcomes and malformed saves.
- Do not download an entire global street dataset into the browser. Load detail by region/zoom or use a suitable vector/tile provider.
- Do not use a historical scan as a modern street basemap without georeferencing and clear date/coverage metadata.
