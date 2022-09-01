use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(change_color)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let outer_text = commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Outside",
                TextStyle {
                    color: Color::WHITE,
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
                default(),
            ),
            ..default()
        })
        .id();

    let _outer_button = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .push_children(&[outer_text])
        .id();

    let inner_text = commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Inside",
                TextStyle {
                    color: Color::WHITE,
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
                default(),
            ),
            ..default()
        })
        .id();

    let inner_button = commands
        .spawn_bundle(ButtonBundle {
            color: NORMAL_BUTTON.into(),
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .push_children(&[inner_text])
        .id();

    let broken_text = commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Broken",
                TextStyle {
                    color: Color::WHITE,
                    font_size: 16.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
                default(),
            ),
            ..default()
        })
        .id();

    let _container = commands
        .spawn_bundle(ButtonBundle {
            color: Color::BLACK.into(),
            style: Style {
                margin: Rect::all(Val::Auto),
                padding: Rect::all(Val::Px(50.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(Interaction::default())
        .push_children(&[inner_button, broken_text])
        .id();
}

fn change_color(
    mut query: Query<(&Interaction, &mut UiColor), (With<Button>, Changed<Interaction>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        color.0 = match interaction {
            Interaction::Clicked => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}
