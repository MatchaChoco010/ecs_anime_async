use legion::systems::Builder;

pub trait SystemBundle {
    // fn build(world: &mut World, resource: &mut Resources, schedule: &mut Builder);
    fn build(schedule: &mut Builder);
}
