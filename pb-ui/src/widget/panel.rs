use bevy::{prelude::*, ui::FocusPolicy};

use pb_assets::AssetHandles;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w> UiBuilder<'w, '_> {
    pub fn panel(&mut self, theme: &Theme, style: Node) -> UiBuilder<'w, '_> {
        self.spawn((
            Node {
                padding: UiRect::all(theme.gutter),
                ..style
            },
            BackgroundColor(theme.panel),
            FocusPolicy::Block,
            Pickable::IGNORE,
            theme.outline,
        ))
    }

    pub fn panel_close_button(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
        panel_id: Entity,
    ) -> UiBuilder<'w, '_> {
        self.icon_button(theme, assets.close_icon.clone(), theme.icon_size())
            .on_click(
                move |_: Trigger<'_, Pointer<Click>>, mut commands: Commands| {
                    commands.entity(panel_id).despawn()
                },
            );
        self.reborrow()
    }
}
