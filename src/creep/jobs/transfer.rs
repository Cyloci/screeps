use screeps::{
    Creep, ErrorCode, HasPosition, ObjectId, ResourceType, SharedCreepProperties, Structure,
    StructureObject,
};

use super::{CreepJob, CreepJobResult, Radius};

pub fn transfer(
    creep: &Creep,
    resource_type: ResourceType,
    structure_id: ObjectId<Structure>,
) -> CreepJobResult {
    let Some(structure) = structure_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve structure: {}", structure_id));
    };
    let structure_object = StructureObject::from(structure);
    let Some(transferable) = structure_object.as_transferable() else {
        return CreepJobResult::Failed(format!("structure is not transferable: {}", structure_id));
    };
    if creep.store().get_used_capacity(Some(resource_type)) == 0 {
        return CreepJobResult::Finished;
    }
    creep
        .transfer(transferable, resource_type, None)
        .map_or_else(
            |err| match err {
                ErrorCode::NotInRange => {
                    CreepJobResult::NewJob(CreepJob::MoveTo(transferable.pos(), Radius(1)))
                }
                _ => {
                    return CreepJobResult::Failed(format!("couldn't transfer: {:?}", err));
                }
            },
            |_| CreepJobResult::Ongoing,
        )
}
