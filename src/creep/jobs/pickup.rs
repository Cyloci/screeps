use screeps::{Creep, ErrorCode, HasPosition, ObjectId, Resource};

use super::{CreepJob, CreepJobResult, Radius};

pub fn pickup(creep: &Creep, resource_id: ObjectId<Resource>) -> CreepJobResult {
    let Some(resource) = resource_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve resource: {}", resource_id));
    };
    if resource.amount() == 0 {
        return CreepJobResult::Finished;
    }
    if creep
        .store()
        .get_free_capacity(Some(resource.resource_type()))
        == 0
    {
        return CreepJobResult::Finished;
    }
    creep.pickup(&resource).map_or_else(
        |err| match err {
            ErrorCode::NotInRange => {
                CreepJobResult::NewJob(CreepJob::MoveTo(resource.pos(), Radius(1)))
            }
            _ => {
                return CreepJobResult::Failed(format!("couldn't pickup: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
