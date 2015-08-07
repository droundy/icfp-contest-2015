use super::*;
use Direction::*;

impl Cell {
    fn moved(&self, c: Direction) -> Cell {
        let mut out = self.clone();
        match c {
            W => {
                out.x -= 1;
            },
            E => {
                out.x += 1;
            },
            SE => {
                if out.y & 1 == 1 {
                    out.x += 1;
                }
                out.y += 1;
            },
            SW => {
                if out.y & 1 == 0 {
                    out.x -= 1;
                }
                out.y += 1;
            },
        }
        out
    }
}

impl State {
    fn is_invalid(&self, c: Cell) -> bool {
        if c.x < 0 || c.x >= self.width || c.y >= self.height || c.y < 0 {
            return true;
        }
        self.is_filled(c)
    }
    fn apply(&self, c: Command) -> Self {
        let mut s = self.clone();
        if s.game_over {
            return s;
        }
        if s.unit_sequence.len() == 0 {
            s.game_over = true;
            s.score = 0;
            return s;
        }
        match c {
            Command::Move(d) => {
                println!("moving {:?} to the {:?}", s.unit_sequence[0], d);
                s.unit_sequence[0].pivot = s.unit_sequence[0].pivot.moved(d);
                let mut will_lock = false;
                for i in 0..s.unit_sequence[0].members.len() {
                    let c = s.unit_sequence[0].members[i].moved(d);
                    s.unit_sequence[0].members[i] = c;
                    if self.is_visited(c) {
                        s.game_over = true;
                        s.score = 0;
                        return s;
                    }
                    if self.is_invalid(c) {
                        // FIXME need to lock unit
                        will_lock = true;
                        break;
                    } else {
                        *s.visited(c) = true;
                    }
                }
                if will_lock {
                    // undo any rotation or translation we have done
                    for i in 0..s.unit_sequence[0].members.len() {
                        s.unit_sequence[0].members[i] = self.unit_sequence[0].members[i];
                    }
                    s.unit_sequence[0].pivot = self.unit_sequence[0].pivot;
                    s.lock_unit();
                }
            },
            Command::RotateClockwise => {
            },
            Command::RotateCounterClockwise => {
            },
        };
        s
    }
    fn lock_unit(&mut self) {
        let u = self.unit_sequence[0].clone();
        for c in u.members {
            *self.filled(c) = true;
        }
        for x in 0..self.width {
            for y in 0..self.height {
                let c = Cell{ x: x, y: y };
                *self.visited(c) = false;
            }
        }
        self.unit_sequence = self.unit_sequence[1..].into();
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
    use Direction::*;

    #[test]
    fn apply_works() {
        let mut s0 = State::new();
        let u = Unit{ members: vec![Cell{ x: 5, y: 5 }],
                      pivot: Cell{ x: 5, y: 5}
        };
        s0.unit_sequence.push(u);
        let mut s = s0.apply(Command::Move(W));
        assert_eq!(false, s.game_over);
        assert_eq!(0, s.score);
        assert_eq!(4, s.unit_sequence[0].members[0].x);
        assert_eq!(4, s.unit_sequence[0].pivot.x);
        assert_eq!(5, s.unit_sequence[0].members[0].y);
        assert_eq!(5, s.unit_sequence[0].pivot.y);

        s = s0.apply(Command::Move(W)).apply(Command::Move(W))
            .apply(Command::Move(W)).apply(Command::Move(W)).apply(Command::Move(W))
            .apply(Command::Move(W)).apply(Command::Move(W)).apply(Command::Move(W));
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

        s = s0.apply(Command::Move(E));
        assert_eq!(false, s.game_over);
        assert_eq!(0, s.score);
        assert_eq!(6, s.unit_sequence[0].members[0].x);
        assert_eq!(6, s.unit_sequence[0].pivot.x);
        assert_eq!(5, s.unit_sequence[0].members[0].y);
        assert_eq!(5, s.unit_sequence[0].pivot.y);

        s = s0.apply(Command::Move(E)).apply(Command::Move(E))
            .apply(Command::Move(E)).apply(Command::Move(E)).apply(Command::Move(E))
            .apply(Command::Move(E)).apply(Command::Move(E)).apply(Command::Move(E));
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);
    }
}
