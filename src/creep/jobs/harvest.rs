use screeps::{Creep, ErrorCode, HasPosition, ObjectId, Source};

use super::{CreepJob, CreepJobResult, Radius};

pub fn mine_source(creep: &Creep, source_id: ObjectId<Source>) -> CreepJobResult {
    let Some(source) = source_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve source: {}", source_id));
    };
    creep.harvest(&source).map_or_else(
        |err| match err {
            ErrorCode::NotInRange => {
                CreepJobResult::NewJob(CreepJob::MoveTo(source.pos().pos(), Radius(1)))
            }
            _ => {
                return CreepJobResult::Failed(format!("couldn't harvest: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
