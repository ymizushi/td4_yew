mod components;
use components::Model;

use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
