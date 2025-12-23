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

#[derive(Component)]
pub struct HtmlUiSpawned;

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
