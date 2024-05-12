use bevy::{ecs::system::SystemId, prelude::*, ui::FocusPolicy, utils::HashMap};

use crate::theme::ButtonTheme;

#[derive(Clone, Copy, Default, Component, Hash, PartialEq, Eq)]
pub struct ButtonCommand(pub &'static str);

#[derive(Default, Resource)]
pub struct CallbackDefinitions(HashMap<ButtonCommand, SystemId>);

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

pub fn register_button_command<M, S>(app: &mut App, id: ButtonCommand, system: S)
where
    S: IntoSystem<(), (), M> + 'static,
{
    let system_id = app.world.register_system(system);
    app.world
        .get_resource_or_insert_with(CallbackDefinitions::default)
        .0
        .insert(id, system_id);
}

pub fn on_button_press(
    mut commands: Commands,
    definitions: Res<CallbackDefinitions>,
    interaction_q: Query<(&Interaction, &ButtonCommand), Changed<Interaction>>,
) {
    for (&interaction, &button_callback) in &interaction_q {
        if interaction == Interaction::Pressed && button_callback != ButtonCommand::default() {
            commands.run_system(definitions.0[&button_callback]);
        }
    }
}
