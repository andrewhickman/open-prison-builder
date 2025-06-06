use bevy::{prelude::*, reflect::ReflectMut};

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
        T::from_reflect(&*self.value).ok_or_else(|| {
            format!(
                "expected form to have value of type '{}' but found '{}'",
                T::short_type_path(),
                self.value.reflect_short_type_path()
            )
            .into()
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
    type Traversal = &'static ChildOf;

    const AUTO_PROPAGATE: bool = true;
}

impl Event for FormSubmit {
    type Traversal = &'static ChildOf;

    const AUTO_PROPAGATE: bool = true;
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

pub fn submit(trigger: Trigger<Pointer<Click>>, mut commands: Commands) -> Result {
    commands.trigger_targets(FormSubmit, trigger.target());
    Ok(())
}

fn update(
    mut trigger: Trigger<FormUpdate>,
    mut form_q: Query<(&mut Form, Option<&FormField>)>,
) -> Result {
    let (mut form, field) = form_q.get_mut(trigger.target())?;

    let ReflectMut::Struct(value) = form.value.reflect_mut() else {
        return Err(format!(
            "Unexpected form value type '{}' for {:?}",
            form.value.reflect_short_type_path(),
            trigger.target()
        )
        .into());
    };

    let Some(field_value) = value.field_mut(&trigger.name) else {
        return Err(format!(
            "Form value of type '{}' for {:?} does not have field '{}'",
            value.reflect_short_type_path(),
            trigger.target(),
            trigger.name
        )
        .into());
    };

    if let Err(error) = field_value.try_apply(&*trigger.value) {
        return Err(format!(
            "Error updating field '{}' for {:?}: {}",
            &trigger.name,
            trigger.target(),
            error,
        )
        .into());
    }

    if let Some(field) = field {
        trigger.value = form.value.reflect_clone()?;
        trigger.name.clone_from(&field.name);
    } else {
        trigger.propagate(false);
    }

    Ok(())
}
