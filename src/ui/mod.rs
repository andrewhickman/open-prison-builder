mod button;
mod main_menu;
mod menu_bar;
mod spinner;

use bevy::prelude::*;
use bevy::ui::{FocusPolicy, UiSystem};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::commands;
use crate::loading::TextureAssets;
use crate::theme::ButtonTheme;
use crate::{theme::Theme, GameState};

pub use self::button::{
    register_button_command, ButtonCommand, ButtonCommandDefinitions, ButtonCommandInput,
};
pub use self::menu_bar::spawn_menu_bar;
pub use self::spinner::{spawn_spinner, Spinner, SpinnerBundle};

use self::main_menu::MainMenuPlugin;
use crate::material::Material;

pub struct UiPlugin;

#[derive(Resource)]
pub struct UiMarkers {
    pub root: Entity,
    pub content: Entity,
    pub menu_bar: Entity,
    pub top_left: Entity,
    pub top_right: Entity,
    pub bottom: Entity,
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub enum MenuKind {
    Build,
    View,
    Manage,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin);

        app.init_resource::<ButtonCommandDefinitions>();
        app.add_systems(PreUpdate, button::on_button_press.after(UiSystem::Focus));
        app.add_systems(
            Update,
            (spinner::update_spinners).run_if(in_state(GameState::Running)),
        );
        app.add_systems(OnEnter(GameState::Running), spawn_game_ui);
        app.add_systems(OnExit(GameState::Running), despawn_game_ui);

        register_button_command(app, commands::BUILD_MENU, spawn_build_menu);
        register_button_command(app, commands::VIEW_MENU, spawn_empty_menu);
        register_button_command(app, commands::MANAGE_MENU, spawn_empty_menu);
    }
}

fn spawn_game_ui(mut commands: Commands, theme: Res<Theme>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .id();

    let content = commands
        .spawn((
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
        .set_parent(root)
        .id();

    let body = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Stretch,
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            ..Default::default()
        })
        .set_parent(root)
        .id();

    let top = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Stretch,
                align_items: AlignItems::Stretch,
                width: Val::Percent(100.),
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .set_parent(body)
        .id();

    let top_left = commands
        .spawn((NodeBundle {
            style: Style {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },))
        .set_parent(top)
        .id();
    let top_right = commands
        .spawn((NodeBundle {
            style: Style {
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },))
        .set_parent(top)
        .id();

    let bottom = commands
        .spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_grow: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },))
        .set_parent(body)
        .id();

    let menu_bar = spawn_menu_bar(
        &mut commands,
        theme,
        vec![
            ("Build".to_owned(), commands::BUILD_MENU),
            ("View".to_owned(), commands::VIEW_MENU),
            ("Manage".to_owned(), commands::MANAGE_MENU),
        ],
    )
    .set_parent(body)
    .id();

    commands.insert_resource(UiMarkers {
        root,
        content,
        menu_bar,
        top_left,
        top_right,
        bottom,
    });
}

fn spawn_empty_menu(_: In<ButtonCommandInput>) {
    info!("hello!");
}

fn spawn_build_menu(
    _: In<ButtonCommandInput>,
    mut commands: Commands,
    markers: Res<UiMarkers>,
    theme: Res<Theme>,
    assets: Res<TextureAssets>,
    menu_q: Query<(Entity, &MenuKind)>,
) {
    if let Some((existing_menu, _)) = menu_q.iter().find(|(_, &k)| k == MenuKind::Build) {
        commands.entity(existing_menu).despawn_recursive();
        return;
    }

    commands.entity(markers.bottom).despawn_descendants();

    commands
        .spawn((
            MenuKind::Build,
            NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    align_self: AlignSelf::End,
                    display: Display::Grid,
                    flex_direction: FlexDirection::Row,
                    grid_template_columns: vec![RepeatedGridTrack::repeat_many(
                        GridTrackRepetition::AutoFill,
                        vec![GridTrack::px(100.)],
                    )],
                    grid_template_rows: vec![RepeatedGridTrack::repeat_many(
                        GridTrackRepetition::AutoFill,
                        vec![GridTrack::px(100.)],
                    )],
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
        ))
        .set_parent(markers.bottom)
        .with_children(|builder| {
            for mat in Material::iter() {
                builder.spawn((
                    AtlasImageBundle {
                        image: UiImage::new(assets.materials.clone()),
                        texture_atlas: TextureAtlas {
                            index: mat.index(TilePos::default()).0 as usize,
                            layout: assets.materials_layout.clone(),
                        },
                        ..Default::default()
                    },
                    Button,
                    ButtonTheme::Image,
                    Interaction::default(),
                    commands::SELECT_MATERIAL.with_input(mat),
                ));
            }
        });
}

fn despawn_game_ui(mut commands: Commands, markers: Res<UiMarkers>) {
    commands.remove_resource::<UiMarkers>();
    commands.entity(markers.root).despawn_recursive();
}
