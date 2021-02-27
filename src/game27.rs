extern crate rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    First,
    Second,
}
const SIZE: usize = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game27 {
    board: [Vec<Piece>; SIZE],
    first_turn: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Move(usize, usize),
    Pass,
}
impl Action {
    #[allow(dead_code)]
    fn parse(s: &str) -> Result<Action, String> {
        let v: Vec<&str> = s.split(" ").collect();
        match v[0] {
            "move" => Ok(Action::Move(v[1].parse().unwrap(), v[2].parse().unwrap())),
            "pass" => Ok(Action::Pass),
            _ => Err(format!("Unknown played: {:?}", v)),
        }
    }
}

impl Game27 {
    fn new() -> Game27 {
        use Piece::*;
        let mut board : [Vec<Piece>; SIZE]= Default::default();
        board[0] = vec![First; SIZE];
        board[SIZE-1] = vec![Second; SIZE];
        Game27 { board, first_turn: true }
    }
    fn playable(&self) -> Vec<Action> {
        let mut res = vec![];
        for c in 0..SIZE {
            if self.board[c].len() > 0 && self.board[c][0] == self.active() {
                for i in 1..self.board[c].len()+1 {
                    let d = self.move_to(c);
                    if 0 <= d && d < SIZE as isize{
                        res.push(Action::Move(c, i))
                    }
                }
            }
        }
        if res.is_empty() {
            res.push(Action::Pass)
        }
        return res;
    }
    fn count_tower(&self) -> usize {
        let mut res = 0;
        for c in 0..SIZE {
            if self.board[c].len() > 0 && self.board[c][0] == self.active() {
                res += 1;
            }
        }
        return res;
    }
    fn move_to(&self, c: usize) -> isize {
        c as isize + self.count_tower() as isize * (if self.first_turn { 1 } else { -1 })
    }
    fn active(&self) -> Piece {
        if self.first_turn {
            Piece::First
        } else {
            Piece::Second
        }
    }
    fn is_end(&self) -> bool {
        if self.playable()[0] == Action::Pass {
            let mut s = self.clone();
            s.first_turn = !s.first_turn;
            return s.playable()[0] == Action::Pass
        }
        return false;
    }
    fn act(&mut self, a: Action) -> Result<(), String> {
        if self.is_end() {
            return Err(format!("game is over"));
        }
        match a {
            Action::Move(c, i) => {
                if !(c < SIZE) {
                    return Err(format!("The column is over SIZE: c = {}", c))
                }
                if !(self.board[c].len() > 0) {
                    return Err(format!("The column has no tower: c = {}", c))

                }
                if !(self.board[c][0] == self.active()) {
                    return Err(format!("The tower is not yours"))
                }
                if !(0 < i && i <= self.board[c].len()) {
                    return Err(format!("The move is over the tower: i = {}", i))
                }
                let d = self.move_to(c);
                if !(0 <= d && d < SIZE as isize) {
                    return Err(format!("The column it moved to is over SIZE: move_to = {}", d))
                }
                let from_tower = self.board[c].clone();
                self.board[c] = from_tower[i..from_tower.len()].to_vec();
                self.board[d as usize].splice(0..0, from_tower[0..i].to_vec());
            }
            Action::Pass => {
                let playable = self.playable();
                if  !(playable.len() == 1 && playable[0] == Action::Pass) {
                    return Err(format!("There are playable moves"))
                }
            }
        }
        self.first_turn = !self.first_turn;
        Ok(())
    }
    fn result(&self) -> isize {
        let f = self.board[SIZE-1].len();
        let s = self.board[0].len();
        f as isize - s as isize
    }
}
use std::fmt;
impl fmt::Display for Game27 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = |p| match p {
            Piece::First => "O",
            Piece::Second => "X",
        };
        let mut s = String::new();
        for c in 0..SIZE {
            for i in 0..self.board[c].len() {
                let c = ch(self.board[c][i]);
                s = format!("{}{}", s, c);
            }
            s = format!("{}\n", s);
        }
        if self.is_end() {
            s = format!("{}Over! Result: {}\n", s, self.result());
        } else {
            s = format!("{}{}'s turn\n", s, if self.first_turn { "X" } else { "O" });
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Input {
    Init(usize),
    PlayedMove(usize, usize),
    PlayedPass,
    Res(isize),
    Wait,
}
impl Input {
    fn parse(input: &str) -> Result<Input, String> {
        let v: Vec<&str> = input.split(" ").collect();
        match v[0] {
            "init" => Ok(Input::Init(v[1].parse().unwrap())),
            "played" => match v[1] {
                "move" => Ok(Input::PlayedMove(
                    v[2].parse().unwrap(),
                    v[3].parse().unwrap(),
                )),
                "pass" => Ok(Input::PlayedPass),
                _ => Err(format!("Unknown Input: {:?}", v)),
            },
            "result" => Ok(Input::Res(v[1].parse().unwrap())),
            "wait" => Ok(Input::Wait),
            _ => Err(format!("Unknown Input: {:?}", v)),
        }
    }
    
}

#[derive(Debug, Clone)]
struct RandomPlayer {
    board: Option<Game27>,
    first: bool,
}
impl RandomPlayer {
    fn new() -> RandomPlayer {
        RandomPlayer {
            board: None,
            first: false,
        }
    }
    fn play(&mut self, input: &str) -> Option<String> {
        let i = Input::parse(input).unwrap();
        match i {
            Input::Init(p) => {
                if p == 0 {
                    self.first = true
                } else {
                    self.first = false
                }
                self.board = Some(Game27::new());
                None
            }
            Input::PlayedMove(c, i) => {
                let b = self.board.as_mut().unwrap();
                b.act(Action::Move(c, i)).unwrap();
                None
            }
            Input::PlayedPass => {
                let b = self.board.as_mut().unwrap();
                b.act(Action::Pass).unwrap();
                None
            }
            Input::Res(_) => None,
            Input::Wait => {
                use rand::thread_rng;
                use rand::seq::SliceRandom;
                let b = self.board.as_mut().unwrap();
                let p = b.playable();
                let mut rng = thread_rng();
                let a = p.choose(&mut rng).unwrap();
                b.act(*a).unwrap();
                match a {
                    Action::Move(y, x) => Some(format!("move {} {}", y, x)),
                    Action::Pass => Some(format!("pass"))
                }
            }
        }
    }
}
pub fn start() -> Result<(), String> {
    use std::io::{self, BufRead};
    let mut player = RandomPlayer::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let resp = player.play(&line.unwrap());
        if let Some(resp) = resp {
            println!("{}", resp);
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board() {
        let mut board = Game27::new();
        let playable = board.playable();

        let mut expected_playable = vec![];
        for i in 1..SIZE+1 {
           expected_playable.push((0, i))
        }

        fn same_action(e: Vec<Action>, g: Vec<(usize, usize)>) {
            assert_eq!(e.len(), g.len());
            for (y, x) in g {
                assert!(e.iter().any(|a| a == &Action::Move(y, x)))
            }
        }
        println!("{:?}", board);
        println!("{:?}", playable);
        same_action(playable, expected_playable);

        board.act(Action::Move(0, 4)).unwrap();
        println!("{}", board);
        println!("{:?}", board.playable());
        assert_eq!(board.board[0].len(), SIZE - 4);
        assert_eq!(board.board[1].len(), 4);

        let moves = [(8, 8), (1, 4), (7, 4), (3, 4), (7, 3), (0, 4)];
        for (c, i) in &moves {
            println!("{} {}", c, i);
            board.act(Action::Move(*c, *i)).unwrap();
            println!("{}", board);
            println!("{:?}", board.playable());
        }
    }
    #[test]
    fn test_player() {
        let mut p0 = RandomPlayer::new();
        let mut p1 = RandomPlayer::new();
        p0.play("init 0");
        p1.play("init 1");
        for _ in 0..100 {
            println!("{:?}", p0.board.as_mut().unwrap().playable());
            let r = p0.play("wait").unwrap();
            println!("{:?}", r);
            let p = Action::parse(&r).unwrap();
            p1.board.as_mut().unwrap().act(p).unwrap();
            println!("X\n{:?}", p0.board.as_mut().unwrap());
            if p1.board.as_mut().unwrap().is_end() {
                break;
            }

            println!("{:?}", p1.board.as_mut().unwrap().playable());
            let r = p1.play("wait").unwrap();
            println!("{:?}", r);
            let p = Action::parse(&r).unwrap();
            p0.board.as_mut().unwrap().act(p).unwrap();
            println!("O\n{:?}", p1.board.as_mut().unwrap());
            if p0.board.as_mut().unwrap().is_end() {
                break;
            }
        }
        let result = p1.board.as_mut().unwrap().result();
        println!("Result: {}", result)
    }
}
