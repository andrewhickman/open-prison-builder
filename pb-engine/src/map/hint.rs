use rstar::{RTree, primitives::GeomWithData};
use spade::{HasPosition, HintGenerator, Point2, Triangulation, handles::FixedVertexHandle};

#[derive(Debug, Default, Clone)]
pub struct RTreeHintGenerator {
    tree: RTree<GeomWithData<[f32; 2], FixedVertexHandle>>,
}

impl HintGenerator<f32> for RTreeHintGenerator {
    fn get_hint(&self, position: Point2<f32>) -> FixedVertexHandle {
        match self.tree.nearest_neighbor(&[position.x, position.y]) {
            Some(nearest) => nearest.data,
            None => FixedVertexHandle::from_index(0),
        }
    }

    fn notify_vertex_lookup(&self, _: FixedVertexHandle) {}

    fn notify_vertex_inserted(&mut self, vertex: FixedVertexHandle, position: spade::Point2<f32>) {
        self.tree
            .insert(GeomWithData::new([position.x, position.y], vertex))
    }

    fn notify_vertex_removed(
        &mut self,
        swapped_in_point: Option<spade::Point2<f32>>,
        vertex: FixedVertexHandle,
        position: spade::Point2<f32>,
    ) {
        self.tree.remove_at_point(&[position.x, position.y]);

        if let Some(new_position) = swapped_in_point {
            if let Some(prev) = self
                .tree
                .locate_at_point_int_mut(&[new_position.x, new_position.y])
            {
                prev.data = vertex;
            }
        }
    }

    fn initialize_from_triangulation<TR, V>(triangulation: &TR) -> Self
    where
        TR: Triangulation<Vertex = V>,
        V: HasPosition<Scalar = f32>,
    {
        let tree = RTree::bulk_load(
            triangulation
                .vertices()
                .map(|vertex| {
                    GeomWithData::new([vertex.position().x, vertex.position().y], vertex.fix())
                })
                .collect(),
        );

        RTreeHintGenerator { tree }
    }
}
