use bevy::log::warn;
use bevy::prelude::Bundle;
use bevy::prelude::Visibility;
use bevy::ui::*;

#[derive(Debug, Clone)]
pub struct Style {
    pub visibility: Visibility,
    pub position: PositionType,
    pub display: Display,
    pub flex_direction: FlexDirection,
    pub justify_content: JustifyContent,
    pub justify_items: JustifyItems,
    pub justify_self: JustifySelf,
    pub align_content: AlignContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
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

        let mut visibility = Visibility::default();
        let mut position = PositionType::default();
        let mut display = Display::Block;
        let mut flex_direction = FlexDirection::default();
        let mut width = auto();
        let mut height = auto();
        let mut justify_content = JustifyContent::default();
        let mut justify_items = JustifyItems::default();
        let mut justify_self = JustifySelf::default();
        let mut align_content = AlignContent::default();
        let mut align_items = AlignItems::default();
        let mut align_self = AlignSelf::default();

        for class in classes {
            match class {
                "visible" => visibility = Visibility::Visible,
                "invisible" => visibility = Visibility::Hidden,

                "relative" => position = PositionType::Relative,
                "absolute" => position = PositionType::Absolute,

                "block" => display = Display::Block,
                "grid" => display = Display::Grid,
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

                "justify-items-start" => justify_items = JustifyItems::Start,
                "justify-items-end" => justify_items = JustifyItems::End,
                "justify-items-center" => justify_items = JustifyItems::Center,
                "justify-items-stretch" => justify_items = JustifyItems::Stretch,
                "justify-items-normal" => justify_items = JustifyItems::Default,

                "justify-self-auto" => justify_self = JustifySelf::Auto,
                "justify-self-start" => justify_self = JustifySelf::Start,
                "justify-self-end" => justify_self = JustifySelf::End,
                "justify-self-center" => justify_self = JustifySelf::Center,
                "justify-self-stretch" => justify_self = JustifySelf::Stretch,

                "content-normal" => align_content = AlignContent::Default,
                "content-center" => align_content = AlignContent::Center,
                "content-start" => align_content = AlignContent::FlexStart,
                "content-end" => align_content = AlignContent::FlexEnd,
                "content-between" => align_content = AlignContent::SpaceBetween,
                "content-around" => align_content = AlignContent::SpaceAround,
                "content-evenly" => align_content = AlignContent::SpaceEvenly,
                "content-stretch" => align_content = AlignContent::Stretch,

                "items-start" => align_items = AlignItems::FlexStart,
                "items-end" => align_items = AlignItems::FlexEnd,
                "items-center" => align_items = AlignItems::Center,
                "items-stretch" => align_items = AlignItems::Stretch,
                "items-baseline" => align_items = AlignItems::Baseline,

                "self-auto" => align_self = AlignSelf::Auto,
                "self-start" => align_self = AlignSelf::FlexStart,
                "self-end" => align_self = AlignSelf::FlexEnd,
                "self-center" => align_self = AlignSelf::Center,
                "self-stretch" => align_self = AlignSelf::Stretch,
                "self-baseline" => align_self = AlignSelf::Baseline,

                "place-content-center" => {
                    align_content = AlignContent::Center;
                    justify_content = JustifyContent::Center;
                }
                "place-content-start" => {
                    align_content = AlignContent::FlexStart;
                    justify_content = JustifyContent::FlexStart;
                }
                "place-content-end" => {
                    align_content = AlignContent::FlexEnd;
                    justify_content = JustifyContent::FlexEnd;
                }
                "place-content-between" => {
                    align_content = AlignContent::SpaceBetween;
                    justify_content = JustifyContent::SpaceBetween;
                }
                "place-content-around" => {
                    align_content = AlignContent::SpaceAround;
                    justify_content = JustifyContent::SpaceAround;
                }
                "place-content-evenly" => {
                    align_content = AlignContent::SpaceEvenly;
                    justify_content = JustifyContent::SpaceEvenly;
                }
                "place-content-stretch" => {
                    align_content = AlignContent::Stretch;
                    justify_content = JustifyContent::Stretch;
                }

                "place-items-start" => {
                    align_items = AlignItems::FlexStart;
                    justify_items = JustifyItems::Start;
                }
                "place-items-end" => {
                    align_items = AlignItems::FlexEnd;
                    justify_items = JustifyItems::End;
                }
                "place-items-center" => {
                    align_items = AlignItems::Center;
                    justify_items = JustifyItems::Center;
                }
                "place-items-stretch" => {
                    align_items = AlignItems::Stretch;
                    justify_items = JustifyItems::Stretch;
                }
                "place-items-baseline" => {
                    align_items = AlignItems::Baseline;
                    justify_items = JustifyItems::Baseline;
                }

                "place-self-auto" => {
                    align_self = AlignSelf::Auto;
                    justify_self = JustifySelf::Auto;
                }
                "place-self-start" => {
                    align_self = AlignSelf::FlexStart;
                    justify_self = JustifySelf::Start;
                }
                "place-self-end" => {
                    align_self = AlignSelf::FlexEnd;
                    justify_self = JustifySelf::End;
                }
                "place-self-center" => {
                    align_self = AlignSelf::Center;
                    justify_self = JustifySelf::Center;
                }
                "place-self-stretch" => {
                    align_self = AlignSelf::Stretch;
                    justify_self = JustifySelf::Stretch;
                }

                "w-full" => width = percent(100.0),
                "h-full" => height = percent(100.0),
                "size-full" => {
                    width = percent(100);
                    height = percent(100.0);
                }
                _ => {
                    warn!("Unsupported style class: {class}");
                }
            }
        }

        Self {
            visibility,
            position,
            display,
            flex_direction,
            justify_content,
            justify_items,
            justify_self,
            align_content,
            align_items,
            align_self,
            width,
            height,
        }
    }

    pub fn to_node(&self) -> impl Bundle {
        (
            Node {
                position_type: self.position,
                display: self.display,
                flex_direction: self.flex_direction,
                justify_content: self.justify_content,
                justify_items: self.justify_items,
                justify_self: self.justify_self,
                align_content: self.align_content,
                align_items: self.align_items,
                align_self: self.align_self,
                width: self.width,
                height: self.height,
                ..Default::default()
            },
            self.visibility,
        )
    }
}
