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

pub fn parse_action(s: &str) -> Action {
    let mut times: u8 = 0;
    let mut opt: Action;

    Action {times: times, act: opt}
}
