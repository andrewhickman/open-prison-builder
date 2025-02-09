use bevy::ecs::system::{Adapt, IntoAdapterSystem, System, SystemIn};

pub fn run_if<S, P>(system: S, predicate: P) -> IntoAdapterSystem<RunIfAdapter<P>, S> {
    IntoAdapterSystem::new(RunIfAdapter { predicate }, system)
}

#[derive(Clone)]
pub struct RunIfAdapter<P> {
    predicate: P,
}

impl<S, P> Adapt<S> for RunIfAdapter<P>
where
    S: System<Out = ()>,
    P: for<'a> FnMut(&mut SystemIn<'a, S>) -> bool + Send + Sync + 'static,
{
    type In = <S as System>::In;
    type Out = ();

    fn adapt(
        &mut self,
        mut input: SystemIn<'_, S>,
        run_system: impl FnOnce(SystemIn<'_, S>) -> <S as System>::Out,
    ) -> Self::Out {
        if (self.predicate)(&mut input) {
            run_system(input);
        }
    }
}
