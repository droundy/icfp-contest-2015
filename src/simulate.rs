use super::*;

impl State {
    fn apply(&self, c: Command) -> Self {
        let mut s = self.clone();
        if s.game_over {
            println!("already over");
            return s;
        }
        if s.unit_sequence.len() == 0 {
            s.game_over = true;
            s.score = 0;
        }
        match c {
            Command::MoveW => {
                println!("ehllo {:?}", s.unit_sequence);
                println!("here {:?}", s.unit_sequence[0]);
                println!("here {:?}", s.unit_sequence[0].pivot.x );
                s.unit_sequence[0].pivot.x -= 1;
                for i in 0..s.unit_sequence[0].members.len() {
                    s.unit_sequence[0].members[i].x -= 1;
                    if s.unit_sequence[0].members[i].x < 0 {
                        s.game_over = true;
                        s.score = 0;
                    }
                }
            },
            Command::MoveE => {
            },
            Command::MoveSW => {
            },
            Command::MoveSE => {
            },
            Command::RotateClockwise => {
            },
            Command::RotateCounterClockwise => {
            },
        };
        s
    }
}

pub fn score_commands(cmds: Vec<Command>, s0: &State) -> State {
    let mut s = s0.clone();
    for c in cmds {
        s = s.apply(c);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn apply_command_to_state_works() {
        let mut s0 = State::new();
        let u = Unit{ members: vec![Cell{ x: 5, y: 5 }],
                      pivot: Cell{ x: 5, y: 5}
        };
        s0.unit_sequence.push(u);
        let s = s0.apply(Command::MoveW);
        assert_eq!(false, s.game_over);
        assert_eq!(0, s.score);
        assert_eq!(4, s.unit_sequence[0].members[0].x);
        assert_eq!(4, s.unit_sequence[0].pivot.x);
        let sx = s0.apply(Command::MoveW).apply(Command::MoveW)
            .apply(Command::MoveW).apply(Command::MoveW).apply(Command::MoveW)
            .apply(Command::MoveW).apply(Command::MoveW).apply(Command::MoveW);
        assert_eq!(true, sx.game_over);
        assert_eq!(0, sx.score);
    }
}
