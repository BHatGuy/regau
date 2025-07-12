use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
pub struct StateMachine {
    transitions: HashSet<(u64, u64, char)>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            transitions: HashSet::new(),
        }
    }

    pub fn add_transition(&mut self, from: u64, to: u64, input: char) {
        self.transitions.insert((from, to, input));
    }

    pub fn is_dfa(&self) -> bool {
        let mut states = HashSet::new();
        for (from, to, _) in &self.transitions {
            states.insert(*from);
            states.insert(*to);
        }

        for state in states {
            let mut uniq = HashSet::new();
            let all_uniq = self
                .transitions
                .iter()
                .filter(|(from, _, _)| *from == state)
                .map(|(_, _, input)| *input)
                .all(|x| uniq.insert(x));

            let any_and_another = uniq.contains(&'.') && uniq.len() > 1;

            if !all_uniq || any_and_another {
                return false;
            }
        }

        true
    }
}

impl FromStr for StateMachine {
    type Err = String; // TODO: Own error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state_machine = StateMachine::new();
        let mut state = 0;
        let mut last_literal = ' '; // TODO: Option
        for chr in s.chars() {
            if chr == '*' {
                state_machine.add_transition(state, state, last_literal);
                continue;
            }

            let next_state = state + 1;
            state_machine.add_transition(state, next_state, chr);
            last_literal = chr;
            state = next_state;
        }

        Ok(state_machine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_dfa() {
        let mut state_machine = StateMachine::new();

        state_machine.add_transition(0, 1, 'a');
        state_machine.add_transition(0, 1, 'x');
        state_machine.add_transition(0, 2, 'c');
        state_machine.add_transition(1, 2, 'b');

        assert!(state_machine.is_dfa())
    }

    #[test]
    fn construct_nfa() {
        let mut state_machine = StateMachine::new();

        state_machine.add_transition(0, 1, 'a');
        state_machine.add_transition(0, 2, 'a');
        state_machine.add_transition(1, 2, 'b');

        assert!(!state_machine.is_dfa())
    }

    #[test]
    fn construct_dfa_any() {
        let mut state_machine = StateMachine::new();

        state_machine.add_transition(0, 1, 'a');
        state_machine.add_transition(0, 1, '.');
        state_machine.add_transition(1, 2, 'b');

        assert!(!state_machine.is_dfa())
    }
}
