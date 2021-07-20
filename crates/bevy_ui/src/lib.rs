mod anchors;
// mod flex;
mod layout;
mod focus;
mod margins;
mod render;
mod ui_node;

pub mod entity;
pub mod update;
pub mod widget;

pub use anchors::*;
pub use layout::*;
pub use focus::*;
pub use margins::*;
use layout::LayoutCache;
use morphorm::{LayoutType, PositionType, Units};
pub use render::*;
pub use ui_node::*;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{entity::*, ui_node::*, widget::Button, Anchors, Interaction, Margins};
}

use bevy_app::prelude::*;
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
    system::IntoSystem,
};
use bevy_input::InputSystem;
use bevy_math::{Rect, Size};
use bevy_render::RenderStage;
use bevy_transform::TransformSystem;
use update::ui_z_system;

#[derive(Default)]
pub struct UiPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum UiSystem {
    /// After this label, the ui flex state has been updated
    Flex,
    Focus,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LayoutCache>()
            // .register_type::<LayoutType>()
            // .register_type::<PositionType>()
            // .register_type::<Units>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                ui_focus_system
                    .system()
                    .label(UiSystem::Focus)
                    .after(InputSystem),
            )
            // add these stages to front because these must run before transform update systems
            .add_system_to_stage(
                CoreStage::PostUpdate,
                widget::text_system.system().before(UiSystem::Flex),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                widget::image_node_system.system().before(UiSystem::Flex),
            )
            // .add_system_to_stage(
            //     CoreStage::PostUpdate,
            //     flex_node_system
            //         .system()
            //         .label(UiSystem::Flex)
            //         .before(TransformSystem::TransformPropagate),
            // )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                layout_node_system
                    .system()
                    .label(UiSystem::Flex)
                    .before(TransformSystem::TransformPropagate),
            )
            // .add_system_to_stage(
            //     CoreStage::PostUpdate,
            //     ui_z_system
            //         .system()
            //         .after(UiSystem::Flex)
            //         .before(TransformSystem::TransformPropagate),
            // )
            .add_system_to_stage(RenderStage::Draw, widget::draw_text_system.system());

        crate::render::add_ui_graph(app.world_mut());
    }
}
