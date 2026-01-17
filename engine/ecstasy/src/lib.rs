mod ecs;

pub mod prelude {
    pub use crate::ecs::*;
}

#[cfg(test)]
mod tests {
    use bitvec::{bitarr, order::Lsb0};

    use crate::{ecs::{MAX_COMPONENTS, System}, type_names};

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
            fn get_component_types(&self) -> Vec<&'static str> {
                type_names!(Health, Armour)
            }
        }

        let test = Test { };

        let comp_types = test.get_component_types();
        
        assert_eq!(comp_types, vec!["ecstasy::tests::system_macro_test::Health", "ecstasy::tests::system_macro_test::Armour"])
    }

    #[test]
    fn type_names_test() {
        struct Health;
        struct Armour;

        let test = type_names!(Health, Armour);

        assert_eq!(vec!["ecstasy::tests::type_names_test::Health", "ecstasy::tests::type_names_test::Armour"], test);
    }
}
