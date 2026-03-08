pub mod basic;
pub mod qm;

pub mod prelude {
    pub use crate::qm::{AssetLoader, AssetManager};
}

#[cfg(test)]
mod tests {
    use std::sync::Weak;

    use crate::{basic::TextLoader, qm::AssetManager};

    #[test]
    fn load_asset() {
        let mut am = AssetManager::new();

        am.register(Box::new(TextLoader::default()));

        let s: Weak<String> = am.get_asset("./test_assets/test.txt".to_string());

        let Some(string) = s.upgrade() else {
            panic!("No asset found!")
        };

        assert_eq!(*string, "Hello, World!\n".to_string())
    }

    #[test]
    fn load_many_same() {
        let mut am = AssetManager::new();

        am.register(Box::new(TextLoader::default()));

        let path = "./test_assets/test.txt".to_string();

        println!("s0");
        let s0: Weak<String> = am.get_asset(path.clone());
        println!("s1");
        let s1: Weak<String> = am.get_asset(path);

        let (Some(string0), Some(string1)) = (s0.upgrade(), s1.upgrade()) else {
            panic!("Failed to load an asset!")
        };

        assert_eq!(*string0, "Hello, World!\n".to_string());
        assert_eq!(*string1, "Hello, World!\n".to_string())
    }
}
