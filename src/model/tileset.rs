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
    /// Size of a tile (width or height)
    fn tile_size(&self) -> u16;
    /// Number of tiles
    fn len(&self) -> usize;
    /// Retrieve a specific tile
    fn get(&self, tile_id: usize) -> &Tile;
}

impl<'a> dyn Tileset + 'a {
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
    size: u16,
    content: Vec<Rgba>,
}

impl Tile {
    pub fn new(size: u16, content: Vec<Rgba>) -> Self {
        Self { size, content }
    }

    // returns the size (width or height) of the tile
    pub fn size(&self) -> u16 {
        self.size
    }

    pub fn get(&self, x: u16, y: u16) -> &Rgba {
        if x >= self.size {
            panic!("x position larger than width")
        }
        if y >= self.size {
            panic!("y position larger than height")
        }

        &self.content[(x + self.size * y) as usize]
    }
}

impl Index<(u16, u16)> for Tile {
    type Output = Rgba;

    #[inline]
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

#[cfg(test)]
mod tests {

    mod tile {

        use super::super::*;

        #[test]
        fn get_position() {
            let tile = Tile::new(2, vec![Rgba::GREEN, Rgba::BLUE, Rgba::RED, Rgba::BLACK]);

            let pixel = tile[(0, 0)];
            assert_eq!(Rgba::GREEN, pixel);
            let pixel = tile[(1, 0)];
            assert_eq!(Rgba::BLUE, pixel);
            let pixel = tile[(0, 1)];
            assert_eq!(Rgba::RED, pixel);
            let pixel = tile[(1, 1)];
            assert_eq!(Rgba::BLACK, pixel);
        }

        #[test]
        fn get_is_the_same_as_index() {
            let tile = Tile::new(2, vec![Rgba::GREEN, Rgba::BLUE, Rgba::RED, Rgba::BLACK]);

            assert_eq!(tile[(0, 0)], *tile.get(0, 0));
            assert_eq!(tile[(1, 0)], *tile.get(1, 0));
            assert_eq!(tile[(0, 1)], *tile.get(0, 1));
            assert_eq!(tile[(1, 1)], *tile.get(1, 1));
        }

        #[test]
        #[should_panic(expected = "x position larger")]
        fn x_too_large() {
            let tile = Tile::new(2, vec![Rgba::GREEN, Rgba::BLUE, Rgba::RED, Rgba::BLACK]);

            let _ = tile[(2, 0)];
        }

        #[test]
        #[should_panic(expected = "y position larger")]
        fn y_too_large() {
            let tile = Tile::new(2, vec![Rgba::GREEN, Rgba::BLUE, Rgba::RED, Rgba::BLACK]);

            let _ = tile[(0, 2)];
        }

        #[test]
        fn size() {
            let tile = Tile::new(2, vec![Rgba::GREEN, Rgba::BLUE, Rgba::RED, Rgba::BLACK]);
            let _ = tile.size();
        }
    }
}
