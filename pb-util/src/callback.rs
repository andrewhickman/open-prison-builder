use std::future::Future;

use bevy::{
    ecs::{
        error::HandleError,
        system::command::{run_system_cached, run_system_cached_with},
        world::CommandQueue,
    },
    prelude::*,
    tasks::{AsyncComputeTaskPool, IoTaskPool},
};
use crossbeam_channel::{Receiver, Sender};

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_io(f: impl Future<Output = ()> + Send + 'static) {
    IoTaskPool::get().spawn(f).detach();
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_io(f: impl Future<Output = ()> + 'static) {
    IoTaskPool::get().spawn(f).detach();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_compute(f: impl Future<Output = ()> + Send + 'static) {
    AsyncComputeTaskPool::get().spawn(f).detach();
}

#[cfg(target_arch = "wasm32")]
pub fn spawn_compute(f: impl Future<Output = ()> + 'static) {
    AsyncComputeTaskPool::get().spawn(f).detach();
}

pub struct CallbackPlugin;

#[derive(Clone, Resource)]
pub struct CallbackSender(Sender<CommandQueue>);

#[derive(Resource)]
pub struct CallbackReceiver(Receiver<CommandQueue>);

impl Plugin for CallbackPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = crossbeam_channel::unbounded();
        app.insert_resource(CallbackSender(sender));
        app.insert_resource(CallbackReceiver(receiver));

        app.add_systems(Update, apply_callbacks);
    }
}

impl CallbackSender {
    pub fn send(&self, command: impl Command) {
        let mut queue = CommandQueue::default();
        queue.push(command);
        self.send_batch(queue);
    }

    pub fn run_system_cached<S>(&self, system: S)
    where
        S: System<In = (), Out = ()> + Send + 'static,
    {
        self.send(run_system_cached(system).handle_error())
    }

    pub fn run_system_cached_with<I, S, M>(&self, system: S, input: I)
    where
        I: Send + 'static,
        S: IntoSystem<In<I>, (), M> + Send + 'static,
        M: 'static,
    {
        self.send(run_system_cached_with(system, input).handle_error())
    }

    pub fn send_batch(&self, queue: CommandQueue) {
        if let Err(err) = self.0.send(queue) {
            warn!("Dropping callback commands: {}", err);
        }
    }
}

pub fn apply_callbacks(mut commands: Commands, receiver: Res<CallbackReceiver>) {
    while let Ok(mut command) = receiver.0.try_recv() {
        commands.append(&mut command);
    }
}
