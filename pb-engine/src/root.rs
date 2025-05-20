use bevy::{ecs::system::SystemParam, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Transform)]
pub struct Root;

impl Root {
    pub fn bundle() -> impl Bundle {
        (Root, Name::new(Root::type_path()))
    }
}

#[derive(SystemParam)]
pub struct RootQuery<'w, 's> {
    root_q: Option<Single<'w, Entity, With<Root>>>,
    parent_q: Query<'w, 's, &'static ChildOf>,
}

impl RootQuery<'_, '_> {
    pub fn parent(&self, entity: Entity) -> Result<Entity> {
        Ok(self.parent_q.get(entity)?.parent())
    }

    pub fn is_descendant_of_root(&self, entity: Entity) -> bool {
        let Some(root) = &self.root_q else {
            return false;
        };

        self.parent_q
            .iter_ancestors(entity)
            .any(|parent| parent == **root)
    }
}
