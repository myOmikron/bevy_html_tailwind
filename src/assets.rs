use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::LoadContext;
use bevy::asset::io::Reader;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use roxmltree::Document;
use thiserror::Error;

use crate::internal::dom::XFont;
use crate::internal::dom::XNode;

#[derive(Asset, TypePath, Debug)]
pub struct HtmlTailwind {
    pub dom: XNode,
    pub fonts: HashMap<String, Handle<Font>>,
    pub default_font: Option<Handle<Font>>,
}

#[derive(Default)]
pub struct HtmlUiAssetLoader;

impl AssetLoader for HtmlUiAssetLoader {
    type Asset = HtmlTailwind;
    type Settings = ();
    type Error = HtmlUiLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut doc_raw = String::new();
        reader.read_to_string(&mut doc_raw).await?;
        let document = Document::parse(&doc_raw)?;

        let mut fonts = HashMap::new();
        let mut default_font = None;
        let mut body_node = None;

        for child in document.root().children() {
            if !child.is_element() {
                continue;
            }

            let tag_name = child.tag_name().name().to_lowercase();
            match tag_name.as_str() {
                "html" => {
                    for child in child.children() {
                        let tag_name = child.tag_name().name().to_lowercase();
                        if !child.is_element() {
                            continue;
                        }

                        match tag_name.as_str() {
                            "head" => {
                                for head_child in child.children() {
                                    let tag_name = head_child.tag_name().name().to_lowercase();
                                    match tag_name.as_str() {
                                        "font" => XFont::convert(
                                            head_child,
                                            load_context,
                                            &mut fonts,
                                            &mut default_font,
                                        ),
                                        _ => {}
                                    }
                                }
                            }
                            "body" => {
                                for body_child in child.children() {
                                    let tag_name = body_child.tag_name().name().to_lowercase();
                                    match tag_name.as_str() {
                                        "div" | "button" | "span" | "p" => {
                                            body_node = Some(XNode::convert(body_child));
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "div" | "button" | "span" | "p" => {
                    body_node = Some(XNode::convert(child));
                }
                _ => {}
            }
        }

        let mut body_node = body_node.expect("no body node");
        body_node.resolve(load_context, &fonts, default_font.clone());

        let root = HtmlTailwind {
            dom: body_node,
            fonts,
            default_font,
        };

        info!("Root node: {root:#?}");

        Ok(root)
    }

    fn extensions(&self) -> &[&str] {
        &["html", "xml"]
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
