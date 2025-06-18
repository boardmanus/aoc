#[derive(Debug, Clone, Copy, PartialEq)]
struct File(u16);

#[derive(Debug, Clone, Copy, PartialEq)]
struct MemoryBlock {
    file: Option<File>,
    loc: usize,
    len: usize,
}

type Memory = Vec<Option<File>>;

fn checksum(mem: &Memory) -> usize {
    mem.iter()
        .enumerate()
        .filter(|a| a.1.is_some())
        .map(|a| a.0 * a.1.unwrap().0 as usize)
        .sum()
}

fn defrag(mem: &Memory) -> Memory {
    let mut new_mem = mem.clone();
    let free_space = mem.iter().enumerate().filter(|mem_loc| mem_loc.1.is_none());
    let mut data = mem
        .iter()
        .enumerate()
        .rev()
        .filter(|mem_loc| mem_loc.1.is_some());
    for (free_i, _) in free_space {
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

fn defrag2(mem_blocks: &[MemoryBlock], mem: &Memory) -> Memory {
    let mut new_mem = mem.clone();
    let mut free_space = mem_blocks
        .iter()
        .filter(|a| a.file.is_none())
        .copied()
        .collect::<Vec<_>>();

    mem_blocks
        .iter()
        .rev()
        .filter(|a| a.file.is_some())
        .for_each(|blk| {
            let gap = free_space
                .iter()
                .enumerate()
                .find(|(_i, free_blk)| free_blk.len >= blk.len);

            if let Some((i, free)) = gap {
                if free.loc < blk.loc {
                    let free = free_space.get_mut(i).unwrap();
                    (0..blk.len).for_each(|i| {
                        new_mem[free.loc + i] = blk.file;
                        new_mem[blk.loc + i] = None
                    });
                    free.loc += blk.len;
                    free.len -= blk.len;
                }
            }
        });

    new_mem
}

fn parse_input_as_nums<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.trim_end().chars().filter_map(|c| c.to_digit(10))
}

fn parse_input_as_memory_blocks<'a>(input: &'a str) -> impl Iterator<Item = MemoryBlock> + 'a {
    let mut id = 0;
    let mut loc = 0;
    parse_input_as_nums(input).enumerate().map(move |(i, len)| {
        let file = if i % 2 == 0 { Some(File(id)) } else { None };
        let len = len as usize;
        let blk = MemoryBlock { file, loc, len };
        id += if i % 2 == 0 { 1 } else { 0 };
        loc += len;
        blk
    })
}

fn parse_input(input: &str) -> (Vec<MemoryBlock>, Memory) {
    let mem_blocks: Vec<_> = parse_input_as_memory_blocks(input).collect();
    let mem_size: usize = mem_blocks.iter().map(|mb| mb.len).sum();
    let mut mem = vec![None; mem_size];
    mem_blocks
        .iter()
        .for_each(|&MemoryBlock { file, loc, len }| {
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
            vec![
                MemoryBlock {
                    file: Some(File(0)),
                    loc: 0,
                    len: 1
                },
                MemoryBlock {
                    file: None,
                    loc: 1,
                    len: 2
                },
                MemoryBlock {
                    file: Some(File(1)),
                    loc: 3,
                    len: 3
                }
            ]
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
