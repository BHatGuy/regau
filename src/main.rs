mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "^a.*b.c$";
    let state_machine: StateMachine = reg.parse().unwrap();

    println!("{reg}");
    println!("{state_machine:?}");
    println!("{}", state_machine.is_dfa());
}
