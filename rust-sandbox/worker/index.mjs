// Worker

import wasm_init, * as wasm from "wasm/rustwasm.js"
import wasm_module from "wasm/rustwasm_bg.wasm"
import { Counter } from "wasm/rustwasm.js";

export default {
  async fetch(request, env) {
    return await handleRequest(request, env);
  }
}

export {Counter}

async function handleRequest(request, env) {
    await wasm_init(wasm_module);

    return wasm.main('fetch', request, env);
}