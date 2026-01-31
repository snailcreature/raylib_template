pub mod types;

use std::{
    any::type_name,
    collections::{BTreeSet, HashMap, VecDeque},
    sync::{Arc, Mutex},
    thread::spawn,
};

use bitvec::{bitarr, order::Lsb0};
use types::*;
use uuid7::uuid7;

/*--- COMPONENTS ---*/
/// Stores and maintains Components.
pub struct ComponentManager {
    /// Map of Component type names to ComponentMasks and ComponentSignatures.
    component_types: HashMap<String, (ComponentMask, ComponentSignature)>,
    /// Vec of ComponentVecs containing instances of Components.
    component_instances: Arc<Vec<Box<dyn IComponentVec>>>,
    /// The number of registered Components.
    components: ComponentMask,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            component_instances: Arc::new(Vec::new()),
            components: 0,
        }
    }

    /// Register a given Component type.
    pub fn register<Component: 'static + Send>(&mut self) -> () {
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
            Vec::from_iter(core::array::from_fn::<
                Arc<Mutex<Option<Component>>>,
                MAX_ENTITIES,
                _,
            >(|_| Arc::new(Mutex::new(None))))
            .into();

        let instances = Arc::get_mut(&mut self.component_instances).unwrap();

        instances.push(Box::new(new_vec));

        self.components += 1;
    }

    /// Get the ComponentSignature of the given Component.
    pub fn get_signature<Component: 'static + Send>(&self) -> ComponentSignature {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (_, sig) = self.component_types[name];

        sig
    }

    pub fn get_signature_from_name(&self, name: &String) -> ComponentSignature {
        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (_, sig) = self.component_types[name];

        sig
    }

    /// Get the instance of a given Component for an Entity, if that instance exists.
    pub fn get_component<Component: 'static + Send>(
        &mut self,
        entity: Entity,
    ) -> Option<Arc<Mutex<Option<Component>>>> {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let instances = Arc::get_mut(&mut self.component_instances).unwrap();

        let component_vec = instances[ind].as_any_mut();

        if let Some(com_vec) = component_vec.downcast_ref::<ComponentVecRef<Component>>() {
            if let Some(component) = com_vec.get(entity) {
                return Some(component.clone());
            }
        }
        None
    }

    /// Set the instance of a Component for a given Entity.
    pub fn set_component<Component: 'static + Send>(
        &mut self,
        entity: Entity,
        component: Component,
    ) -> () {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let instances = Arc::get_mut(&mut self.component_instances).unwrap();

        let component_vec = instances[ind]
            .as_any_mut()
            .downcast_mut::<ComponentVecRef<Component>>()
            .unwrap();

        let cv = Arc::get_mut(component_vec).unwrap();

        let com_ref = &cv[entity];

        let mut com = com_ref.lock().unwrap();

        *com = Some(component);
    }

    /// Remove the Component instance for the given Entity.
    pub fn remove_component<Component: 'static + Send>(&mut self, entity: Entity) -> () {
        let name = type_name::<Component>();

        if !self.component_types.contains_key(name) {
            panic!("Component {} does not exist!", name)
        }

        let (ind, _) = self.component_types[name];

        let instances = Arc::get_mut(&mut self.component_instances).unwrap();

        let component_vec = instances[ind]
            .as_any_mut()
            .downcast_mut::<ComponentVecRef<Component>>()
            .unwrap();

        let cv = Arc::get_mut(component_vec).unwrap();

        let com_ref = &cv[entity];

        let mut com = com_ref.lock().unwrap();

        *com = None;
    }
}

/*--- ENTITIES ---*/
pub struct EntityManager {
    /// Queue of available Entity identifiers.
    entity_shelf: Arc<VecDeque<Entity>>,
    /// Signatures of all Entities.
    signatures: Arc<Vec<Arc<Mutex<EntitySignature>>>>,
    /// Number of living Entities.
    living: Arc<Mutex<Entity>>,
    /// Vec of unique identifiers for Entities, allowing for persistent storage.
    entity_uuid: Arc<Vec<Mutex<Option<String>>>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entity_shelf: Arc::new(VecDeque::from_iter(core::array::from_fn::<
                _,
                MAX_ENTITIES,
                _,
            >(|i| i))),
            signatures: Arc::new(vec![
                Arc::new(Mutex::new(
                    bitarr!(Entity, Lsb0; 0; MAX_COMPONENTS)
                ));
                MAX_ENTITIES
            ]),
            living: Arc::new(Mutex::new(0)),
            entity_uuid: Arc::new(Vec::from_iter(core::array::from_fn::<_, MAX_ENTITIES, _>(
                |_| Mutex::new(None),
            ))),
        }
    }

    /// Create a new entity.
    pub fn spawn(&mut self) -> (Entity, String) {
        let mut living = self.living.lock().unwrap();
        if *living >= MAX_ENTITIES {
            panic!("Too many Entities!")
        }

        let es = Arc::get_mut(&mut self.entity_shelf).unwrap();
        let id: Entity = es.pop_front().unwrap();

        let uuid = uuid7().to_string();

        let mut _eu = self.entity_uuid.clone();
        let eu = &*_eu;

        let mut pos = eu[id].lock().unwrap();

        *pos = Some(uuid.clone());

        *living += 1;

        (id, uuid)
    }

    /// Create an entity using an existing unique identifier.
    pub fn load(&mut self, uuid: String) -> Entity {
        let mut living = self.living.lock().unwrap();
        if *living >= MAX_ENTITIES {
            panic!("Too many Entities!")
        }

        let es = Arc::get_mut(&mut self.entity_shelf).unwrap();
        let id: Entity = es.pop_front().unwrap();

        let eu = Arc::get_mut(&mut self.entity_uuid).unwrap();

        let mut pos = eu[id].lock().unwrap();

        *pos = Some(uuid.clone());

        *living += 1;

        id
    }

    /// Destroy an Entity.
    pub fn destroy(&mut self, entity: Entity) -> () {
        let mut living = self.living.lock().unwrap();
        if *living >= MAX_ENTITIES {
            panic!("Too many Entities!")
        }

        let sigs = Arc::get_mut(&mut self.signatures).unwrap();
        let mut sig = sigs[entity].lock().unwrap();
        *sig = bitarr!(Entity, Lsb0; 0; MAX_COMPONENTS);

        let eu = Arc::get_mut(&mut self.entity_uuid).unwrap();
        let mut pos = eu[entity].lock().unwrap();
        *pos = None;

        *living -= 1;
    }

    /// Manipulate an Entity's signature to reflect the assignment of a Component.
    pub fn assign(&mut self, entity: Entity, component_sig: ComponentSignature) -> () {
        let sigs = Arc::get_mut(&mut self.signatures).unwrap();
        let mut sig = sigs[entity].lock().unwrap();

        *sig |= component_sig;
    }

    /// Modify an Entity's signature to reflect the unassignment of a Component.
    pub fn unassign(&mut self, entity: Entity, component_sig: ComponentSignature) -> () {
        let sigs = Arc::get_mut(&mut self.signatures).unwrap();
        let mut sig = sigs[entity].lock().unwrap();

        *sig ^= component_sig;
    }

    /// Get the EntitySignature for an Entity.
    pub fn get_signature(&self, entity: Entity) -> EntitySignature {
        let sigs = Arc::clone(&self.signatures);
        let sig = sigs[entity].lock().unwrap();

        *sig
    }

    /// Get an Entity's uuid.
    pub fn get_uuid(&self, entity: Entity) -> Option<String> {
        let eu = Arc::clone(&self.entity_uuid);
        let uuid = eu[entity].lock().unwrap();

        uuid.clone()
    }
}

/*--- WORLD ---*/
/// Coordinates Entities and Components between Systems.
pub struct World {
    /// EntityManager for the World.
    entity_manager: Arc<EntityManager>,
    /// ComponentManager for the World.
    component_manager: Arc<ComponentManager>,
    /// Entities that have been updated.
    pub dirty: Arc<Mutex<BTreeSet<Entity>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: Arc::new(EntityManager::new()),
            component_manager: Arc::new(ComponentManager::new()),
            dirty: Arc::new(Mutex::new(BTreeSet::new())),
        }
    }

    /*--- ENTITIES ---*/
    /// Spawn a new blank Entity.
    pub fn create_entity(&mut self) -> (Entity, String) {
        let em = Arc::get_mut(&mut self.entity_manager).unwrap();

        let (ent, id) = em.spawn();

        let _dirty = self.dirty.clone();
        let mut dirty = _dirty.lock().unwrap();
        dirty.insert(ent);

        (ent, id)
    }

    /// Destroy an Entity.
    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        let em = Arc::get_mut(&mut self.entity_manager).unwrap();
        em.destroy(entity);

        let _dirty = self.dirty.clone();
        let mut dirty = _dirty.lock().unwrap();
        dirty.insert(entity);
    }

    /*--- COMPONENTS ---*/
    /// Register a Component in the World.
    pub fn register<Component: 'static + Send>(&mut self) -> () {
        let cm = Arc::get_mut(&mut self.component_manager).unwrap();
        cm.register::<Component>();
    }

    /// Assign an instance of a Component to an Entity.
    pub fn assign<Component: 'static + Send>(
        &mut self,
        entity: Entity,
        component: Component,
    ) -> () {
        let em = Arc::get_mut(&mut self.entity_manager).unwrap();
        let cm = Arc::get_mut(&mut self.component_manager).unwrap();

        let component_sig = cm.get_signature::<Component>();

        em.assign(entity, component_sig);
        cm.set_component(entity, component);

        let _dirty = self.dirty.clone();
        let mut dirty = _dirty.lock().unwrap();
        dirty.insert(entity);
    }

    /// Unassign a Component from an Entity.
    pub fn unassign<Component: 'static + Send>(&mut self, entity: Entity) -> () {
        let em = Arc::get_mut(&mut self.entity_manager).unwrap();
        let cm = Arc::get_mut(&mut self.component_manager).unwrap();

        let component_sig = cm.get_signature::<Component>();

        em.unassign(entity, component_sig);
        cm.remove_component::<Component>(entity);

        let _dirty = self.dirty.clone();
        let mut dirty = _dirty.lock().unwrap();
        dirty.insert(entity);
    }

    /// Get a thread-safe mutable reference to a Component of an Entity.
    pub fn get_component<Component: 'static + Send>(
        &mut self,
        entity: Entity,
    ) -> Option<Arc<Mutex<Option<Component>>>> {
        let cm = Arc::get_mut(&mut self.component_manager).unwrap();
        cm.get_component::<Component>(entity)
    }

    pub fn get_component_signature_from_name(&self, name: String) -> ComponentSignature {
        let cm = self.component_manager.clone();
        cm.get_signature_from_name(&name)
    }
}

/*--- SYSTEMS ---*/
pub struct SystemManager {
    /// All the Systems that have been registered.
    instances: Arc<Vec<Arc<Mutex<Box<dyn System + Send>>>>>,
    /// Signature and instance index of each System.
    signatures: HashMap<String, (SystemSignature, usize)>,
    /// Map of which Entities are affected by each System.
    index: HashMap<String, Arc<BTreeSet<Entity>>>,
    /// Number of registered Systems.
    systems: usize,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Vec::new()),
            signatures: HashMap::new(),
            index: HashMap::new(),
            systems: 0,
        }
    }

    /// Register an instance of a new System type.
    pub fn register<Sys: 'static + System + Send>(
        &mut self,
        system: Sys,
        signature: SystemSignature,
    ) -> () {
        let name = type_name::<Sys>().to_string();

        self.signatures
            .insert(name.clone(), (signature, self.systems));

        self.index.insert(name, Arc::new(BTreeSet::new()));

        let instances = Arc::get_mut(&mut self.instances).unwrap();

        instances.push(Arc::new(Mutex::new(Box::new(system))));

        self.systems += 1;
    }

    /// Ensure the Entity index for each System is up to date.
    pub fn clean(&mut self, entity: Entity, ent_sig: EntitySignature) -> () {
        for (name, (sys_sig, _)) in &self.signatures {
            if let Some(i) = self.index.get_mut(name) {
                let index = Arc::get_mut(i).unwrap();
                if ent_sig & sys_sig == *sys_sig {
                    index.insert(entity);
                } else {
                    index.remove(&entity);
                }
            }
        }
    }

    /// Start all systems in their own thread.
    pub fn start(&mut self, dt: f32, world: &Arc<Mutex<World>>) -> () {
        let mut handles = vec![];

        let instances = Arc::get_mut(&mut self.instances).unwrap();
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.index.get(name) {
                let w = Arc::clone(world);

                let e = Arc::clone(&entities);

                let delta = dt;

                let system = instances[*index].clone();

                let handle = spawn(move || {
                    let mut sys = system.lock().unwrap();
                    let mut world = w.lock().unwrap();
                    sys.start(delta, &mut world, &e);
                });

                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// Update all systems in their own thread.
    pub fn update(&mut self, dt: f32, world: &Arc<Mutex<World>>) -> () {
        let mut handles = vec![];

        let instances = Arc::get_mut(&mut self.instances).unwrap();
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.index.get(name) {
                let w = Arc::clone(world);

                let e = Arc::clone(&entities);

                let delta = dt;

                let system = instances[*index].clone();

                let handle = spawn(move || {
                    let mut sys = system.lock().unwrap();
                    let mut world = w.lock().unwrap();
                    sys.update(delta, &mut world, &e);
                });

                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// Stop all systems in their own thread.
    pub fn stop(&mut self, dt: f32, world: &Arc<Mutex<World>>) -> () {
        let mut handles = vec![];

        let instances = Arc::get_mut(&mut self.instances).unwrap();
        for (name, (_, index)) in &self.signatures {
            if let Some(entities) = self.index.get(name) {
                let w = Arc::clone(world);

                let e = Arc::clone(&entities);

                let delta = dt;

                let system = instances[*index].clone();

                let handle = spawn(move || {
                    let mut sys = system.lock().unwrap();
                    let mut world = w.lock().unwrap();
                    sys.stop(delta, &mut world, &e);
                });

                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

/// Manage Systems and a World together, such that the Systems act on the Entities and Components
/// of the World.
pub struct WorldManager {
    world: Arc<Mutex<World>>,
    system_manager: SystemManager,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(World::new())),
            system_manager: SystemManager::new(),
        }
    }

    /*--- ENTITIES ---*/
    /// Spawn a new blank Entity.
    pub fn create_entity(&mut self) -> (Entity, String) {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();
        world.create_entity()
    }

    /// Destroy an Entity.
    pub fn destroy_entity(&mut self, entity: Entity) -> () {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();

        world.destroy_entity(entity);
    }

    /*--- COMPONENTS ---*/
    /// Register a Component in the World.
    pub fn register_component<Component: 'static + Send>(&mut self) -> () {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();

        world.register::<Component>();
    }

    /// Assign a registered Component to an Entity.
    pub fn assign<Component: 'static + Send>(
        &mut self,
        entity: Entity,
        component: Component,
    ) -> () {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();

        world.assign(entity, component);
        self.clean();
    }

    /// Get a Component of an Entity.
    /// !! DO NOT use this to remove Components from Entities. Use WorldManager.unassign !!
    pub fn get_component<Component: 'static + Send>(
        &mut self,
        entity: Entity,
    ) -> Option<Arc<Mutex<Option<Component>>>> {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();

        world.get_component::<Component>(entity)
    }

    /// Remove a Component from an Entity.
    pub fn unassign<Component: 'static + Send>(&mut self, entity: Entity) -> () {
        let _world = Arc::clone(&self.world);
        let mut world = _world.lock().unwrap();

        world.unassign::<Component>(entity);
        self.clean();
    }

    /*--- SYSTEMS ---*/
    /// Register a System with the SystemManager.
    pub fn register_system<T: 'static + System + Send>(&mut self, system: T) -> () {
        let mut sig: SystemSignature = bitarr!(usize, Lsb0; 0; MAX_COMPONENTS);

        let _world = Arc::clone(&self.world);
        let world = _world.lock().unwrap();

        for component_name in system.get_component_types() {
            let comp_sig = world.get_component_signature_from_name(component_name.to_string());

            sig |= comp_sig;
        }

        self.system_manager.register(system, sig);
    }

    /// Run each registered System's `start` function in a thread.
    pub fn systems_start(&mut self, dt: Option<f32>) -> () {
        self.system_manager.start(dt.unwrap_or(0.0), &self.world);
        self.clean();
    }

    /// Run each registered System's `update` function in a thread.
    pub fn systems_update(&mut self, dt: f32) -> () {
        self.system_manager.update(dt, &self.world);
        self.clean();
    }

    /// Run each registered System's `stop` function in a thread.
    pub fn systems_stop(&mut self, dt: f32) -> () {
        self.system_manager.stop(dt, &self.world);
        self.clean();
    }

    /// Reindex all System's Entity indices.
    fn clean(&mut self) -> () {
        let _world = Arc::clone(&self.world);
        let world = _world.lock().unwrap();

        let _dirty = world.dirty.clone();
        let mut dirty = _dirty.lock().unwrap();

        for entity in dirty.iter() {
            let sig = world.entity_manager.get_signature(*entity);
            self.system_manager.clean(*entity, sig);
        }

        dirty.clear();
    }
}
