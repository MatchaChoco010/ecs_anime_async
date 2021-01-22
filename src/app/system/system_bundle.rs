use legion::{systems::Builder, Resources, World};

pub trait SystemBundle {
    fn build(world: &mut World, resource: &mut Resources, schedule: &mut Builder);
}
