# Map and historical source register

This document records candidate sources; it does not mean every source has already been imported into the application.

## Base geography and real roads

### Natural Earth
- Website: https://www.naturalearthdata.com/downloads/
- Use: global land polygons, coastlines, country boundaries and selected physical features.
- Role: lightweight overview geography, not street-level detail.

### OpenStreetMap (OSM)
- Website: https://www.openstreetmap.org/
- Download guidance: https://wiki.openstreetmap.org/wiki/Downloading_data
- Use: contemporary cities, named places, roads and mapped building footprints.
- License: Open Data Commons Open Database License (ODbL); attribution and share-alike obligations may apply to derived databases.
- Integration: use regional extracts or a suitable tile/vector provider. Do not bulk-query the public API or treat standard tile hosting as unlimited free service. Verify attribution, caching, rate limits and provider terms before production.

## Historical maps and historical geography

### British Library / Pelagios: Medieval maps, itineraries and charts
- Collection: https://bl.iro.bl.uk/entities/digitalcollection/5763e79b-bc5a-42f8-8bb3-a7b11baa7a75
- Use: digitized medieval mappae mundi, itineraries and portolan charts, predominantly 14th–15th century.
- Important: these are historical documents, not necessarily geographically accurate modern basemaps. Check each item's rights and metadata before bundling.

### Medieval Black Sea Project
- Reference index: https://medievalblackseaproject.princeton.edu/the-black-sea-in-medieval-maps-2/
- Use: links and catalog information for historical maps, including the Hereford world map, Vesconte maps and Fra Mauro.
- Check rights at the holding institution for each scan.

### Medieval Digital Resources — map bibliography
- Resource: https://mdr-maa.org/resource/medieval-maps-and-mapping-resources/
- Use: research starting point for digitized medieval maps. A bibliography is not a blanket license to republish images.

### Euratlas historical vector data
- Duke Library description: https://library.duke.edu/data/sources/euratlas
- UVA Library description: https://library.virginia.edu/data/datasources/licensed/euratlas
- Use: historical political boundaries, cities, rivers and seas for specific centuries.
- Important: access and reuse are restricted by provider license; do not bundle until permission and redistribution terms are verified.

## Layer policy

1. Current geography: real coastlines and current roads, clearly dated.
2. Historical overlay: georeferenced borders and settlements, with year/period and source.
3. Medieval fantasy layer: generated names, castles, villages, roads and factions, visually distinguished from sourced data.
4. User edits: user-owned records saved separately from source layers.

Each enabled layer should show its source, license, date, and whether features are real, historical or procedurally generated. OSM-derived views must show required attribution and comply with the selected provider's terms.
