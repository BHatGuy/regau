mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "(a|b(xx|yy))c*d(a|b)";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();
    dfa.matches("");

    println!("{}", state_machine.to_dot());
}
