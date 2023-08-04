use std::{cell::RefCell, collections::HashMap};

use js_sys::JsString;

use log::{debug, warn};
use screeps::{raw_memory, RoomName};
use serde::{Deserialize, Serialize};

use crate::{
    creep::{CreepMemory, CreepName},
    room::RoomMemory,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GlobalMemory {
    pub creeps: HashMap<CreepName, CreepMemory>,
    pub rooms: HashMap<RoomName, RoomMemory>,
}

thread_local! {
    static GLOBAL_MEMORY: RefCell<GlobalMemory> = RefCell::new(GlobalMemory::default());
}

pub fn load() {
    debug!("parsing raw memory into global memory");
    let global_memory = match serde_json::from_str(&String::from(raw_memory::get())) {
        Ok(global_memory) => global_memory,
        Err(e) => {
            warn!("failed to parse raw memory: {:?}", e);
            GlobalMemory::default()
        }
    };
    GLOBAL_MEMORY.with(|global_memory_refcell| {
        global_memory_refcell.replace(global_memory);
    });
}

pub fn with<F, R>(f: F)
where
    F: FnOnce(&mut GlobalMemory) -> R,
{
    GLOBAL_MEMORY.with(|global_memory_refcell| {
        let memory = &mut global_memory_refcell.borrow_mut();
        f(memory)
    });
}

pub fn save() {
    debug!("storing global memory in raw memory");
    GLOBAL_MEMORY.with(|global_memory_refcell| {
        match serde_json::to_string(global_memory_refcell) {
            Ok(v) => {
                raw_memory::set(&JsString::from(v));
            }
            Err(e) => {
                warn!("memory write error: {:?}", e);
            }
        }
    });
}
