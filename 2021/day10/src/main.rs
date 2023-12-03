use std::collections::VecDeque;

fn corrupt_score(t: Option<Token>) -> usize {
    match t {
        Some(Token::End(')')) => 3,
        Some(Token::End(']')) => 57,
        Some(Token::End('}')) => 1197,
        Some(Token::End('>')) => 25137,
        _ => 0,
    }
}

fn token_score(t: Token) -> usize {
    match t {
        Token::End(')') => 1,
        Token::End(']') => 2,
        Token::End('}') => 3,
        Token::End('>') => 4,
        _ => 0,
    }
}

fn auto_complete_score(v: &VecDeque<Token>) -> usize {
    let score = v.iter().rev().fold(0, |acc, t| 5 * acc + token_score(*t));
    println!("{:?} => {score}", v);
    score
}

fn to_end(s: Token) -> Option<Token> {
    match s {
        Token::Start(c) => match c {
            '(' => Some(Token::End(')')),
            '[' => Some(Token::End(']')),
            '{' => Some(Token::End('}')),
            '<' => Some(Token::End('>')),
            _ => None,
        },
        _ => None,
    }
}

fn to_token(c: char) -> Option<Token> {
    if "([{<".contains(c) {
        Some(Token::Start(c))
    } else if ")]}>".contains(c) {
        Some(Token::End(c))
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Start(char),
    End(char),
}

fn match_bad_token(q: &mut VecDeque<Token>, t: Token) -> Option<Token> {
    match t {
        Token::Start(_) => {
            if let Some(e) = to_end(t) {
                q.push_back(e);
            }
            None
        }
        Token::End(e) => {
            if let Some(Token::End(expected)) = q.pop_back() {
                if e != expected {
                    return Some(t);
                }
            }
            None
        }
        _ => None,
    }
}

fn auto_complete_tokens(line: &str) -> Option<VecDeque<Token>> {
    let mut q = VecDeque::<Token>::new();
    let bad_token = line.chars().map(|c| to_token(c)).find_map(|mt| {
        if let Some(t) = mt {
            match_bad_token(&mut q, t)
        } else {
            None
        }
    });

    if let Some(_) = bad_token {
        None
    } else {
        Some(q)
    }
}

fn bad_end_token(line: &str) -> Option<Token> {
    let mut q = VecDeque::<Token>::new();
    line.chars().map(|c| to_token(c)).find_map(|mt| {
        if let Some(t) = mt {
            match_bad_token(&mut q, t)
        } else {
            None
        }
    })
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().filter(|s| !s.is_empty()).collect()
}

fn solve_part1(lines: &[&str]) -> usize {
    lines.iter().map(|l| corrupt_score(bad_end_token(l))).sum()
}

fn solve_part2(lines: &[&str]) -> usize {
    let mut ac = lines
        .iter()
        .flat_map(|l| auto_complete_tokens(l))
        .map(|q| auto_complete_score(&q))
        .collect::<Vec<_>>();
    ac.sort();
    println!("{:?}", ac);
    ac[ac.len() / 2]
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let lines = parse(INPUT);
    let part1 = solve_part1(&lines);
    println!("Part1: {part1}");
    let part2 = solve_part2(&lines);
    println!("Part2: {part2}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    fn to_token_vd(s: &str) -> VecDeque<Token> {
        s.chars().flat_map(|c| to_token(c)).collect()
    }

    #[test]
    fn test_part1() {
        let lines = parse(TEST_INPUT);
        assert_eq!(solve_part1(&lines), 26397);
    }

    #[test]
    fn test_part2() {
        let lines = parse(TEST_INPUT);
        assert_eq!(solve_part2(&lines), 288957);
    }

    #[test]
    fn test_auto_complete_score() {
        assert_eq!(auto_complete_score(&to_token_vd(")")), 1);
        assert_eq!(auto_complete_score(&to_token_vd("))")), 6);
        assert_eq!(auto_complete_score(&to_token_vd(")]")), 7);
    }

    #[test]
    fn test_auto_complete_tokens() {
        assert_eq!(auto_complete_tokens("((([>)))"), None);
        assert_eq!(auto_complete_tokens("({[]}[][]"), Some(to_token_vd(")")));
        assert_eq!(
            auto_complete_tokens("({[]}[][][(<{"),
            Some(to_token_vd(")])>}"))
        );
    }

    #[test]
    fn test_process_line() {
        assert_eq!(bad_end_token("(<[{}]<>)>"), Some(Token::End(')')));
    }
    #[test]
    fn test_parse() {
        assert_eq!(parse("11\n22\n33\n\n"), vec!["11", "22", "33"]);
    }
}
