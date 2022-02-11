use crate::models::geometry::PhyPos;
use crate::models::UniqueEntityId;
use bevy::ecs::entity::EntityMap;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::scene::serde::{SceneDeserializer, SceneSerializer};
use bevy::utils::HashMap;
use bincode::read_types::SliceReader;
use bincode::Infinite;
use bytes::Bytes;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkRequest {
    RequestWorld,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkResponse {
    ResponseWorld(Bytes),
}

pub fn get_type_registry() -> TypeRegistry {
    let registry = TypeRegistry::default();
    let mut lock = registry.write();
    lock.register::<UniqueEntityId>();
    lock.register::<PhyPos>();
    drop(lock);
    registry
}

pub fn make_world_backup(world: &World) -> Bytes {
    let registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&world, &registry);
    bincode::serialize(&SceneSerializer::new(&scene, registry), Infinite)
        .unwrap()
        .into()
}

pub fn restore_world_backup(world: &mut World, scene: &[u8]) {
    let registry = world.get_resource::<TypeRegistry>().unwrap();
    let registry = registry.read();

    let mut deserializer = bincode::Deserializer::new(SliceReader::new(scene), Infinite);
    let scene_deserializer = SceneDeserializer {
        type_registry: &*registry,
    };

    let scene = scene_deserializer.deserialize(&mut deserializer).unwrap();
    drop(registry);

    let mut map = EntityMap::default();
    let uuid_map1 = scene
        .entities
        .iter()
        .filter_map(|e| {
            Some((
                e.components
                    .iter()
                    .find(|x| x.type_name() == std::any::type_name::<UniqueEntityId>())
                    .map(|x| x.any().downcast_ref::<UniqueEntityId>().unwrap().clone())?,
                Entity::from_raw(e.entity),
            ))
        })
        .collect::<HashMap<_, _>>();
    let mut state: SystemState<Query<(Entity, &UniqueEntityId)>> = SystemState::new(world);
    for item in state.get(world).iter() {
        let (entity, uid) = item;
        if let Some(remote_id) = uuid_map1.get(&uid) {
            map.insert(remote_id.clone(), entity.clone());
        }
    }
    scene.write_to_world(world, &mut map).unwrap();
}

pub fn encode_response(msg: &NetworkResponse) -> Bytes {
    bincode::serialize(msg, Infinite).unwrap().into()
}

pub fn decode_response(msg: &[u8]) -> NetworkResponse {
    bincode::deserialize(msg).unwrap()
}

pub fn encode_request(msg: &NetworkRequest) -> Bytes {
    bincode::serialize(msg, Infinite).unwrap().into()
}

pub fn decode_request(msg: &[u8]) -> NetworkRequest {
    bincode::deserialize(msg).unwrap()
}
