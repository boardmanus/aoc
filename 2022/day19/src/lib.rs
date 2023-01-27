mod rts;
mod utils;
use std::{cmp::Ordering, collections::HashMap};
use svg::Document;

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

    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        let rr = self.0.robot_recipes;
        format!(
            "Blueprint: Ore({}, {}, {}), Clay({}, {}, {}), Obsidian({}, {}, {}), Geode({}, {}, {})",
            rr[0].quantity[0],
            rr[0].quantity[1],
            rr[0].quantity[2],
            rr[1].quantity[0],
            rr[1].quantity[1],
            rr[1].quantity[2],
            rr[2].quantity[0],
            rr[2].quantity[1],
            rr[2].quantity[2],
            rr[3].quantity[0],
            rr[3].quantity[1],
            rr[3].quantity[2],
        )
    }
}

#[wasm_bindgen]
pub struct WebState {
    state: rts::State,
    path: rts::Path,
}

impl WebState {
    fn cmp(&self, other: &WebState) -> Ordering {
        let ares = &self.state.resources.quantity;
        let bres = &other.state.resources.quantity;
        if ares[3] != bres[3] {
            return ares[3].cmp(&bres[3]);
        }

        let ar = &self.state.robots.quantity;
        let br = &other.state.robots.quantity;
        if ar[3] != br[3] {
            ar[3].cmp(&br[3])
        } else if ar[2] != br[2] {
            ar[2].cmp(&br[2])
        } else if ar[1] != br[1] {
            ar[1].cmp(&br[1])
        } else {
            ar[0].cmp(&br[0])
        }
    }

    fn to_svg(&self, blueprint: &WebBluePrint, num_items: f32) -> impl svg::Node {
        let res = &self.state.resources.quantity;
        let rob = &self.state.robots.quantity;
        let ft: String = if let Some(f) = self.state.factory_target {
            f.to_string()
        } else {
            String::from("None")
        };
        let resources = format!(
            "Resources({},{},{},{}), Robots({},{},{},{}), Factory({})",
            res[0], res[1], res[2], res[3], rob[0], rob[1], rob[2], rob[3], ft
        );

        let r = rob[0] * 255 / blueprint.0.max_robots.quantity[0];
        let g = rob[1] * 255 / blueprint.0.max_robots.quantity[1];
        let b = rob[2] * 255 / blueprint.0.max_robots.quantity[2];

        let rect = svg::node::element::Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 100.0 / num_items)
            .set("height", 100)
            .set("fill", format!("rgb({r}, {g}, {b})"))
            .set("stroke", "black")
            .set("stroke-width", "0px")
            .add(svg::node::element::Title::new().add(svg::node::Text::new(resources)));
        rect
    }
}

#[wasm_bindgen]
pub struct WebTime {
    tick: usize,
    states: HashMap<rts::State, rts::Path>,
}

#[wasm_bindgen]
impl WebTime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebTime {
        let mut states = HashMap::default();
        states.insert(rts::State::default(), rts::Path::Empty);
        WebTime { tick: 0, states }
    }

    #[wasm_bindgen]
    pub fn tick(&self) -> usize {
        self.tick
    }

    #[wasm_bindgen]
    pub fn update(&self, blueprint: &WebBluePrint) -> WebTime {
        let mut states = HashMap::default();

        self.states
            .iter()
            .for_each(|(state, path)| state.update(&blueprint.0, path.clone(), &mut states));
        WebTime {
            tick: self.tick + 1,
            states,
        }
    }

    #[wasm_bindgen]
    pub fn to_svg(&self, blueprint: &WebBluePrint) -> String {
        const SIDE: usize = 64;
        let side = SIDE as f64;

        let mut document = Document::new().set("viewBox", (0, 0, 100, 50));
        let mut x = 0.0;
        let mut bob = self
            .states
            .iter()
            .map(|(state, path)| WebState {
                state: *state,
                path: path.clone(),
            })
            .collect::<Vec<_>>();
        bob.sort_by(WebState::cmp);

        let num_items = bob.len() as f32;
        document = bob.iter().fold(document, |doc, ws| {
            let g = svg::node::element::Group::new()
                .set(
                    "transform",
                    format!("translate({},{})", x * 100.0 / num_items, 0),
                )
                .add(ws.to_svg(blueprint, num_items));
            x += 1.0;
            doc.add(g)
        });
        let svg_str = document.to_string();
        log!("{svg_str}");
        svg_str
    }
}
