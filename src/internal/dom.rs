use bevy::asset::LoadContext;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use crate::bundle::HtmlId;
use crate::internal::tailwind::Style;

pub struct XFont;

impl XFont {
    pub(crate) fn convert(
        node: roxmltree::Node,
        load_context: &mut LoadContext,
        fonts: &mut HashMap<String, Handle<Font>>,
        default_font: &mut Option<Handle<Font>>,
    ) {
        let name = node.attribute("name");
        let Some(src) = node.attribute("src") else {
            return;
        };

        let handle = load_context.load(src.to_string());

        match name {
            Some("default") | None => {
                *default_font = Some(handle);
            }
            Some(name) if name == "default" => {
                *default_font = Some(handle);
            }
            Some(name) => {
                fonts.insert(name.to_string(), handle);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum XNode {
    Div(XDiv),
    Text(XText),
    Img(XImg),
    Button(XButton),
}

impl XNode {
    pub fn convert(node: roxmltree::Node) -> Self {
        match node.node_type() {
            roxmltree::NodeType::Element => match node.tag_name().name().to_lowercase().as_str() {
                "div" => Self::Div(XDiv::convert(node)),
                "p" | "span" => Self::Text(XText::convert(node)),
                "img" => Self::Img(XImg::convert(node)),
                "button" => Self::Button(XButton::convert(node)),
                _ => panic!("Unsupported tag: {}", node.tag_name().name()),
            },
            _ => panic!("Unsupported node type: {:?}", node.node_type()),
        }
    }

    pub fn resolve(
        &mut self,
        load_context: &mut LoadContext,
        fonts: &HashMap<String, Handle<Font>>,
        default_font: Option<Handle<Font>>,
    ) {
        match self {
            XNode::Div(x) => x.resolve(load_context, fonts, default_font.clone()),
            XNode::Text(x) => x.resolve(load_context, fonts, default_font.clone()),
            XNode::Img(x) => x.resolve(load_context, fonts, default_font.clone()),
            XNode::Button(x) => x.resolve(load_context, fonts, default_font.clone()),
        }
    }

    pub(crate) fn apply_to_entity(&self, commands: &mut EntityCommands) {
        match self {
            XNode::Div(x) => x.apply_to_entity(commands),
            XNode::Text(x) => x.apply_to_entity(commands),
            XNode::Img(x) => x.apply_to_entity(commands),
            XNode::Button(x) => x.apply_to_entity(commands),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XDiv {
    pub id: Option<String>,
    pub classes: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XDiv {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut id = None;
        let mut classes = "".to_string();
        let content = node
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string());
        let mut children = Vec::new();

        for attribute in node.attributes() {
            match attribute.name() {
                "class" => classes = attribute.value().to_string(),
                "id" => id = Some(attribute.value().to_string()),
                _ => {}
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            id,
            classes,
            style: Style::default(),
            content,
            children,
        }
    }

    pub fn resolve(
        &mut self,
        load_context: &mut LoadContext,
        fonts: &HashMap<String, Handle<Font>>,
        default_font: Option<Handle<Font>>,
    ) {
        self.style = Style::parse(&self.classes, fonts, default_font.clone());
        for child in &mut self.children {
            child.resolve(load_context, fonts, default_font.clone());
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
        if let Some(id) = &self.id {
            commands.insert(HtmlId(id.clone()));
        }

        if let Some(content) = &self.content {
            commands.with_child(Text::new(content));
        }

        commands.with_children(|parent| {
            for child in &self.children {
                let mut child_entity_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_entity_commands);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct XText {
    pub id: Option<String>,
    pub classes: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XText {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut id = None;
        let mut classes = String::new();
        let content = node
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string());
        let mut children = Vec::new();

        for attribute in node.attributes() {
            match attribute.name() {
                "class" => classes = attribute.value().to_string(),
                "id" => id = Some(attribute.value().to_string()),
                _ => {}
            }
        }
        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            id,
            classes,
            style: Style::default(),
            content,
            children,
        }
    }

    pub fn resolve(
        &mut self,
        load_context: &mut LoadContext,
        fonts: &HashMap<String, Handle<Font>>,
        default_font: Option<Handle<Font>>,
    ) {
        self.style = Style::parse(&self.classes, fonts, default_font.clone());
        for child in &mut self.children {
            child.resolve(load_context, fonts, default_font.clone());
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
        if let Some(id) = &self.id {
            commands.insert(HtmlId(id.clone()));
        }

        if let Some(content) = &self.content {
            commands.insert(Text::new(content));
        }

        commands.with_children(|parent| {
            for child in &self.children {
                let mut child_entity_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_entity_commands);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct XImg {
    pub id: Option<String>,
    pub src: String,
    pub classes: String,
    pub style: Style,
    pub image_handle: Handle<Image>,
    pub children: Vec<XNode>,
}

impl XImg {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut id = None;
        let mut src = "".to_string();
        let mut classes = "".to_string();
        let mut children = Vec::new();

        for attribute in node.attributes() {
            match attribute.name() {
                "src" => src = attribute.value().to_string(),
                "class" => classes = attribute.value().to_string(),
                "id" => id = Some(attribute.value().to_string()),
                _ => {}
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            id,
            src,
            classes,
            style: Style::default(),
            image_handle: Handle::default(),
            children,
        }
    }

    pub fn resolve(
        &mut self,
        load_context: &mut LoadContext,
        fonts: &HashMap<String, Handle<Font>>,
        default_font: Option<Handle<Font>>,
    ) {
        self.image_handle = load_context.load(&self.src);
        self.style = Style::parse(&self.classes, fonts, default_font.clone());
        for child in &mut self.children {
            child.resolve(load_context, fonts, default_font.clone());
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
        commands.insert(ImageNode {
            image: self.image_handle.clone(),
            ..Default::default()
        });
        if let Some(id) = &self.id {
            commands.insert(HtmlId(id.clone()));
        }

        commands.with_children(|parent| {
            for child in &self.children {
                let mut child_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_commands);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct XButton {
    pub id: Option<String>,
    pub classes: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}
impl XButton {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut id = None;
        let content = node
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string());
        let mut classes = "".to_string();
        let mut children = Vec::new();

        for attribute in node.attributes() {
            match attribute.name() {
                "class" => classes = attribute.value().to_string(),
                "id" => id = Some(attribute.value().to_string()),
                _ => {}
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            id,
            classes,
            style: Style::default(),
            content,
            children,
        }
    }

    pub fn resolve(
        &mut self,
        load_context: &mut LoadContext,
        fonts: &HashMap<String, Handle<Font>>,
        default_font: Option<Handle<Font>>,
    ) {
        self.style = Style::parse(&self.classes, fonts, default_font.clone());
        for child in &mut self.children {
            child.resolve(load_context, fonts, default_font.clone());
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert((Button, self.style.to_node()));

        if let Some(id) = &self.id {
            commands.insert(HtmlId(id.clone()));
        }

        if let Some(content) = &self.content {
            commands.with_child(Text::new(content));
        }

        commands.with_children(|parent| {
            for child in &self.children {
                let mut child_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_commands);
            }
        });
    }
}
