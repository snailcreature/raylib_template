mod ecs;

pub mod prelude {
    pub use crate::ecs::*;
}

#[cfg(test)]
mod tests {
    use super::{prelude::World};

    #[test]
    fn entity_test() {
        let mut world = World::new();

        let entity0 = world.create_entity();

        assert_eq!(entity0, 0);
    }

    #[test]
    fn component_test() {
        struct Health;

        let mut world = World::new();

        world.register_component::<Health>();

        let health_type = world.get_component_type::<Health>();

        assert_eq!(health_type, 1, "Got {}", health_type);
    }
}
