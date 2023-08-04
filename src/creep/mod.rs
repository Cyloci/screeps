use log::warn;
use screeps::{game, Creep, Part, SharedCreepProperties, StructureSpawn};
use serde::{Deserialize, Serialize};

use crate::memory::GlobalMemory;

pub mod jobs;
pub mod roles;

use jobs::CreepJob;
use roles::CreepRole;

use self::jobs::CreepJobResult;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreepName(String);

impl From<String> for CreepName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<String> for CreepName {
    fn into(self) -> String {
        self.0
    }
}

impl Into<String> for &CreepName {
    fn into(self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreepMemory {
    pub role: CreepRole,
    pub jobs: Vec<CreepJob>,
}

pub fn run_creep(creep: Creep, memory: &mut GlobalMemory) {
    if creep.spawning() {
        return;
    }
    let creep_memory = memory.creeps.get_mut(&creep.name().into());
    let Some(creep_memory) = creep_memory else {
        warn!("creep '{}' has no memory", creep.name());
        return;
    };
    let Some(job) = creep_memory.jobs.pop()
        .or_else(|| {
            creep_memory.role.get_new_job(&creep)
        })
    else {
        warn!("creep '{}' has no job", creep.name());
        return;
    };
    match job.run(&creep) {
        CreepJobResult::NewJob(new_job) => {
            creep_memory.jobs.push(job);
            creep_memory.jobs.push(new_job);
        }
        CreepJobResult::Finished => {}
        CreepJobResult::Failed(reason) => {
            warn!("creep '{}' failed job: {:?}", creep.name(), reason);
        }
        CreepJobResult::Ongoing => {
            creep_memory.jobs.push(job);
        }
    };
}

pub fn spawn_creep(
    spawn: &StructureSpawn,
    memory: &mut GlobalMemory,
    role: CreepRole,
    jobs: Vec<CreepJob>,
    body: &[Part],
) {
    let name = format!("{:?}-{}", role, game::time());
    let result = spawn.spawn_creep(body, &name);
    if result.is_ok() {
        memory
            .creeps
            .insert(CreepName(name), CreepMemory { role, jobs });
    }
}
