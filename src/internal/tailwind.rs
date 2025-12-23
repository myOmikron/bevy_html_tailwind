use std::sync::LazyLock;

use bevy::color::Color;
use bevy::log::info;
use bevy::log::warn;
use bevy::prelude::Bundle;
use bevy::prelude::Visibility;
use bevy::text::TextColor;
use bevy::ui::*;
use regex::Regex;

pub struct TailwindRegex {
    pub width: Regex,
    pub min_width: Regex,
    pub max_width: Regex,
    pub height: Regex,
    pub min_height: Regex,
    pub max_height: Regex,
    pub border: Regex,
    pub border_x: Regex,
    pub border_y: Regex,
    pub border_l: Regex,
    pub border_r: Regex,
    pub border_t: Regex,
    pub border_b: Regex,
    pub border_color: Regex,
    pub background_color: Regex,
    pub padding: Regex,
    pub padding_x: Regex,
    pub padding_y: Regex,
    pub padding_l: Regex,
    pub padding_r: Regex,
    pub padding_t: Regex,
    pub padding_b: Regex,
    pub margin: Regex,
    pub margin_x: Regex,
    pub margin_y: Regex,
    pub margin_t: Regex,
    pub margin_b: Regex,
    pub margin_l: Regex,
    pub margin_r: Regex,
    pub text_color: Regex,
    pub z_index: Regex,
    pub grid_template_columns: Regex,
}

pub static REGEX: LazyLock<TailwindRegex> = LazyLock::new(|| TailwindRegex {
    width: Regex::new(r"^w-\[(\d+)px]$").unwrap(),
    min_width: Regex::new(r"^min-w-\[(\d+)px]$").unwrap(),
    max_width: Regex::new(r"^max-w-\[(\d+)px]$").unwrap(),
    height: Regex::new(r"^h-\[(\d+)px]$").unwrap(),
    min_height: Regex::new(r"^min-h-\[(\d+)px]$").unwrap(),
    max_height: Regex::new(r"^max-h-\[(\d+)px]$").unwrap(),
    border: Regex::new(r"^border-(\d+)$").unwrap(),
    border_x: Regex::new(r"^border-x-(\d+)$").unwrap(),
    border_y: Regex::new(r"^border-y-(\d+)$").unwrap(),
    border_l: Regex::new(r"^border-l-(\d+)$").unwrap(),
    border_r: Regex::new(r"^border-r-(\d+)$").unwrap(),
    border_t: Regex::new(r"^border-t-(\d+)$").unwrap(),
    border_b: Regex::new(r"^border-b-(\d+)$").unwrap(),
    border_color: Regex::new(
        r"^border-\[#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?]$",
    )
    .unwrap(),
    background_color: Regex::new(
        r"^bg-\[#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?]$",
    )
    .unwrap(),
    padding: Regex::new(r"^p-\[(\d+)px]$").unwrap(),
    padding_x: Regex::new(r"px-\[(\d+)px]$").unwrap(),
    padding_y: Regex::new(r"py-\[(\d+)px]$").unwrap(),
    padding_t: Regex::new(r"pt-\[(\d+)px]$").unwrap(),
    padding_b: Regex::new(r"pb-\[(\d+)px]$").unwrap(),
    padding_l: Regex::new(r"pl-\[(\d+)px]$").unwrap(),
    padding_r: Regex::new(r"pr-\[(\d+)px]$").unwrap(),
    margin: Regex::new(r"^m-\[(\d+)px]$").unwrap(),
    margin_x: Regex::new(r"mx-\[(\d+)px]$").unwrap(),
    margin_y: Regex::new(r"my-\[(\d+)px]$").unwrap(),
    margin_t: Regex::new(r"mt-\[(\d+)px]$").unwrap(),
    margin_b: Regex::new(r"mb-\[(\d+)px]$").unwrap(),
    margin_l: Regex::new(r"ml-\[(\d+)px]$").unwrap(),
    margin_r: Regex::new(r"mr-\[(\d+)px]$").unwrap(),
    text_color: Regex::new(
        r"text-\[#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?]$",
    )
    .unwrap(),
    z_index: Regex::new(r"^(-)?z-(\d+)$").unwrap(),
    grid_template_columns: Regex::new(
        r"^grid-cols-(?:(\d+)|\[((?:\d+fr|\d+px|auto)(?:_\d+fr|_\d+px|_auto)*)])$",
    )
    .unwrap(),
});

#[derive(Debug, Clone, PartialEq)]
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
    pub min_width: Val,
    pub max_width: Val,
    pub height: Val,
    pub min_height: Val,
    pub max_height: Val,
    pub border_color: BorderColor,
    pub border: UiRect,
    pub background_color: BackgroundColor,
    pub padding: UiRect,
    pub margin: UiRect,
    pub text_color: TextColor,
    pub z_index: ZIndex,
    pub grid_template_columns: Vec<RepeatedGridTrack>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            visibility: Default::default(),
            position: Default::default(),
            display: Display::Block,
            flex_direction: Default::default(),
            justify_content: Default::default(),
            justify_items: Default::default(),
            justify_self: Default::default(),
            align_content: Default::default(),
            align_items: Default::default(),
            align_self: Default::default(),
            width: Default::default(),
            min_width: Default::default(),
            max_width: Default::default(),
            height: Default::default(),
            min_height: Default::default(),
            max_height: Default::default(),
            border_color: BorderColor::DEFAULT,
            background_color: BackgroundColor::DEFAULT,
            border: UiRect::default(),
            margin: UiRect::default(),
            padding: UiRect::default(),
            text_color: TextColor::BLACK,
            z_index: ZIndex::default(),
            grid_template_columns: vec![],
        }
    }
}

impl Style {
    pub fn parse(classes: &str) -> Self {
        let classes = classes
            .trim()
            .split_ascii_whitespace()
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();

        let mut style = Style::default();

        for class in classes {
            match class {
                "visible" => style.visibility = Visibility::Visible,
                "invisible" => style.visibility = Visibility::Hidden,

                "relative" => style.position = PositionType::Relative,
                "absolute" => style.position = PositionType::Absolute,

                "block" => style.display = Display::Block,
                "grid" => style.display = Display::Grid,
                "flex" => style.display = Display::Flex,
                "hidden" => style.display = Display::None,

                "flex-col" => style.flex_direction = FlexDirection::Column,
                "flex-row" => style.flex_direction = FlexDirection::Row,
                "flex-row-reverse" => style.flex_direction = FlexDirection::RowReverse,
                "flex-col-reverse" => style.flex_direction = FlexDirection::ColumnReverse,

                "justify-start" => style.justify_content = JustifyContent::FlexStart,
                "justify-center" => style.justify_content = JustifyContent::Center,
                "justify-end" => style.justify_content = JustifyContent::FlexEnd,
                "justify-between" => style.justify_content = JustifyContent::SpaceBetween,
                "justify-around" => style.justify_content = JustifyContent::SpaceAround,
                "justify-evenly" => style.justify_content = JustifyContent::SpaceEvenly,
                "justify-stretch" => style.justify_content = JustifyContent::Stretch,
                "justify-normal" => style.justify_content = JustifyContent::Default,

                "justify-items-start" => style.justify_items = JustifyItems::Start,
                "justify-items-end" => style.justify_items = JustifyItems::End,
                "justify-items-center" => style.justify_items = JustifyItems::Center,
                "justify-items-stretch" => style.justify_items = JustifyItems::Stretch,
                "justify-items-normal" => style.justify_items = JustifyItems::Default,

                "justify-self-auto" => style.justify_self = JustifySelf::Auto,
                "justify-self-start" => style.justify_self = JustifySelf::Start,
                "justify-self-end" => style.justify_self = JustifySelf::End,
                "justify-self-center" => style.justify_self = JustifySelf::Center,
                "justify-self-stretch" => style.justify_self = JustifySelf::Stretch,

                "content-normal" => style.align_content = AlignContent::Default,
                "content-center" => style.align_content = AlignContent::Center,
                "content-start" => style.align_content = AlignContent::FlexStart,
                "content-end" => style.align_content = AlignContent::FlexEnd,
                "content-between" => style.align_content = AlignContent::SpaceBetween,
                "content-around" => style.align_content = AlignContent::SpaceAround,
                "content-evenly" => style.align_content = AlignContent::SpaceEvenly,
                "content-stretch" => style.align_content = AlignContent::Stretch,

                "items-start" => style.align_items = AlignItems::FlexStart,
                "items-end" => style.align_items = AlignItems::FlexEnd,
                "items-center" => style.align_items = AlignItems::Center,
                "items-stretch" => style.align_items = AlignItems::Stretch,
                "items-baseline" => style.align_items = AlignItems::Baseline,

                "self-auto" => style.align_self = AlignSelf::Auto,
                "self-start" => style.align_self = AlignSelf::FlexStart,
                "self-end" => style.align_self = AlignSelf::FlexEnd,
                "self-center" => style.align_self = AlignSelf::Center,
                "self-stretch" => style.align_self = AlignSelf::Stretch,
                "self-baseline" => style.align_self = AlignSelf::Baseline,

                "place-content-center" => {
                    style.align_content = AlignContent::Center;
                    style.justify_content = JustifyContent::Center;
                }
                "place-content-start" => {
                    style.align_content = AlignContent::FlexStart;
                    style.justify_content = JustifyContent::FlexStart;
                }
                "place-content-end" => {
                    style.align_content = AlignContent::FlexEnd;
                    style.justify_content = JustifyContent::FlexEnd;
                }
                "place-content-between" => {
                    style.align_content = AlignContent::SpaceBetween;
                    style.justify_content = JustifyContent::SpaceBetween;
                }
                "place-content-around" => {
                    style.align_content = AlignContent::SpaceAround;
                    style.justify_content = JustifyContent::SpaceAround;
                }
                "place-content-evenly" => {
                    style.align_content = AlignContent::SpaceEvenly;
                    style.justify_content = JustifyContent::SpaceEvenly;
                }
                "place-content-stretch" => {
                    style.align_content = AlignContent::Stretch;
                    style.justify_content = JustifyContent::Stretch;
                }

                "place-items-start" => {
                    style.align_items = AlignItems::FlexStart;
                    style.justify_items = JustifyItems::Start;
                }
                "place-items-end" => {
                    style.align_items = AlignItems::FlexEnd;
                    style.justify_items = JustifyItems::End;
                }
                "place-items-center" => {
                    style.align_items = AlignItems::Center;
                    style.justify_items = JustifyItems::Center;
                }
                "place-items-stretch" => {
                    style.align_items = AlignItems::Stretch;
                    style.justify_items = JustifyItems::Stretch;
                }
                "place-items-baseline" => {
                    style.align_items = AlignItems::Baseline;
                    style.justify_items = JustifyItems::Baseline;
                }

                "place-self-auto" => {
                    style.align_self = AlignSelf::Auto;
                    style.justify_self = JustifySelf::Auto;
                }
                "place-self-start" => {
                    style.align_self = AlignSelf::FlexStart;
                    style.justify_self = JustifySelf::Start;
                }
                "place-self-end" => {
                    style.align_self = AlignSelf::FlexEnd;
                    style.justify_self = JustifySelf::End;
                }
                "place-self-center" => {
                    style.align_self = AlignSelf::Center;
                    style.justify_self = JustifySelf::Center;
                }
                "place-self-stretch" => {
                    style.align_self = AlignSelf::Stretch;
                    style.justify_self = JustifySelf::Stretch;
                }

                "w-full" => style.width = percent(100.0),
                "h-full" => style.height = percent(100.0),
                "size-full" => {
                    style.width = percent(100);
                    style.height = percent(100.0);
                }

                "border-black" => style.border_color = BorderColor::all(Color::BLACK),
                "border-white" => style.border_color = BorderColor::all(Color::WHITE),

                "border" => style.border = UiRect::all(px(1)),
                "border-x" => {
                    style.border = UiRect {
                        left: px(1.0),
                        right: px(1.0),
                        ..style.border
                    }
                }
                "border-y" => {
                    style.border = UiRect {
                        top: px(1.0),
                        bottom: px(1.0),
                        ..style.border
                    }
                }
                "border-l" => {
                    style.border = UiRect {
                        left: px(1.0),
                        ..style.border
                    }
                }
                "border-r" => {
                    style.border = UiRect {
                        right: px(1.0),
                        ..style.border
                    }
                }
                "border-b" => {
                    style.border = UiRect {
                        bottom: px(1.0),
                        ..style.border
                    }
                }
                "border-t" => {
                    style.border = UiRect {
                        top: px(1.0),
                        ..style.border
                    }
                }

                "p-px" => style.padding = UiRect::all(px(1.0)),
                "p-auto" => style.padding = UiRect::all(auto()),
                "px-auto" => {
                    style.padding = UiRect {
                        left: auto(),
                        right: auto(),
                        ..style.padding
                    }
                }
                "py-auto" => {
                    style.padding = UiRect {
                        top: auto(),
                        bottom: auto(),
                        ..style.padding
                    }
                }
                "m-px" => style.margin = UiRect::all(px(1.0)),
                "m-auto" => style.margin = UiRect::all(auto()),
                "mx-auto" => {
                    style.margin = UiRect {
                        left: auto(),
                        right: auto(),
                        ..style.margin
                    }
                }
                "my-auto" => {
                    style.margin = UiRect {
                        top: auto(),
                        bottom: auto(),
                        ..style.margin
                    }
                }

                "text-white" => style.text_color = TextColor::WHITE,
                "text-black" => style.text_color = TextColor::BLACK,

                _ => {
                    if REGEX.width.is_match(class) {
                        let Some(captures) = REGEX.width.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.width = px(px_val);
                    } else if REGEX.min_width.is_match(class) {
                        let Some(captures) = REGEX.min_width.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.min_width = px(px_val);
                    } else if REGEX.max_width.is_match(class) {
                        let Some(captures) = REGEX.max_width.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.max_width = px(px_val);
                    } else if REGEX.height.is_match(class) {
                        let Some(captures) = REGEX.height.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.height = px(px_val);
                    } else if REGEX.min_height.is_match(class) {
                        let Some(captures) = REGEX.min_height.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.min_height = px(px_val);
                    } else if REGEX.max_height.is_match(class) {
                        let Some(captures) = REGEX.max_height.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.max_height = px(px_val);
                    } else if REGEX.border.is_match(class) {
                        let Some(captures) = REGEX.border.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect::all(px(px_val));
                    } else if REGEX.border_x.is_match(class) {
                        let Some(captures) = REGEX.border_x.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            left: px(px_val),
                            right: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_y.is_match(class) {
                        let Some(captures) = REGEX.border_y.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            top: px(px_val),
                            bottom: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_t.is_match(class) {
                        let Some(captures) = REGEX.border_t.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            top: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_b.is_match(class) {
                        let Some(captures) = REGEX.border_b.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            bottom: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_l.is_match(class) {
                        let Some(captures) = REGEX.border_l.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            left: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_r.is_match(class) {
                        let Some(captures) = REGEX.border_r.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.border = UiRect {
                            right: px(px_val),
                            ..style.border
                        };
                    } else if REGEX.border_color.is_match(class) {
                        let Some(captures) = REGEX.border_color.captures(class) else {
                            continue;
                        };
                        let color_r =
                            u8::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
                        let color_g =
                            u8::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap();
                        let color_b =
                            u8::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
                        let color_a = captures
                            .get(4)
                            .map(|x| u8::from_str_radix(x.as_str(), 16).unwrap())
                            .unwrap_or(255);

                        style.border_color =
                            BorderColor::all(Color::srgba_u8(color_r, color_g, color_b, color_a));
                    } else if REGEX.background_color.is_match(class) {
                        let Some(captures) = REGEX.background_color.captures(class) else {
                            continue;
                        };
                        let color_r =
                            u8::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
                        let color_g =
                            u8::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap();
                        let color_b =
                            u8::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
                        let color_a = captures
                            .get(4)
                            .map(|x| u8::from_str_radix(x.as_str(), 16).unwrap())
                            .unwrap_or(255);

                        style.background_color =
                            BackgroundColor(Color::srgba_u8(color_r, color_g, color_b, color_a));
                    } else if REGEX.padding.is_match(class) {
                        let Some(captures) = REGEX.padding.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect::all(px(px_val));
                    } else if REGEX.padding_x.is_match(class) {
                        let Some(captures) = REGEX.padding_x.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            left: px(px_val),
                            right: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.padding_y.is_match(class) {
                        let Some(captures) = REGEX.padding_y.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            top: px(px_val),
                            bottom: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.padding_l.is_match(class) {
                        let Some(captures) = REGEX.padding_l.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            left: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.padding_r.is_match(class) {
                        let Some(captures) = REGEX.padding_r.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            right: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.padding_b.is_match(class) {
                        let Some(captures) = REGEX.padding_b.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            bottom: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.padding_t.is_match(class) {
                        let Some(captures) = REGEX.padding_t.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.padding = UiRect {
                            top: px(px_val),
                            ..style.padding
                        };
                    } else if REGEX.margin.is_match(class) {
                        let Some(captures) = REGEX.margin.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect::all(px(px_val));
                    } else if REGEX.margin_x.is_match(class) {
                        let Some(captures) = REGEX.margin_x.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            left: px(px_val),
                            right: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.margin_y.is_match(class) {
                        let Some(captures) = REGEX.margin_y.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            top: px(px_val),
                            bottom: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.margin_l.is_match(class) {
                        let Some(captures) = REGEX.margin_l.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            left: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.margin_r.is_match(class) {
                        let Some(captures) = REGEX.margin_r.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            right: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.margin_b.is_match(class) {
                        let Some(captures) = REGEX.margin_b.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            bottom: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.margin_t.is_match(class) {
                        let Some(captures) = REGEX.margin_t.captures(class) else {
                            continue;
                        };
                        let px_val = captures.get(1).unwrap().as_str().parse::<u64>().unwrap();
                        style.margin = UiRect {
                            top: px(px_val),
                            ..style.margin
                        };
                    } else if REGEX.text_color.is_match(class) {
                        let Some(captures) = REGEX.text_color.captures(class) else {
                            continue;
                        };
                        let color_r =
                            u8::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
                        let color_g =
                            u8::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap();
                        let color_b =
                            u8::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
                        let color_a = captures
                            .get(4)
                            .map(|x| u8::from_str_radix(x.as_str(), 16).unwrap())
                            .unwrap_or(255);

                        style.text_color =
                            TextColor(Color::srgba_u8(color_r, color_g, color_b, color_a));
                    } else if REGEX.z_index.is_match(class) {
                        let Some(captures) = REGEX.z_index.captures(class) else {
                            continue;
                        };

                        let negative = captures.get(1).map(|x| x.as_str() == "-").unwrap_or(false);
                        let index = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

                        style.z_index = ZIndex(if negative {
                            -1 * index as i32
                        } else {
                            index as i32
                        });
                    } else if REGEX.grid_template_columns.is_match(class) {
                        let Some(captures) = REGEX.grid_template_columns.captures(class) else {
                            continue;
                        };

                        if let Some(c) = captures.get(1) {
                            let columns = c.as_str().parse::<u16>().unwrap();
                            style.grid_template_columns = RepeatedGridTrack::auto(columns);
                            continue;
                        }

                        let Some(matches) = captures.get(2).map(|x| x.as_str()) else {
                            continue;
                        };

                        let mut columns = vec![];
                        for part in matches.split('_') {
                            if part == "auto" {
                                columns.push(RepeatedGridTrack::auto(1));
                            } else if part.ends_with("fr") {
                                let count = part.trim_end_matches("fr").parse::<u16>().unwrap();
                                columns.push(RepeatedGridTrack::fr(1, count as f32));
                            } else if part.ends_with("px") {
                                let count = part.trim_end_matches("px").parse::<u16>().unwrap();
                                columns.push(RepeatedGridTrack::px(1, count as f32));
                            }
                        }
                        style.grid_template_columns = columns;
                    } else {
                        warn!("Unsupported style class: {class}");
                    }
                }
            }
        }

        style
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
                min_width: self.min_width,
                max_width: self.max_width,
                height: self.height,
                min_height: self.min_height,
                max_height: self.max_height,
                border: self.border,
                padding: self.padding,
                margin: self.margin,
                grid_template_columns: self.grid_template_columns.clone(),
                ..Default::default()
            },
            self.visibility,
            self.border_color,
            self.background_color,
            self.text_color,
            self.z_index,
        )
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::px;
    use bevy::ui::UiRect;
    use bevy::ui::ZIndex;
    use bevy::ui::percent;

    use crate::internal::tailwind::Style;

    #[test]
    fn test_0() {
        let input = vec![
            (
                "w-[20px]",
                Style {
                    width: px(20),
                    ..Default::default()
                },
            ),
            (
                "min-w-[120px]",
                Style {
                    min_width: px(120),
                    ..Default::default()
                },
            ),
            (
                "w-full max-w-[200px] min-w-[120px]",
                Style {
                    width: percent(100),
                    min_width: px(120),
                    max_width: px(200),
                    ..Default::default()
                },
            ),
            (
                "ml-[2px] my-[20px]",
                Style {
                    margin: UiRect {
                        left: px(2.0),
                        bottom: px(20.0),
                        top: px(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ),
            (
                "z-20",
                Style {
                    z_index: ZIndex(20),
                    ..Default::default()
                },
            ),
            (
                "-z-20",
                Style {
                    z_index: ZIndex(-20),
                    ..Default::default()
                },
            ),
        ];

        for (test, res) in input {
            assert_eq!(Style::parse(test), res);
        }
    }
}
