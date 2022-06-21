mod extraction;

use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::text::DefaultTextPipeline;
use bevy::ui::RenderUiSystem;
use extraction::extract_ui_text_cursor;

#[derive(Clone, Copy, Debug, Default, Deref, DerefMut)]
#[derive(Component)]
pub struct UiTextCursor(pub usize);

#[derive(Clone, Debug)]
#[derive(Component)]
pub struct UiTextCursorBlink {
    pub on: bool,
    pub remaining: f32,
    pub time_on: f32,
    pub time_off: f32,
}

impl Default for UiTextCursorBlink {
    fn default() -> Self {
        let t = 0.2;
        Self { 
            on: false,
            remaining: t,
            time_on: 2. * t, 
            time_off: t,
        }
    }
}



#[derive(Clone, Debug)]
#[derive(Component)]
pub struct UiTextCursorStyle {
    pub cursor_color: Color,
    pub cursor_height: Val,
    pub cursor_width: Val,
}

impl Default for UiTextCursorStyle {
    fn default() -> Self {
        Self {
            cursor_color: Color::WHITE,
            cursor_height: Val::Auto,
            cursor_width: Val::Auto
        }
    }
}

fn update_cursor_blink(
    time: Res<Time>,
    mut query: Query<&mut UiTextCursorBlink>,
) {
    query.for_each_mut(|mut blinker| {
        blinker.remaining -= time.delta_seconds();
        if blinker.remaining <= 0. {
            blinker.on = !blinker.on;
            blinker.remaining = 
                if blinker.on {
                    blinker.time_on
                } else {
                    blinker.time_off
                };
        }
    });
}

fn report_text_details(
    mut flag: Local<bool>,
    text_pipeline: Res<DefaultTextPipeline>,
    uinode_query: Query<(
        Entity,
        &Text,
        &Node,
        &GlobalTransform,
        &Visibility,
        &UiTextCursor,
        &UiTextCursorStyle,
        Option<&UiTextCursorBlink>,
        Option<&CalculatedClip>,
    )>,
) {
    if !*flag {
        uinode_query.for_each(|(entity, text, ..)| {
            if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
                let text_glyphs = &text_layout.glyphs;
                println!("text details");
                let sections = 
                    text.sections.iter().map(|section|
                        section.value.as_str()
                    );
                println!("\tsections: {:?}", sections);
                println!("\tglyph count: {}", text_glyphs.len());
                println!("\tfont? size: {:#.1?}", text_layout.size);
                for (i, positioned_glyph) in text_glyphs.iter().enumerate() {
                    println!("glyph: {i}, {:#?}", positioned_glyph);
                }
                *flag = true;
            }
        });
    }
}

pub struct BevyUiTextCursorPlugin;

impl Plugin for BevyUiTextCursorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(
            CoreStage::PostUpdate, update_cursor_blink
        )
        .add_system(
            report_text_details
        );

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app.add_system_to_stage(
            RenderStage::Extract, 
            extract_ui_text_cursor.after(RenderUiSystem::ExtractNode)
        );
    
    }
}