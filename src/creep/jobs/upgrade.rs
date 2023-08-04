use screeps::{Creep, ErrorCode, HasPosition, ObjectId, ResourceType, StructureController};

use super::{CreepJob, CreepJobResult, Radius};

pub fn upgrade(creep: &Creep, controller_id: ObjectId<StructureController>) -> CreepJobResult {
    let Some(controller) = controller_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve controller: {}", controller_id));
    };
    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
        return CreepJobResult::Finished;
    }
    creep.upgrade_controller(&controller).map_or_else(
        |err| match err {
            ErrorCode::NotInRange => {
                CreepJobResult::NewJob(CreepJob::MoveTo(controller.pos().pos(), Radius(3)))
            }
            _ => {
                return CreepJobResult::Failed(format!("couldn't upgrade: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
