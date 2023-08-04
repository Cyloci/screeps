use screeps::{Creep, ErrorCode, HasPosition, ObjectId, ResourceType, Structure};

use super::{CreepJob, CreepJobResult, Radius};

pub fn repair(creep: &Creep, structure_id: ObjectId<Structure>) -> CreepJobResult {
    let Some(structure) = structure_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve structure: {}", structure_id));
    };
    if structure.hits() == structure.hits_max() {
        return CreepJobResult::Finished;
    }
    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
        return CreepJobResult::Finished;
    }
    creep.repair(&structure).map_or_else(
        |err| match err {
            ErrorCode::NotInRange => {
                CreepJobResult::NewJob(CreepJob::MoveTo(structure.pos().pos(), Radius(3)))
            }
            _ => {
                return CreepJobResult::Failed(format!("couldn't repair: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
