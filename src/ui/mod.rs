mod menu_bar;
mod spinner;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::loading::TextureAssets;
use crate::{theme::Theme, GameState};

pub use self::menu_bar::spawn_menu_bar;
pub use self::spinner::{spawn_spinner, Spinner, SpinnerBundle};

use crate::material::Material;

pub struct UiPlugin;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct GameContent;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spinner::update_spinners, menu_bar::on_play_button_clicked).run_if(in_state(GameState::Running)),
        );
        app.add_systems(OnEnter(GameState::Running), spawn_game_ui);
        app.add_systems(OnExit(GameState::Running), despawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, theme: Res<Theme>) {
    let ui_root = commands
        .spawn((
            GameUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            GameContent,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Interaction::default(),
        ))
        .set_parent(ui_root);

    spawn_menu_bar(
        &mut commands,
        theme,
        vec![
            ("Build".to_owned(), Box::new(spawn_build_menu)),
            ("View".to_owned(), Box::new(spawn_empty_menu)),
            ("Manage".to_owned(), Box::new(spawn_empty_menu)),
        ],
    )
    .set_parent(ui_root);
}

fn spawn_empty_menu(_: &mut ChildBuilder, _: &Theme, _: &TextureAssets) {
}

fn spawn_build_menu(commands: &mut ChildBuilder, theme: &Theme, assets: &TextureAssets) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    align_self: AlignSelf::Start,
                    display: Display::Grid,
                    flex_direction: FlexDirection::Row,
                    grid_template_columns: vec![RepeatedGridTrack::repeat_many(GridTrackRepetition::AutoFill, vec![GridTrack::px(100.)])],
                    grid_template_rows: vec![RepeatedGridTrack::repeat_many(GridTrackRepetition::AutoFill, vec![GridTrack::px(100.)])],
                    grid_auto_flow: GridAutoFlow::Row,
                    column_gap: Val::Px(10.),
                    row_gap: Val::Px(10.),
                    padding: UiRect::all(Val::Px(10.)),
                    border: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                border_color: BorderColor(theme.text()),
                background_color: BackgroundColor(theme.ui_background()),
                focus_policy: FocusPolicy::Block,
                ..Default::default()
            },
        )).with_children(|builder| {
            for mat in Material::iter() {
                builder.spawn((
                    AtlasImageBundle {
                        image: UiImage::new(assets.atlas.clone()),
                        texture_atlas: TextureAtlas {
                            index: mat.index(TilePos::default()).0 as usize,
                            layout: assets.atlas_layout.clone(),
                        },
                        ..Default::default()
                    },
                ));
            }
        });
}

fn despawn_game_ui(mut commands: Commands, ui_q: Query<Entity, With<GameUi>>) {
    for entity in ui_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
