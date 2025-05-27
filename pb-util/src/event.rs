use std::marker::PhantomData;

use bevy::prelude::*;

pub trait AddComponentEvent {
    fn add_component_event<E: Event, T: Component>(&mut self) -> &mut Self;

    fn add_insert_event<T: Component>(&mut self) -> &mut Self {
        self.add_component_event::<OnInsert, T>()
    }

    fn add_remove_event<T: Component>(&mut self) -> &mut Self {
        self.add_component_event::<OnRemove, T>()
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ComponentEvent<E, T> {
    pub target: Entity,
    marker: PhantomData<(E, T)>,
}

impl AddComponentEvent for App {
    fn add_component_event<E: Event, T: Component>(&mut self) -> &mut Self {
        self.add_event::<ComponentEvent<E, T>>()
            .add_observer(observer::<E, T>)
    }
}

fn observer<E: Event, T: Component>(
    trigger: Trigger<E, T>,
    mut component_e: EventWriter<ComponentEvent<E, T>>,
) {
    component_e.write(ComponentEvent {
        target: trigger.target(),
        marker: PhantomData,
    });
}
