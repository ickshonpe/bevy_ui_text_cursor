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
        &UiTextCursorStyle { cursor_color, cursor_min, cursor_max },
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
                    let atlas = texture_atlases
                        .get(text_glyph.atlas_info.texture_atlas.clone_weak())
                        .unwrap();
                    // let texture = atlas.texture.clone_weak();
                    let index = text_glyph.atlas_info.glyph_index as usize;
                    let rect = atlas.textures[index];
                    let atlas_size = Some(atlas.size);

                    let transform =
                        Mat4::from_rotation_translation(transform.rotation, transform.translation)
                            * Mat4::from_scale(transform.scale / scale_factor)
                            * Mat4::from_translation(
                                alignment_offset * scale_factor + text_glyph.position.extend(0.),
                            );

                    extracted_uinodes.uinodes.push(ExtractedUiNode {
                        transform,
                        color: cursor_color,
                        rect,
                        image: DEFAULT_IMAGE_HANDLE.typed(),
                        atlas_size,
                        clip: clip.map(|clip| clip.clip),
                    });
                }
            }
        }
    }
}
