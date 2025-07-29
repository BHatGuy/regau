mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "ab|ad|cd|.x";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();

    println!("{}", dfa.to_dot());
}
