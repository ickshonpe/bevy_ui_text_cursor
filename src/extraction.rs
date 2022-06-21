use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::text::DefaultTextPipeline;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::window::WindowId;

use crate::UiTextCursor;
use crate::UiTextCursorBlink;
use crate::UiTextCursorStyle;


pub fn extract_ui_text_cursor(
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    text_pipeline: Res<DefaultTextPipeline>,
    windows: Res<Windows>,
    uinode_query: Query<(
        Entity,
        &Node,
        &GlobalTransform,
        &Text,
        &Visibility,
        &UiTextCursor,
        &UiTextCursorStyle,
        Option<&UiTextCursorBlink>,
        Option<&CalculatedClip>,
    )>,
) {
    let mut extracted_uinodes = render_world.resource_mut::<ExtractedUiNodes>();

    let scale_factor = windows.scale_factor(WindowId::primary()) as f32;

    for (entity, 
        uinode, 
        transform,
        text, 
        visibility,
        &UiTextCursor(cursor_index),
        style,
        blink,
        clip
    ) in uinode_query.iter() {

        if !visibility.is_visible {
            continue;
        }
        // Skip if size is set to zero (e.g. when a parent is set to `Display::None`)
        if uinode.size == Vec2::ZERO {
            continue;
        }
        if let Some(blink) = blink {
            if !blink.on {
                continue;
            }
        }
        if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
            let text_glyphs = &text_layout.glyphs;
            let alignment_offset = (uinode.size / -2.0).extend(0.0);

            for (glyph_index, text_glyph) in text_glyphs.iter().enumerate() {
                // let color = text.sections[text_glyph.section_index].style.color;
                if glyph_index == cursor_index {
                    let auto_height =
                        text_glyphs.iter().max_by(|g, h|
                            g.size.y.partial_cmp(&h.size.y).unwrap_or(std::cmp::Ordering::Equal)
                        )
                        .map(|g| g.size.y)
                        .unwrap_or(0.0);
                    let auto_width =
                        text_glyphs.iter().max_by(|g, h|
                            g.size.x.partial_cmp(&h.size.x).unwrap_or(std::cmp::Ordering::Equal)
                        )
                        .map(|g| g.size.x)
                        .unwrap_or(0.0);
                    let atlas = texture_atlases
                        .get(text_glyph.atlas_info.texture_atlas.clone_weak())
                        .unwrap();
                    let index = text_glyph.atlas_info.glyph_index as usize;
                    
                    let transform =
                        Mat4::from_rotation_translation(transform.rotation, transform.translation)
                            * Mat4::from_scale(transform.scale / scale_factor)
                            * Mat4::from_translation(
                                alignment_offset * 
                                scale_factor + text_glyph.position.extend(0.),
                            );
                    
                        
                    let cursor_height = match style.cursor_height {
                        Val::Undefined => 0.0,
                        Val::Auto => auto_height * 1.2,
                        Val::Px(height) => height,
                        Val::Percent(p) => 100_f32.recip() * p * auto_height,
                    };

                    let cursor_width = match style.cursor_width {
                        Val::Undefined => 0.0,
                        Val::Auto => auto_width,
                        Val::Px(width) => width,
                        Val::Percent(p) => 100_f32.recip() * p * text_glyph.size.x
                    };

                    let cursor_size = vec2(cursor_width, cursor_height);

                    let rect = bevy::sprite::Rect { min: Vec2::ZERO, max: cursor_size };
                    extracted_uinodes.uinodes.push(ExtractedUiNode {
                        transform,
                        color: style.cursor_color,
                        rect,
                        image: DEFAULT_IMAGE_HANDLE.typed(),
                        atlas_size: None,
                        clip: clip.map(|clip| clip.clip),
                    });
                }
            }
        }
    }
}
