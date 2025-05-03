use std::time::Duration;

use anyhow::Result;

use bevy::prelude::*;
use pb_engine::{
    save::{save, SaveParam},
    EngineState,
};
use pb_store::Store;
use pb_util::{callback::CallbackSender, spawn_io, AsDynError};

use crate::message::Message;

pub const INTERVAL: Duration = Duration::from_secs(5 * 60);

pub fn run_condition(
    state: Res<State<EngineState>>,
    time: Res<Time<Real>>,
    mut timer: Local<Option<Timer>>,
) -> bool {
    if !matches!(state.get(), EngineState::Running(_)) {
        return false;
    }

    let timer = timer.get_or_insert_with(|| Timer::new(INTERVAL, TimerMode::Repeating));
    timer.tick(time.delta());
    timer.just_finished()
}

pub fn run(
    world: &World,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: Res<Store>,
    callback: Res<CallbackSender>,
) {
    let EngineState::Running(root) = *state.get() else {
        error!("Failed to autosave: not running");
        return;
    };

    let scene = save(world, &save_p, root);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.set("saves/autosave.json", scene).await;
        callback.send_oneshot_system_with_input(on_save_complete, res);
    });

    fn on_save_complete(In(res): In<Result<()>>, mut message_e: EventWriter<Message>) {
        match res {
            Ok(()) => {
                message_e.write(Message::info("Autosave succeeded"));
            }
            Err(error) => {
                error!(error = error.as_dyn_error(), "Autosave failed");
                message_e.write(Message::error(&error));
            }
        }
    }
}
