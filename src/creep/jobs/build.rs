use screeps::{ConstructionSite, Creep, ErrorCode, HasPosition, ObjectId, ResourceType};

use super::{CreepJob, CreepJobResult, Radius};

pub fn build(creep: &Creep, construction_site_id: ObjectId<ConstructionSite>) -> CreepJobResult {
    let Some(construction_site) = construction_site_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve construction site: {}", construction_site_id));
    };
    if creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0 {
        return CreepJobResult::Finished;
    }
    creep.build(&construction_site).map_or_else(
        |err| match err {
            ErrorCode::NotInRange => {
                CreepJobResult::NewJob(CreepJob::MoveTo(construction_site.pos().pos(), Radius(3)))
            }
            _ => {
                return CreepJobResult::Failed(format!("couldn't build: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
