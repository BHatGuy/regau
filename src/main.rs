mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "a.*c";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();
    let res = state_machine.matches("accscccc");

    println!("{reg}");
    println!("{res}");
    println!("{dfa:?}");
    let test = dfa.is_dfa();
    println!("{test}");
}
