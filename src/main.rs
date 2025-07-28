mod regex;

use crate::regex::StateMachine;

fn main() {
    let reg = "a.*bbc*d";
    let state_machine: StateMachine = reg.parse().unwrap();

    let dfa = state_machine.to_dfa();
    let res = state_machine.matches("apbbccccccd");

    println!("{reg}");
    println!("{res}");
    println!("{dfa:?}");
}
