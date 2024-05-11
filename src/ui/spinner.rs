use std::f32;

use bevy::{
    ecs::{prelude::*, system::EntityCommands},
    hierarchy::{BuildChildren, Children},
    math::{Quat, Rect, Vec2},
    render::{
        color::Color,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    time::Time,
    transform::components::{GlobalTransform, Transform},
    ui::{
        node_bundles::NodeBundle, BackgroundColor, BorderColor, FocusPolicy, Node, PositionType,
        Style, Val, ZIndex,
    },
};

#[derive(Bundle, Default)]
pub struct SpinnerBundle {
    pub spinner: Spinner,
    pub node: Node,
    pub style: Style,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub focus_policy: FocusPolicy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub z_index: ZIndex,
}

#[derive(Default, Component)]
pub struct Spinner {
    pub size: f32,
    pub color: BackgroundColor,
    pub progress: f32,
}

#[derive(Component)]
pub struct SpinnerSpoke {
    index: u32,
}

const SPOKE_COUNT: u32 = 12;

pub fn spawn_spinner<'c>(
    commands: &'c mut Commands,
    mut spinner: SpinnerBundle,
) -> EntityCommands<'c> {
    let color = spinner.background_color;
    let size = spinner.spinner.size;
    let progress = spinner.spinner.progress;

    spinner.style.width = Val::Px(size);
    spinner.style.height = Val::Px(size);
    spinner.background_color.0 = Color::rgba(0., 0., 0., 0.0);

    let mut commands = commands.spawn(spinner);

    let center = Vec2::splat(size / 2.0);
    let spoke_rect = Rect::new(
        center.x + size * 0.25,
        center.y + size * 0.04,
        center.x + size * 0.5,
        center.y - size * 0.04,
    );

    commands.with_children(|builder| {
        for index in 0..SPOKE_COUNT {
            let angle = ((index as f32 + 0.5) / SPOKE_COUNT as f32) * f32::consts::TAU;

            let mut transform = Transform::IDENTITY;
            transform.rotate_around(center.extend(0.0), Quat::from_rotation_z(angle));
            let rotated_center = transform
                .transform_point(spoke_rect.center().extend(0.))
                .truncate();

            builder.spawn((
                SpinnerSpoke { index },
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(rotated_center.x - spoke_rect.width() / 2.0),
                        top: Val::Px(rotated_center.y - spoke_rect.height() / 2.0),
                        width: Val::Px(spoke_rect.width()),
                        height: Val::Px(spoke_rect.height()),
                        ..Default::default()
                    },
                    background_color: spoke_color(color, progress, index),
                    transform: Transform::from_rotation(transform.rotation),
                    ..Default::default()
                },
            ));
        }
    });

    commands
}

pub fn update_spinners(
    time: Res<Time>,
    mut spinner_q: Query<(&mut Spinner, &Children)>,
    mut spoke_q: Query<(&SpinnerSpoke, &mut BackgroundColor)>,
) {
    for (mut spinner, children) in spinner_q.iter_mut() {
        spinner.progress = (spinner.progress + time.delta_seconds()).rem_euclid(1.0);

        for &spoke in children {
            if let Ok((spoke, mut color)) = spoke_q.get_mut(spoke) {
                *color = spoke_color(spinner.color, spinner.progress, spoke.index)
            }
        }
    }
}

fn spoke_color(original: BackgroundColor, progress: f32, index: u32) -> BackgroundColor {
    let step = (index as f32 + 0.5) / SPOKE_COUNT as f32;
    let fade = (progress - step).rem_euclid(1.0);

    BackgroundColor(original.0.with_a(original.0.a() * fade))
}
