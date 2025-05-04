use std::f32;

use bevy::prelude::*;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w> UiBuilder<'w, '_> {
    pub fn spinner(&mut self, theme: &Theme, size: f32) -> UiBuilder<'w, '_> {
        let mut parent = self.spawn((
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            Spinner::default(),
        ));

        let center = Vec2::splat(size / 2.0);
        let spoke_rect = Rect::new(
            center.x + size * 0.25,
            center.y + size * 0.04,
            center.x + size * 0.5,
            center.y - size * 0.04,
        );

        for index in 0..SPOKE_COUNT {
            let angle = ((index as f32 + 0.5) / SPOKE_COUNT as f32) * f32::consts::TAU;

            let mut transform = Transform::IDENTITY;
            transform.rotate_around(center.extend(0.0), Quat::from_rotation_z(angle));
            let rotated_center = transform
                .transform_point(spoke_rect.center().extend(0.))
                .truncate();

            parent.spawn((
                SpinnerSpoke { index },
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(rotated_center.x - spoke_rect.width() / 2.0),
                    top: Val::Px(rotated_center.y - spoke_rect.height() / 2.0),
                    width: Val::Px(spoke_rect.width()),
                    height: Val::Px(spoke_rect.height()),
                    ..Default::default()
                },
                spoke_color(theme, 0.0, index),
                Transform::from_rotation(transform.rotation),
            ));
        }

        parent
    }
}

#[derive(Default, Component)]
pub struct Spinner {
    progress: f32,
}

#[derive(Component)]
pub struct SpinnerSpoke {
    index: u32,
}

const SPOKE_COUNT: u32 = 12;

pub fn update(
    time: Res<Time>,
    theme: Res<Theme>,
    mut spinner_q: Query<(&mut Spinner, &Children)>,
    mut spoke_q: Query<(&SpinnerSpoke, &mut BackgroundColor)>,
) -> Result {
    for (mut spinner, children) in spinner_q.iter_mut() {
        spinner.progress = (spinner.progress + time.delta_secs()).rem_euclid(1.0);

        for &spoke in children {
            let (spoke, mut color) = spoke_q.get_mut(spoke)?;
            *color = spoke_color(&theme, spinner.progress, spoke.index)
        }
    }
    Ok(())
}

fn spoke_color(theme: &Theme, progress: f32, index: u32) -> BackgroundColor {
    let step = (index as f32 + 0.5) / SPOKE_COUNT as f32;
    let fade = (progress - step).rem_euclid(1.0);

    BackgroundColor(theme.text.with_alpha(theme.text.alpha() * fade))
}
