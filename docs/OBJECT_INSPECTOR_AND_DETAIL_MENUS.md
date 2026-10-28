# Object Detail Menu and Inspector System

## Interaction contract

- **Left click / tap:** select an object and open a compact inspector.
- **Right click / long press:** open a contextual action menu without navigating away from the current view.
- **Keyboard:** Escape closes menus; Enter opens the selected object's primary detail panel.
- **Home button:** return to the configured home planet and restore its saved camera/zoom state.
- Selection is shared across map, globe, galaxy, simulation and turn-based strategy views.

## Inspector layout

### Identity panel
Stable ID, display name, object type/subtype, parent object, generation seed, generator version, creation date and provenance badge: measured, derived, procedural, user-authored, or mixed.

### Geography panel
Latitude/longitude, elevation, terrain, biome, climate, geological unit, nearby water, neighboring settlements, route connections, and source dataset. Only show values supported by the selected layer; display unknown rather than fabricating scientific data.

### Planet and star panels
- Star: spectral classification, catalog ID, distance, apparent/absolute magnitude, coordinates, source and epoch.
- Planet: class/subclass, radius, mass, gravity, orbit, atmosphere, temperature, hydrology, geology, moons, surface layer coverage and provenance.
- Surface: tile coordinates, elevation, slope, temperature, precipitation, biome, mineral/resource deposits and source resolution.
- Biosphere: taxonomy, population estimates, habitats, food web, migration, health, adaptation and extinction history. Fictional biosphere values must be visibly marked simulated.

### Settlement panel
Population, growth rate, households, jobs, food/water, wealth, resources, fortifications, buildings, faction, government, culture, trade links, defense, unrest, diseases/hazards and last simulation turn.

### Road / route panel
Road class, surface, length, travel time, seasonal closures, toll, traffic, trade capacity, risk, owner, endpoints, path geometry and whether it came from a historical source or procedural generation.

### Strategy panel
Owner, alliance, diplomacy, army strength, garrison, resources, supply, control percentage, fortifications, actions available, last combat and turn history.

### Creature / life panel
Species ID, taxonomy, biome range, population, movement mode, speed, needs, diet, behavior profile, intelligence/AI state, faction/guild, health, threats, reproduction, migration route and observed/simulated status.

## Menu behavior

Use a responsive right-side drawer on desktop and a bottom sheet on narrow/mobile screens. Keep a compact summary visible while detailed sections expand/collapse. Use tabs or accordions for Identity, Physical, Ecology, Population, Economy, Politics, Military, History and Debug/Provenance. Do not make the inspector wider than the viewport. Respect safe areas and support mouse, touch and keyboard input.

## Data model

Every selectable object uses a stable entity_id, entity_type, parent_id, world_id, source/provenance, seed (if generated), and a typed details payload. Use server IDs rather than deriving IDs from display names. Avoid embedding huge geometry or image blobs in inspector JSON; use map-layer/tile URLs and pagination.

## Implementation status

This is the intended interaction and field contract. The current browser prototype has basic selection and a few HTML details, not the full responsive inspector described here. Build this incrementally and connect each panel to API data rather than filling every field with placeholder numbers.
