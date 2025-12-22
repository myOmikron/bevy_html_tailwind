use bevy::log::warn;
use bevy::prelude::Bundle;
use bevy::ui::*;

#[derive(Debug, Clone)]
pub struct Style {
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub width: Val,
    pub height: Val,
}

impl Style {
    pub fn parse(classes: &str) -> Self {
        let classes = classes
            .trim()
            .split_ascii_whitespace()
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();

        let mut display = Display::default();
        let mut flex_direction = FlexDirection::default();
        let mut width = auto();
        let mut height = auto();
        let mut justify_content = JustifyContent::default();
        let mut align_items = AlignItems::default();

        for class in classes {
            match class {
                "flex" => display = Display::Flex,
                "flex-col" => flex_direction = FlexDirection::Column,
                "flex-row" => flex_direction = FlexDirection::Row,
                "flex-row-reverse" => flex_direction = FlexDirection::RowReverse,
                "flex-col-reverse" => flex_direction = FlexDirection::ColumnReverse,

                "justify-start" => justify_content = JustifyContent::FlexStart,
                "justify-center" => justify_content = JustifyContent::Center,
                "justify-end" => justify_content = JustifyContent::FlexEnd,
                "justify-between" => justify_content = JustifyContent::SpaceBetween,
                "justify-around" => justify_content = JustifyContent::SpaceAround,
                "justify-evenly" => justify_content = JustifyContent::SpaceEvenly,
                "justify-stretch" => justify_content = JustifyContent::Stretch,
                "justify-normal" => justify_content = JustifyContent::Default,

                "items-start" => align_items = AlignItems::FlexStart,
                "items-end" => align_items = AlignItems::FlexEnd,
                "items-center" => align_items = AlignItems::Center,
                "items-stretch" => align_items = AlignItems::Stretch,
                "items-baseline" => align_items = AlignItems::Baseline,

                "w-full" => width = percent(100.0),
                "h-full" => height = percent(100.0),
                "size-full" => {
                    width = percent(100);
                    height = percent(100.0);
                }

                "grid" => display = Display::Grid,

                _ => {
                    warn!("Unsupported style class: {class}");
                }
            }
        }

        Self {
            display,
            flex_direction,
            justify_content,
            align_items,
            width,
            height,
        }
    }

    pub fn to_node(&self) -> impl Bundle {
        (Node {
            display: self.display,
            flex_direction: self.flex_direction,
            justify_content: self.justify_content,
            align_items: self.align_items,
            width: self.width,
            height: self.height,
            ..Default::default()
        },)
    }
}
