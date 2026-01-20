//! Macros for simplifying the process of setting up an ECS.
pub mod helpers {
    /// Creates a `Vec<&'static str>` of known type names.
    /// Will not work on generics.
    #[macro_export]
    macro_rules! type_names {
        () => {
            Vec::<&'static str>::new()
        };
        ($($a:ty),+) => {
            vec!($(std::any::type_name::<$a>()),+)
        };
    }
}
