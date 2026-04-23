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

#[derive(Copy, Clone, PartialEq)]
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
pub struct Entity {
    pub id: u8,
    pub kind: Kind,
    pub status: Stats,
    pub location: Point,
}

impl Entity {
    pub fn mv(&mut self, d: Directions, l: Point) {
        match d {
            Directions::R => { if l.x > self.location.x+1 { self.location.x += 1 } },
            Directions::L => { if self.location.x > 0 { self.location.x -= 1 } },
            Directions::U => { if self.location.y > 0 { self.location.y -= 1 } },
            Directions::D => { if l.y > self.location.y+1 { self.location.y += 1 } },
        };
    }
}

#[derive(Clone, PartialEq)]
pub enum Kind {
    Character,
    Enemy,
    Pet,
}

#[derive(PartialEq)]
pub enum Symbol {
    Enemy,
    EnemyRange,
    Char,
    // CharRange,
    Background,
    Unknown,
}

// impl Symbol {
//     pub fn color()
// }
