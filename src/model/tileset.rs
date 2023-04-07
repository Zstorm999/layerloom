pub mod dummy_tileset;

use std::ops::Index;

use egui::Rgba;

/// A trait defining shared functionality between all type of tilesets,
/// namely getting each tiles in the tileset.
///
/// Supports the `into_iter` method for ease of use.
///
/// ```
/// use dummy_tileset::DummyTileset;
/// let tileset = DummyTileset::default();
/// for tile in tileset {
///     // do something
/// }
/// ```
pub trait Tileset {
    fn len(&self) -> usize;
    fn get(&self, tile_id: usize) -> &Tile;
}

impl dyn Tileset {
    pub fn iter(&self) -> TilesetIterator {
        TilesetIterator::new(self)
    }
}

impl Index<usize> for dyn Tileset {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

/// Iterates on a Tileset
pub struct TilesetIterator<'a> {
    tileset: &'a dyn Tileset,
    next_value: usize,
}

impl<'a> TilesetIterator<'a> {
    pub fn new(tileset: &'a dyn Tileset) -> Self {
        TilesetIterator {
            tileset,
            next_value: 0,
        }
    }
}

impl<'a> Iterator for TilesetIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_value < self.tileset.len() {
            let ret = self.tileset.get(self.next_value);
            self.next_value += 1;

            return Some(ret);
        }
        None
    }
}

pub struct Tile {
    width: u16,
    height: u16,
    content: Vec<Rgba>,
}

impl Tile {
    pub fn new(width: u16, height: u16, content: Vec<Rgba>) -> Self {
        Self {
            width,
            height,
            content,
        }
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn get(&self, x: u16, y: u16) -> &Rgba {
        if x > self.width {
            panic!("x position larger than width")
        }
        if y > self.height {
            panic!("y position larger than height")
        }

        &self.content[(x + self.width * y) as usize]
    }
}

impl Index<(u16, u16)> for Tile {
    type Output = Rgba;

    #[inline]
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}
