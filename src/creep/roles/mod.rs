use screeps::Creep;
use serde::{Deserialize, Serialize};

use super::jobs::CreepJob;

mod builder;
mod hauler;
mod miner;
mod upgrader;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CreepRole {
    Miner,
    Hauler,
    Upgrader,
    Builder,
}

impl CreepRole {
    pub fn get_new_job(&self, creep: &Creep) -> Option<CreepJob> {
        match self {
            Self::Miner => miner::get_new_job(&creep),
            Self::Hauler => hauler::get_new_job(&creep),
            Self::Upgrader => upgrader::get_new_job(&creep),
            Self::Builder => builder::get_new_job(&creep),
        }
    }
}
