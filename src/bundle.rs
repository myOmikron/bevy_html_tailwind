use bevy::prelude::*;

use crate::assets::HtmlTailwind;

#[derive(Component, Reflect, Default)]
pub struct HtmlTailwindHandle {
    pub handle: Handle<HtmlTailwind>,
}

impl From<Handle<HtmlTailwind>> for HtmlTailwindHandle {
    fn from(handle: Handle<HtmlTailwind>) -> Self {
        Self { handle }
    }
}

/// Component that stores the HTML ID of a node
#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HtmlId(pub String);

/// Marker component that is attached when the node tree is spawned on the entity
#[derive(Component)]
pub struct HtmlTailwindSpawned;

/// Bundle for HTML UI nodes with Tailwind CSS styling
#[derive(Bundle)]
pub struct HtmlTailwindBundle {
    pub ui: HtmlTailwindHandle,
    pub visibility: Visibility,
    pub name: Name,
    pub transform: Transform,
}

impl Default for HtmlTailwindBundle {
    fn default() -> Self {
        Self {
            ui: Default::default(),
            visibility: Visibility::Visible,
            name: Name::default(),
            transform: Transform::default(),
        }
    }
}
