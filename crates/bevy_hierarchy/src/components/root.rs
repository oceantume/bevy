use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityMap, MapEntities, MapEntitiesError},
    reflect::{ReflectComponent, ReflectMapEntities},
    world::{FromWorld, World},
};
use bevy_reflect::Reflect;
use std::ops::Deref;

// TODO: Clean this up and change documentation, make sure everything works and makes sense.

/// Holds a reference to the root entity of this entity.
/// This component should only be present on entities that actually have a parent entity.
#[derive(Component, Debug, Eq, PartialEq, Reflect)]
#[reflect(Component, MapEntities, PartialEq)]
pub struct Root(pub(crate) Entity);

impl Root {
    /// Gets the [`Entity`] ID of the parent.
    pub fn get(&self) -> Entity {
        self.0
    }
}

// TODO: We need to impl either FromWorld or Default so Parent can be registered as Reflect.
// This is because Reflect deserialize by creating an instance and apply a patch on top.
// However Parent should only ever be set with a real user-defined entity.  Its worth looking into
// better ways to handle cases like this.
impl FromWorld for Root {
    fn from_world(_world: &mut World) -> Self {
        Root(Entity::from_raw(u32::MAX))
    }
}

impl MapEntities for Root {
    fn map_entities(&mut self, entity_map: &EntityMap) -> Result<(), MapEntitiesError> {
        // Root of an entity in the new world can be in outside world, in which case it
        // should not be mapped.
        if let Ok(mapped_entity) = entity_map.get(self.0) {
            self.0 = mapped_entity;
        }
        Ok(())
    }
}

impl Deref for Root {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
