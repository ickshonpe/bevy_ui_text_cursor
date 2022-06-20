# Bevy Ui Text Cursor

text cursor for bevy_ui

supports Bevy 0.7

## Usage

Add the plugin to your app

```rust
app.add_plugin(bevy_ui_text_cursor::BevyUiTextCursorPlugin)
```

Then you can spawn a text bundle with a cursor like so:

```rust
commands.spawn_bundle(
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Hello, World".to_string(), 
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
    }
)
.insert(UiTextCursor::default())
.insert(UiTextCursorStyle::default());
```

And it will draw a cursor over the H.
UiTextCursor is just a newtype containing a usize.
If UiTextCursor contains N then the cursor will be drawn over the Nth glyph in the Text component.
There is also a ```UiTextCursorBlink``` component you can add to get a blinking cursor.

## Examples

```
cargo run --example hello_world
```

## Notes

Very rough work in progress. Not tested. Not spent even in an hour working on this crate. Many problems:

* The cursor doesn't stop at the last glyph
* Control characters like ```\n``` etc will probably mess things up.
* You have to handle inputs and moving the cursor yourself.
* The cursor is the size of the glyph its drawn over, and isn't drawn at all over spaces.

All are easily fixable though.