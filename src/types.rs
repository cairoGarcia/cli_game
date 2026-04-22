#[derive(Clone, PartialEq)]
pub enum Mode {
    Easy,
    Medium,
    Hard,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Directions {
    R,
    L,
    U,
    D,
}

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, PartialEq)]
pub struct Game {
    pub round: u8,
    pub enemie_count: u8,
    pub mode: Mode,
}

#[derive(Clone, PartialEq)]
pub struct Stats {
    pub hp: i8,
    pub attack: i8,
    pub move_amount: i8,
}

#[derive(Clone, PartialEq)]
pub struct Character {
    pub status: Stats,
    pub location: Point,
}

#[derive(Clone, PartialEq)]
pub struct Enemy {
    pub status: Stats,
    pub id: i8,
    pub location: Point,
}

#[derive(PartialEq)]
pub enum Symbol {
    Enemy,
    EnemyRange,
    Char,
    // CharRange,
    Background,
    Unknown
}
