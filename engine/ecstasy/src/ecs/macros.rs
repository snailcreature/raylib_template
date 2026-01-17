#[macro_use]
pub mod helpers {
    #[macro_export]
    macro_rules! _transcribe {
        ($a:ty) => {
            std::any::type_name::<$a>()
        };
        ($a:ty, $($b:ty),+) => ([std::any::type_name::<$a>(), $crate::_transcribe!($($b),+)]);
        () => {
            
        };
    }
    
    #[macro_export]
    macro_rules! type_names {
        ($($a:ty),*) => {
            Vec::from($crate::_transcribe!($($a),*))
        };
    }
}

