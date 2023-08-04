use getrandom::register_custom_getrandom;

use log::{debug, info};
use screeps::game;
use wasm_bindgen::prelude::*;

mod creep;
mod custom_getrandom;
mod logging;
mod memory;
mod room;

use crate::{creep::run_creep, custom_getrandom::custom_getrandom, room::run_room};

#[wasm_bindgen]
pub fn setup() {
    memory::load();
    logging::setup_logging(logging::Debug);
    console_error_panic_hook::set_once();
    register_custom_getrandom!(custom_getrandom);
}

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    debug!(
        "loop starting at game time: {}. cpu: {}",
        game::time(),
        game::cpu::get_used()
    );

    if let Some(reset_flag) = game::flags().get("reset".to_string()) {
        info!("resetting!");
        for creep in game::creeps().values() {
            let _ = creep.suicide();
            reset_flag.remove();
        }
        return;
    }

    memory::with(|mut memory| {
        for room in game::rooms().values() {
            run_room(room, &mut memory);
        }

        for creep in game::creeps().values() {
            run_creep(creep, &mut memory);
        }

        memory
            .creeps
            .retain(|k, _| game::creeps().get(k.into()).is_some());
    });

    memory::save();
    info!("loop done! cpu: {}", game::cpu::get_used());
}
