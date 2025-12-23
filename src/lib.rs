use bevy::prelude::*;

use crate::assets::HtmlTailwind;
use crate::assets::HtmlUiAssetLoader;
use crate::registry::HtmlTailwindRegistry;
use crate::systems::apply_markers;
use crate::systems::spawn_ui;
use crate::systems::sync_system;

pub mod prelude {
    pub use crate::HtmlTailwindPlugin;
    pub use crate::assets::HtmlTailwind;
    pub use crate::bundle::HtmlTailwindBundle;
    pub use crate::registry::HtmlTailwindAppExt;
}

mod assets;
mod bundle;
mod internal;
mod registry;
mod systems;

pub struct HtmlTailwindPlugin {
    pub hot_reload: bool,
}

impl Default for HtmlTailwindPlugin {
    fn default() -> Self {
        Self { hot_reload: true }
    }
}

impl Plugin for HtmlTailwindPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<HtmlTailwind>()
            .init_asset_loader::<HtmlUiAssetLoader>()
            .init_resource::<HtmlTailwindRegistry>()
            .add_systems(Update, (spawn_ui, apply_markers).chain());

        if self.hot_reload {
            app.add_systems(Update, sync_system);
        }
    }
}
