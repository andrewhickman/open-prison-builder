use std::{any::Any, sync::Arc};

use bevy::{ecs::system::SystemId, prelude::*, ui::FocusPolicy, utils::HashMap};

use crate::theme::ButtonTheme;

#[derive(Default, Component)]
pub struct ButtonCommand {
    id: &'static str,
    input: ButtonCommandInput,
}

#[derive(Default, Clone)]
pub struct ButtonCommandInput {
    any: Option<Arc<dyn Any + Send + Sync>>,
}

#[derive(Default, Resource)]
pub struct ButtonCommandDefinitions(HashMap<&'static str, SystemId<ButtonCommandInput>>);

#[derive(Default, Bundle)]
pub struct ButtonBundle {
    pub theme: ButtonTheme,
    pub command: ButtonCommand,
    pub node: Node,
    pub button: Button,
    pub style: Style,
    pub interaction: Interaction,
    pub focus_policy: FocusPolicy,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub image: UiImage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub z_index: ZIndex,
}

pub fn register_button_command<M, S>(app: &mut App, command: ButtonCommand, system: S)
where
    S: IntoSystem<ButtonCommandInput, (), M> + 'static,
{
    let system_id = app.world.register_system(system);
    app.world
        .get_resource_or_insert_with(ButtonCommandDefinitions::default)
        .0
        .insert(command.id, system_id);
}

pub fn on_button_press(
    mut commands: Commands,
    definitions: Res<ButtonCommandDefinitions>,
    interaction_q: Query<(&Interaction, &ButtonCommand), Changed<Interaction>>,
) {
    for (&interaction, button_callback) in &interaction_q {
        if interaction == Interaction::Pressed && !button_callback.id.is_empty() {
            info!("Invoking command {}", button_callback.id);
            commands.run_system_with_input(
                definitions.0[&button_callback.id],
                button_callback.input.clone(),
            );
        }
    }
}

impl ButtonCommand {
    pub const fn new(id: &'static str) -> Self {
        ButtonCommand {
            id,
            input: ButtonCommandInput { any: None },
        }
    }

    pub fn with_input<T>(self, input: T) -> Self
    where
        T: Any + Send + Sync,
    {
        ButtonCommand {
            id: self.id,
            input: ButtonCommandInput {
                any: Some(Arc::new(input)),
            },
        }
    }
}

impl ButtonCommandInput {
    pub fn get<T>(&self) -> &T
    where
        T: Any,
    {
        self.any
            .as_ref()
            .expect("button input not set")
            .downcast_ref::<T>()
            .expect("button input type not of expected type")
    }
}
