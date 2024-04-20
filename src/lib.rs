use std::{
    any::{type_name, TypeId}, collections::HashMap
};

use events::EventEmitter;

use self::{command_buffer::CommandBuffer, components::Component, entities::Entity, query::Query, world::World};

pub mod command_buffer;
pub mod components;
pub mod entities;
pub mod errors;
pub mod query;
pub mod resources;
pub mod events;
mod tests;
pub mod world;
pub use ecs_macro;


type SystemAction<T> = fn(data: &mut T, Query, &[Entity], &mut CommandBuffer, EventEmitter);
pub struct SystemBuilder<T> {
    comp_signatures: HashMap<TypeId, u32>,
    signature: u32,
    name: String,
    data: Option<T>,
    action: Option<SystemAction<T>>,
}

impl<T> SystemBuilder<T> {
    pub fn new(comp_signatures: HashMap<TypeId, u32>) -> Self {
        Self {
            comp_signatures,
            signature: 0,
            name: type_name::<T>().to_owned(),
            data: None,
            action: None
        }
    }

    pub fn with_system_data(&mut self, data: T) -> &mut Self {
        self.data = Some(data);
        self
    }

    pub fn with_action(&mut self, action: SystemAction<T>) -> &mut Self {
        self.action = Some(action);
        self
    }

    pub fn with_component<C: Component + 'static>(&mut self) -> &mut Self {
        let comp_id = TypeId::of::<C>();
        let comp_sig = self.comp_signatures.get(&comp_id).unwrap();
        self.signature |= comp_sig;
        self
    }

    pub fn build(self) -> impl System {
        GameSystem {
            signature: self.signature,
            entities: Vec::new(),
            action: self.action.unwrap(),
            data: self.data.unwrap(),
            name: self.name,
        }
    }
}


pub trait System {
    fn call(&mut self, world: &World) -> CommandBuffer;
    fn signature(&self) -> u32;
    fn name(&self) -> &str;
    fn add_entity(&mut self, entity: Entity);
    fn remove_entity(&mut self, entity: &Entity);
}
pub struct GameSystem<T> {
    pub name: String,
    pub signature: u32,
    entities: Vec<Entity>,
    data: T,
    action: SystemAction<T>
}

impl <T> System for GameSystem<T>{
    fn call(&mut self, world: &World) -> CommandBuffer {
        let mut buffer = CommandBuffer::new();
        let query = world.query();
        let emiter = world.emiter();
        (self.action)(&mut self.data, query, &self.entities, &mut buffer, emiter);
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