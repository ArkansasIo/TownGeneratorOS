# Medieval Fantasy City Generator

This repository is a fork of the source code for [Medieval Fantasy City Generator by Watabou](https://watabou.itch.io/medieval-fantasy-city-generator/).

## GitHub Pages (HTML5)

A GitHub Actions workflow builds this Haxe/OpenFL project as an HTML5 release and deploys it to GitHub Pages when code is pushed to `master`. You can also start the workflow manually from **Actions → Build and deploy TownGeneratorOS to GitHub Pages → Run workflow**.

### First-time Pages setup

1. Open [Settings → Pages](https://github.com/ArkansasIo/TownGeneratorOS/settings/pages).
2. Under **Build and deployment**, select **GitHub Actions** as the source.
3. Open [Actions](https://github.com/ArkansasIo/TownGeneratorOS/actions/workflows/pages.yml) and run **Build and deploy TownGeneratorOS to GitHub Pages**, or push a commit to `master`.
4. Wait for the build and deploy jobs to succeed. The published URL appears in the deployment job and in **Settings → Pages**.

The workflow builds `project.xml` using Haxe, Lime, OpenFL, and msignal, then publishes `Export/html5/bin`. The first run is the compatibility check; if the legacy source or dependencies fail to compile, the Actions log will show the failing step.

## Build locally

Install Haxe and Haxelib, then run:

```sh
haxelib install lime 7.3.0
haxelib install openfl 8.9.0
haxelib install msignal 1.2.5
haxelib run lime build html5 -release
```

The source currently lacks some later upstream features, including waterbodies and the options UI.

## Credits and licensing

This project is based on Watabou's Medieval Fantasy City Generator. Review [LICENSE](LICENSE) before redistributing or publishing modified builds, and retain applicable attribution.
