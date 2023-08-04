use rand::{seq::SliceRandom, Rng};
use screeps::{
    find::{MY_CONSTRUCTION_SITES, MY_SPAWNS, STRUCTURES},
    Creep, HasTypedId, MaybeHasTypedId, ResourceType, Structure,
};

use crate::creep::jobs::CreepJob;

pub fn get_new_job(creep: &Creep) -> Option<CreepJob> {
    if creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 {
        if rand::thread_rng().gen_bool(0.5) {
            let construction_site = creep
                .room()
                .unwrap()
                .find(MY_CONSTRUCTION_SITES, None)
                .choose(&mut rand::thread_rng())
                .cloned();
            construction_site
                .and_then(|site| site.try_id())
                .map(|id| CreepJob::Build(id))
        } else {
            let mut structures: Vec<Structure> = creep
                .room()
                .unwrap()
                .find(STRUCTURES, None)
                .into_iter()
                .map(|so| so.as_structure().clone())
                .collect();
            structures.sort_by(|a, b| a.hits().cmp(&b.hits()));
            structures
                .pop()
                .map(|structure| structure.id())
                .map(|id| CreepJob::Repair(id))
        }
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
