use std::{any::TypeId, future::Future, marker::PhantomData};

use bevy::{
    ecs::{
        system::BoxedSystem,
        world::{Command, CommandQueue},
    },
    prelude::*,
    tasks::IoTaskPool,
    utils::HashMap,
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

        app.add_systems(Update, apply_callbacks.run_if(apply_callbacks_condition));
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
        self.send(run_oneshot_system(system))
    }

    pub fn send_oneshot_system_with_input<I, S, M>(&self, system: S, input: I)
    where
        I: Send + 'static,
        S: IntoSystem<I, (), M> + Send + 'static,
        M: 'static,
    {
        self.send(run_oneshot_system_with_input(system, input))
    }

    pub fn send_batch(&self, queue: CommandQueue) {
        self.0.send(queue).expect("channel disconnected");
    }
}

pub fn apply_callbacks_condition(receiver: Res<CallbackReceiver>) -> bool {
    !receiver.0.is_empty()
}

pub fn apply_callbacks(mut commands: Commands, receiver: Res<CallbackReceiver>) {
    while let Ok(mut command) = receiver.0.try_recv() {
        commands.append(&mut command);
    }
}

pub fn run_oneshot_system<S, M>(system: S) -> impl Command
where
    S: IntoSystem<(), (), M> + Send + 'static,
    M: 'static,
{
    run_oneshot_system_with_input(system, ())
}

pub fn run_oneshot_system_with_input<I, S, M>(system: S, input: I) -> impl Command
where
    I: Send + 'static,
    S: IntoSystem<I, (), M> + Send + 'static,
    M: 'static,
{
    RunOneShotSystem {
        system,
        input,
        marker: PhantomData,
    }
}

struct RunOneShotSystem<I, S, M> {
    system: S,
    input: I,
    marker: PhantomData<fn(M) -> M>,
}

#[derive(Resource)]
struct SystemMap<I>(HashMap<TypeId, BoxedSystem<I>>);

impl<I, S, M> Command for RunOneShotSystem<I, S, M>
where
    I: Send + 'static,
    S: IntoSystem<I, (), M> + Send + 'static,
    M: 'static,
{
    fn apply(self, world: &mut World) {
        let mut map = world.get_resource_or_insert_with::<SystemMap<I>>(default);
        let mut system = match map.0.remove(&self.system.system_type_id()) {
            Some(system) => system,
            None => {
                let mut system = IntoSystem::into_system(self.system);
                system.initialize(world);
                Box::new(system)
            }
        };

        system.run(self.input, world);

        world
            .get_resource_mut::<SystemMap<I>>()
            .unwrap()
            .0
            .insert(system.type_id(), system);
    }
}

impl<I> Default for SystemMap<I> {
    fn default() -> Self {
        SystemMap(default())
    }
}
