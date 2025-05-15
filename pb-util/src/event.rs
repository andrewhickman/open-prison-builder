use std::marker::PhantomData;

use bevy::prelude::*;

pub trait AddComponentEvents {
    fn add_inserted_event<T: Component>(&mut self) -> &mut Self;
}

#[derive(Event, Debug, Clone, Copy)]
pub struct Inserted<T> {
    pub target: Entity,
    marker: PhantomData<T>,
}

impl AddComponentEvents for App {
    fn add_inserted_event<T: Component>(&mut self) -> &mut Self {
        self.add_event::<Inserted<T>>().add_observer(on_insert::<T>)
    }
}

fn on_insert<T: Component>(
    trigger: Trigger<OnInsert, T>,
    mut inserted_e: EventWriter<Inserted<T>>,
) {
    inserted_e.write(Inserted {
        target: trigger.target(),
        marker: PhantomData,
    });
}
