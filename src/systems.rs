use bevy::prelude::*;

use crate::assets::HtmlTailwind;
use crate::bundle::HtmlId;
use crate::bundle::HtmlTailwindHandle;
use crate::bundle::HtmlTailwindSpawned;
use crate::registry::HtmlTailwindRegistry;

/// Spawn the node tree on the entity
///
/// Marks the entity as finished with the HtmlUiSpawned component
pub fn spawn_ui(
    mut commands: Commands,
    assets: Res<Assets<HtmlTailwind>>,
    ui: Query<(Entity, &HtmlTailwindHandle), Without<HtmlTailwindSpawned>>,
) {
    for (entity, handle) in ui {
        let Some(asset) = assets.get(handle.handle.id()) else {
            continue;
        };

        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(HtmlTailwindSpawned);
            asset.dom.apply_to_entity(&mut entity_commands);
        }
    }
}

/// Applies marker components to newly created HTML nodes
pub fn apply_markers(
    mut commands: Commands,
    registry: Option<Res<HtmlTailwindRegistry>>,
    new_nodes: Query<(Entity, &HtmlId), Added<HtmlId>>,
) {
    let Some(registry) = registry else { return };

    for (entity, html_id) in &new_nodes {
        if let Ok(mut entity_cmd) = commands.get_entity(entity) {
            registry.apply_markers(&html_id.0, &mut entity_cmd);
        }
    }
}

/// Hot-reloading
pub fn sync_system(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<HtmlTailwind>>,
    assets: Res<Assets<HtmlTailwind>>,
    ui: Query<(Entity, &HtmlTailwindHandle), With<HtmlTailwindSpawned>>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id } = event {
            for (entity, handle) in ui.iter() {
                if handle.handle.id() == *id {
                    let Some(asset) = assets.get(*id) else {
                        continue;
                    };

                    if let Ok(mut entity_cmd) = commands.get_entity(entity) {
                        entity_cmd.despawn_children();

                        asset.dom.apply_to_entity(&mut entity_cmd);

                        info!("UI hot-reloaded for entity {:?}", entity);
                    }
                }
            }
        }
    }
}
