use std::{any::type_name, collections::{HashMap, VecDeque}, fmt::Debug};
use bitvec::{BitArr, bitarr, order::Lsb0};
use uuid7::uuid7;

pub type ComponentType = usize;

pub const MAX_COMPONENTS: ComponentType = size_of::<ComponentType>() * 8;

pub type Entity = usize;

pub const MAX_ENTITIES: Entity = 5000;

type EntitySignature = BitArr!(for MAX_ENTITIES, in Entity);

type SystemSignature = BitArr!(for MAX_COMPONENTS, in ComponentType);

trait IComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self) -> ();
}

impl<T: 'static> IComponentVec for Vec<Option<T>> {
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

pub struct ComponentManager {
    component_types: HashMap<String, ComponentType>,
    component_index_map: HashMap<ComponentType, usize>,
    component_instances: Vec<Box<dyn IComponentVec>>,
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

    pub fn get_component_type(&self, name: String) -> ComponentType {
        if !self.component_types.contains_key(&name) {
            panic!("Component does not exist");
        }

        self.component_types[&name]
    }

    pub fn get_type<T: 'static>(&self) -> ComponentType {
        let name: &str = type_name::<T>();

        if !self.component_types.contains_key(name) {
            panic!("Component does not exist");
        }

        self.component_types[name]
    }

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

pub struct World {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity: Entity) -> () {
        self.entity_manager.destroy_entity(entity)
    }

    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        self.entity_manager.get_signature(entity)
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }

    pub fn get_component_type<T: 'static>(&self) -> ComponentType {
        self.component_manager.get_type::<T>()
    }
}
