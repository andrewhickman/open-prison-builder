use bevy::{ecs::world::OnDespawn, prelude::*};
use pb_assets::AssetHandles;
use pb_engine::pawn::ai::path::PathQuery;
use pb_render::pawn::PawnHighlight;

use crate::{
    action::Action,
    input::picking::{
        physics::pawn::{CancelPawn, ClickPawn, SelectPawn},
        point::ClickPoint,
    },
    theme::Theme,
};

#[derive(Default, Debug, Component, TypePath)]
#[require(Action, Name = Name::new(DefaultAction::type_path()))]
pub struct DefaultAction {
    highlight: Option<Entity>,
    state: DefaultActionState,
}

#[derive(Default, Debug)]
enum DefaultActionState {
    #[default]
    Default,
    SelectedPawn {
        pawn: Entity,
        highlight: Entity,
    },
}

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        DefaultAction::default(),
        children![
            Observer::new(select_pawn),
            Observer::new(cancel_pawn),
            Observer::new(click_pawn),
            Observer::new(click_point),
        ],
    ));
}

pub fn cancel(
    _: Trigger<OnDespawn, DefaultAction>,
    mut commands: Commands,
    mut action: Single<&mut DefaultAction>,
) -> Result {
    action.cancel(&mut commands)
}

fn select_pawn(
    trigger: Trigger<SelectPawn>,
    mut action: Single<&mut DefaultAction>,
    mut commands: Commands,
    assets: Res<AssetHandles>,
    theme: Res<Theme>,
) -> Result {
    action.select_pawn(trigger.pawn, &mut commands, &assets, &theme)
}

fn cancel_pawn(
    _: Trigger<CancelPawn>,
    mut action: Single<&mut DefaultAction>,
    mut commands: Commands,
) -> Result {
    action.cancel_pawn(&mut commands)
}

fn click_pawn(
    trigger: Trigger<ClickPawn>,
    mut action: Single<&mut DefaultAction>,
    mut commands: Commands,
    assets: Res<AssetHandles>,
    theme: Res<Theme>,
) -> Result {
    action.click_pawn(trigger.pawn, &mut commands, &assets, &theme)
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut commands: Commands,
    mut action: Single<&mut DefaultAction>,
    path_q: PathQuery,
) -> Result {
    action.click_point(&mut commands, &path_q, trigger.point)
}

impl DefaultAction {
    fn select_pawn(
        &mut self,
        pawn: Entity,
        commands: &mut Commands,
        assets: &AssetHandles,
        theme: &Theme,
    ) -> Result {
        if !self.is_selected(pawn) {
            self.cancel_pawn(commands)?;

            let highlight = commands
                .spawn(PawnHighlight::bundle(
                    assets,
                    pawn,
                    theme.accent.with_alpha(0.38),
                ))
                .id();
            self.highlight = Some(highlight);
        }

        Ok(())
    }

    fn cancel_pawn(&mut self, commands: &mut Commands) -> Result {
        if let Some(highlight) = self.highlight.take() {
            commands.entity(highlight).despawn();
        }
        Ok(())
    }

    fn click_pawn(
        &mut self,
        pawn: Entity,
        commands: &mut Commands,
        assets: &AssetHandles,
        theme: &Theme,
    ) -> Result {
        if !self.is_selected(pawn) {
            self.cancel(commands)?;

            let highlight = commands
                .spawn(PawnHighlight::bundle(
                    assets,
                    pawn,
                    theme.accent.with_alpha(0.88),
                ))
                .id();
            self.state = DefaultActionState::SelectedPawn { pawn, highlight };
        }

        Ok(())
    }

    fn click_point(&mut self, commands: &mut Commands, path_q: &PathQuery, to: Vec2) -> Result {
        match self.state {
            DefaultActionState::Default => (),
            DefaultActionState::SelectedPawn { pawn, .. } => {
                info!("move {pawn} to {to}");
                match path_q.path(pawn, to) {
                    Some(path) => {
                        commands.spawn(path);
                    }
                    None => warn!("no path found for {pawn} to {to}"),
                }
            }
        }

        Ok(())
    }

    fn cancel(&mut self, commands: &mut Commands) -> Result {
        self.cancel_pawn(commands)?;

        if let DefaultActionState::SelectedPawn { highlight, .. } = self.state {
            commands.entity(highlight).despawn();
        }

        Ok(())
    }

    fn is_selected(&self, entity: Entity) -> bool {
        match self.state {
            DefaultActionState::Default => false,
            DefaultActionState::SelectedPawn { pawn, .. } => pawn == entity,
        }
    }
}
