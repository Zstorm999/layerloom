use eframe::epaint::RectShape;
use egui::{emath, epaint, pos2, vec2, Frame, Pos2, Rect, Rounding, Shape, Ui, Vec2};

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
        Frame::canvas(ui.style()).show(ui, |ui| self.show_tiles(ui, tileset));
    }

    fn show_tiles(&mut self, ui: &mut Ui, tileset: &Box<dyn Tileset>) {
        if tileset.len() == 0 {
            return;
            // early return since the code will fail
        }

        // we don’t care about the tileset inside the box, so get it out
        let tileset = tileset.as_ref();

        let widget_width = ui.available_width();
        let base_position = ui.next_widget_position();

        let first_tile_size = tileset[0].size();
        let min_size = pos2(
            first_tile_size.0 as f32 * self.size_factor,
            first_tile_size.1 as f32 * self.size_factor,
        );

        let (positions, max_positions): (Vec<_>, Vec<_>) = tileset
            .iter()
            .scan((base_position, min_size), |(pos, max_pos), tile| {
                let (width, height) = tile.size();

                let new_max_width = pos.x + width as f32 * self.size_factor;

                if new_max_width - base_position.x > widget_width {
                    // this tile does not fit, jump to next line
                    pos.y += height as f32 * self.size_factor;
                    // reset x
                    pos.x = base_position.x;
                    // also this becomes the new maximum y
                    max_pos.y = pos.y + height as f32 * self.size_factor;
                }

                // this is a valid position for our tile
                let draw_position = *pos;

                // increment the position
                pos.x += width as f32 * self.size_factor;
                if pos.x > max_pos.x {
                    // update max if needed
                    max_pos.x = pos.x;
                }

                Some((draw_position, *max_pos))
            })
            .unzip();

        let max_position = max_positions.last().unwrap().to_vec2();

        let shapes = tileset
            .iter()
            .zip(positions)
            .flat_map(|(tile, pos)| get_tile_shapes(tile, pos, self.size_factor));

        println!("{:?}", widget_width);
        println!("{:?}", max_position);

        let (_id, _rect) = ui.allocate_space(max_position);
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
