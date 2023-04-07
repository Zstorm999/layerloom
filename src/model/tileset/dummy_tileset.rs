use egui::Rgba;

use super::{Tile, Tileset};

///
/// A default tileset, which provides 8x8 tiles
/// consisting of the 6 additive base colours, plus black and white.
///
///
pub struct DummyTileset {
    tiles: Vec<Tile>,
}

impl Default for DummyTileset {
    fn default() -> Self {
        let tiles = vec![
            new_tile(1.0, 0.0, 0.0), // red
            new_tile(0.0, 1.0, 0.0), // green
            new_tile(0.0, 0.0, 1.0), // blue
            new_tile(1.0, 0.0, 1.0), // magenta
            new_tile(1.0, 1.0, 0.0), // yellow
            new_tile(0.0, 1.0, 1.0), // cyan
            new_tile(0.0, 0.0, 0.0), // white
            new_tile(1.0, 1.0, 1.0), // black
        ];

        Self { tiles }
    }
}

impl Tileset for DummyTileset {
    fn len(&self) -> usize {
        self.tiles.len()
    }

    fn get(&self, tile_id: usize) -> &Tile {
        &self.tiles[tile_id]
    }
}

fn new_tile(r: f32, g: f32, b: f32) -> Tile {
    Tile::new(
        8,
        8,
        vec![Rgba::from_rgb(r, g, b)]
            .into_iter()
            .cycle()
            .take(64)
            .collect(),
    )
}
