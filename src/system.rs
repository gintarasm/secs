
use std::{
    any::{type_name, TypeId}, collections::HashMap
};

use crate::events::EventEmitter;

use crate::{command_buffer::CommandBuffer, components::Component, entities::Entity, query::Query, world::World};


pub trait System {
    fn action(&mut self, query: Query, entities: &[Entity], command_buffer: &mut CommandBuffer, emitter: EventEmitter);
}


pub struct SystemBuilder<T: System> {
    comp_signatures: HashMap<TypeId, u32>,
    signature: u32,
    name: String,
    system: Option<T>,
}

impl<T: System> SystemBuilder<T> {
    pub fn new(comp_signatures: HashMap<TypeId, u32>) -> Self {
        Self {
            comp_signatures,
            signature: 0,
            name: type_name::<T>().to_owned(),
            system: None
        }
    }

    pub fn with_action(mut self, system: T) -> Self {
        self.system = Some(system);
        self
    }

    pub fn with_component<C: Component + 'static>(mut self) -> Self {
        let comp_id = TypeId::of::<C>();
        let comp_sig = self.comp_signatures.get(&comp_id).unwrap();
        self.signature |= comp_sig;
        self
    }

    pub fn build(self) -> impl InternalSystem {
        GameSystem {
            signature: self.signature,
            entities: Vec::new(),
            system: self.system.unwrap(),
            name: self.name,
        }
    }
}


pub trait InternalSystem {
    fn call(&mut self, world: &World) -> CommandBuffer;
    fn signature(&self) -> u32;
    fn name(&self) -> &str;
    fn add_entity(&mut self, entity: Entity);
    fn remove_entity(&mut self, entity: &Entity);
}
pub struct GameSystem<T: System> {
    pub name: String,
    pub signature: u32,
    entities: Vec<Entity>,
    system: T
}

impl <T: System> InternalSystem for GameSystem<T>{
    fn call(&mut self, world: &World) -> CommandBuffer {
        let mut buffer = CommandBuffer::new();
        let query = world.query();
        let emiter = world.emiter();
        self.system.action(query, &self.entities, &mut buffer, emiter);
        buffer
    }

    fn signature(&self) -> u32 {
        self.signature
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add_entity(&mut self, entity: Entity) {
     self.entities.push(entity);   
    }

    fn remove_entity(&mut self, entity: &Entity) {
        self.entities.retain(|e| e.0 != entity.0);
    }

}