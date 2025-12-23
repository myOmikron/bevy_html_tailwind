use bevy::asset::LoadContext;
use bevy::prelude::*;

use crate::internal::tailwind::Style;

#[derive(Debug, Clone)]
pub enum XNode {
    Div(XDiv),
    Text(XText),
    Img(XImg),
    Button(XButton),
}

impl XNode {
    pub fn convert(node: roxmltree::Node, load_context: &mut LoadContext) -> Self {
        match node.node_type() {
            roxmltree::NodeType::Element => match node.tag_name().name().to_lowercase().as_str() {
                "div" => Self::Div(XDiv::convert(node, load_context)),
                "p" | "span" => Self::Text(XText::convert(node, load_context)),
                "img" => Self::Img(XImg::convert(node, load_context)),
                "button" => Self::Button(XButton::convert(node, load_context)),
                _ => panic!("Unsupported tag: {}", node.tag_name().name()),
            },
            _ => panic!("Unsupported node type: {:?}", node.node_type()),
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
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XDiv {
    pub fn convert(node: roxmltree::Node, load_context: &mut LoadContext) -> Self {
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
                children.push(XNode::convert(child, load_context));
            }
        }

        Self {
            id,
            style: Style::parse(&classes),
            content,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
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
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XText {
    pub fn convert(node: roxmltree::Node, load_context: &mut LoadContext) -> Self {
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
                children.push(XNode::convert(child, load_context));
            }
        }

        Self {
            id,
            style: Style::parse(&classes),
            content,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());

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
    pub style: Style,
    pub image_handle: Handle<Image>,
    pub children: Vec<XNode>,
}

impl XImg {
    pub fn convert(node: roxmltree::Node, load_context: &mut LoadContext) -> Self {
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

        let image_handle = load_context.load(&src);

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child, load_context));
            }
        }

        Self {
            id,
            style: Style::parse(&classes),
            image_handle,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
        commands.insert(ImageNode {
            image: self.image_handle.clone(),
            ..Default::default()
        });

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
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}
impl XButton {
    pub fn convert(node: roxmltree::Node, load_context: &mut LoadContext) -> Self {
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
                children.push(XNode::convert(child, load_context));
            }
        }

        Self {
            id,
            style: Style::parse(&classes),
            content,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert((Button, self.style.to_node()));

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
