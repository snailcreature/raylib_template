//! A basic entity component system
use std::{any::type_name, collections::{HashMap, VecDeque}, fmt::Debug};
use bitvec::{BitArr, array::BitArray, bitarr, order::Lsb0, view::BitViewSized};
use uuid7::uuid7;

/// Numerical representation of a component, expressed as a power of 2.
///
/// e.g. The first component will have value 1 (2^0), followed by 2 (2^1),
/// then 4 (2^2).
pub type ComponentType = usize;

/// The maximum number of components able to be registered.
pub const MAX_COMPONENTS: ComponentType = size_of::<ComponentType>() * 8;

/// Numerical representation of an entity, used as an index for component vectors.
pub type Entity = usize;

/// The maximum number of entities that can exist at once.
pub const MAX_ENTITIES: Entity = 5000;

/// A signature used to annotate an entity denoting which components it has assigned to it.
/// Assign component => `entity = entity | component`
/// Remove component => `entity = entity ^ component`
type EntitySignature = BitArr!(for MAX_ENTITIES, in Entity);

/// A signature used to filter entities to pass to this system.
/// If `entity & system == system`, then entity is affected by system.
type SystemSignature = BitArr!(for MAX_COMPONENTS, in ComponentType);

/// Trait for vectors of component values.
trait TComponentVec {
    /// Get an immutable version of the vector.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Get a mutable version of the vector
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// Add a None to the vector.
    fn push_none(&mut self) -> ();
}

impl<T: 'static> TComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) -> () {
        self.push(None);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

/// Object for storing and managing components.
pub struct ComponentManager { 
    /// A hash map of component type names to their numerical type.
    component_types: HashMap<String, ComponentType>,
    /// Maps the numerical type of a component to the index of the component vector.
    component_index_map: HashMap<ComponentType, usize>,
    /// A vector of vectors containing each instance of a component.
    component_instances: Vec<Box<dyn TComponentVec>>,
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
    /// ```rust
    /// struct Health(i32);
    ///
    /// let cm = ComponentManager::new();
    /// cm.register_component::<Health>();
    /// ```
    pub fn register_component<T: 'static>(&mut self) -> () {
        let type_name: &str = type_name::<T>();

        if self.component_types.contains_key(type_name) {
            panic!("Component already registered");
        }

        if self.components >= MAX_COMPONENTS {
            panic!("No more available components")
        }

        let component_type = (2 as usize)
            .pow(self.components
                .try_into()
                .unwrap_or_else(|x| panic!("Output: {}", x))
                );
            

        self.component_types.insert(type_name.to_string(), component_type);
        self.component_index_map.insert(component_type, self.components);

        let new_vec: Vec<Option<T>> = Vec::from_iter(std::array::from_fn::<Option<T>, MAX_ENTITIES, _>(|_| None));
        self.component_instances.push(Box::new(new_vec));

        self.components += 1;
    }

    /// Get the numerical representation of the named component.
    pub fn get_component_type(&self, name: String) -> ComponentType {
        if !self.component_types.contains_key(&name) {
            panic!("Component does not exist");
        }

        self.component_types[&name]
    }

    /// Get the numerical representation of the given component.
    pub fn get_type<T: 'static>(&self) -> ComponentType {
        let name: &str = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        self.component_types[name]
    }

    /// Returns whether the given entity has the given component.
    pub fn has<T: 'static>(&self, entity: Entity) -> bool {
        let type_name: &str = type_name::<T>();

        if !self.component_types.contains_key(type_name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[type_name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any();

        if let Some(component_vec) = component_vec
            .downcast_ref::<Vec<Option<T>>>()
        {
            if let Some(_) = component_vec[entity]
            {
                return true;
            }
        }
        false
    }

    /// Set the value of a component for an entity.
    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_name: &str = type_name::<T>();

        if !self.component_types.contains_key(type_name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[type_name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any_mut();

        if let Some(component_vec) = component_vec
                .downcast_mut::<Vec<Option<T>>>()
        {
            component_vec[entity] = Some(component);
        }
    }

    /// Set the value of a component for an entity to `None`.
    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        let type_name: &str = type_name::<T>();

        if !self.component_types.contains_key(type_name) {
            panic!("Component does not exist");
        }

        let component_type = self.component_types[type_name];
        let component_index = self.component_index_map[&component_type];

        let component_vec = self.component_instances[component_index].as_any_mut();

        if let Some(component_vec) = component_vec
            .downcast_mut::<Vec<Option<T>>>()
        {
            component_vec[entity] = None;
        }
    }
}

#[derive(Debug)]
pub struct EntityManager {
    available_entities: VecDeque<Entity>,
    signatures: Vec<EntitySignature>,
    living_entity_count: Entity,
    entity_uuid: HashMap<Entity, String>,
    uuid_entity: HashMap<String, Entity>
}

impl EntityManager {
    pub fn new() -> Self {
        Self { 
            available_entities: VecDeque::from_iter(core::array::from_fn::<_, MAX_ENTITIES, _>(|i| i)), 
            signatures: vec![bitarr!(Entity, Lsb0; 0; MAX_ENTITIES); MAX_ENTITIES], 
            living_entity_count: 0,
            entity_uuid: HashMap::new(),
            uuid_entity: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if self.living_entity_count + 1 >= MAX_ENTITIES {
            panic!("Too many entities!");
        }

        let id: Entity = *self.available_entities.front().unwrap();
        self.available_entities.pop_front();
        
        let uuid = uuid7().to_string();
        self.entity_uuid.insert(id, uuid.clone());
        self.uuid_entity.insert(uuid, id);

        self.living_entity_count += 1;

        id
    }

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

    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        if entity >= MAX_ENTITIES {
            panic!("Entity out of range");
        }

        self.signatures[entity] = bitarr!(Entity, Lsb0; 0; MAX_ENTITIES);
        
        let uuid = &self.entity_uuid[&entity];
        self.uuid_entity.remove_entry(uuid);
        self.entity_uuid.remove_entry(&entity);

        self.available_entities.push_back(entity);

        self.living_entity_count -= 1;
    }

    pub fn set_signature(&mut self, entity: Entity, signature: EntitySignature) -> () {
        if entity >= MAX_ENTITIES {
            panic!("Entity out of range");
        }

        self.signatures[entity] = signature;
    }

    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.signatures[entity]
    }

    pub fn get_uuid_of_entity(&self, entity: Entity) -> &String {
        &self.entity_uuid[&entity]
    }

    pub fn get_entity_from_uuid(&self, uuid: &String) -> Entity {
        self.uuid_entity[uuid]
    }
}

pub trait TSystem {
    fn new() -> Self;
    fn start(&mut self, dt: f32, world: *mut World) -> ();
    fn update(&mut self, dt: f32, world: *mut World) -> ();
}

struct SystemManager {
}

impl SystemManager {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct World {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: SystemManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: SystemManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        self.entity_manager.destroy_entity(entity)
    }

    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.entity_manager.get_signature(entity)
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }

    pub(crate) fn get_component_type<T: 'static>(&self) -> ComponentType {
        self.component_manager.get_type::<T>()
    }

    pub fn assign<Component: 'static>(&mut self, entity: Entity, component: Component) -> () {
        let mut entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        entity_sig = entity_sig | component_type.into_bitarray();

        self.entity_manager.set_signature(entity, entity_sig);
        self.component_manager.set_component(entity, component);
    }

    pub fn remove<Component: 'static>(&mut self, entity: Entity) -> () {
        let mut entity_sig = self.get_signature(entity);
        let component_type = self.get_component_type::<Component>();

        entity_sig = entity_sig ^ component_type.into_bitarray();

        self.entity_manager.set_signature(entity, entity_sig);
        self.component_manager.remove_component::<Component>(entity);
    }
}
