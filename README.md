![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/zstorm999/layerloom/rust.yml)

# LayerLoom

LayerLoom is an iteration of my previous work of summer 2022 [tilemap editor](https://github.com/Zstorm999/tilemap_editor). 

LayerLoom allows you to create easily via a simple and intuitive GUI a layer and entity map for 2D games such that can be found in the GBA or SNES era (Zelda-style). 

It consists of an editor to easily edit the different layers of the map, and an exporter/library to import the map directly into your rust project (that can directly be used in the `build.rs` file).

## How to install

For now the project is still in a very early stage, so there is no official release. If you wish to try it anyway, feel free to clone the repo and build it with cargo. It should build fine on windows and linux (support for macOS is not planned but as I rely on libraries that do support it that should be fine).

## Roadmap 

- [X] general app layout and ui
- [ ] single layer drawing and sprites import
- [ ] multiple layers
- [ ] entity layers
- [ ] library for easy import into your Rust project

This is the baseline at the start of the project. Other features I would like to add include :
- ability to generate entities and tile types directly from the UI and export them to your project easily (direct generation of rust files)
- ready to use templates for GBA, SNES, NES…

These additional features are not part of my objectives, and might never come to life depending on my motivation 😅.

## Why not keeping the old project ?

The old [tilemap editor](https://github.com/Zstorm999/tilemap_editor) project is still functional, but I found several problems with it :

- the code is not very clean
- the architecture is a bit messy and not very well separated (view/model)
- I find the `Iced` library a bit limited at the moment

To expand on the last point, `Iced` is a fantastic library, and I definitely intend to use it on other projects. However at this time, I find it not mature enough to support this project, mostly because of its lack of widgets (I miss popup windows), and the styling mechanic is a bit off for me at the moment.    
I would also like to try other libraries, I tried `Druid` but it just didn’t click for me. So we’re going for `egui` with `eframe` instead !
