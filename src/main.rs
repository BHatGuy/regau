mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "(a|b(xx|yy))cd(a|b)";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();

    println!("{}", dfa.to_dot());
}
