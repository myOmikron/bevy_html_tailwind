use bevy::prelude::*;

use crate::assets::HtmlUi;

#[derive(Component, Reflect, Default)]
pub struct HtmlUiHandle {
    pub handle: Handle<HtmlUi>,
}

impl From<Handle<HtmlUi>> for HtmlUiHandle {
    fn from(handle: Handle<HtmlUi>) -> Self {
        Self { handle }
    }
}

#[derive(Component)]
pub struct HtmlUiSpawned;

#[derive(Bundle)]
pub struct HtmlUiBundle {
    pub ui: HtmlUiHandle,
    pub visibility: Visibility,
    pub name: Name,
    pub transform: Transform,
}

impl Default for HtmlUiBundle {
    fn default() -> Self {
        Self {
            ui: Default::default(),
            visibility: Visibility::Visible,
            name: Name::default(),
            transform: Transform::default(),
        }
    }
}
