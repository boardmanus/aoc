mod rts;
mod utils;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use log;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    log!("Initialized!");
}

const BLUEPRINT_STR: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";


#[wasm_bindgen]
pub struct WebBluePrint(rts::BluePrint);

#[wasm_bindgen]
impl WebBluePrint {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> WebBluePrint {
        Self(rts::BluePrint::parse_line(input).unwrap())
    }
    
    #[wasm_bindgen]
    pub fn default() -> Self {
        WebBluePrint::new(BLUEPRINT_STR)
    }
}


#[wasm_bindgen]
pub struct WebState(rts::State, rts::Path);

#[wasm_bindgen]
impl WebState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebState {
        WebState(rts::State::default(), rts::Path::Empty)
    }
}

#[wasm_bindgen]
pub struct WebTime(usize, HashMap<rts::State, rts::Path>);

#[wasm_bindgen]
impl WebTime {

    #[wasm_bindgen(constructor)]
    pub fn new() -> WebTime {
        let mut states = HashMap::default();
        states.insert(rts::State::default(), rts::Path::Empty);
        WebTime(0, states)
    }

    #[wasm_bindgen]
    pub fn update(
        &self,
        blueprint: &WebBluePrint) -> WebTime {
            let mut new_states = HashMap::default();

            self.1.iter().for_each(|x| x.0.update(&blueprint.0, x.1.clone(), &mut new_states));
            WebTime(self.0 + 1, new_states)
    }
}
