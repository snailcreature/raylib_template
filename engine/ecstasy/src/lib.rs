//! A basic Entity Component System setup.
pub mod ecs;
pub mod ecs_thread;

pub mod prelude {
    pub use crate::ecs::{Entity, System, WorldManager, macros::helpers};
    pub use crate::type_names;
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bitvec::{bitarr, order::Lsb0};

    use crate::{
        ecs::{MAX_COMPONENTS, System, WorldManager},
        type_names,
    };

    use super::ecs::World;

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

        impl System for Test {
            fn get_component_types(&self) -> Vec<&'static str> {
                type_names!(Health, Armour)
            }
        }

        let test = Test {};

        let comp_types = test.get_component_types();

        assert_eq!(
            comp_types,
            vec![
                "ecstasy::tests::system_macro_test::Health",
                "ecstasy::tests::system_macro_test::Armour"
            ]
        )
    }

    #[test]
    fn type_names_test() {
        struct Health;
        struct Armour;
        struct Damage;

        let test0: Vec<&'static str> = type_names!();
        let test1: Vec<&'static str> = type_names!(Health);
        let test2: Vec<&'static str> = type_names!(Health, Armour);
        let test3: Vec<&'static str> = type_names!(Health, Armour, Damage);

        assert_eq!(Vec::<&'static str>::new(), test0);
        assert_eq!(vec!["ecstasy::tests::type_names_test::Health"], test1);
        assert_eq!(
            vec![
                "ecstasy::tests::type_names_test::Health",
                "ecstasy::tests::type_names_test::Armour"
            ],
            test2
        );
        assert_eq!(
            vec![
                "ecstasy::tests::type_names_test::Health",
                "ecstasy::tests::type_names_test::Armour",
                "ecstasy::tests::type_names_test::Damage"
            ],
            test3
        );
    }

    #[test]
    fn full_test() {
        struct Health(f32);

        struct HealthSystem;

        impl System for HealthSystem {
            fn get_component_types(&self) -> Vec<&'static str> {
                type_names!(Health)
            }

            fn start(
                &mut self,
                _dt: f32,
                _world: &mut World,
                _entities: &std::collections::BTreeSet<crate::prelude::Entity>,
            ) -> () {
                let mut components = _world.borrow_component_vec::<Health>().unwrap();
                println!("{}", _entities.len());
                for entity in _entities {
                    if let Some(health) = &mut components[*entity] {
                        health.0 *= 2.0;
                    }
                }
            }

            fn update(
                &mut self,
                _dt: f32,
                _world: &mut World,
                _entities: &std::collections::BTreeSet<crate::prelude::Entity>,
            ) -> () {
                let components = &mut _world.borrow_component_vec::<Health>().unwrap();
                for entity in _entities {
                    if let Some(health) = &mut components[*entity] {
                        health.0 += 2.0;
                    }
                }
            }

            fn stop(
                &mut self,
                _dt: f32,
                _world: &mut World,
                _entities: &std::collections::BTreeSet<crate::prelude::Entity>,
            ) -> () {
                let mut components = _world.borrow_component_vec::<Health>().unwrap();
                for entity in _entities {
                    if let Some(health) = components[*entity].as_mut() {
                        health.0 /= 2.0;
                    }
                }
            }
        }

        let mut world: WorldManager = WorldManager::new();

        world.register_component::<Health>();
        world.register_system(HealthSystem {});

        let (e0, _) = world.create_entity();
        let (e1, _) = world.create_entity();

        world.assign(e0, Health(1.0));
        world.assign(e1, Health(3.0));

        println!("{}", world.get_component::<Health>(e0).unwrap().0);

        world.systems_start(None);
        println!("{}", world.get_component::<Health>(e0).unwrap().0);

        world.systems_update(0.0);
        println!("{}", world.get_component::<Health>(e0).unwrap().0);

        world.systems_stop(0.0);
        println!("{}", world.get_component::<Health>(e0).unwrap().0);

        let r0 = world.get_component::<Health>(e0).unwrap().0;
        let r1 = world.get_component::<Health>(e1).unwrap().0;
        assert_eq!(r0, 2.0, "e0: {}", r0);
        assert_eq!(r1, 4.0, "e1: {}", r1);
    }

    #[test]
    fn full_thread_test() {
        struct Health(i32);
        struct Shield(i32);
        struct Attack(i32);

        struct BattleSystem {}

        impl BattleSystem {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl crate::ecs_thread::types::System for BattleSystem {
            fn get_component_types(&self) -> Vec<&'static str> {
                type_names!(Health, Shield, Attack)
            }

            fn start(
                &mut self,
                _dt: f32,
                _world: &mut crate::ecs_thread::World,
                _entities: &std::collections::BTreeSet<crate::ecs_thread::types::Entity>,
            ) -> () {
                for _ in 0..5 {
                    let (e, _) = _world.create_entity();

                    _world.assign(e, Health(10));
                    _world.assign(e, Shield(2));
                    _world.assign(e, Attack(6));
                }
            }

            fn update(
                &mut self,
                _dt: f32,
                _world: &mut crate::ecs_thread::World,
                _entities: &std::collections::BTreeSet<crate::ecs_thread::types::Entity>,
            ) -> () {
                for entity in _entities {
                    let _attack = _world.get_component::<Attack>(*entity).unwrap();
                    let _shield = _world.get_component::<Shield>(*entity).unwrap();
                    let _health = _world.get_component::<Health>(*entity).unwrap();

                    let mut attack = _attack.lock().unwrap();
                    let mut shield = _shield.lock().unwrap();
                    let mut health = _health.lock().unwrap();

                    match (&mut *health, &mut *attack, &mut *shield) {
                        (Some(health), Some(attack), Some(shield)) => {
                            health.0 -= attack.0 - shield.0;
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut world = crate::ecs_thread::WorldManager::new();

        world.register_component::<Health>();
        world.register_component::<Shield>();
        world.register_component::<Attack>();

        world.register_system(BattleSystem::new());

        world.systems_start(None);
        world.systems_update(0.0);
        world.systems_stop(0.0);
    }
}
