use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use log::warn;
use screeps::{
    find::{MY_SPAWNS, SOURCES},
    HasPosition, HasTypedId, ObjectId, OwnedStructureProperties, Part, Position, Room, Source,
};

use serde::{Deserialize, Serialize};

use crate::{
    creep::{jobs::CreepJob, roles::CreepRole, spawn_creep},
    memory::GlobalMemory,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMemory {
    pub mining_position: Position,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomMemory {
    pub sources: HashMap<ObjectId<Source>, SourceMemory>,
}

pub fn run_room(room: Room, memory: &mut GlobalMemory) {
    let Some(controller) = room.controller() else {
        return;
    };
    if !controller.my() {
        return;
    }
    match controller.level() {
        1 => run_level_1_room(room, memory),
        _ => run_level_2_room(room, memory),
    }
}

fn run_level_1_room(room: Room, memory: &mut GlobalMemory) {
    let spawns = room.find(MY_SPAWNS, None);
    let Some(spawn) = spawns.get(0) else {
        warn!("no spawn in room: {}", room.name());
        return;
    };

    let room_memory = memory.rooms.entry(room.name()).or_insert_with(|| {
        let sources: HashMap<ObjectId<Source>, SourceMemory> = room
            .find(SOURCES, None)
            .into_iter()
            .map(|source| {
                (
                    source.id(),
                    SourceMemory {
                        mining_position: source.pos().pos(),
                    },
                )
            })
            .collect();
        RoomMemory { sources }
    });

    let creeps_in_roles = memory
        .creeps
        .values()
        .into_group_map_by(|creep_memory| &creep_memory.role);

    let num_miners = creeps_in_roles
        .get(&CreepRole::Miner)
        .map_or(0, |creeps| creeps.len());

    let num_haulers = creeps_in_roles
        .get(&CreepRole::Hauler)
        .map_or(0, |creeps| creeps.len());

    let num_upgraders = creeps_in_roles
        .get(&CreepRole::Upgrader)
        .map_or(0, |creeps| creeps.len());

    let mut free_source_ids = get_free_source_ids(&room, memory);

    if num_miners < 1 {
        if let Some(source_id) = free_source_ids.pop() {
            spawn_creep(
                spawn,
                memory,
                CreepRole::Miner,
                vec![CreepJob::MineSource(source_id)],
                &[Part::Move, Part::Work],
            );
            return;
        }
    }

    if num_haulers < 1 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Hauler,
            vec![],
            &[Part::Move, Part::Carry],
        );
        return;
    }

    if let Some(source_id) = free_source_ids.pop() {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Miner,
            vec![CreepJob::MineSource(source_id)],
            &[Part::Move, Part::Work],
        );
        return;
    }

    if num_haulers < 4 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Hauler,
            vec![],
            &[Part::Move, Part::Carry],
        );
        return;
    }

    if num_upgraders < 3 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Upgrader,
            vec![],
            &[Part::Move, Part::Carry, Part::Work],
        );
        return;
    }
}

fn run_level_2_room(room: Room, memory: &mut GlobalMemory) {
    let spawns = room.find(MY_SPAWNS, None);
    let Some(spawn) = spawns.get(0) else {
        warn!("no spawn in room: {}", room.name());
        return;
    };

    let creeps_in_roles = memory
        .creeps
        .values()
        .into_group_map_by(|creep_memory| &creep_memory.role);

    let num_haulers = creeps_in_roles
        .get(&CreepRole::Hauler)
        .map_or(0, |creeps| creeps.len());

    let num_upgraders = creeps_in_roles
        .get(&CreepRole::Upgrader)
        .map_or(0, |creeps| creeps.len());

    let num_builders = creeps_in_roles
        .get(&CreepRole::Builder)
        .map_or(0, |creeps| creeps.len());

    let mut free_source_ids = get_free_source_ids(&room, memory);

    if let Some(source_id) = free_source_ids.pop() {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Miner,
            vec![CreepJob::MineSource(source_id)],
            &[Part::Move, Part::Work, Part::Move, Part::Work],
        );
        return;
    }

    if num_haulers < 4 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Hauler,
            vec![],
            &[Part::Move, Part::Carry, Part::Move, Part::Carry],
        );
        return;
    }

    if num_upgraders < 6 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Upgrader,
            vec![],
            &[Part::Move, Part::Carry, Part::Work],
        );
        return;
    }

    if num_builders < 4 {
        spawn_creep(
            spawn,
            memory,
            CreepRole::Builder,
            vec![],
            &[Part::Move, Part::Move, Part::Carry, Part::Work],
        );
        return;
    }
}

fn get_plain_terrain_around_position(position: Position) {
    for dx in vec![-1, 0, 1] {
        for dy in vec![-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let objects = position.checked_add((dx, dy)).unwrap().look().unwrap();
        }
    }
}

fn get_free_source_ids(room: &Room, memory: &mut GlobalMemory) -> Vec<ObjectId<Source>> {
    let taken_source_ids: HashSet<&ObjectId<Source>> = memory
        .creeps
        .values()
        .filter_map(|creep_memory| match creep_memory.jobs.as_slice() {
            [CreepJob::MineSource(source_id), ..] => Some(source_id),
            _ => None,
        })
        .collect();

    let free_source_ids: Vec<ObjectId<Source>> = room
        .find(SOURCES, None)
        .iter()
        .map(|s| s.id())
        .filter(|id| !taken_source_ids.contains(id))
        .collect();

    free_source_ids
}
