use bevy_ecs::reflect::ReflectComponent;
use bevy_math::{Rect, Size, Vec2};
use bevy_reflect::{Reflect, ReflectDeserialize};
use bevy_render::renderer::RenderResources;
use morphorm::{PositionType, LayoutType, Units};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Default, RenderResources, Reflect)]
#[reflect(Component)]
pub struct UINode {
    pub size: Vec2,
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize, Reflect)]
#[reflect_value(PartialEq, Serialize, Deserialize)]
pub enum Val {
    Undefined,
    Auto,
    Px(f32),
    Percent(f32),
}

impl Default for Val {
    fn default() -> Self {
        Val::Undefined
    }
}

impl Add<f32> for Val {
    type Output = Val;

    fn add(self, rhs: f32) -> Self::Output {
        match self {
            Val::Undefined => Val::Undefined,
            Val::Auto => Val::Auto,
            Val::Px(value) => Val::Px(value + rhs),
            Val::Percent(value) => Val::Percent(value + rhs),
        }
    }
}

impl AddAssign<f32> for Val {
    fn add_assign(&mut self, rhs: f32) {
        match self {
            Val::Undefined | Val::Auto => {}
            Val::Px(value) => *value += rhs,
            Val::Percent(value) => *value += rhs,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub position_type: PositionType,
    pub layout_type: LayoutType,

    pub left: Units,
    pub right: Units,
    pub top: Units,
    pub bottom: Units,

    pub min_left: Units,
    pub max_left: Units,
    pub min_right: Units,
    pub max_right: Units,
    pub min_top: Units,
    pub max_top: Units,
    pub min_bottom: Units,
    pub max_bottom: Units,

    pub width: Units,
    pub height: Units,

    pub min_width: Units,
    pub max_width: Units,
    pub min_height: Units,
    pub max_height: Units,

    pub child_left: Units,
    pub child_right: Units,
    pub child_top: Units,
    pub child_bottom: Units,

    pub row_between: Units,
    pub col_between: Units,

    pub grid_rows: Vec<Units>,
    pub grid_cols: Vec<Units>,

    pub row_index: usize,
    pub col_index: usize,
    pub row_span: usize,
    pub col_span: usize,

    pub border: Units,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct CalculatedSize {
    pub size: Size,
}
