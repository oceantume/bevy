use bevy::{
    prelude::*,
    text::{DefaultTextPipeline, TextCaret, TextSelection},
    winit::WinitSettings,
};

/// This example illustrates how to spawn a text input and use its value.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(test1)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "He\nllo, ".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            },
                            TextSection {
                                value: "Wor".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            },
                            TextSection {
                                value: "ld!".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..default()
                })
                .insert(TextCaret { position: 5 })
                .insert(TextSelection { range: 0..7 });
        });
}

#[derive(Component)]
struct TextInputInfo {
    // TODO: store entities for text?
}

fn test1(
    text_pipeline: Res<DefaultTextPipeline>,
    uinode_query: Query<(
        Entity,
        &Node,
        &GlobalTransform,
        &Text,
        &Visibility,
        Option<&CalculatedClip>,
    )>,
) {
    for (entity, uinode, transform, text, visibility, clip) in uinode_query.iter() {
        if !visibility.is_visible {
            continue;
        }
        // Skip if size is set to zero (e.g. when a parent is set to `Display::None`)
        if uinode.size == Vec2::ZERO {
            continue;
        }
        if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
            let text_glyphs = &text_layout.glyphs;
            let alignment_offset = (uinode.size / -2.0).extend(0.0);

            for text_glyph in text_glyphs {
                //println!("{:?} {:?}", transform, uinode.size);
                //println!("{:?}", text_glyph);

                // todo: try to add a point to the location of each character.
                // todo: add a background for each?

                // we can probably create a UI node per character for this, at least for now.
                //  an optimization would be to create one node per section (on the same line if multi line is supported).

                // instead of doing this, could we have a secondary extract that renders selection?
                //  this could actually be part of the main extraction with Optional<TextSelection> or something. Because we'll eventually need to change text color as well.
                // then we could use the text pipeline stuff to get bounds for clicking & hover later or something
                // this can save us a bunch of trouble with storing extra information per character, and trying to match ui nodes with characters.
            }
        }
    }
}
