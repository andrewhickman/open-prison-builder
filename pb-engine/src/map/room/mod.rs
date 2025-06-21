pub mod contents;
pub mod link;
pub mod mesh;
pub mod path;

use bevy::prelude::*;
use spade::handles::{FixedFaceHandle, OUTER_FACE, PossiblyOuterTag};

use crate::map::room::{link::RoomLinks, mesh::RoomMesh, path::RoomPathCache};

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility, RoomLinks, RoomMesh, RoomPathCache)]
#[component(immutable)]
pub struct Room {
    faces: Vec<FixedFaceHandle<PossiblyOuterTag>>,
}

impl Room {
    pub fn is_outer(&self) -> bool {
        self.faces[0] == OUTER_FACE
    }

    pub(crate) fn faces(&self) -> &[FixedFaceHandle<PossiblyOuterTag>] {
        &self.faces
    }

    pub(crate) fn bundle(faces: Vec<FixedFaceHandle<PossiblyOuterTag>>) -> impl Bundle {
        debug_assert!(!faces.is_empty());
        (Name::new("room"), Room { faces })
    }
}
