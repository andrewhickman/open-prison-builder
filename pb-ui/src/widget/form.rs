use anyhow::{Context, Result};
use bevy::{prelude::*, reflect::ReflectMut};

use pb_util::try_res_s;
use smol_str::SmolStr;

use crate::widget::UiBuilder;

#[derive(Component)]
pub struct Form {
    value: Box<dyn PartialReflect>,
}

#[derive(Component)]
pub struct FormField {
    name: SmolStr,
}

#[derive(Component)]
pub struct FormUpdate {
    pub target: Entity,
    pub name: SmolStr,
    pub value: Box<dyn PartialReflect>,
}

#[derive(Component)]
pub struct FormSubmit;

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

impl Event for FormUpdate {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
}

impl Event for FormSubmit {
    type Traversal = &'static Parent;

    const AUTO_PROPAGATE: bool = true;
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

impl<'w> UiBuilder<'w, '_> {
    pub fn form<T>(&mut self, style: Node, value: T) -> UiBuilder<'w, '_>
    where
        T: Reflect,
    {
        let mut form = self.container(style);
        form.observe(update);
        form.insert(Form {
            value: Box::new(value),
        });
        form
    }
}

pub fn submit(trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger_targets(FormSubmit, trigger.entity());
}

fn update(mut event: Trigger<FormUpdate>, mut form_q: Query<(&mut Form, Option<&FormField>)>) {
    let (mut form, field) = try_res_s!(form_q.get_mut(event.entity()));

    let ReflectMut::Struct(value) = form.value.reflect_mut() else {
        error!(
            "Unexpected form value type '{}' for {:?}",
            form.value.reflect_short_type_path(),
            event.entity()
        );
        return;
    };

    let Some(field_value) = value.field_mut(&event.name) else {
        error!(
            "Form value of type '{}' for {:?} does not have field '{}'",
            value.reflect_short_type_path(),
            event.entity(),
            event.name
        );
        return;
    };

    if let Err(error) = field_value.try_apply(&*event.value) {
        error!(
            "Error updating field '{}' for {:?}: {}",
            &event.name,
            event.entity(),
            error,
        );
        return;
    }

    if let Some(field) = field {
        event.value = form.value.clone_value();
        event.name.clone_from(&field.name);
    } else {
        event.propagate(false);
    }
}
