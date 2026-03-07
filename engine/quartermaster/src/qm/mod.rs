use std::{path::Path, sync::Weak};

pub trait AssetLoader<AssetType> {
    fn load_asset(path: &Path) -> Weak<AssetType>;
}
