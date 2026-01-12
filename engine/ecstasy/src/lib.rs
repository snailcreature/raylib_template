mod ecs;

pub mod prelude {
    pub use crate::ecs::*;
}

#[cfg(test)]
mod tests {
    use bitvec::{bitarr, order::Lsb0};

    use crate::{ecs::{MAX_COMPONENTS, System}};

    use super::{prelude::World};

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

        let mut test = bitarr!(usize, Lsb0; 0; MAX_COMPONENTS);
        test.set(MAX_COMPONENTS - 1, true);
        assert_eq!(health_type, test);
    }

    #[test]
    fn system_macro_test() {
        struct Health;
        struct Armour;

        struct Test;

        impl System<(Health, Armour)> for Test {
            fn new() -> Self {
                Self
            }

            fn start(&mut self, dt: f32, world: *mut World, entities: Vec<(Health, Armour)>) {
                
            }

            fn update(&mut self, dt: f32, world: *mut World, entities: Vec<(Health, Armour)>) -> () {
                
            }

            fn stop(&mut self, dt: f32, world: *mut World, entities: Vec<(Health, Armour)>) -> () {
                
            }

            fn get_component_types(&self) -> Vec<&'static str> {
                vec!["Health", "Armour"]
            }
        }

        let test = Test::new();

        let comp_types = test.get_component_types();
        
        assert_eq!(comp_types, vec!["Health", "Armour"])
    }
}
