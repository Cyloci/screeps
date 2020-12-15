import {
  cleanMemoryOfDeadCreeps,
  flushCreepCachedTarget,
  getCreepState,
  setCreepCachedTarget,
  setCreepState,
} from "memory";

import { ErrorMapper } from "utils/ErrorMapper";
import { checkEvents } from "eventCheckers";
import config from "config";
import { runAction } from "actionRunners";

const newState = (newStateId?: number): newStateId is number =>
  newStateId !== undefined;

const runCreep = (creep: Creep) => {
  if (!(creep.name in config.creeps)) {
    console.log(`killing ${creep.name} as they are not in config`);
    creep.suicide();
    return;
  }
  const transitions = getCreepState(creep).transitions;
  const newStateId = checkEvents(transitions, creep);
  if (newState(newStateId)) {
    setCreepState(creep, newStateId);
    flushCreepCachedTarget(creep);
  }
  const action = getCreepState(creep).action;
  const { target } = runAction(action)(creep);
  if (
    (creep.spawning || newState(newStateId)) &&
    target !== null &&
    !(target instanceof RoomPosition || target instanceof Flag)
  ) {
    setCreepCachedTarget(creep, target);
  }
};

// Reversed because last creep definition is spawned
const configCreeps = Object.entries(config.creeps).reverse();

const replaceDeadCreeps = () =>
  configCreeps.forEach(([name, { body, memory }]) => {
    if (!(name in Game.creeps)) {
      Game.spawns.Spawn1.spawnCreep(body, name, { memory });
    }
  });

export const loop = ErrorMapper.wrapLoop(() => {
  console.log(`Current game tick is ${Game.time}`);
  cleanMemoryOfDeadCreeps();
  replaceDeadCreeps();
  Object.values(Game.creeps).forEach(runCreep);
});
