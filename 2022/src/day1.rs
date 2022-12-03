use crate::aoc;

pub struct Day1_1;

impl aoc::Aoc<u32> for Day1_1 {
    fn day(&self) -> u32 {
        1
    }
    fn puzzle_name(&self) -> &str {
        "Calorie Counting"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        let food = split_lines(lines);
        total_cals(&food).max().unwrap_or(0)
    }
}

pub struct Day1_2;
impl aoc::Aoc<u32> for Day1_2 {
    fn day(&self) -> u32 {
        1
    }
    fn puzzle_name(&self) -> &str {
        "Top 3 Calorie Counting"
    }
    fn solve(&self, lines: &Vec<String>) -> u32 {
        let food = split_lines(lines);
        let mut total_cals: Vec<u32> = total_cals(&food).collect();
        total_cals.sort_by(|a, b| a.cmp(b).reverse());
        total_cals[0..3].iter().sum()
    }
}

type ElfFood = Vec<u32>;

pub fn total_cals(food: &Vec<ElfFood>) -> impl Iterator<Item = u32> + '_ {
    food.iter().map(|food| food.into_iter().sum())
}

pub fn split_lines(lines: &Vec<String>) -> Vec<ElfFood> {
    let mut elves_food: Vec<ElfFood> = vec![ElfFood::new()];
    lines
        .iter()
        .fold(&mut elves_food, |col, cal_str| -> &mut Vec<ElfFood> {
            let elf_food = col.last_mut().unwrap();
            match cal_str.as_str() {
                "" => {
                    col.push(ElfFood::new());
                    col
                }
                _ => {
                    elf_food.push(cal_str.parse().unwrap_or(0));
                    col
                }
            }
        });

    elves_food
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_lines_correctly() {
        let lines = vec![
            String::from("1"),
            String::from("2"),
            String::default(),
            String::from("3"),
            String::from("4"),
        ];

        let res = split_lines(&lines);

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].len(), 2);
        assert_eq!(res[1].len(), 2);
        assert_eq!(res[0][0], 1);
        assert_eq!(res[0][1], 2);
        assert_eq!(res[1][0], 3);
        assert_eq!(res[1][1], 4);
    }
}
