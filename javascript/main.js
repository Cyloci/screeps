"use strict";
let wasm_module;

const MODULE_NAME = "screeps-starter-rust";

function console_error(...args) {
  console.log(...args);
  Game.notify(args.join(" "));
}

module.exports.loop = function () {
  delete global.Memory;
  global.Memory = {};
  try {
    if (wasm_module) {
      wasm_module.loop();
    } else {
      console.log("reloading wasm");
      if (Game.cpu.bucket < 500) {
        console.log(
          "we are running out of time, pausing compile!" +
            JSON.stringify(Game.cpu)
        );
        return;
      }
      if (MODULE_NAME in require.cache) {
        delete require.cache[MODULE_NAME];
      }
      wasm_module = require(MODULE_NAME);
      wasm_module.initialize_instance();

      // rust functions
      wasm_module.setup();
      wasm_module.loop();
    }
  } catch (error) {
    console_error("caught exception:", error);
    if (error.stack) {
      console_error("stack trace:", error.stack);
    }
    console_error("resetting VM next tick.");
    wasm_module = null;
  }
};
