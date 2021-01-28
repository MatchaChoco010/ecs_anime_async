use legion::systems::Builder;

pub trait SystemBundle {
    fn build(schedule: &mut Builder);
}
