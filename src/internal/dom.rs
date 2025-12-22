use bevy::prelude::*;

use crate::internal::tailwind::Style;

#[derive(Debug, Clone)]
pub enum XNode {
    Div(XDiv),
    P(XP),
    Img(XImg),
    Button(XButton),
}

impl XNode {
    pub fn convert(node: roxmltree::Node) -> Self {
        match node.node_type() {
            roxmltree::NodeType::Element => match node.tag_name().name() {
                "div" => Self::Div(XDiv::convert(node)),
                "p" => Self::P(XP::convert(node)),
                "img" => Self::Img(XImg::convert(node)),
                "button" => Self::Button(XButton::convert(node)),
                _ => panic!("Unsupported tag: {}", node.tag_name().name()),
            },
            _ => panic!("Unsupported node type: {:?}", node.node_type()),
        }
    }

    pub(crate) fn apply_to_entity(&self, commands: &mut EntityCommands) {
        match self {
            XNode::Div(x) => x.apply_to_entity(commands),
            XNode::P(x) => x.apply_to_entity(commands),
            XNode::Img(x) => x.apply_to_entity(commands),
            XNode::Button(x) => x.apply_to_entity(commands),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XDiv {
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XDiv {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut classes = "".to_string();
        let content = node.text().map(|text| text.to_string());
        let mut children = Vec::new();

        for attribute in node.attributes() {
            if attribute.name() == "class" {
                classes = attribute.value().to_string();
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            style: Style::parse(&classes),
            content,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());

        commands.with_children(|parent| {
            if let Some(content) = &self.content {
                parent.spawn((Text::new(content),));
            }

            for child in &self.children {
                let mut child_entity_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_entity_commands);
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct XP {
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}

impl XP {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut classes = String::new();
        let content = node.text().map(|text| text.to_string());
        let mut children = Vec::new();

        for attribute in node.attributes() {
            if attribute.name() == "class" {
                classes = attribute.value().to_string();
            }
        }
        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
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
    pub style: Style,
    pub src: String,
    pub children: Vec<XNode>,
}
impl XImg {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut src = "".to_string();
        let mut classes = "".to_string();
        let mut children = Vec::new();

        for attribute in node.attributes() {
            if attribute.name() == "src" {
                src = attribute.value().to_string();
            }
            if attribute.name() == "class" {
                classes = attribute.value().to_string();
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            style: Style::parse(&classes),
            src,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert(self.style.to_node());
        commands.insert(ImageNode::default());

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
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<XNode>,
}
impl XButton {
    pub fn convert(node: roxmltree::Node) -> Self {
        let mut content = None;
        let mut classes = "".to_string();
        let mut children = Vec::new();

        if let Some(text) = node.text() {
            if !text.is_empty() {
                content = Some(text.to_string());
            }
        }

        for attribute in node.attributes() {
            if attribute.name() == "class" {
                classes = attribute.value().to_string();
            }
        }

        for child in node.children() {
            if child.is_element() {
                children.push(XNode::convert(child));
            }
        }

        Self {
            style: Style::parse(&classes),
            content,
            children,
        }
    }

    fn apply_to_entity(&self, commands: &mut EntityCommands) {
        commands.insert((Button, self.style.to_node()));

        if let Some(content) = &self.content {
            commands.insert(Text::new(content));
        }

        commands.with_children(|parent| {
            for child in &self.children {
                let mut child_commands = parent.spawn_empty();
                child.apply_to_entity(&mut child_commands);
            }
        });
    }
}
