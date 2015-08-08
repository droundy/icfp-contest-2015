use super::*;
use Direction::*;
use std::ops::{Sub,Add};

/// A vector in a Bravais lattice with basis vectors in the E and SE
/// directions.  We can define addition and scalar multiplication
/// meaningfully on this lattice.  And most importantly, rotation is
/// easy.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Lattice {
    pub x: i32,
    pub y: i32,
}

impl Lattice {
    pub fn new(x: i32, y:i32) -> Lattice {
        Lattice{x: x, y: y}
    }
    fn rotated(&self, c: Clock) -> Lattice {
        match c {
            Clock::Wise => Lattice::new(-self.y, self.x + self.y),
            Clock::Counter => Lattice::new(self.y + self.x, -self.x),
        }
    }
}
impl Add<Lattice> for Lattice {
    type Output = Lattice;

    fn add(self, rhs: Lattice) -> Lattice {
        Lattice::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub<Lattice> for Lattice {
    type Output = Lattice;

    fn sub(self, rhs: Lattice) -> Lattice {
        Lattice::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl From<Cell> for Lattice {
    fn from(c: Cell) -> Lattice {
        Lattice::new(c.x - c.y/2, c.y)
    }
}
impl From<Lattice> for Cell {
    fn from(c: Lattice) -> Cell {
        Cell::new(c.x + c.y/2, c.y)
    }
}

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

impl Unit {
    fn command(&mut self, c: Command) {
        let piv = Lattice::from(self.pivot);
        match c {
            Command::Move(d) => self.pivot = self.pivot.moved(d),
            _ => (),
        };
        match c {
            Command::Move(d) =>         for i in 0 .. self.members.len() {
                self.members[i] = self.members[i].moved(d);
            },
            Command::Rotate(r) =>
                for i in 0 .. self.members.len() {
                    let dc = Lattice::from(self.members[i]) - piv;
                    self.members[i] = Cell::from(piv + dc.rotated(r));
                },
        }
    }
}

impl State {
    fn is_invalid(&self, c: Cell) -> bool {
        if c.x < 0 || c.x >= self.width || c.y >= self.height || c.y < 0 {
            return true;
        }
        self.is_filled(c)
    }
    pub fn apply_sequence(&self, cs: &[Command]) -> Self {
        let mut s = self.clone();
        for c in cs.iter() {
            s = s.apply(*c);
        }
        s
    }
    pub fn apply(&self, c: Command) -> Self {
        let mut s = self.clone();
        if s.game_over {
            return s;
        }
        if s.unit_sequence.len() == 0 {
            s.game_over = true;
            s.score = 0;
            return s;
        }
        s.unit_sequence[0].command(c);
        s.visited.push(self.unit_sequence[0].clone());
        if s.visited.contains(&s.unit_sequence[0]) {
            // We have visited this position/orientation before!
            s.game_over = true;
            s.score = 0;
            return s;
        }
        if s.unit_sequence[0].members.iter().any(|&c| self.is_invalid(c)) {
            // undo any rotation or translation we have done
            for i in 0..s.unit_sequence[0].members.len() {
                s.unit_sequence[0].members[i] = self.unit_sequence[0].members[i];
            }
            s.unit_sequence[0].pivot = self.unit_sequence[0].pivot;
            s.lock_unit();
        }
        // If we moved down, then we will never return to our former
        // location, so we can optimize by clearing the former
        // history.
        match c {
            Command::Move(SE) | Command::Move(SW) => s.visited.truncate(0),
            _ => ()
        }
        s
    }
    fn lock_unit(&mut self) {
        let u = self.unit_sequence[0].clone();
        let size = u.members.len() as i32;
        for c in u.members {
            *self.filled(c) = true;
        }
        // clear out visited since new unit hasn't visited anything
        self.visited.truncate(0);
        self.unit_sequence = self.unit_sequence[1..].into();
        let mut ls = 0;
        let w = self.width as usize;
        for y in (0 .. self.height as usize).rev() {
            let mut killme = true;
            for x in 0 .. w {
                if !self.is_filled(Cell{x:x as i32,y:y as i32}) {
                    killme = false;
                    break;
                }
            }
            if killme {
                ls += 1;
                for i in (w .. y*w).rev() {
                    self.filled_array[i] = self.filled_array[i-w];
                }
                for x in 0 .. w {
                    self.filled_array[x] = false;
                }
            }
        }
        self.score += size + 100 * (1 + ls) * ls / 2;

        // need to make sure new unit starts in valid place, or just end game
        if self.unit_sequence.len() > 0 && self.unit_sequence[0].members.iter().any(|&c| self.is_invalid(c)) {
            self.game_over = true;
        }
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
    fn to_from_lattice() {
        let c = Cell::new(5,3);
        let l = Lattice::from(c);
        let c1 = Cell::from(l);
        assert_eq!(c, c1);
    }

    #[test]
    fn game_loss_on_revisit() {
        println!("");
        let mut s0 = State::new();
        let u = Unit{ members: vec![Cell{ x: 4, y: 5 },Cell{ x: 5, y: 5 }],
                      pivot: Cell{ x: 4, y: 5}
        };
        s0.score = 100;
        s0.unit_sequence.push(u);
        let s0 = s0; // mark s0 immutable now for clarity.
        let mut s = s0.apply(Command::Move(W));
        println!("{}", s.visualize());
        s = s.apply(Command::Move(E));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

        s = s0.clone();
        println!("Rotating clockwise");
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

        s = s0.clone();
        println!("Rotating counterclockwise");
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);


        s = s0.clone();
        println!("Combining rotation with translation");
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        s = s.apply(Command::Move(W));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Move(W));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Move(E));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Move(E));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

    }

    #[test]
    fn game_loss_on_revisit_with_symmetry() {
        println!("");
        let mut s0 = State::new();
        let u = Unit{ members: vec![Cell{ x: 3, y: 5 },Cell{ x: 4, y: 5 },Cell{ x: 5, y: 5 }],
                      pivot: Cell{ x: 4, y: 5}
        };
        s0.score = 100;
        s0.unit_sequence.push(u);
        let s0 = s0; // mark s0 immutable now for clarity.

        let mut s = s0.clone();
        println!("Rotating clockwise");
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Wise));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

        s = s0.clone();
        println!("Rotating counterclockwise");
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(false, s.game_over);
        assert_eq!(100, s.score);
        s = s.apply(Command::Rotate(Clock::Counter));
        println!("{}", s.visualize());
        assert_eq!(true, s.game_over);
        assert_eq!(0, s.score);

    }

    #[test]
    fn apply_works() {
        let mut s0 = State::new();
        let u = Unit{ members: vec![Cell{ x: 5, y: 5 }],
                      pivot: Cell{ x: 5, y: 5}
        };
        s0.unit_sequence.push(u);
        let s0 = s0; // mark s0 immutable now for clarity.
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

    #[test]
    fn play_a_game() {
        use Command::Move;

        let states = input_to_states(&Input::from_json("problems/problem_0.json"));
        let mut cmds: Vec<Command> = Vec::new();
        let mut s0 = states[0].clone();
        println!("Starting position");
        println!("{}", s0.visualize());

        while !s0.game_over {
            for &cmd in [Move(SE), Move(SW)].iter() {
                if !s0.game_over {
                    s0 = s0.apply(cmd);
                    cmds.push(cmd);
                    println!("Score: {}", s0.score);
                    println!("{}", s0.visualize());
                }
            }
        }
        println!("Solution: {}", commands_to_string(cmds.clone()));
        println!("score: {}", s0.score);

        assert_eq!(s0.score, 13); // This has been confirmed on the leaderboard!
    }

    #[test]
    fn official_sample() {
        //use Command::Move;

        let states = input_to_states(&Input::from_json("problems/problem_6.json"));
        let mut s0 = states[0].clone();
        println!("Starting position");
        println!("{}", s0.visualize());

        for c in string_to_commands("iiiiiiimimiiiiiimmimiiiimimimmimimimimmeemmimimiimmmmimmimiimimimmimmimeeemmmimimmimeeemiimiimimimiiiipimiimimmmmeemimeemimimimmmmemimmimmmiiimmmiiipiimiiippiimmmeemimiipimmimmipppimmimeemeemimiieemimmmm") {
            if s0.game_over {
                break;
            }
            s0 = s0.apply(c);
            println!("Score: {}", s0.score);
            println!("{}", s0.visualize());
        }
        println!("score: {}", s0.score);

        assert_eq!(s0.score, 61);
    }

    #[test]
    fn view_boards() {
        for i in (0..24) {
            let states = input_to_states(&Input::from_json(format!("problems/problem_{}.json", i)));
            println!("Problem {}:", i);
            println!("{}", states[0].visualize());
        }
    }
}
