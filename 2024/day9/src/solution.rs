#[derive(Debug, Clone, Copy, PartialEq)]
struct File(u16);

type Memory = Vec<Option<File>>;
type MemoryBlocks = Vec<(Option<File>, usize, usize)>;

fn checksum(mem: &Memory) -> usize {
    mem.iter()
        .enumerate()
        .filter(|a| a.1.is_some())
        .map(|a| a.0 * a.1.unwrap().0 as usize)
        .sum()
}

fn defrag(mem: &Memory) -> Memory {
    let mut new_mem = mem.clone();
    let mut free_space = mem.iter().enumerate().filter(|mem_loc| mem_loc.1.is_none());
    let mut data = mem
        .iter()
        .enumerate()
        .rev()
        .filter(|mem_loc| mem_loc.1.is_some());
    while let Some((free_i, _)) = free_space.next() {
        if let Some((data_i, &data_id)) = data.next() {
            if free_i < data_i {
                new_mem[free_i] = data_id;
                new_mem[data_i] = None;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    new_mem
}

fn defrag2(mem_blocks: &MemoryBlocks, mem: &Memory) -> Memory {
    let mut new_mem = mem.clone();
    let mut free_space = mem_blocks
        .iter()
        .filter(|a| a.0.is_none())
        .map(|&a| a)
        .collect::<Vec<_>>();

    mem_blocks.iter().rev().filter(|a| a.0.is_some()).for_each(
        |&(data_file, data_idx, data_len)| {
            let gap = free_space
                .iter()
                .enumerate()
                .find(|(_i, (_, _, free_len))| *free_len >= data_len);

            if let Some((i, free)) = gap {
                if free.1 < data_idx {
                    let free = free_space.get_mut(i).unwrap();
                    (0..data_len).for_each(|i| {
                        new_mem[free.1 + i] = data_file;
                        new_mem[data_idx + i] = None
                    });
                    free.1 += data_len;
                    free.2 -= data_len;
                }
            }
        },
    );

    new_mem
}

fn parse_input_as_nums(input: &str) -> Vec<u32> {
    input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn parse_input_as_memory_blocks(input: &str) -> MemoryBlocks {
    let nums: Vec<u32> = parse_input_as_nums(input);
    let mut id = 0;
    let mut idx = 0;
    nums.iter()
        .enumerate()
        .map(|(i, &len)| {
            let f = if i % 2 == 0 {
                assert!(len > 0);
                (Some(File(id)), idx, len as usize)
            } else {
                (None, idx, len as usize)
            };
            if i % 2 == 0 {
                id += 1;
            }
            idx += len as usize;
            f
        })
        .collect()
}

fn parse_input(input: &str) -> (MemoryBlocks, Memory) {
    let mem_blocks = parse_input_as_memory_blocks(input);
    let mem_size: usize = mem_blocks.iter().map(|mb| mb.2).sum();
    let mut mem = vec![None; mem_size];
    mem_blocks.iter().for_each(|&(file, loc, len)| {
        (0..len).for_each(|j| mem[loc + j] = file);
    });
    (mem_blocks, mem)
}

pub fn part1(input: &str) -> usize {
    let (_mem_blocks, mem) = parse_input(input);
    let defragged_mem = defrag(&mem);
    checksum(&defragged_mem)
}

pub fn part2(input: &str) -> usize {
    let (mem_blocks, mem) = parse_input(input);
    let defragged_mem = defrag2(&mem_blocks, &mem);
    checksum(&defragged_mem)
}

#[cfg(test)]
mod tests {

    use super::*;

    pub const TEST_INPUT: &str = include_str!("data/input_example");
    pub const TEST_ANSWER: usize = 1928;
    pub const TEST_INPUT_2: &str = TEST_INPUT;
    pub const TEST_ANSWER_2: usize = 2858;

    #[test]
    fn test_parse_input() {
        let (mem_blocks, mem) = parse_input("123");
        assert_eq!(
            mem,
            vec![
                Some(File(0)),
                None,
                None,
                Some(File(1)),
                Some(File(1)),
                Some(File(1))
            ]
        );
        assert_eq!(
            mem_blocks,
            vec![(Some(File(0)), 0, 1), (None, 1, 2), (Some(File(1)), 3, 3)]
        );
    }

    #[test]
    fn test_defrag() {
        let (_mem_blocks, mem) = parse_input("123");
        let mem = defrag(&mem);
        assert_eq!(
            mem,
            vec![
                Some(File(0)),
                Some(File(1)),
                Some(File(1)),
                Some(File(1)),
                None,
                None
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), TEST_ANSWER);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), TEST_ANSWER_2);
    }
}
