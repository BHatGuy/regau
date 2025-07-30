mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "(ab)*(xx|yy)*";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();
    dfa.matches("");

    println!("{}", state_machine.to_dot());
}
