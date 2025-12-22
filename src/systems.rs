use bevy::prelude::*;

use crate::assets::HtmlUi;
use crate::bundle::HtmlUiHandle;
use crate::bundle::HtmlUiSpawned;

/// Initial UI spawn
pub fn spawn_ui(
    mut commands: Commands,
    assets: Res<Assets<HtmlUi>>,
    ui: Query<(Entity, &HtmlUiHandle), Without<HtmlUiSpawned>>,
) {
    for (entity, handle) in ui {
        let Some(asset) = assets.get(handle.handle.id()) else {
            continue;
        };

        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(HtmlUiSpawned);
            asset.dom.apply_to_entity(&mut entity_commands);
        }
    }
}

/// Hot-reloading
pub fn sync_system(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<HtmlUi>>,
    assets: Res<Assets<HtmlUi>>,
    ui: Query<(Entity, &HtmlUiHandle), With<HtmlUiSpawned>>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id } = event {}
    }
}
