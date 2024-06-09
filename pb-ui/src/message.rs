use std::time::Duration;

use bevy::prelude::*;

use bevy_mod_picking::picking_core::Pickable;
use pb_assets::Assets;
use pb_util::AsDynError;

use crate::{layout::Layout, theme::Theme, widget::UiBuilder};

#[derive(Event, Debug, Clone)]
pub struct Message {
    text: String,
    level: MessageLevel,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MessageLevel {
    Info,
    Error,
}

#[derive(Component)]
pub struct MessageExpiry(Duration);

impl Message {
    pub fn info(text: impl Into<String>) -> Self {
        Message {
            text: text.into(),
            level: MessageLevel::Info,
        }
    }

    pub fn error<'a, M: ?Sized>(error: &impl AsDynError<'a, M>) -> Self {
        Message {
            text: error.to_string_compact(),
            level: MessageLevel::Error,
        }
    }
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn messages(&mut self) -> UiBuilder<'w, '_> {
        self.container(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::ZERO, Val::Auto, Val::ZERO, Val::Auto),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        })
    }

    pub fn message(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        time: &Time<Real>,
        message: &Message,
    ) -> UiBuilder<'w, '_> {
        match message.level {
            MessageLevel::Info => self
                .spawn(TextBundle::from_section(
                    message.text.clone(),
                    theme.normal_text.clone(),
                ))
                .insert(Pickable::IGNORE)
                .insert(MessageExpiry(
                    time.elapsed().saturating_add(Duration::from_secs(15)),
                )),
            MessageLevel::Error => self
                .error_message(theme, assets, message.text.clone())
                .insert(MessageExpiry(
                    time.elapsed().saturating_add(Duration::from_secs(30)),
                )),
        };

        self.reborrow()
    }
}

pub fn spawn_messages(
    commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    time: Res<Time<Real>>,
    mut message_e: EventReader<Message>,
    layout: Res<Layout>,
) {
    let mut builder = UiBuilder::new(commands, layout.messages);

    for message in message_e.read() {
        builder.message(&theme, &assets, &time, message);
    }
}

pub fn despawn_messages(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut timer_q: Query<(Entity, &MessageExpiry)>,
) {
    for (entity, expiry) in &mut timer_q {
        if expiry.0 < time.elapsed() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
