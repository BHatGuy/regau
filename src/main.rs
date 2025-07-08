#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    Start,
    End,
    Any,
    Literal(char),
}

pub fn parse(regex: &str) -> Vec<Atom> {
    let mut atoms = Vec::new();

    for c in regex.chars() {
        match c {
            '^' => atoms.push(Atom::Start),
            '$' => atoms.push(Atom::End),
            '.' => atoms.push(Atom::Any),
            _ => atoms.push(Atom::Literal(c)),
        }
    }

    atoms
}

pub fn rmatch(input: &str, regex: &[Atom]) -> bool {
    let mut regex_iter = regex.iter();

    for c in input.chars() {
        if let Some(next) = regex_iter.next() {
            match next {
                Atom::Start => todo!(),
                Atom::End => todo!(),
                Atom::Any => {}
                Atom::Literal(l) => {
                    if *l != c {
                        return false;
                    }
                }
            }
        } else {
            return false;
        }
    }

    if let Some(atom) = regex_iter.next() {
        if *atom != Atom::End {
            return false;
        }
    }

    true
}

fn main() {
    let test_regex = parse("....");
    let res = rmatch("abc", &test_regex);
    println!("{res}");
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_only_literals() {
        let result = parse("abc");
        let expected = vec![Atom::Literal('a'), Atom::Literal('b'), Atom::Literal('c')];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_any() {
        let result = parse("a.c.");
        let expected = vec![Atom::Literal('a'), Atom::Any, Atom::Literal('c'), Atom::Any];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_start() {
        let result = parse("^a");
        let expected = vec![Atom::Start, Atom::Literal('a')];
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_end() {
        let result = parse("a$");
        let expected = vec![Atom::Literal('a'), Atom::End];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod match_tests {
    use super::*;

    #[test]
    fn match_literals() {
        let regex = parse("abcd");

        assert!(rmatch("abcd", &regex));
        assert!(rmatch("_abcd_", &regex));

        assert!(!rmatch("_abc_", &regex));
        assert!(!rmatch("abc", &regex));
    }

    #[test]
    fn match_any() {
        let regex = parse("a.c.");

        assert!(rmatch("abcd", &regex));
        assert!(rmatch("aacc", &regex));
        assert!(rmatch("_aacc_", &regex));

        assert!(!rmatch("abc", &regex));
        assert!(!rmatch("_abc", &regex));
    }
}
