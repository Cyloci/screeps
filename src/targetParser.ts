import {
  FindFilter,
  FindOpts,
  TargetDescription,
  ValueEqual,
  WithinBounds,
} from "state/targets";
import { Predicate, composePredicates } from "utils/utils";
import { flushCreepCachedTarget, getCreepCachedTarget } from "memory";

const parseValueEqualFilter = (filter: ValueEqual) => {
  switch (filter.property) {
    case "structureType":
      return (s: StructureContainer) => s[filter.property] === filter.value;
  }
};

const parseWithinBoundsFilter = (filter: WithinBounds) => {
  const { min, max, isPercent } = filter.opts;
  switch (filter.property) {
    case "amount":
      return (s: { amount: number }) => {
        if (isPercent) {
          throw new Error("can not use percent when filtering on amount");
        }
        const value = s.amount;
        return min <= value && value <= max;
      };
    case "energy":
      return (s: { store: StoreDefinition }) => {
        const value = isPercent
          ? s.store[RESOURCE_ENERGY] / s.store.getCapacity(RESOURCE_ENERGY)
          : s.store[RESOURCE_ENERGY];
        return min <= value && value <= max;
      };
    case "hits":
      return (s: { hits: number; hitsMax: number }) => {
        const value = isPercent ? s.hits / s.hitsMax : s.hits;
        return min <= value && value <= max;
      };
  }
};

const parseFindFilter = (filter: FindFilter): Predicate<any> => {
  switch (filter.type) {
    case "valueEqual":
      return parseValueEqualFilter(filter);
    case "withinBounds":
      return parseWithinBoundsFilter(filter);
  }
};

interface ScreepsFindOpts {
  filter: Predicate<any>;
}

const parseFindOpts = (findOpts: FindOpts): ScreepsFindOpts => ({
  filter: findOpts.filters
    .map(parseFindFilter)
    .reduce(composePredicates, () => true),
});

export const getTarget = <F extends FindConstant = FindConstant>(
  target: TargetDescription<F>,
  creep: Creep,
): FindTypes[F] | null => {
  switch (target.type) {
    case "closest": {
      const cachedTarget = getCreepCachedTarget<F>(creep);
      const screepFindOpts = parseFindOpts(target.opts);
      if (cachedTarget) {
        if (screepFindOpts.filter(cachedTarget)) {
          return cachedTarget;
        } else {
          flushCreepCachedTarget(creep);
        }
      }
      return creep.pos.findClosestByPath(target.find, screepFindOpts);
    }
    case "specific":
      return Game.getObjectById(target.targetId);
  }
};
