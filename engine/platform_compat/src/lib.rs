pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod path {
    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    use core_foundation::bundle::*;

    #[macro_export]
    macro_rules! ppath {
        ($path:literal) => {
            platform_compat::path::get_bundle_path($path)
        };
    }

    #[cfg(all(target_os = "macos", not(debug_assertions)))]
    pub fn get_bundle_path(path: &str) -> String {
        let bundle = CFBundle::main_bundle();
        let bundle_path = bundle.path().unwrap();
        let bundle_path_str = bundle_path.display();
        let resource_path = bundle.resources_path().unwrap();
        let resource_path_str = resource_path.to_str().unwrap();
        format!("{bundle_path_str}/{resource_path_str}/{path}").to_string()
    }

    #[cfg(any(not(target_os = "macos"), debug_assertions))]
    pub fn get_bundle_path(path: &str) -> String {
        path.into()
    }
}
