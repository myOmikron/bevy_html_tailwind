use bevy::prelude::*;

use crate::assets::HtmlUi;
use crate::assets::HtmlUiAssetLoader;
use crate::systems::spawn_ui;
use crate::systems::sync_system;

pub mod prelude {
    pub use crate::HtmlTailwindPlugin;
    pub use crate::assets::HtmlUi;
    pub use crate::bundle::HtmlUiBundle;
}

mod assets;
mod bundle;
mod internal;
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
        app.init_asset::<HtmlUi>()
            .init_asset_loader::<HtmlUiAssetLoader>()
            .add_systems(Update, spawn_ui);

        if self.hot_reload {
            app.add_systems(Update, sync_system);
        }
    }
}
