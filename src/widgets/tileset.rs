use eframe::epaint::RectShape;
use egui::{emath, epaint, pos2, vec2, Pos2, Rect, Rounding, Shape, Ui, Vec2};

use crate::model::tileset::{Tile, Tileset};

pub struct TilesetWidget {
    size_factor: f32,
}

impl Default for TilesetWidget {
    fn default() -> Self {
        TilesetWidget { size_factor: 1.0 }
    }
}

impl TilesetWidget {
    /// creates a new widget
    pub fn new(size_factor: f32) -> Self {
        Self { size_factor }
    }

    /// display the widget with the specified tileset
    pub fn show(&mut self, ui: &mut Ui, tileset: &Box<dyn Tileset>) {
        // we don’t care about the tileset inside the box, so get it out
        let tileset = tileset.as_ref();

        let width = ui.available_width();
        let base_position = ui.next_widget_position();

        let (_id, _rect) = ui.allocate_space(vec2(8.0, 8.0) * self.size_factor);

        let shapes = get_tile_shapes(&tileset[0], base_position, self.size_factor);

        ui.painter().extend(shapes);
    }
}

fn get_tile_shapes(tile: &Tile, position: Pos2, size_factor: f32) -> Vec<Shape> {
    let (width, height) = tile.size();

    let mut shapes = vec![];

    let square_size = Vec2::splat(size_factor);

    for x in 0..width {
        for y in 0..height {
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
