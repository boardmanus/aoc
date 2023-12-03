use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

struct Node {
    element: char,
    next: ListNode,
}

type ListNode = Option<Box<Node>>;

impl Node {
    fn new(element: char, next: ListNode) -> ListNode {
        Some(Box::new(Node { element, next }))
    }

    fn insert_after(&mut self, element: char) {
        let next = self.next.take();
        self.next = Node::new(element, next);
    }
}

type PolymerMap = HashMap<(char, char), char>;

struct ListIter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for ListIter<'a> {
    type Item = &'a char;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

struct ListIterMut<'a> {
    next: Option<&'a mut Node>,
}

impl<'a> ListIterMut<'a> {
    fn insert_after(&mut self, element: char) {
        if let Some(node) = self.next.as_deref_mut() {
            node.insert_after(element);
            self.next();
        }
    }

    fn need_insertion(&self, map: &PolymerMap) -> Option<char> {
        if let Some(node) = self.next.as_deref() {
            if let Some(next_node) = node.next.as_deref() {
                if let Some(new_element) = map.get(&(node.element, next_node.element)) {
                    return Some(*new_element);
                }
            }
        }
        None
    }

    fn find_next(&mut self, map: &PolymerMap) -> Option<char> {
        if let Some(c) = self.need_insertion(map) {
            return Some(c);
        }

        while let Some(_) = self.next() {
            if let Some(c) = self.need_insertion(map) {
                return Some(c);
            }
        }

        None
    }
}

impl<'a> Iterator for ListIterMut<'a> {
    type Item = &'a mut char;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

struct List {
    head: ListNode,
}

impl List {
    fn new(head: ListNode) -> List {
        List { head }
    }

    fn len(&self) -> usize {
        self.iter().count()
    }

    fn iter(&self) -> ListIter<'_> {
        ListIter {
            next: self.head.as_deref(),
        }
    }
    fn iter_mut(&mut self) -> ListIterMut<'_> {
        ListIterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seen_first = false;
        self.iter().fold(Ok(()), |_, n| {
            if seen_first {
                write!(f, "->")?;
            }
            seen_first = true;
            write!(f, "[{n}]")
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

struct PairInsertionMap {
    template: List,
    map: PolymerMap,
    counts: HashMap<char, usize>,
}

fn update_count(counts: &mut HashMap<char, usize>, c: char) {
    counts.entry(c).and_modify(|count| *count += 1).or_insert(1);
}

impl PairInsertionMap {
    fn new(template: List, map: PolymerMap) -> Self {
        let counts = template
            .iter()
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                update_count(&mut acc, *c);
                acc
            });
        PairInsertionMap {
            template,
            map,
            counts,
        }
    }

    fn insert(&mut self) {
        let mut it = self.template.iter_mut();
        while let Some(e) = it.find_next(&self.map) {
            it.insert_after(e);
            update_count(&mut self.counts, e);
            it.next();
        }
        //println!("{}", self.template);
    }
}

fn parse_pim<'a>(input: &'a str) -> PairInsertionMap {
    let mut lines = input.lines();
    let template_str = lines.next().expect("template");
    lines.next();
    let map = lines.fold(PolymerMap::new(), |mut acc, line| {
        let mut line_it = line.split(" -> ");
        let mut pair: std::str::Chars = line_it.next().expect("pair").chars();
        acc.insert(
            (
                pair.next().expect("firstchar"),
                pair.next().expect("secondchar"),
            ),
            line_it
                .next()
                .expect("element")
                .chars()
                .next()
                .expect("char"),
        );
        acc
    });

    let node: ListNode = template_str
        .chars()
        .rev()
        .fold(None, |acc, c| Node::new(c, acc));

    let template = List::new(node);
    PairInsertionMap::new(template, map)
}

fn parse(input: &str) -> (PolymerCount, char, PolymerMap) {
    let mut lines = input.lines();
    let template_str = lines.next().expect("template");
    let (last_char, template) =
        template_str
            .chars()
            .fold((None, PolymerCount::new()), |mut acc, c| {
                if let Some(prev) = acc.0 {
                    *acc.1.entry((prev, c)).or_default() += 1;
                }
                acc.0 = Some(c);
                acc
            });

    lines.next();
    let map = lines.fold(HashMap::<(char, char), char>::new(), |mut acc, line| {
        let mut line_it = line.split(" -> ");
        let mut pair = line_it.next().expect("pair").chars();
        acc.insert(
            (
                pair.next().expect("firstchar"),
                pair.next().expect("secondchar"),
            ),
            line_it
                .next()
                .expect("element")
                .chars()
                .next()
                .expect("char"),
        );
        acc
    });

    (template, last_char.expect("there"), map)
}

type PolymerCount = HashMap<(char, char), usize>;

fn polymerize(polymers: PolymerCount, map: &PolymerMap) -> PolymerCount {
    polymers
        .into_iter()
        .fold(PolymerCount::new(), |mut acc, (pair, count)| {
            if let Some(c) = map.get(&pair) {
                *acc.entry((pair.0, *c)).or_default() += count;
                *acc.entry((*c, pair.1)).or_default() += count;
            }
            acc
        })
}

type ElementCount = HashMap<char, usize>;
fn element_freq(polymers: &PolymerCount) -> ElementCount {
    polymers
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, (pair, count)| {
            *acc.entry(pair.0).or_default() += count;
            acc
        })
}

fn solve_part1(input: &str) -> usize {
    let mut pim = parse_pim(input);
    for i in 0..10 {
        pim.insert();
    }
    let max_it = pim.counts.iter().map(|p| p.1);
    let min_it = max_it.clone();
    max_it.max().unwrap() - min_it.min().unwrap()
}

fn solve_part2(input: &str) -> usize {
    let (template, last_char, map) = parse(input);
    let polymers = (0..40).fold(template, |acc, _| polymerize(acc, &map));

    let mut elements = element_freq(&polymers);
    *elements.entry(last_char).or_default() += 1;

    let min_it = elements.iter().map(|x| x.1);
    let max_it = min_it.clone();

    max_it.max().expect("max") - min_it.min().expect("min")
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let part1 = solve_part1(INPUT);
    println!("Part1: {part1}");
    let part2 = solve_part2(INPUT);
    println!("Part2: {part2}");
    solve_part2(INPUT);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 2188189693529);
    }
}
