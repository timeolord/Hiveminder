use bevy::prelude::{Plugin, ResMut, App, SystemSet, State};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::GameState::{WorldGen, MainMenu, self};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
        .add_system_set(
            SystemSet::on_update(MainMenu)
                .with_system(main_menu)
        );
    }
}

fn main_menu(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::SidePanel::left("left").default_width(400.0).show(egui_context.ctx_mut(), |_| {});
    egui::SidePanel::right("right").default_width(400.0).show(egui_context.ctx_mut(), |_| {});
    egui::TopBottomPanel::top("top").exact_height(200.0).show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            ui.heading("Hiveminder");
        });
    });
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::BottomUp), |ui| {
            if ui.button("New Game").clicked() {
                state.set(WorldGen).unwrap();
            }
        })
    });
}