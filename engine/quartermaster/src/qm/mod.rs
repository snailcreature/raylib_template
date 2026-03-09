use std::{
    any::{Any, type_name},
    collections::HashMap,
    sync::{Arc, Weak},
};

/// Trait for casting structs to Any.
trait IAsAny {
    /// Get a reference to the object.
    fn as_any(&self) -> &dyn Any;
}

/// Implementation of an AssetLoader for a specific AssetType.
pub trait AssetLoader<AssetType: 'static> {
    /// Load an asset from a given location on disc.
    fn load_asset(&self, path: &String) -> AssetType;
}

impl<AssetType: 'static> IAsAny for Box<dyn AssetLoader<AssetType>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

impl<AssetType: ?Sized + 'static> IAsAny for Weak<AssetType> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}

/// Manages the different assets and [AssetLoader]s.
pub struct AssetManager {
    asset_loaders: HashMap<String, Box<dyn IAsAny>>,
    loaded_assets: HashMap<String, Box<dyn IAsAny>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_loaders: HashMap::new(),
            loaded_assets: HashMap::new(),
        }
    }

    /// Register an [AssetLoader] for a given AssetType.
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
    pub fn get_asset<AssetType: 'static>(&mut self, path: String) -> Arc<AssetType> {
        let name = type_name::<AssetType>();

        if !self.asset_loaders.contains_key(name) {
            panic!("No loader registered for AssetType {name}");
        }

        if let Some(asset_raw) = self.loaded_assets.get(&path.to_string()).clone() {
            let Some(asset_arc) = asset_raw.as_any().downcast_ref::<Weak<AssetType>>() else {
                panic!("Failed to get reference!");
            };

            if let Some(asset) = Weak::upgrade(asset_arc) {
                return asset;
            }
        }

        if let Some(loader) = self.asset_loaders.get(name) {
            let loader = loader
                .as_any()
                .downcast_ref::<Box<dyn AssetLoader<AssetType>>>()
                .unwrap();

            let asset_raw: AssetType = loader.load_asset(&path);

            let asset = Arc::new(asset_raw);
            let asset_ref: Weak<AssetType> = Arc::downgrade(&asset);
            self.loaded_assets
                .insert(path.to_string(), Box::new(asset_ref));

            return asset;
        }

        panic!("Failed to load AssetType {name} from {path}")
    }
}
