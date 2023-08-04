use screeps::{find::MY_SPAWNS, Creep, HasTypedId, ResourceType, Structure};

use crate::creep::jobs::CreepJob;

pub fn get_new_job(creep: &Creep) -> Option<CreepJob> {
    if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
        creep
            .room()
            .and_then(|room| room.controller())
            .map(|controller| CreepJob::Upgrade(controller.id()))
    } else {
        let spawn: Structure = creep
            .room()
            .unwrap()
            .find(MY_SPAWNS, None)
            .pop()
            .unwrap()
            .into();
        Some(CreepJob::Withdraw(spawn.id(), ResourceType::Energy))
    }
}
