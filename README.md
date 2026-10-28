# World Map Generator OS

A procedural Earth-style world-map and settlement-generation workspace based on the source structure of Watabou's Medieval Fantasy City Generator. The original city generator remains in the Haxe/OpenFL source tree; the new browser-based world-map editor is served as a lightweight static app.

## Open the app

- **GitHub Pages homepage:** once deployment succeeds, use the URL shown in [Settings → Pages](https://github.com/ArkansasIo/TownGeneratorOS/settings/pages).
- **Source page:** [world-map.html](./world-map.html)
- **World map data:** [Natural Earth land geometry](./Assets/world-land.geojson) and [country boundaries](./Assets/world-countries.geojson).

## World map features

- Earth-style continent silhouettes loaded from GeoJSON.
- Pan and zoom controls for the 2D canvas, including wheel zoom and drag navigation.
- Seed field for repeatable procedural settlement generation.
- Generate cities, towns, villages, ports, and fortified settlements.
- Add settlements manually by double-clicking land; select, rename, or delete markers.
- Choose terrain and political-border overlays.
- Export world data to JSON and import a previously exported JSON world.
- Responsive layout for desktop and mobile screens.

The current editor is a prototype: continent outlines use a simplified equirectangular display, and settlement placement uses procedural point generation. It is not a navigable globe, and the settlement system is not yet connected to the original Haxe city-layout engine.

## GitHub Pages deployment

The workflow [Deploy World Map Generator OS to GitHub Pages](https://github.com/ArkansasIo/TownGeneratorOS/actions/workflows/pages.yml) publishes `index.html`, `world-map.html`, and the GeoJSON assets. It runs on relevant pushes to `master` or can be started manually from **Actions → Deploy World Map Generator OS to GitHub Pages → Run workflow**.

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
