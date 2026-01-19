//! A basic entity component system
use bitvec::{BitArr, bitarr, order::Lsb0};
use std::{
    any::type_name,
    cell::{RefCell, RefMut},
    collections::{BTreeSet, HashMap, VecDeque},
    fmt::Debug,
};
use uuid7::uuid7;

pub mod macros;

type ComponentMask = usize;

/// Unique signature of a component represented as a bit array.
pub type ComponentSignature = BitArr!(for size_of::<usize>() * 8, in usize);

/// The maximum number of components able to be registered.
pub const MAX_COMPONENTS: usize = size_of::<usize>() * 8;

/// Numerical representation of an entity, used as an index for component vectors.
pub type Entity = usize;

/// The maximum number of entities that can exist at once.
pub const MAX_ENTITIES: Entity = 5000;

/// A signature used to annotate an entity denoting which components it has assigned to it.
/// Assign component => `entity = entity | component`
/// Has component => `entity & component == component`
/// Remove component => `entity = entity ^ component`
type EntitySignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/// A signature used to filter entities to pass to this system.
/// If `entity & system == system`, then entity is affected by system.
type SystemSignature = BitArr!(for MAX_COMPONENTS, in ComponentMask);

/// Trait for vectors of component values.
trait TComponentVec {
    /// Get an immutable version of the vector.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Get a mutable version of the vector
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

type ComponentVec<T> = Vec<Option<T>>;
type ComponentVecRef<T> = RefCell<ComponentVec<T>>;
type ComponentVecMut<'a, T> = RefMut<'a, ComponentVec<T>>;

impl<T: 'static> TComponentVec for ComponentVecRef<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

// vec![bitarr!(Entity, Lsb0; 0; MAX_ENTITIES); MAX_ENTITIES]
/// Object for storing and managing components.
pub struct ComponentManager {
    /// A hash map of component type names to their numerical type.
    component_types: HashMap<String, ComponentSignature>,
    /// Maps the numerical type of a component to the index of the component vector.
    component_index_map: HashMap<ComponentSignature, usize>,
    /// A vector of vectors containing each instance of a component.
    pub(crate) component_instances: Vec<Box<dyn TComponentVec>>,
    /// The number of existing component types. i.e. the size of component_instances, *not* the sum
    /// of all component instances stored.
    components: usize,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            component_index_map: HashMap::new(),
            component_instances: Vec::new(),
            components: 0,
        }
    }

    /// Register a new component type.
    ///
    /// Example:
    /// ```rust,ignore
    /// struct Health(i32);
    ///
    /// let cm = ComponentManager::new();
    /// cm.register_component::<Health>();
    /// ```
    pub fn register_component<T: 'static>(&mut self) -> () {
        let name: &str = type_name::<T>();

        if self.component_types.contains_key(name) {
            panic!("Component already registered");
        }

        if self.components > MAX_COMPONENTS {
            panic!("No more available components")
        }

        let mut component_type: ComponentSignature = bitarr!(usize, Lsb0; 0; MAX_COMPONENTS);

        component_type.set((MAX_COMPONENTS - 1) - self.components, true);

        self.component_types
            .insert(name.to_string(), component_type);
        self.component_index_map
            .insert(component_type, self.components);

        let new_vec: RefCell<Vec<Option<T>>> =
            Vec::from_iter(std::array::from_fn::<Option<T>, MAX_ENTITIES, _>(|_| None)).into();
        self.component_instances.push(Box::new(new_vec));

        self.components += 1;
    }

    /// Get the numerical representation of the named component.
    pub fn get_component_type(&self, name: String) -> ComponentSignature {
        if !self.component_types.contains_key(&name) {
            panic!("Component does not exist");
        }

        self.component_types[&name]
    }

    pub fn get_component_index(&self, component_type: &ComponentSignature) -> usize {
        self.component_index_map[component_type]
    }

    /// Get the numerical representation of the given component.
    pub fn get_type<T: 'static>(&self) -> ComponentSignature {
        let name: &str = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        self.component_types[name]
    }

    /// Set the value of a component for an entity.
    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let name: &str = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any_mut();

        if let Some(component_vec) = component_vec.downcast_mut::<ComponentVecRef<T>>() {
            component_vec.get_mut()[entity] = Some(component);
        }
    }

    pub fn borrow_component_vec<T: 'static>(&self) -> Option<ComponentVecMut<'_, T>> {
        let name = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = &self.component_instances[component_index];

        if let Some(component_vec) = component_vec.as_any().downcast_ref::<ComponentVecRef<T>>() {
            return Some(component_vec.borrow_mut());
        }
        None
    }

    /// Get a reference to a given entity's component instance.
    pub fn get_component<T: 'static>(&mut self, entity: Entity) -> Option<&T> {
        let name = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any_mut();

        if let Some(component_vec) = component_vec.downcast_mut::<ComponentVecRef<T>>() {
            if let Some(component) = component_vec.get_mut().get_mut(entity) {
                return Some(component.as_mut().unwrap());
            }
        }
        None
    }

    /// Set the value of a component for an entity to `None`.
    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        let name: &str = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any_mut();

        if let Some(component_vec) = component_vec.downcast_mut::<ComponentVecRef<T>>() {
            component_vec.get_mut()[entity] = None;
        }
    }
}

/// Structure for managing up to `MAX_ENTITIES` entities.
#[derive(Debug)]
pub struct EntityManager {
    /// Queue of all available entity numerical identifiers.
    available_entities: VecDeque<Entity>,
    /// Vector of signatures with length equal to the maximum number of entities.
    /// This allows indexing by the `Entity` type.
    signatures: Vec<EntitySignature>,
    /// Current number of entities.
    living_entity_count: Entity,
    /// Map of entities to uuid7 values. Including uuids allows for persistence of entities between
    /// program executions.
    entity_uuid: HashMap<Entity, String>,
    /// Map of uuid7 values to entities. Allows for reverse lookup for persistent logic.
    uuid_entity: HashMap<String, Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            available_entities: VecDeque::from_iter(core::array::from_fn::<_, MAX_ENTITIES, _>(
                |i| i,
            )),
            signatures: vec![bitarr!(Entity, Lsb0; 0; MAX_COMPONENTS); MAX_ENTITIES],
            living_entity_count: 0,
            entity_uuid: HashMap::new(),
            uuid_entity: HashMap::new(),
        }
    }

    /// Creates a new blank entity, returning its numerical identifier and its uuid.
    pub fn create_entity(&mut self) -> (Entity, String) {
        if self.living_entity_count + 1 >= MAX_ENTITIES {
            panic!("Too many entities!");
        }

        let id: Entity = *self.available_entities.front().unwrap();
        self.available_entities.pop_front();

        let uuid = uuid7().to_string();
        self.entity_uuid.insert(id, uuid.clone());
        self.uuid_entity.insert(uuid.clone(), id);

        self.living_entity_count += 1;

        (id, uuid)
    }

    /// Creates a new blank entity with a pre-defined uuid.
    pub fn load_entity(&mut self, uuid: String) -> Entity {
        if self.living_entity_count + 1 >= MAX_ENTITIES {
            panic!("Too many entities!");
        }

        if self.uuid_entity.contains_key(&uuid) {
            panic!("Entity with uuid {} alredy exists", uuid);
        }

        let id: Entity = *self.available_entities.front().unwrap();
        self.available_entities.pop_front();

        self.entity_uuid.insert(id, uuid.clone());
        self.uuid_entity.insert(uuid.clone(), id);

        self.living_entity_count += 1;

        id
    }

    /// Destroys an entity and returns its numerical identifier to the queue.
    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        if entity >= MAX_ENTITIES {
            panic!("Entity out of range");
        }

        self.signatures[entity] = bitarr!(Entity, Lsb0; 0; MAX_COMPONENTS);

        let uuid = &self.entity_uuid[&entity];
        self.uuid_entity.remove_entry(uuid);
        self.entity_uuid.remove_entry(&entity);

        self.available_entities.push_back(entity);

        self.living_entity_count -= 1;
    }

    /// Sets the signature of a given entity.
    pub fn set_signature(&mut self, entity: Entity, signature: EntitySignature) -> () {
        if entity >= MAX_ENTITIES {
            panic!("Entity out of range");
        }

        self.signatures[entity] = signature;
    }

    /// Get the signature of a given entity.
    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.signatures[entity]
    }

    /// Get the uuid of a given entity.
    pub fn get_uuid(&self, entity: Entity) -> &String {
        &self.entity_uuid[&entity]
    }

    /// Get an entity value from its uuid.
    pub fn get_entity_from_uuid(&self, uuid: &String) -> Entity {
        self.uuid_entity[uuid]
    }
}

/// Trait that all systems for Ecstasy should implement.
pub trait System {
    /// Process that should run when the system first starts.
    fn start(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
    /// Process to run every frame.
    fn update(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
    /// Process to run when the system is stopped.
    fn stop(&mut self, _dt: f32, _world: &mut World, _entities: &BTreeSet<Entity>) -> () {}
    /// Helper function for `SystemManager` that should return a Vec of the names of the components
    /// this system works on in the order presented in the generic.
    ///
    /// For example,
    /// ```rust,ignore
    /// struct DamageSystem;
    ///
    /// impl System<(Health, Armour)> for DamageSystem {
    ///     fn new() -> Self {
    ///         Self {  }
    ///     }
    ///
    ///     fn get_component_types(&self) -> Vec<&'static str> {
    ///         vec!["Health", "Armour"]
    ///     }
    /// }
    ///
    /// ```
    ///
    /// Use the `type_names!` macro for accurate component type names.
    fn get_component_types(&self) -> Vec<&'static str>;
}

/// Structure for managing systems.
struct SystemManager {
    systems_instances: Vec<Box<dyn System>>,
    signatures: HashMap<String, (SystemSignature, usize)>,
    entity_index: HashMap<String, BTreeSet<Entity>>,
    systems: usize,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems_instances: Vec::new(),
            signatures: HashMap::new(),
            entity_index: HashMap::new(),
            systems: 0,
        }
    }

    /// Register an instance of a system of the given type.
    pub fn register<T: 'static + System>(&mut self, signature: SystemSignature, system: T) -> () {
        let name = type_name::<T>().to_string();

        self.signatures
            .insert(name.clone(), (signature, self.systems));

        self.entity_index.insert(name, BTreeSet::new());

        self.systems_instances.push(Box::new(system));
        self.systems += 1;
    }

    /// Run the start function of each registered system.
    pub fn start(&mut self, dt: f32, world: &mut World) -> () {
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.entity_index.get(name) {
                self.systems_instances[*index].start(dt, world, entities);
            }
        }
    }

    /// Run the update function of each registered system.
    pub fn update(&mut self, dt: f32, world: &mut World) -> () {
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.entity_index.get(name) {
                self.systems_instances[*index].update(dt, world, entities);
            }
        }
    }

    /// Run the stop function of each registered system.
    pub fn stop(&mut self, dt: f32, world: &mut World) -> () {
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.entity_index.get(name) {
                self.systems_instances[*index].stop(dt, world, entities);
            }
        }
    }

    /// Index an entity by its signature compared to the signature of each registered function.
    pub fn index(&mut self, entity: Entity, ent_sig: EntitySignature) -> () {
        for (name, (sys_sig, _)) in &self.signatures {
            if let Some(index) = self.entity_index.get_mut(name) {
                if ent_sig & sys_sig == *sys_sig {
                    index.insert(entity);
                }
            }
        }
    }

    /// Remove all relevant indexed references to an entity based on its signature.
    pub fn deindex(&mut self, entity: Entity, ent_sig: EntitySignature) -> () {
        for (name, (sys_sig, _)) in &self.signatures {
            if let Some(index) = self.entity_index.get_mut(name) {
                if ent_sig & sys_sig != *sys_sig {
                    index.remove(&entity);
                }
            }
        }
    }

    pub fn clean(&mut self, entity: Entity, ent_sig: EntitySignature) {
        for (name, (sys_sig, _)) in &self.signatures {
            if let Some(index) = self.entity_index.get_mut(name) {
                if ent_sig & sys_sig == *sys_sig {
                    index.insert(entity);
                } else if ent_sig & sys_sig != *sys_sig {
                    index.remove(&entity);
                }
            }
        }
    }
}

/// Composite structure for holding and managing all aspects of Ecstasy.
pub struct World {
    /// Manager for entities.
    entity_manager: EntityManager,
    /// Manager for components.
    component_manager: ComponentManager,
    /// Entities that have been updated.
    pub dirty: BTreeSet<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            dirty: BTreeSet::new(),
        }
    }

    /// Create a new entity, returning its numerical identifier and uuid.
    pub fn create_entity(&mut self) -> (Entity, String) {
        let (ent, id) = self.entity_manager.create_entity();

        self.dirty.insert(ent);

        (ent, id)
    }

    /// Destroy an entity.
    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        self.entity_manager.destroy_entity(entity);
        self.dirty.insert(entity);
    }

    /// Get the signature of an entity.
    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.entity_manager.get_signature(entity)
    }

    /// Register a new component type.
    pub fn register_component<T: 'static>(&mut self) -> () {
        self.component_manager.register_component::<T>();
    }

    /// Get the numerical representation of a component.
    pub(crate) fn get_component_type<T: 'static>(&self) -> ComponentSignature {
        self.component_manager.get_type::<T>()
    }

    pub(crate) fn get_component_type_from_name(
        &self,
        component_name: String,
    ) -> ComponentSignature {
        self.component_manager.get_component_type(component_name)
    }

    /// Assign a given component to a given entity.
    ///
    /// For example:
    /// ```rust,ignore
    /// struct Health(i32);
    ///
    /// let world = World::new();
    /// world.register_component::<Health>();
    ///
    /// let (e0, _) = world.create_entity();
    ///
    /// world.assign(e0, Health(100));
    ///
    /// ```
    pub fn assign<Component: 'static>(&mut self, entity: Entity, component: Component) -> () {
        let mut entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        entity_sig = entity_sig | component_type;

        self.entity_manager.set_signature(entity, entity_sig);
        self.component_manager.set_component(entity, component);
        self.dirty.insert(entity);
    }

    /// Returns whether the given entity has the given component.
    pub fn has<Component: 'static>(&self, entity: Entity) -> bool {
        let entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        entity_sig & component_type == component_type
    }

    /// Get a given component value for a given entity.
    pub fn get_component<T: 'static>(&mut self, entity: Entity) -> Option<&T> {
        if !self.has::<T>(entity) {
            return None;
        }
        self.component_manager.get_component::<T>(entity)
    }

    pub fn borrow_component_vec<T: 'static>(&self) -> Option<ComponentVecMut<'_, T>> {
        let component_type = self.get_component_type::<T>();
        let component_index = self.component_manager.get_component_index(&component_type);

        let component_vec = &self.component_manager.component_instances[component_index];

        if let Some(component_vec) = component_vec.as_any().downcast_ref::<ComponentVecRef<T>>() {
            return Some(component_vec.borrow_mut());
        }
        None
    }

    /// Remove a component from a given entity.
    pub fn remove<Component: 'static>(&mut self, entity: Entity) -> () {
        let mut entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        self.component_manager.remove_component::<Component>(entity);

        entity_sig = entity_sig ^ component_type;

        self.entity_manager.set_signature(entity, entity_sig);
        self.dirty.insert(entity);
    }
}

pub struct WorldManager {
    world: World,
    system_manager: SystemManager,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            system_manager: SystemManager::new(),
        }
    }

    /*--- ENTITIES ---*/
    /// Create a new entity, returning its numerical identifier and uuid.
    pub fn create_entity(&mut self) -> (Entity, String) {
        self.world.create_entity()
    }

    /// Destroy an entity.
    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        let ent_sig = self.world.get_signature(entity);
        self.system_manager.deindex(entity, ent_sig);

        self.world.destroy_entity(entity)
    }

    /// Get the signature of an entity.
    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.world.get_signature(entity)
    }

    /// Register a new component type.
    pub fn register_component<T: 'static>(&mut self) -> () {
        self.world.register_component::<T>();
    }

    /// Get the numerical representation of a component.
    pub(crate) fn get_component_type<T: 'static>(&self) -> ComponentSignature {
        self.world.get_component_type::<T>()
    }

    /// Assign a given component to a given entity.
    ///
    /// For example:
    /// ```rust,ignore
    /// struct Health(i32);
    ///
    /// let world = World::new();
    /// world.register_component::<Health>();
    ///
    /// let (e0, _) = world.create_entity();
    ///
    /// world.assign(e0, Health(100));
    ///
    /// ```
    pub fn assign<Component: 'static>(&mut self, entity: Entity, component: Component) -> () {
        self.world.assign(entity, component);

        let entity_sig = self.get_signature(entity);

        self.system_manager.index(entity, entity_sig);
    }

    /// Returns whether the given entity has the given component.
    pub fn has<Component: 'static>(&self, entity: Entity) -> bool {
        let entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        entity_sig & component_type == component_type
    }

    /// Get a given component value for a given entity.
    pub fn get_component<T: 'static>(&mut self, entity: Entity) -> Option<&T> {
        self.world.get_component::<T>(entity)
    }

    /// Remove a component from a given entity.
    pub fn remove<Component: 'static>(&mut self, entity: Entity) -> () {
        self.world.remove::<Component>(entity);
        let entity_sig = self.get_signature(entity);

        self.system_manager.deindex(entity, entity_sig);
    }

    /// Register an instance of a given system type.
    ///
    /// Each system type can only have one (1) registered instance.
    pub fn register_system<T: 'static + System>(&mut self, system: T) -> () {
        let mut sig: SystemSignature = bitarr!(usize, Lsb0; 0; MAX_COMPONENTS);

        for component_name in system.get_component_types() {
            let comp_sig = self
                .world
                .get_component_type_from_name(component_name.to_string());

            sig |= comp_sig;
        }

        self.system_manager.register(sig, system);
    }

    fn clean(&mut self) {
        for entity in &self.world.dirty {
            let sig = self.get_signature(*entity);

            self.system_manager.clean(*entity, sig);
        }
    }

    /// Start systems.
    pub fn systems_start(&mut self, dt: Option<f32>) -> () {
        self.system_manager
            .start(dt.unwrap_or(0.0), &mut self.world);
        self.clean();
    }

    /// Update systems once a frame.
    pub fn systems_update(&mut self, dt: f32) -> () {
        self.system_manager.update(dt, &mut self.world);
        self.clean();
    }

    /// Stop all systems.
    pub fn systems_stop(&mut self, dt: f32) -> () {
        self.system_manager.stop(dt, &mut self.world);
        self.clean();
    }

    /// Check if a given entity will be affected by a system based on the system's signature.
    pub fn matches(&self, entity: Entity, system_sig: SystemSignature) -> bool {
        let entity_sig: EntitySignature = self.world.get_signature(entity);

        entity_sig & system_sig == system_sig
    }

    /// Get all entities that match against a system's signature.
    pub fn get_all_matches(&self, system_sig: SystemSignature) -> BTreeSet<Entity> {
        let mut matched: BTreeSet<Entity> = BTreeSet::new();
        for i in 0..MAX_COMPONENTS {
            if self.matches(i, system_sig) {
                matched.insert(i);
            }
        }
        matched
    }
}
