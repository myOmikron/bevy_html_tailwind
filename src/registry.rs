use std::collections::HashMap;

use bevy::prelude::*;

/// Resource that marks components on entities based on HTML IDs
#[derive(Resource, Default)]
pub struct HtmlMarkerRegistry {
    markers: HashMap<String, Vec<Box<dyn Fn(&mut EntityCommands) + Send + Sync>>>,
}

impl HtmlMarkerRegistry {
    pub(crate) fn add_marker<M: Component + Default>(&mut self, html_id: String) {
        self.markers
            .entry(html_id)
            .or_default()
            .push(Box::new(|cmd| {
                cmd.insert(M::default());
            }));
    }

    pub(crate) fn add_marker_with<M: Component + Clone + Send + Sync + 'static>(
        &mut self,
        html_id: String,
        marker: M,
    ) {
        self.markers
            .entry(html_id)
            .or_default()
            .push(Box::new(move |cmd| {
                cmd.insert(marker.clone());
            }));
    }

    /// Applies registered markers to an entity
    pub(crate) fn apply_markers(&self, html_id: &str, commands: &mut EntityCommands) {
        if let Some(markers) = self.markers.get(html_id) {
            for apply_fn in markers {
                apply_fn(commands);
            }
        }
    }
}

/// Extension trait for registering HTML markers
pub trait HtmlMarkerAppExt {
    fn register_html_marker<M: Component + Default>(
        &mut self,
        html_id: impl Into<String>,
    ) -> &mut Self;

    fn register_html_marker_with<M: Component + Clone + Send + Sync + 'static>(
        &mut self,
        html_id: impl Into<String>,
        marker: M,
    ) -> &mut Self;
}

impl HtmlMarkerAppExt for App {
    fn register_html_marker<M: Component + Default>(
        &mut self,
        html_id: impl Into<String>,
    ) -> &mut Self {
        let html_id = html_id.into();

        self.world_mut()
            .resource_mut::<HtmlMarkerRegistry>()
            .add_marker::<M>(html_id);

        self
    }

    fn register_html_marker_with<M: Component + Clone + Send + Sync + 'static>(
        &mut self,
        html_id: impl Into<String>,
        marker: M,
    ) -> &mut Self {
        let html_id = html_id.into();

        self.world_mut()
            .resource_mut::<HtmlMarkerRegistry>()
            .add_marker_with(html_id, marker);

        self
    }
}
