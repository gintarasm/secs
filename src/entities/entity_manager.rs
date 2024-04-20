use std::any::{type_name, TypeId};
use std::collections::{HashMap, VecDeque};

use log::info;

use crate::components::component_manager::ComponentManager;
use crate::errors::EcsErrors;
use crate::components::Component;

use super::Entity;

struct EntityIdGenerator {
    current_free_id: usize,
    freed_entities: VecDeque<usize>,
}

impl EntityIdGenerator {
    pub fn new() -> Self {
        Self {
            current_free_id: 0,
            freed_entities: VecDeque::new(),
        }
    }

    pub fn get_id(&mut self) -> usize {
        if self.freed_entities.is_empty() {
            let id = self.current_free_id;
            self.current_free_id += 1;
            id
        } else {
            self.freed_entities.pop_front().unwrap()
        }
    }

    pub fn free_id(&mut self, id: usize) {
        self.freed_entities.push_back(id)
    }

    pub fn is_id_used(&self, id: usize) -> bool {
        self.current_free_id >= id && !self.freed_entities.contains(&id)
    }
}

pub struct EntityManager<'a> {
    id_generator: EntityIdGenerator,
    pub entity_component_signatures: Vec<u32>,
    pub component_manager: ComponentManager<'a>,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> Self {
        Self {
            id_generator: EntityIdGenerator::new(),
            entity_component_signatures: vec![],
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.id_generator.get_id();

        if entity_id >= self.entity_component_signatures.len() {
            self.entity_component_signatures.resize(entity_id + 10, 0);
        } else {
            self.entity_component_signatures[entity_id] = 0;
        }

            info!("Entity created with id = {entity_id}");

        Entity(entity_id)
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
            info!("Removing entity id = {}", entity.0);

        self.entity_component_signatures[entity.0] = 0;
        self.component_manager.remove_all(entity);
        self.id_generator.free_id(entity.0);
    }

    pub fn add_component<T: Component + 'static>(
        &mut self,
        entity: &Entity,
        component: T,
    ) -> Result<(), EcsErrors> {
        let comp_mask = self.component_manager.add_component(entity, component);

        if !self.id_generator.is_id_used(entity.0) {
            return Err(EcsErrors::EntityDoesNotExist(entity.0));
        }

        self.entity_component_signatures[entity.0] |= comp_mask;

        Ok(())
    }

    pub fn remove_component<T: Component + 'static>(
        &mut self,
        entity: &Entity,
    ) -> Result<(), EcsErrors> {
        if !self.id_generator.is_id_used(entity.0) {
            return Err(EcsErrors::EntityDoesNotExist(entity.0));
        }

        let comp_mask = self.component_manager.get_mask::<T>().unwrap();
        self.entity_component_signatures[entity.0] &= !comp_mask;
        let _ = self.component_manager.remove::<T>(entity);

        info!(
            "Removing component {} from Entity Id = {}",
            type_name::<T>(),
            entity.0
        );

        Ok(())
    }
    pub fn remove_component_for_id(
        &mut self,
        entity: &Entity,
        comp_id: &TypeId
    ) -> Result<(), EcsErrors> {
        if !self.id_generator.is_id_used(entity.0) {
            return Err(EcsErrors::EntityDoesNotExist(entity.0));
        }

        let comp_mask = self.component_manager.get_mask_for_id(comp_id).unwrap();
        self.entity_component_signatures[entity.0] &= !comp_mask;
        let _ = self.component_manager.remove_with_id(entity, comp_id);

        info!(
            "Removing component {} from Entity Id = {}",
            "Unknown",
            entity.0
        );

        Ok(())
    }

    pub fn has_component<T: Component + 'static>(
        &self,
        entity: &Entity,
    ) -> Result<bool, EcsErrors> {
        if !self.id_generator.is_id_used(entity.0) {
            return Err(EcsErrors::EntityDoesNotExist(entity.0));
        }

        let comp_mask = self.component_manager.get_mask::<T>().unwrap();

        let signature = self.entity_component_signatures.get(entity.0).unwrap();

        Ok((*signature & comp_mask) == *comp_mask)
    }

    pub fn get_signature(&self, entity: &Entity) -> Result<&u32, EcsErrors> {
        if !self.id_generator.is_id_used(entity.0) {
            return Err(EcsErrors::EntityDoesNotExist(entity.0));
        }

        Ok(self.entity_component_signatures.get(entity.0).unwrap())
    }

    pub fn get_component_signatures(&self) -> HashMap<TypeId, u32> {
        self.component_manager.component_bit_masks.clone()
    }
}
