use day12::Grid;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{grid:?}");
}
