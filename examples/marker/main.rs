use bevy::prelude::*;
use bevy_html_tailwind::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Default)]
pub struct ExitGameMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()))
        .add_plugins(HtmlTailwindPlugin::default())
        // Register html id
        .register_html_marker::<ExitGameMarker>("marker-main-exit")
        .add_systems(Startup, startup)
        .add_systems(Update, handle_button)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup camera
    commands.spawn(Camera2d);

    // Spawn UI
    let handle = asset_server.load("marker/main.html");
    commands.spawn(HtmlTailwindBundle {
        ui: handle.into(),
        name: Name::new("Main UI"),
        ..Default::default()
    });
}

fn handle_button(
    interaction: Query<&Interaction, With<ExitGameMarker>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for interaction in interaction.iter() {
        if *interaction == Interaction::Pressed {
            app_exit_writer.write(AppExit::Success);
        }
    }
}
