use screeps::{Creep, HasPosition, Position, SharedCreepProperties};

use super::{CreepJobResult, Radius};

pub fn move_to(creep: &Creep, position: Position, Radius(radius): Radius) -> CreepJobResult {
    let (dx, dy) = creep.pos().pos() - position;
    if dx.abs() <= radius && dy.abs() <= radius {
        return CreepJobResult::Finished;
    }
    creep.move_to(position).map_or_else(
        |err| match err {
            _ => {
                return CreepJobResult::Failed(format!("couldn't move: {:?}", err));
            }
        },
        |_| CreepJobResult::Ongoing,
    )
}
