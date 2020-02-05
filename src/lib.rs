use wasm_bindgen::prelude::*;
use usvg::{Tree, Options};
use pathfinder_svg::BuiltSVG;

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug);
}

#[wasm_bindgen]
pub fn draw(svg: &[u8]) {
    let tree = Tree::from_data(svg, &Options::default()).unwrap();
    let scene = BuiltSVG::from_tree(tree).scene;

    pathfinder_view::show(scene, pathfinder_view::Config {
        zoom: false,
        pan: false
    });
}

