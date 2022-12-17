use std::{
    collections::{HashMap, HashSet},
    fmt,
};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
enum Cell {
    // Start position
    Start,
    // End position
    End,
    // Square with given elevation
    Square(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(e) => e,
        }
    }
}

struct CellRecord {
    prev: Option<GridCoord>,
}

#[wasm_bindgen]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    visited: HashMap<GridCoord, CellRecord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Self {
        let first_line = input.lines().next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();
        let mut data = vec![];
        for c in input.chars() {
            let cell = match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Square(c as u8 - b'a'),
                '\r' | '\n' => continue,
                _ => panic!("invalid character: {c}"),
            };
            data.push(cell);
        }
        Self {
            width,
            height,
            data,
            current: Default::default(),
            visited: Default::default(),
            num_steps: 0,
        }
    }
}

impl Grid {
    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_elev = self.cell(coord).unwrap().elevation();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x + dx as usize,
                y: coord.y + dy as usize,
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(|&coord| {
                let other_elev = self.cell(coord).unwrap().elevation();
                other_elev <= curr_elev + 1
            })
        })
    }
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen]
    pub fn step(&mut self) {
        if self.current.is_empty() {
            // find start coordinate
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let Cell::Start = self.cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited.insert(start_coord, CellRecord { prev: None });
            return;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for ncoord in self.walkable_neighbors(curr) {
                if visited.contains_key(&ncoord) {
                    // don't visit it again!
                    continue;
                }
                visited.insert(ncoord, CellRecord { prev: Some(curr) });
                next.insert(ncoord);
            }
        }
        self.current = next;
        self.visited = visited;
        self.num_steps += 1;
    }

    #[wasm_bindgen]
    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }

    #[wasm_bindgen]
    pub fn num_cells(&self) -> usize {
        self.width * self.height
    }

    #[wasm_bindgen]
    pub fn num_steps(&self) -> usize {
        self.num_steps
    }
}

use svg::Document;

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen]
    pub fn to_svg(&self) -> String {
        const SIDE: usize = 64;
        let side = SIDE as f64;

        let mut document =
            Document::new().set("viewBox", (0, 0, self.width * SIDE, self.height * SIDE));

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let (title, r, g, b) = match cell {
                    Cell::Start => ("start".to_string(), 216, 27, 96),
                    Cell::End => ("end".to_string(), 30, 136, 229),
                    Cell::Square(elevation) => {
                        let title = format!("elevation {elevation}");
                        let elevation = *elevation as f32 / 25.0;
                        let f = (elevation * 180.0) as u8;
                        (title, f, f, f)
                    }
                };
                let rect = svg::node::element::Rectangle::new()
                    .set("x", x * SIDE)
                    .set("y", y * SIDE)
                    .set("width", SIDE)
                    .set("height", SIDE)
                    .set("fill", format!("rgb({r}, {g}, {b})"))
                    .set("stroke", "white")
                    .set("stroke-width", "2px")
                    .add(svg::node::element::Title::new().add(svg::node::Text::new(title)));
                document = document.add(rect);
            }
        }

        let defs = svg::node::element::Definitions::new().add(
            svg::node::element::Marker::new()
                .set("id", "arrowhead")
                .set("markerWidth", 10)
                .set("markerHeight", 7)
                .set("refX", 7)
                .set("refY", 3.5)
                .set("orient", "auto")
                .add(
                    svg::node::element::Polygon::new()
                        .set("points", "0 0, 10 3.5, 0 7")
                        // `stroke-context` is supposed to work but I couldn't
                        // get it to work.
                        .set("fill", "#ffc107"),
                ),
        );
        document = document.add(defs);

        for coord in self.visited.keys() {
            let circle = svg::node::element::Circle::new()
                .set("cx", (coord.x as f64 + 0.5) * side)
                .set("cy", (coord.y as f64 + 0.5) * side)
                .set("r", side * 0.1)
                .set("fill", "#fff");
            document = document.add(circle);
        }

        for coord in self.current.iter() {
            let circle = svg::node::element::Circle::new()
                .set("cx", (coord.x as f64 + 0.5) * side)
                .set("cy", (coord.y as f64 + 0.5) * side)
                .set("r", side * 0.1)
                .set("fill", "#ffc107");
            document = document.add(circle);

            let record = self.visited.get(coord).unwrap();
            let mut curr = record;
            let mut coord = *coord;
            while let Some(prev) = curr.prev.as_ref() {
                curr = self.visited.get(prev).unwrap();

                let (x, y) = (prev.x as f64, prev.y as f64);
                let dx = coord.x as f64 - x;
                let dy = coord.y as f64 - y;

                let line = svg::node::element::Line::new()
                    .set("x1", (x + 0.5 + dx * 0.2) * side)
                    .set("y1", (y + 0.5 + dy * 0.2) * side)
                    .set("x2", (x + 0.5 + dx * 0.8) * side)
                    .set("y2", (y + 0.5 + dy * 0.8) * side)
                    .set("stroke", "#ffc107")
                    .set("stroke-width", "1.5px")
                    .set("marker-end", "url(#arrowhead)");
                document = document.add(line);

                coord = *prev;
            }
        }

        document.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Grid {
    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} grid:", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{grid:?}");
}
