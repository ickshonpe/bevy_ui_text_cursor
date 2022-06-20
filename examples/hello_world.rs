use bevy::input::keyboard::KeyboardInput;
use bevy_ui_text_cursor::*;
use bevy::prelude::*;

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .commands()
        .spawn_bundle(NodeBundle {
            color: UiColor(Color::MAROON),
            style: Style {
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Hello, ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "World".to_string(), 
                            style: TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            }
                        },
                    ],
                    alignment: TextAlignment { vertical: VerticalAlign::Center, horizontal:HorizontalAlign::Center },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(UiTextCursor::default())
            .insert(UiTextCursorBlink::default())
            .insert(UiTextCursorStyle::default());
        });
}

fn move_cursor(
    keys: Res<Input<KeyCode>>,
    mut cursor_query: Query<&mut UiTextCursor>,
    ) {
        cursor_query.for_each_mut(|mut cursor| {
            
            if keys.just_pressed(KeyCode::Left) && 0 < cursor.0 {
                cursor.0 -= 1;    
            }

            if keys.just_pressed(KeyCode::Right) {
                cursor.0 += 1;
            }
        });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(BevyUiTextCursorPlugin)
    .add_startup_system(setup)
    .add_system(move_cursor)
    .run();
}