async function init() {
    await wasm_bindgen("pkg/webgl_bg.wasm").catch(console.error);
    let response = await fetch("Ghostscript_Tiger.svg");
    let data = await response.arrayBuffer();
    wasm_bindgen.draw(new Uint8Array(data));
}
