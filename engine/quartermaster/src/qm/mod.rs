use std::{
    any::{Any, type_name},
    collections::HashMap,
    sync::Weak,
};

trait IAsAny {
    /// Get a reference to the object.
    fn as_any(&self) -> &dyn Any;
}

pub trait AssetLoader<AssetType: 'static> {
    /// Load an asset from a given location on disc.
    fn load_asset(&self, path: &String) -> Weak<AssetType>;
}

impl<AssetType: 'static> IAsAny for Box<dyn AssetLoader<AssetType>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl IAsAny for Weak<dyn Any> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

pub struct AssetManager {
    asset_loaders: HashMap<String, Box<dyn IAsAny>>,
    loaded_assets: HashMap<String, Weak<dyn Any>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_loaders: HashMap::new(),
            loaded_assets: HashMap::new(),
        }
    }

    pub fn register<AssetType: 'static>(&mut self, loader: Box<dyn AssetLoader<AssetType>>) -> () {
        let name = type_name::<AssetType>();

        if self.asset_loaders.contains_key(name) {
            return;
        }

        self.asset_loaders
            .insert(name.to_string(), Box::new(loader));
    }

    /// Will attempt to retreive an existing copy of an asset from memory. Failing this, will call
    /// on the relevant [AssetLoader] implementation to load the asset from disc.
    pub fn get_asset<AssetType: 'static>(&mut self, path: String) -> Weak<AssetType> {
        let name = type_name::<AssetType>();

        if !self.asset_loaders.contains_key(name) {
            panic!("No loader registered for AssetType {name}");
        }

        if let Some(asset) = self.loaded_assets.get(&path.to_string()).cloned() {
            if asset.weak_count() != 0 {
                let asset = asset.as_any().downcast_ref::<Weak<AssetType>>().unwrap();
                return asset.clone();
            }
        }

        if let Some(loader) = self.asset_loaders.get(name) {
            let loader = loader
                .as_any()
                .downcast_ref::<Box<dyn AssetLoader<AssetType>>>()
                .unwrap();

            let asset: Weak<AssetType> = loader.load_asset(&path);

            self.loaded_assets.insert(path.to_string(), asset);

            let registered_asset = self
                .loaded_assets
                .get(&path.to_string())
                .unwrap()
                .as_any()
                .downcast_ref::<Weak<AssetType>>()
                .unwrap();

            return registered_asset.clone();
        }

        panic!("Failed to load AssetType {name} from {path}")
    }
}
