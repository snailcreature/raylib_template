pub mod basic;
pub mod qm;

pub mod prelude {
    pub use crate::qm::{AssetLoader, AssetManager};
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{basic::TextLoader, qm::AssetManager};

    #[test]
    fn load_asset() {
        let mut am = AssetManager::new();

        am.register(Box::new(TextLoader::default()));

        let s: Arc<String> = am.get_asset("./test_assets/test.txt".to_string());

        assert_eq!(*s, "Hello, World!\n".to_string())
    }

    #[test]
    fn load_many_same() {
        let mut am = AssetManager::new();

        am.register(Box::new(TextLoader::default()));

        let path = "./test_assets/test.txt".to_string();

        println!("s0");
        let s0: Arc<String> = am.get_asset(path.clone());
        println!("s1");
        let s1: Arc<String> = am.get_asset(path);

        assert_eq!(*s0, "Hello, World!\n".to_string());
        assert_eq!(*s1, "Hello, World!\n".to_string())
    }

    #[test]
    fn load_unload() {
        let mut am = AssetManager::new();

        am.register(Box::new(TextLoader::default()));

        let path = "./test_assets/test.txt".to_string();

        println!("s0");
        let s0: Arc<String> = am.get_asset(path.clone());
        println!("s1");
        let s1: Arc<String> = am.get_asset(path.clone());

        assert_eq!(*s0, "Hello, World!\n".to_string());
        assert_eq!(*s1, "Hello, World!\n".to_string());

        drop(s0);
        drop(s1);

        println!("s2");
        let s2: Arc<String> = am.get_asset(path.clone());
        println!("s3");
        let s3: Arc<String> = am.get_asset(path);

        assert_eq!(*s2, "Hello, World!\n".to_string());
        assert_eq!(*s3, "Hello, World!\n".to_string());
    }
}
