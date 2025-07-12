use crate::statemachines::StateMachine;

#[derive(Debug, PartialEq, Eq, Hash)]
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

fn rmatch(input: &str, regex: &[Atom]) -> bool {
    todo!();
}

impl From<Vec<Atom>> for StateMachine<Atom> {
    fn from(atoms: Vec<Atom>) -> Self {
        let mut state_machine = StateMachine::new();
        let mut state = 0;
        for atom in atoms {
            let next_state = state + 1;
            state_machine.add_transition(state, next_state, atom);
            state = next_state;
        }

        state_machine
    }
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
