use std::future::Future;

use bevy::{
    ecs::{
        system::RunSystemCachedWith,
        world::{Command, CommandQueue},
    },
    prelude::*,
    tasks::IoTaskPool,
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

    pub fn send_oneshot_system<S>(&self, system: S)
    where
        S: System<In = (), Out = ()> + Send + 'static,
    {
        self.send(run_system_cached(system))
    }

    pub fn send_oneshot_system_with_input<I, S, M>(&self, system: S, input: I)
    where
        I: Send + 'static,
        S: IntoSystem<In<I>, (), M> + Send + 'static,
        M: 'static,
    {
        self.send(run_system_cached_with(system, input))
    }

    pub fn send_batch(&self, queue: CommandQueue) {
        self.0.send(queue).expect("channel disconnected");
    }
}

pub fn apply_callbacks(mut commands: Commands, receiver: Res<CallbackReceiver>) {
    while let Ok(mut command) = receiver.0.try_recv() {
        commands.append(&mut command);
    }
}

pub fn run_system_cached<M, S>(system: S) -> impl Command
where
    S: IntoSystem<(), (), M> + Send + 'static,
    M: 'static,
{
    RunSystemCachedWith::new(system, ())
}

pub fn run_system_cached_with<I, M, S>(
    system: S,
    input: <I as SystemInput>::Inner<'static>,
) -> impl Command
where
    I: SystemInput<Inner<'static>: Send> + Send + 'static,
    M: 'static,
    S: IntoSystem<I, (), M> + Send + 'static,
{
    RunSystemCachedWith::new(system, input)
}
