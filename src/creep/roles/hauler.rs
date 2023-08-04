use rand::seq::SliceRandom;
use screeps::{
    find::{DROPPED_RESOURCES, MY_SPAWNS},
    Creep, HasTypedId, ResourceType, Structure,
};

use crate::creep::jobs::CreepJob;

pub fn get_new_job(creep: &Creep) -> Option<CreepJob> {
    if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
        let spawn: Structure = creep
            .room()
            .unwrap()
            .find(MY_SPAWNS, None)
            .pop()
            .unwrap()
            .into();
        Some(CreepJob::Transfer(spawn.id(), ResourceType::Energy))
    } else {
        creep
            .room()
            .map(|room| room.find(DROPPED_RESOURCES, None))
            .and_then(|resources| resources.choose(&mut rand::thread_rng()).cloned())
            .map(|resource| CreepJob::Pickup(resource.id()))
    }
}
