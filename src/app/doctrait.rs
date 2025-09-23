use dioxus::prelude::*;

#[derive(
    Default, Debug, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Success,
    Accent,
    Muted,
}

#[derive(Default, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

#[derive(Default, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum Animation {
    None,
    Light,
    #[default]
    Full,
}

#[derive(Default, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Side {
    Top,
    Bottom,
    Left,
    #[default]
    Right,
}

#[derive(Default, Clone, Copy, PartialEq, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Variant {
    #[default]
    None,
    Outline,
    Ghost,
}

pub trait IntoVec<T> {
    fn into_vec() -> Vec<T>;
}

impl IntoVec<Color> for Color {
    fn into_vec() -> Vec<Color> {
        vec![
            Color::Default,
            Color::Primary,
            Color::Secondary,
            Color::Success,
            Color::Destructive,
            Color::Accent,
            Color::Muted,
        ]
    }
}

impl IntoVec<Size> for Size {
    fn into_vec() -> Vec<Size> {
        vec![Size::Md, Size::Xs, Size::Sm, Size::Lg, Size::Xl]
    }
}

impl IntoVec<Animation> for Animation {
    fn into_vec() -> Vec<Animation> {
        vec![Animation::Full, Animation::None, Animation::Light]
    }
}

impl IntoVec<Orientation> for Orientation {
    fn into_vec() -> Vec<Orientation> {
        vec![Orientation::Vertical, Orientation::Horizontal]
    }
}

impl IntoVec<Side> for Side {
    fn into_vec() -> Vec<Side> {
        vec![Side::Top, Side::Bottom, Side::Left, Side::Right]
    }
}

impl IntoVec<Variant> for Variant {
    fn into_vec() -> Vec<Variant> {
        vec![Variant::None, Variant::Outline, Variant::Ghost]
    }
}

pub trait DemoComponent {
    fn comp_introduction() -> &'static str;
    fn BuildCompPreview() -> Element;
    fn BuildCompSelectors() -> Element;
}
