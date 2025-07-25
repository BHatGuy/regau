use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct StateMachine {
    transitions: HashSet<(Vec<u64>, Vec<u64>, char)>,
    final_states: Vec<Vec<u64>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            transitions: HashSet::new(),
            final_states: Vec::new(),
        }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut matches = Vec::new();

        for start in 0..input.len() {
            let mut state = vec![0];
            let mut index = start;
            loop {
                let c = input.chars().nth(index);
                if c.is_none() {
                    break;
                }
                let c = c.unwrap();

                if let Some((_, to, _)) = self
                    .transitions
                    .iter()
                    .find(|(from, _, input)| *from == state && (*input == c || *input == '.'))
                {
                    state = to.clone();
                    if self.final_states.contains(&state) {
                        matches.push((start, index));
                        break;
                    }
                } else {
                    break;
                }

                index += 1;
            }
        }

        !matches.is_empty()
    }

    pub fn to_dfa(&self) -> Self {
        if self.is_dfa() {
            return Self {
                transitions: self.transitions.clone(),
                final_states: self.final_states.clone(),
            };
        }

        let mut states_to_handle = vec![vec![0]];
        let mut dfa = StateMachine::new();

        while let Some(state) = states_to_handle.pop() {
            let mut destinations: HashMap<char, Vec<u64>> = HashMap::new();
            for sub_state in state.iter() {
                for (_, to, input) in self
                    .transitions
                    .iter()
                    .filter(|(from, _, _)| from.contains(sub_state))
                {
                    destinations
                        .entry(*input)
                        .and_modify(|x| x.append(&mut to.clone()))
                        .or_insert(to.clone());
                }
            }

            if let Some(any_to) = destinations.get(&'.') {
                let mut any_to = any_to.clone();
                for (input, to) in destinations.iter_mut() {
                    if *input == '.' {
                        continue;
                    }

                    to.append(&mut any_to);
                }
            }
            destinations.remove(&'.');

            for (input, mut to) in destinations {
                if dfa.transitions.iter().filter(|(_, t, _)| *t == to).count() == 0 {
                    let mut t = to.clone();
                    t.sort();
                    states_to_handle.push(t);
                }

                let mut from = state.clone();
                from.sort();
                to.sort();
                let t = (from, to, input);
                dfa.transitions.insert(t);
            }
        }

        let mut final_states = HashSet::new();
        for (_, to, _) in dfa.transitions.iter() {
            for orig_state in self.final_states.iter() {
                for sub in to.iter() {
                    if orig_state.contains(sub) {
                        final_states.insert(to.clone());
                    }
                }
            }
        }

        dfa.final_states = final_states.iter().cloned().collect();

        assert!(dfa.is_dfa());

        dfa
    }

    pub fn is_dfa(&self) -> bool {
        let mut states = HashSet::new();
        for (from, to, _) in &self.transitions {
            states.insert(from.clone());
            states.insert(to.clone());
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
        let mut state = vec![0];
        let mut last_transition = (vec![0], vec![0], ' '); // TODO: Option
        for chr in s.chars() {
            if chr == '*' {
                state_machine.transitions.remove(&last_transition);

                last_transition = (
                    last_transition.0.clone(),
                    last_transition.0,
                    last_transition.2,
                );
                state_machine.transitions.insert(last_transition.clone());
                state[0] -= 1;
                continue;
            }

            let mut next_state = state.clone();
            next_state[0] += 1;

            last_transition = (state, next_state.clone(), chr);
            state_machine.transitions.insert(last_transition.clone());
            state = next_state;
        }
        state_machine.final_states.push(state);

        Ok(state_machine.to_dfa())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_matching() {
        let reg = "a.c";
        let state_machine: StateMachine = reg.parse().unwrap();

        assert!(state_machine.matches("abc"));
        assert!(state_machine.matches("_abc_"));

        assert!(!state_machine.matches("abb"));
        assert!(!state_machine.matches("ac"));
    }

    #[test]
    fn repeat_matching() {
        let reg = "ab*c";
        let state_machine: StateMachine = reg.parse().unwrap();

        assert!(state_machine.matches("ac"));
        assert!(state_machine.matches("abc"));
        assert!(state_machine.matches("abbc"));
        assert!(state_machine.matches("_abbbbc_"));

        assert!(!state_machine.matches("abb"));
        assert!(!state_machine.matches("bbc"));
        assert!(!state_machine.matches("adc"));
    }

    #[test]
    fn repeat_wildcard_matching() {
        let reg = "a.*c";
        let state_machine: StateMachine = reg.parse().unwrap();

        println!("{state_machine:?}");

        assert!(state_machine.matches("ac"));
        assert!(state_machine.matches("apc"));
        assert!(state_machine.matches("axyc"));
        assert!(state_machine.matches("_a12345c_"));

        assert!(!state_machine.matches("abb"));
        assert!(!state_machine.matches("sdhkjdhc"));
    }
}
