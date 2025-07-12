use crate::statemachines::StateMachine;

mod regex;
mod statemachines;

fn main() {
    let atoms = regex::parse("^a.b.c$");

    let state_machine = StateMachine::from(atoms);

    println!("{state_machine:?}");
}
