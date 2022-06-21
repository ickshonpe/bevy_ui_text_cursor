mod extraction;

use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
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
    pub cursor_min: Vec2,
    pub cursor_max: Vec2,
}

impl Default for UiTextCursorStyle {
    fn default() -> Self {
        Self {
            cursor_color: Color::WHITE,
            cursor_min: Vec2::ZERO,
            cursor_max: 8.0 * (2. * Vec2::X + 3. * Vec2::Y),
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

pub struct BevyUiTextCursorPlugin;

impl Plugin for BevyUiTextCursorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(
            CoreStage::PostUpdate, update_cursor_blink
        );

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        render_app.add_system_to_stage(
            RenderStage::Extract, 
            extract_ui_text_cursor.after(RenderUiSystem::ExtractNode)
        );
    
    }
}