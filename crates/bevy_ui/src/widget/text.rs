use crate::{CalculatedSize, UINode, Style, Val};
use bevy_asset::Assets;
use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or, With, Without},
    system::{Local, Query, QuerySet, Res, ResMut},
};
use bevy_math::Size;
use bevy_render::{
    draw::{Draw, DrawContext, Drawable, OutsideFrustum},
    mesh::Mesh,
    prelude::{Msaa, Visible},
    renderer::RenderResourceBindings,
    texture::Texture,
};
use bevy_sprite::{TextureAtlas, QUAD_HANDLE};
use bevy_text::{DefaultTextPipeline, DrawableText, Font, FontAtlasSet, Text, TextError};
use bevy_transform::prelude::GlobalTransform;
use bevy_window::Windows;
use morphorm::Units;

#[derive(Debug, Default)]
pub struct QueuedText {
    entities: Vec<Entity>,
}

fn scale_value(value: f32, factor: f64) -> f32 {
    (value as f64 * factor) as f32
}

/// Defines how min_size, size, and max_size affects the bounds of a text
/// block.
pub fn text_constraint(min_size: Units, size: Units, max_size: Units, scale_factor: f64) -> f32 {
    // Needs support for percentages
    match (min_size, size, max_size) {
        (_, _, Units::Pixels(max)) => scale_value(max, scale_factor),
        (Units::Pixels(min), _, _) => scale_value(min, scale_factor),
        (Units::Auto, Units::Pixels(size), Units::Auto) => scale_value(size, scale_factor),
        (Units::Auto, Units::Pixels(size), Units::Auto) => scale_value(size, scale_factor),
        _ => f32::MAX,
    }
}

/// Computes the size of a text block and updates the TextGlyphs with the
/// new computed glyphs from the layout
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn text_system(
    mut queued_text: Local<QueuedText>,
    mut last_scale_factor: Local<f64>,
    mut textures: ResMut<Assets<Texture>>,
    fonts: Res<Assets<Font>>,
    windows: Res<Windows>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut font_atlas_set_storage: ResMut<Assets<FontAtlasSet>>,
    mut text_pipeline: ResMut<DefaultTextPipeline>,
    mut text_queries: QuerySet<(
        Query<Entity, Or<(Changed<Text>, Changed<Style>)>>,
        Query<Entity, (With<Text>, With<Style>)>,
        Query<(&Text, &Style, &mut CalculatedSize)>,
    )>,
) {
    let scale_factor = if let Some(window) = windows.get_primary() {
        window.scale_factor()
    } else {
        1.
    };

    let inv_scale_factor = 1. / scale_factor;

    #[allow(clippy::float_cmp)]
    if *last_scale_factor == scale_factor {
        // Adds all entities where the text or the style has changed to the local queue
        for entity in text_queries.q0().iter() {
            queued_text.entities.push(entity);
        }
    } else {
        // If the scale factor has changed, queue all text
        for entity in text_queries.q1().iter() {
            queued_text.entities.push(entity);
        }
        *last_scale_factor = scale_factor;
    }

    if queued_text.entities.is_empty() {
        return;
    }

    // Computes all text in the local queue
    let mut new_queue = Vec::new();
    let query = text_queries.q2_mut();
    for entity in queued_text.entities.drain(..) {
        if let Ok((text, style, mut calculated_size)) = query.get_mut(entity) {
            let node_size = Size::new(
                text_constraint(
                    style.min_width,
                    style.width,
                    style.max_width,
                    scale_factor,
                ),
                text_constraint(
                    style.min_height,
                    style.height,
                    style.max_height,
                    scale_factor,
                ),
            );

            match text_pipeline.queue_text(
                entity,
                &fonts,
                &text.sections,
                scale_factor,
                text.alignment,
                node_size,
                &mut *font_atlas_set_storage,
                &mut *texture_atlases,
                &mut *textures,
            ) {
                Err(TextError::NoSuchFont) => {
                    // There was an error processing the text layout, let's add this entity to the
                    // queue for further processing
                    new_queue.push(entity);
                }
                Err(e @ TextError::FailedToAddGlyph(_)) => {
                    panic!("Fatal error when processing text: {}.", e);
                }
                Ok(()) => {
                    let text_layout_info = text_pipeline.get_glyphs(&entity).expect(
                        "Failed to get glyphs from the pipeline that have just been computed",
                    );
                    calculated_size.size = Size {
                        width: scale_value(text_layout_info.size.width, inv_scale_factor),
                        height: scale_value(text_layout_info.size.height, inv_scale_factor),
                    };
                }
            }
        }
    }

    queued_text.entities = new_queue;
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn draw_text_system(
    mut context: DrawContext,
    msaa: Res<Msaa>,
    windows: Res<Windows>,
    meshes: Res<Assets<Mesh>>,
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    text_pipeline: Res<DefaultTextPipeline>,
    mut query: Query<
        (Entity, &mut Draw, &Visible, &Text, &UINode, &GlobalTransform),
        Without<OutsideFrustum>,
    >,
) {
    let scale_factor = if let Some(window) = windows.get_primary() {
        window.scale_factor()
    } else {
        1.
    };

    let font_quad = meshes.get(&QUAD_HANDLE).unwrap();
    let vertex_buffer_layout = font_quad.get_vertex_buffer_layout();

    for (entity, mut draw, visible, text, node, global_transform) in query.iter_mut() {
        if !visible.is_visible {
            continue;
        }

        if let Some(text_glyphs) = text_pipeline.get_glyphs(&entity) {
            let mut drawable_text = DrawableText {
                render_resource_bindings: &mut render_resource_bindings,
                global_transform: *global_transform,
                scale_factor: scale_factor as f32,
                msaa: &msaa,
                text_glyphs: &text_glyphs.glyphs,
                font_quad_vertex_layout: &vertex_buffer_layout,
                sections: &text.sections,
                alignment_offset: (node.size / -2.0).extend(0.0),
            };

            drawable_text.draw(&mut draw, &mut context).unwrap();
        }
    }
}
