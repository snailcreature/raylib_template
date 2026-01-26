pub mod types;

use std::{
    any::type_name,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bitvec::{bitarr, order::Lsb0};
use types::*;

//--- COMPONENTS ---
/// Stores and maintains Components.
pub struct ComponentManager {
    component_types: HashMap<String, (ComponentMask, ComponentSignature)>,
    component_instances: Vec<Box<dyn IComponentVec>>,
    components: ComponentMask,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            component_instances: Vec::new(),
            components: 0,
        }
    }

    /// Register a given Component type.
    pub fn register<Component: 'static>(&mut self) -> () {
        if self.components >= MAX_COMPONENTS {
            panic!("No more available components!")
        }

        let name: &str = type_name::<Component>();

        if self.component_types.contains_key(name) {
            panic!("Component {} already registered!", name)
        }

        let mut component_sig: ComponentSignature = bitarr!(ComponentMask, Lsb0; 0; MAX_COMPONENTS);

        component_sig.set((MAX_COMPONENTS - 1) - self.components, true);

        self.component_types
            .insert(name.to_string(), (self.components, component_sig));

        let new_vec: ComponentVecRef<Component> =
            vec![Arc::new(Mutex::new(None)); MAX_ENTITIES].into();
        self.component_instances.push(Box::new(new_vec));

        self.components += 1;
    }

    /// Get the ComponentSignature of the given Component.
    pub fn get_signature<Component: 'static>(&self) -> ComponentSignature {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (_, sig) = self.component_types[name];

        sig
    }

    /// Get the instance of a given Component for an Entity, if that instance exists.
    pub fn get_component<Component: 'static>(
        &mut self,
        entity: Entity,
    ) -> Option<&Arc<Mutex<Option<Component>>>> {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let component_vec = self.component_instances[ind].as_any_mut();

        if let Some(com_vec) = component_vec.downcast_mut::<ComponentVecRef<Component>>() {
            if let Some(component) = com_vec.get(entity) {
                return Some(component);
            }
        }
        None
    }

    /// Set the instance of a Component for a given Entity.
    pub fn set_component<Component: 'static>(
        &mut self,
        entity: Entity,
        component: Component,
    ) -> () {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let component_vec = self.component_instances[ind]
            .as_any_mut()
            .downcast_mut::<ComponentVecRef<Component>>()
            .unwrap();

        let cv = Arc::get_mut(component_vec).unwrap();

        let com_ref = &cv[entity];

        let mut com = com_ref.lock().unwrap();

        *com = Some(component);
    }

    /// Remove the Component instance for the given Entity.
    pub fn remove_component<Component: 'static>(&mut self, entity: Entity) -> () {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let component_vec = self.component_instances[ind]
            .as_any_mut()
            .downcast_mut::<ComponentVecRef<Component>>()
            .unwrap();

        let cv = Arc::get_mut(component_vec).unwrap();

        let com_ref = &cv[entity];

        let mut com = com_ref.lock().unwrap();

        *com = None;
    }
}
