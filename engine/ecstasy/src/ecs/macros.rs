#[macro_use]
pub mod helpers {
    /// Creates a `Vec<&'static str>` of type names.
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
