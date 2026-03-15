use crate::Directions;

#[derive(Copy, Clone)]
enum Opts {
    Unknown,
    Attack(u8),
    Move(Directions),
    Times(u8),
}

#[derive(Copy, Clone)]
pub struct Action {
    times: u8,
    act: Opts,
}

pub fn parse(s: &str) -> Action {
    let mut times: u8 = 0;
    let mut opt: Action;

    for i in 0..s.len() {
        let index: u8 = i as u8;
        let char = s.chars().nth(i).expect("char:parse");

        if char < '9' && char > '0'  {
            times += (char as u8)-47;
        } else if char ==;
    }

    Action {times: times, act: opt}
}
