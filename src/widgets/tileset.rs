use egui::{vec2, Frame, Pos2, Rect, Sense, Ui, Vec2};

use super::tile::get_tile_shapes;
use crate::model::tileset::Tileset;

pub struct TilesetWidget {
    size_factor: f32,
    selected_tile: Option<usize>,
}

impl Default for TilesetWidget {
    fn default() -> Self {
        TilesetWidget {
            size_factor: 1.0,
            selected_tile: None,
        }
    }
}

impl TilesetWidget {
    /// creates a new widget
    pub fn new(size_factor: f32) -> Self {
        Self {
            size_factor,
            selected_tile: None,
        }
    }

    /// display the widget with the specified tileset
    pub fn show(&mut self, ui: &mut Ui, tileset: &Box<dyn Tileset>) {
        /*ui.with_layout(Layout::top_down(Align::Center), |ui| {
            //self.show_tiles(ui, tileset)

        });*/

        Frame::canvas(ui.style()).show(ui, |ui| {
            let max_size = self.show_tiles(ui, tileset);
            ui.set_max_size(max_size);
        });
    }

    fn show_tiles(&mut self, ui: &mut Ui, tileset: &Box<dyn Tileset>) -> Vec2 {
        if tileset.len() == 0 {
            return Vec2::ZERO;
            // early return since the code will fail
        }

        // we don’t care about the tileset inside the box, so get it out
        let tileset = tileset.as_ref();

        let widget_width = ui.available_width();
        let tile_draw_width = tileset.tile_size() as f32 * self.size_factor;
        let desired_size = needed_size(tile_draw_width, widget_width, tileset.len());

        let base_position = ui.next_widget_position(); //- vec2(widget_width / 2.0, 0.0);
        let base_position = base_position.floor();

        let positions: Vec<_> =
            tile_positions(tileset.len(), tile_draw_width, base_position, widget_width).collect();

        let shapes = tileset
            .iter()
            .zip(positions.iter())
            .flat_map(|(tile, &pos)| get_tile_shapes(tile, pos, self.size_factor));

        let (id, rect) = ui.allocate_space(desired_size);

        let click_response = ui.interact(rect, id, Sense::click());
        if click_response.clicked() {
            // primary button click
            let click_position = click_response.hover_pos().unwrap();

            if let Some(tile_id) = positions.iter().position(|&pos| {
                Rect::from_min_size(pos, Vec2::splat(tile_draw_width)).contains(click_position)
            }) {
                self.selected_tile = Some(tile_id);
                println!("{}", tile_id);
            }
        }
        if click_response.secondary_clicked() {
            println!("right click")
        }

        ui.painter().extend(shapes);

        return desired_size;
    }
}

fn tile_positions(
    nb_tiles: usize,
    draw_size: f32,
    base_position: Pos2,
    max_width: f32,
) -> impl Iterator<Item = Pos2> {
    (0..nb_tiles)
        .into_iter()
        .scan(base_position, move |pos, _| {
            let new_max_width = pos.x + draw_size;

            if new_max_width - base_position.x > max_width {
                // this tile does not fit, jump to next line
                pos.y += draw_size;
                // reset x
                pos.x = base_position.x;
            }

            // this is a valid position for our tile
            let draw_position = *pos;

            // increment the position
            pos.x += draw_size;
            pos.x = pos.x;

            Some(draw_position)
        })
}

fn needed_size(tile_size: f32, max_width: f32, nb_tiles: usize) -> Vec2 {
    let nb_tiles = nb_tiles as f32;

    // if the fit is perfect this is a noop
    let tiles_per_line = f32::floor(max_width / tile_size);
    let total_width = f32::min(tiles_per_line, nb_tiles) * tile_size;

    let nb_lines = f32::ceil(nb_tiles / tiles_per_line);

    let total_height = tile_size * nb_lines;

    vec2(total_width, total_height)
}
