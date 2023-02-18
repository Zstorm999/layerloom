# LayerLoom

LayerLoom is an iteration of my previous work of summer 2022 [tilemap editor](https://github.com/Zstorm999/tilemap_editor). 

LayerLoom allows you to create easily via a simple and intuitive GUI a layer and entity map for 2D games such that can be found in the GBA or SNES era (Zelda-style). 

It consists of an editor to easily edit the different layers of the map, and an exporter/library to import the map directly into your rust project (that can directly be used in the `build.rs` file).

## Roadmap 

- [ ] general app layout and ui
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

To expand on the last point, `Iced` is a fantastic library, and I definitely intend to use it on other projects. However at this time, I find it not mature enough to support this project, mostly because of the lack of menu widget that makes it kinda messy to build clean toolbars.    
I would also like to try other libraries, and `Druid` was suggested to me and seems similar enough to `Iced` to give it a worthy try.    
I’m aware that `Druid` is "deprecated" but its successor `Xilem` feels right now (Jan. 2023) still much of an experiment than a finished product.
