use super::*;

pub fn apply_command_to_state(c: Command, s: &State) -> State {
    unimplemented!()
}

pub fn score_commands(cmds: Vec<Command>, s0: &State) -> State {
    let mut s = s0.clone();
    for c in cmds {
        s = apply_command_to_state(c, &s);
    }
    unimplemented!()
}
