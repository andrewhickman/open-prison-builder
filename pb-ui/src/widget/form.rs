use anyhow::{Context, Result};
use bevy::{prelude::*, reflect::ReflectMut};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{EntityEvent, Listener, ListenerMut, On},
};

use pb_util::try_res;
use smol_str::SmolStr;

use crate::widget::UiBuilder;

#[derive(Component)]
pub struct Form {
    value: Box<dyn Reflect>,
}

#[derive(Component)]
pub struct FormField {
    name: SmolStr,
}

#[derive(EntityEvent, Event)]
#[can_bubble]
pub struct FormUpdate {
    #[target]
    pub target: Entity,
    pub name: SmolStr,
    pub value: Box<dyn Reflect>,
}

#[derive(Clone, EntityEvent, Event)]
#[can_bubble]
pub struct FormSubmit {
    #[target]
    pub target: Entity,
}

impl Form {
    pub fn value<T>(&self) -> Result<T>
    where
        T: FromReflect + TypePath,
    {
        T::from_reflect(&*self.value).with_context(|| {
            format!(
                "expected form to have value of type '{}' but found '{}'",
                T::short_type_path(),
                self.value.reflect_short_type_path()
            )
        })
    }
}

impl FormField {
    pub fn new(name: impl Into<SmolStr>) -> Self {
        FormField { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Clone for FormUpdate {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            name: self.name.clone(),
            value: self.value.clone_value(),
        }
    }
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn form<T>(&mut self, style: Style, value: T) -> UiBuilder<'w, '_>
    where
        T: Reflect,
    {
        let mut form = self.container(style);
        form.insert(On::<FormUpdate>::run(update));
        form.insert(Form {
            value: Box::new(value),
        });
        form
    }
}

pub fn submit(event: Listener<Pointer<Click>>, mut submit_e: EventWriter<FormSubmit>) {
    submit_e.send(FormSubmit {
        target: event.target(),
    });
}

fn update(mut event: ListenerMut<FormUpdate>, mut form_q: Query<(&mut Form, Option<&FormField>)>) {
    let (mut form, field) = try_res!(form_q.get_mut(event.listener()));

    let ReflectMut::Struct(value) = form.value.reflect_mut() else {
        error!(
            "Unexpected form value type '{}' for {:?}",
            form.value.reflect_short_type_path(),
            event.listener()
        );
        return;
    };

    let Some(field_value) = value.field_mut(&event.name) else {
        error!(
            "Form value of type '{}' for {:?} does not have field '{}'",
            value.reflect_short_type_path(),
            event.listener(),
            event.name
        );
        return;
    };

    if let Err(value) = field_value.set(event.value.clone_value()) {
        error!("Error updating field '{}' of form value of type '{}' for {:?}; value was of type '{}' but field is of type '{}'",
            &event.name,
            value.reflect_short_type_path(),
            event.listener(),
            event.value.reflect_short_type_path(),
            field_value.reflect_short_type_path(),
        );
        return;
    }

    if let Some(field) = field {
        event.value = form.value.clone_value();
        event.name.clone_from(&field.name);
    } else {
        event.stop_propagation();
    }
}
