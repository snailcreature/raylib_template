use std::{any::Any, sync::{Arc, Mutex}};
use bitvec::BitArr;

//--- COMPONENTS ---
/// Numerical representation of a component.
pub type ComponentMask = usize;

/// Maximum number of components that can be registered.
pub const MAX_COMPONENTS: ComponentMask = size_of::<ComponentMask>() * 8;

/// Unique signature of a Component, allowing it to be assigned easily to Entities.
pub type ComponentSignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/// Trait for Vecs of Components.
pub trait IComponentVec {
    /// Get a reference to the Vec.
    fn as_any(&self) -> &dyn Any;
    /// Get a mutable reference to the Vec.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A Vec of Components.
pub type ComponentVec<Component> = Vec<Arc<Mutex<Option<Component>>>>;
/// A thread-safe Vec of Components.
pub type ComponentVecRef<Component> = Arc<ComponentVec<Component>>;

impl<Component: 'static> IComponentVec for ComponentVecRef<Component> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

//--- ENTITIES ---
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

//--- SYSTEMS ---
/// Signature of a System, used to identify why Entities should be affected by a given System
/// Affected => `entity_sig & system_sig == system_sig`
pub type SystemSignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);
