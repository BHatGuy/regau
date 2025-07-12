use std::collections::{HashMap, HashSet};

struct StateMachine {
    transitions: HashSet<(u64, u64, char)>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            transitions: HashSet::new(),
        }
    }

    fn add_transition(&mut self, from: u64, to: u64, input: char) {
        self.transitions.insert((from, to, input));
    }

    fn is_dfa(&self) -> bool {
        self.transitions
            .iter()
            .fold(HashMap::new(), |mut acc, (from, _, input)| {
                acc.entry((from, input))
                    .and_modify(|c| *c = *c + 1)
                    .or_insert(1);
                acc
            })
            .iter()
            .all(|((_, _), &count)| count <= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_dfa() {
        let mut state_machine = StateMachine::new();

        state_machine.add_transition(0, 1, 'a');
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
}
