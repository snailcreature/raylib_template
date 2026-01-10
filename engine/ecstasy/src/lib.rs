mod ecs;

pub mod prelude {
    pub use crate::ecs::*;
}

#[cfg(test)]
mod tests {
    use crate::t_system;

    use super::{prelude::World, ecs::macros::*};

    #[test]
    fn entity_test() {
        let mut world = World::new();

        let (entity0, _) = world.create_entity();

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

    #[test]
    fn system_macro_test() {
        struct Health;
        struct Armour;
        t_system!(TestSystem; Health, Armour;);

        struct Test;

        impl TestSystem for Test {
            fn new() -> Self {
                Self
            }

            fn start(&mut self,dt:f32,world: *mut crate::ecs::World,entities:Vec<(Health,Armour)>) -> () {
                
            }

            fn update(&mut self,dt:f32,world: *mut crate::ecs::World,entities:Vec<(Health,Armour)>) -> () {
                
            }

            fn stop(&mut self,dt:f32,world: *mut crate::ecs::World,entities:Vec<(Health,Armour)>) -> () {
                
            }
        }

        let test0 = Test::new();

        assert_eq!(Test::COMPONENTS, ("Health", "Armour"));
    }
}
