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

#[macro_export]
macro_rules! system_type {
    ($($components:tt),+) => {
        ($($components),+)
    };
    () => {
        ()
    };
}
