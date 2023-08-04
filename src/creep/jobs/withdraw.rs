use screeps::{
    Creep, ErrorCode, HasPosition, ObjectId, ResourceType, SharedCreepProperties, Structure,
    StructureObject,
};

use super::{CreepJob, CreepJobResult, Radius};

pub fn withdraw(
    creep: &Creep,
    resource_type: ResourceType,
    structure_id: ObjectId<Structure>,
) -> CreepJobResult {
    let Some(structure) = structure_id.resolve() else {
        return CreepJobResult::Failed(format!("could not resolve structure: {}", structure_id));
    };
    let structure_object = StructureObject::from(structure);
    let Some(withdrawable) = structure_object.as_withdrawable() else {
        return CreepJobResult::Failed(format!("structure is not withdrawable: {}", structure_id));
    };
    if creep.store().get_free_capacity(Some(resource_type)) == 0 {
        return CreepJobResult::Finished;
    }
    creep
        .withdraw(withdrawable, resource_type, None)
        .map_or_else(
            |err| match err {
                ErrorCode::NotInRange => {
                    CreepJobResult::NewJob(CreepJob::MoveTo(withdrawable.pos(), Radius(1)))
                }
                _ => {
                    return CreepJobResult::Failed(format!("couldn't withdraw: {:?}", err));
                }
            },
            |_| CreepJobResult::Ongoing,
        )
}
