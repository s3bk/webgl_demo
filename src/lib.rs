use wasm_bindgen::prelude::*;
use usvg::{Tree, Options};
use pathfinder_svg::BuiltSVG;
use pathfinder_view::{WasmView, Config, Interactive};
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug);
}

#[wasm_bindgen]
pub fn view(canvas: HtmlCanvasElement, svg: &[u8]) -> WasmView {
    let tree = Tree::from_data(svg, &Options::default()).unwrap();
    let scene = BuiltSVG::from_tree(&tree).scene;

    let config = Config {
        zoom: false,
        pan: false
    };
    WasmView::new(canvas, config, Box::new(scene) as Box<dyn Interactive>)
}

