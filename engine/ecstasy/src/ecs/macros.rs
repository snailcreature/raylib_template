/// Create a trait for handling a system's signature.
///
/// For example:
/// ```rust,ignore
/// struct Health(i32);
/// struct Armour(i32);
/// struct Level(i32);
///
/// struct DamageSystem;
///
/// t_system!(TDamageSystem; Health, Armour, Level;);
///
/// impl TDamageSystem for DamageSystem {
///     fn start(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<(Health, Armour, Level)>) -> () {
///         /* Do something... */
///     }
///
///     /* The rest... */
/// }
/// ```
#[macro_export]
macro_rules! t_system {
    ($system_name:ident) => (
        const DO_ONCE: bool = false;
        const COMPONENTS: () = ();
        pub trait $system_name {
            fn new() -> Self;
            fn start(&mut self, dt: f32, world: *mut $crate::ecs::World) -> ();
            fn update(&mut self, dt: f32, world: *mut $crate::ecs::World) -> ();
            fn stop(&mut self, dt: f32, world: *mut $crate::ecs::World) -> ();
            
        }
    );
    ($system_name:ident; $($component:ty),+;) => (
        pub trait $system_name {
            const DO_ONCE: bool = false;
            const COMPONENTS: $crate::type_tuple!($($component),+) = $crate::type_names!($($component),+);
            fn new() -> Self;
            fn start(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
            fn update(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
            fn stop(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
        }
    );
    ($system_name:ident; $($component:ty),+; once;) => (
        pub trait $system_name {
            const DO_ONCE: bool = true;
            const COMPONENTS: $crate::type_tuple!($($component),+) = $crate::type_names!($($component),+);
            fn new() -> Self;
            fn start(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
            fn update(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
            fn stop(&mut self, dt: f32, world: *mut $crate::ecs::World, entities: Vec<($($component),+)>) -> ();
        }
    );
}

#[macro_export]
macro_rules! type_names {
    ($a:ty) => {
        const_format::formatcp!("{}", stringify!($a))
    };
    ($a:ty, $($b:ty),+) => {
        (const_format::formatcp!("{}", stringify!($a)), $crate::type_names!($($b),+))
    };
}

#[macro_export]
macro_rules! type_tuple {
    ($a:ty) => {
        &str
    };
    ($a:ty, $($b:ty),+) => {
        (&str, $crate::type_tuple!($($b),+))
    }
}

