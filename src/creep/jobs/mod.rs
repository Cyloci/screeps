use std::ops::Mul;

use screeps::{
    ConstructionSite, Creep, ObjectId, Position, Resource, ResourceType, Source, Structure,
    StructureController,
};
use serde::{Deserialize, Serialize};

mod build;
mod harvest;
mod move_to;
mod pickup;
mod repair;
mod transfer;
mod upgrade;
mod withdraw;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Radius(i32);

impl Mul for Radius {
    type Output = i32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreepJob {
    MoveTo(Position, Radius),
    MineSource(ObjectId<Source>),
    Transfer(ObjectId<Structure>, ResourceType),
    Withdraw(ObjectId<Structure>, ResourceType),
    Pickup(ObjectId<Resource>),
    Upgrade(ObjectId<StructureController>),
    Build(ObjectId<ConstructionSite>),
    Repair(ObjectId<Structure>),
}

pub enum CreepJobResult {
    Ongoing,
    Finished,
    NewJob(CreepJob),
    Failed(String),
}

impl CreepJob {
    pub fn run(&self, creep: &Creep) -> CreepJobResult {
        match self {
            &Self::MoveTo(position, radius) => move_to::move_to(creep, position, radius),
            &Self::MineSource(source_id) => harvest::mine_source(creep, source_id),
            &Self::Transfer(structure_id, resource_type) => {
                transfer::transfer(creep, resource_type, structure_id)
            }
            &Self::Withdraw(structure_id, resource_type) => {
                withdraw::withdraw(creep, resource_type, structure_id)
            }
            &Self::Pickup(resource_id) => pickup::pickup(creep, resource_id),
            &Self::Upgrade(controller_id) => upgrade::upgrade(creep, controller_id),
            &Self::Build(construction_site_id) => build::build(creep, construction_site_id),
            &Self::Repair(structure_id) => repair::repair(creep, structure_id),
        }
    }
}
