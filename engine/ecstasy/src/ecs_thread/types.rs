use bitvec::BitArr;
use std::{
    any::Any,
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

use crate::ecs_thread::World;

/*--- COMPONENTS ---*/
/// Numerical representation of a component.
pub type ComponentMask = usize;

/// Maximum number of components that can be registered.
pub const MAX_COMPONENTS: ComponentMask = size_of::<ComponentMask>() * 8;

/// Unique signature of a Component, allowing it to be assigned easily to Entities.
pub type ComponentSignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/// Trait for Vecs of Components.
pub trait IComponentVec: Send {
    /// Get a reference to the Vec.
    fn as_any(&self) -> &dyn Any;
    /// Get a mutable reference to the Vec.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A Vec of Components.
pub type ComponentVec<Component> = Vec<Arc<Mutex<Option<Component>>>>;
/// A thread-safe Vec of Components.
pub type ComponentVecRef<Component> = Arc<ComponentVec<Component>>;

impl<Component: 'static + Send> IComponentVec for ComponentVecRef<Component> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

/*--- ENTITIES ---*/
/// Numerical identifier of an Entity.
pub type Entity = usize;

/// Maximum number of Entities.
pub const MAX_ENTITIES: Entity = 5000;

/// Signature of an Entity where each bit acts as a flag representing a component that Entity has
/// or does not have.
/// Assignment => `entity_sig = entity_sig | component_sig`
/// Associated => `entity_sig & component_sig == component_sig`
/// Removal => `entity_sig = entity_sig ^ component_sig`
pub type EntitySignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/*--- SYSTEMS ---*/
/// Signature of a System, used to identify why Entities should be affected by a given System
/// Affected => `entity_sig & system_sig == system_sig`
pub type SystemSignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/// All Systems should implement this trait to allow a SystemManager to coordinate them.
pub trait System {
    /// Helper function for retrieving the names of Components required for this System to
    /// function. SystemManagers use this to determine this System's signature for indexing
    /// Entities to provide to it.
    ///
    /// Use the `type_names!` macro to produce a Vec of accurate Component type names.
    fn get_component_types(&self) -> Vec<&'static str>;
    /// Function to run when the SystemManager first starts.
    fn start(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
    /// Function to run each game loop iteration.
    fn update(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
    /// Function to run when the SystemManager stops.
    fn stop(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
}
