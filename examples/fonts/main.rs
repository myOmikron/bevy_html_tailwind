use bevy::prelude::*;
use bevy_html_tailwind::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()))
        .add_plugins(HtmlTailwindPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup camera
    commands.spawn(Camera2d);

    // Spawn UI
    let handle = asset_server.load("fonts/main.html");
    commands.spawn(HtmlTailwindBundle {
        ui: handle.into(),
        name: Name::new("Main UI"),
        ..Default::default()
    });
}
