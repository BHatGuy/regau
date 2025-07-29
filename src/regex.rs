use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Input {
    Start,
    End,
    AnyOther,
    Any,
    Literal(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StateMachine {
    transitions: HashSet<(Vec<u64>, Vec<u64>, Input)>,
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
        assert!(self.is_dfa());

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

                let exact = self.transitions.iter().find(|(from, _, input)| {
                    *from == state
                        && (match input {
                            Input::Literal(l) => *l == c,
                            _ => false,
                        })
                });
                let any = self
                    .transitions
                    .iter()
                    .find(|(from, _, input)| *from == state && (*input == Input::Any));
                let any_other = self
                    .transitions
                    .iter()
                    .find(|(from, _, input)| *from == state && (*input == Input::AnyOther));

                if let Some((_, to, _)) = exact.or(any).or(any_other) {
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
        let mut handeled = HashSet::new();
        let mut dfa = StateMachine::new();

        while let Some(state) = states_to_handle.pop() {
            handeled.insert(state.clone());

            let mut destinations: HashMap<Input, Vec<u64>> = HashMap::new();
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

            if let Some(any_to) = destinations.get(&Input::Any) {
                let any_to = any_to.clone();
                for (input, to) in destinations.iter_mut() {
                    if *input == Input::Any {
                        continue;
                    }

                    to.append(&mut any_to.clone());
                }
                destinations.insert(Input::AnyOther, any_to);
            }
            destinations.remove(&Input::Any);

            for (input, to) in destinations {
                let mut to: Vec<u64> = to
                    .iter()
                    .cloned()
                    .collect::<HashSet<u64>>()
                    .into_iter()
                    .collect();
                to.sort();
                if !handeled.contains(&to) {
                    states_to_handle.push(to.clone());
                }

                let mut from = state.clone();
                from.sort();
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

            let any_and_another = uniq.contains(&Input::Any) && uniq.len() > 1;

            if !all_uniq || any_and_another {
                return false;
            }
        }

        true
    }

    pub fn to_dot(&self) -> String {
        let mut dot = "digraph graphname{\n".to_owned();

        for (from, to, input) in &self.transitions {
            dot += &format!("\"{from:?}\" -> \"{to:?}\" [ label=\"{input}\" ]\n");
        }

        for state in &self.final_states {
            dot += &format!("\"{state:?}\" [ shape=doublecircle ]\n");
        }

        dot += "}";

        dot
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Start => write!(f, "^"),
            Input::End => write!(f, "$"),
            Input::AnyOther => write!(f, "¤"),
            Input::Any => write!(f, "."),
            Input::Literal(l) => write!(f, "'{l}'"),
        }
    }
}

impl FromStr for StateMachine {
    type Err = String; // TODO: Own error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state_machine = StateMachine::new();
        let mut state = vec![0];
        let mut last_transition = (vec![0], vec![0], Input::Any); // TODO: Option
        let mut group_stack = vec![state.clone()];
        let mut group_leaves = Vec::new();
        let mut group_closed = false;

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

            if chr == '(' {
                group_stack.push(state.clone());
                continue;
            }

            if chr == ')' {
                group_closed = true;
                group_stack.pop();
                continue;
            }

            if chr == '|' {
                group_closed = false;
                group_leaves.push(state);
                state = group_stack.last().unwrap().clone();
                continue;
            }

            let mut next_state = state.clone();
            next_state[0] += 1;
            while state_machine
                .transitions
                .iter()
                .any(|(from, to, _)| *to == next_state || *from == next_state)
            {
                next_state[0] += 1;
            }

            let input = match chr {
                '^' => Input::Start,
                '$' => Input::End,
                '.' => Input::Any,
                c => Input::Literal(c),
            };
            last_transition = (state, next_state.clone(), input);
            state_machine.transitions.insert(last_transition.clone());

            if group_closed {
                for from in group_leaves {
                    last_transition = (from.clone(), next_state.clone(), input);
                    state_machine.transitions.insert(last_transition.clone());
                }
                group_leaves = Vec::new();
            }
            group_closed = false;

            state = next_state;
        }
        state_machine.final_states.push(state);
        state_machine.final_states.append(&mut group_leaves);

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

        assert!(state_machine.matches("ac"));
        assert!(state_machine.matches("apc"));
        assert!(state_machine.matches("axyc"));
        assert!(state_machine.matches("_a12345c_"));

        assert!(!state_machine.matches("abb"));
        assert!(!state_machine.matches("sdhkjdhc"));
    }

    #[test]
    fn single_option() {
        let reg = "a.*b|cd";
        let state_machine: StateMachine = reg.parse().unwrap();

        assert!(state_machine.matches("_ab_"));
        assert!(state_machine.matches("_aajsldkb_"));
        assert!(state_machine.matches("_cd_"));
        assert!(state_machine.matches("_abcd_"));

        assert!(!state_machine.matches("bc"));
        assert!(!state_machine.matches("cb"));
    }

    #[test]
    fn multiple_option() {
        let reg = "ab|cd|ef|gh";
        let state_machine: StateMachine = reg.parse().unwrap();

        assert!(state_machine.matches("_ab_"));
        assert!(state_machine.matches("_cd"));
        assert!(state_machine.matches("_ef"));
        assert!(state_machine.matches("_gh"));

        assert!(!state_machine.matches("ce"));
        assert!(!state_machine.matches("ah"));
    }

    #[test]
    fn groups() {
        let reg = "a(b|c)d";
        let state_machine: StateMachine = reg.parse().unwrap();

        assert!(state_machine.matches("abd"));
        assert!(state_machine.matches("acd"));

        assert!(!state_machine.matches("abcd"));
        assert!(!state_machine.matches("ad"));
    }
}
