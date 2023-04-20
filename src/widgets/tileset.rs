use eframe::epaint::RectShape;
use egui::{vec2, Frame, Pos2, Rect, Rgba, Rounding, Sense, Shape, Stroke, Ui, Vec2};

use super::tile::get_tile_shapes;
use crate::model::tileset::Tileset;

/// Widget to display a tileset
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

    /// get the current tile
    pub fn get_selected(&self) -> Option<usize> {
        self.selected_tile
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

        // compute position of each tile
        let positions: Vec<_> =
            tile_positions(tileset.len(), tile_draw_width, base_position, widget_width).collect();

        // compute shapes of each tile
        let shapes = tileset
            .iter()
            .zip(positions.iter())
            .flat_map(|(tile, &pos)| get_tile_shapes(tile, pos, self.size_factor));

        // draw the tiles
        ui.painter().extend(shapes);

        // interaction rect
        let (id, rect) = ui.allocate_space(desired_size);

        // clicks
        let click_response = ui.interact(rect, id, Sense::click_and_drag());
        if click_response.clicked() || click_response.drag_started_by(egui::PointerButton::Primary)
        {
            // primary button click
            let click_position = click_response.hover_pos().unwrap();

            // is the click on a tile ?
            if let Some(tile_id) = positions.iter().position(|&pos| {
                Rect::from_min_size(pos, Vec2::splat(tile_draw_width)).contains(click_position)
            }) {
                // select it
                self.selected_tile = Some(tile_id);
            }
        }
        if click_response.secondary_clicked() {
            self.selected_tile = None;
        }

        // draw the selected tile if it exists
        if let Some(id) = self.selected_tile {
            ui.painter().add(selected_tile_shape(
                id,
                widget_width,
                tile_draw_width as f32,
                base_position,
            ));
        }

        return desired_size;
    }
}

/// Compute absolute position for all the tiles
fn tile_positions(
    nb_tiles: usize,
    draw_size: f32,
    base_position: Pos2,
    max_width: f32,
) -> impl Iterator<Item = Pos2> {
    let tiles_per_line = f32::floor(max_width / draw_size as f32) as usize;

    (0..nb_tiles)
        .into_iter()
        .map(move |id| tile_position_relative(id, tiles_per_line, draw_size)) // compute relative position
        .map(move |p| p + base_position.to_vec2()) // transform to global
}

/// Compute tile position relative to the widget
fn tile_position_relative(tile_id: usize, tiles_per_line: usize, draw_size: f32) -> Pos2 {
    let y_offset = tile_id / tiles_per_line;
    let x_offset = tile_id % tiles_per_line;

    Pos2 {
        x: draw_size * x_offset as f32,
        y: draw_size * y_offset as f32,
    }
}

/// Compute the total widget size needed for all the tiles
fn needed_size(tile_size: f32, max_width: f32, nb_tiles: usize) -> Vec2 {
    let nb_tiles = nb_tiles as f32;

    // if the fit is perfect this is a noop
    let tiles_per_line = f32::floor(max_width / tile_size);
    let total_width = f32::min(tiles_per_line, nb_tiles) * tile_size;

    let nb_lines = f32::ceil(nb_tiles / tiles_per_line);

    let total_height = tile_size * nb_lines;

    vec2(total_width, total_height)
}

/// Compute the shape for the selected tile
fn selected_tile_shape(
    id: usize,
    widget_width: f32,
    tile_draw_width: f32,
    base_position: Pos2,
) -> Shape {
    let tiles_per_line = f32::floor(widget_width / tile_draw_width as f32) as usize;

    let select_pos =
        tile_position_relative(id, tiles_per_line, tile_draw_width) + base_position.to_vec2();

    Shape::Rect(RectShape::stroke(
        Rect::from_min_size(select_pos, Vec2::splat(tile_draw_width)),
        Rounding::none(),
        Stroke::new(3.0, Rgba::from_gray(0.3)),
    ))
}
