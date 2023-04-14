use eframe::egui;
use egui::{Align, Context, Layout, Ui};
use model::{
    tileset::{dummy_tileset::DummyTileset, Tileset},
    Model,
};
use widgets::TilesetWidget;

mod model;
mod widgets;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Layerloom",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    model: Model,
    tileset_widget: TilesetWidget,
    tileset: Box<dyn Tileset>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            model: Model::default(),
            tileset_widget: TilesetWidget::new(5.0),
            tileset: Box::new(DummyTileset::default()),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // menu bar at the top
        egui::TopBottomPanel::top("menus")
            //.show_separator_line(false)
            .show(ctx, |ui| self.menus(ui, frame));

        // left-side tileset panel
        egui::SidePanel::left("tileset").show(ctx, |ui| self.tileset(ui, frame));

        // right side layers panel
        egui::SidePanel::right("layers")
            .resizable(false)
            .show_animated(ctx, true, |ui| self.layers_panel(ui, frame));

        //toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| self.toolbar(ui, frame));

        // central tilemap
        egui::CentralPanel::default().show(ctx, |ui| self.tilmap(ui, frame));
    }
}

impl MyApp {
    fn menus(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {}
                if ui.button("Open").clicked() {}
                ui.menu_button("Open recent", |ui| {
                    if ui.button("some file").clicked() {}
                    if ui.button("some other file").clicked() {}
                })
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() {}
                if ui.button("Redo").clicked() {}
            });
        });
    }

    fn toolbar(&mut self, _ui: &mut Ui, _frame: &mut eframe::Frame) {}

    fn tileset(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if self.model.is_tileset_loaded() {
            // display tileset

            self.tileset_widget.show(ui, &self.tileset);
        } else {
            ui.label("No tileset loaded");
        }

        // bottom open tileset button
        egui::TopBottomPanel::bottom("open tileset")
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                    if ui.button("Open tileset").clicked() {
                        //this button does nothing for now
                    }
                })
            });
    }

    fn layers_panel(&mut self, _ui: &mut Ui, _frame: &mut eframe::Frame) {}

    fn tilmap(&mut self, _ui: &mut Ui, _frame: &mut eframe::Frame) {
        // draw tilemap
    }
}
