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
pub mod system;

