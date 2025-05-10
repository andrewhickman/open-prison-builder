use std::time::Duration;

use bevy::prelude::*;
use pb_engine::{EngineState, save::SaveParam};
use pb_store::Store;
use pb_util::{callback::CallbackSender, spawn_io};

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

pub fn run(save_p: SaveParam, store: Res<Store>, callback: Res<CallbackSender>) -> Result {
    let scene = save_p.save()?;

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.set("saves/autosave.json", scene).await;
        callback.run_system_cached_with(on_save_complete, res);
    });

    fn on_save_complete(In(res): In<Result<()>>, mut message_e: EventWriter<Message>) {
        match res {
            Ok(()) => {
                message_e.write(Message::info("Autosave succeeded"));
            }
            Err(error) => {
                error!("Autosave failed: {error}");
                message_e.write(Message::error(&error));
            }
        }
    }

    Ok(())
}
