use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::asset::io::Reader;
use bevy::prelude::*;
use roxmltree::Document;
use thiserror::Error;

use crate::internal::dom::XNode;

#[derive(Asset, TypePath, Debug)]
pub struct HtmlUi {
    pub dom: XNode,
}

#[derive(Default)]
pub struct HtmlUiAssetLoader;

impl AssetLoader for HtmlUiAssetLoader {
    type Asset = HtmlUi;
    type Settings = ();
    type Error = HtmlUiLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut doc_raw = String::new();
        reader.read_to_string(&mut doc_raw).await?;

        let document = Document::parse(&doc_raw)?;

        document.root().children().next().unwrap();

        Ok(HtmlUi {
            dom: XNode::convert(document.root().children().next().unwrap()),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["html"]
    }
}

#[derive(Error, Debug)]
pub enum HtmlUiLoadError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Decode Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("XML parse Error: {0}")]
    XMLError(#[from] roxmltree::Error),
}
