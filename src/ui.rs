use bevy::prelude::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_scoreboard);
    }
}

fn update_scoreboard(time: Res<Time>, mut query: Query<&mut Text, With<ScoreBoardUI>>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{:.1}", time.elapsed_seconds());
}

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Scoreboard
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                ),
                TextSection::new(
                    "TODO",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: SCOREBOARD_TEXT_PADDING,
                    left: SCOREBOARD_TEXT_PADDING,
                    ..default()
                },
                ..default()
            }),
        )
        .insert(ScoreBoardUI);
}

#[derive(Component)]
struct ScoreBoardUI;
