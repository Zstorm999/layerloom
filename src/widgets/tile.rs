use eframe::epaint::RectShape;
use egui::{vec2, Pos2, Rect, Rounding, Shape, Vec2};

use crate::model::tileset::Tile;

/// Retrieve the shapes necessary to draw a tile (scaled pixels)
pub fn get_tile_shapes(tile: &Tile, position: Pos2, size_factor: f32) -> Vec<Shape> {
    let tile_size = tile.size();

    let mut shapes = vec![];

    let square_size = Vec2::splat(size_factor);

    for x in 0..tile_size {
        for y in 0..tile_size {
            let rect_pos = position + vec2(x as f32 * size_factor, y as f32 * size_factor);

            shapes.push(Shape::Rect(RectShape::filled(
                Rect::from_min_size(rect_pos, square_size),
                Rounding::none(),
                tile[(x, y)],
            )));
        }
    }

    shapes
}
