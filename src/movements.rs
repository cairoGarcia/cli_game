pub fn move_up(y: i8) -> i8 {
    match y {
        a if a <= 0 => 0,
        _ => y - 1,
    }
}

pub fn move_down(y: i8, y_cap: i8) -> i8 {
    match y {
        a if a >= y_cap - 1 => y_cap - 1,
        _ => y + 1,
    }
}

pub fn move_left(x: i8) -> i8 {
    match x {
        a if a <= 0 => 0,
        _ => x - 1,
    }
}

pub fn move_right(x: i8, x_cap: i8) -> i8 {
    match x {
        a if a >= x_cap - 1 => x_cap - 1,
        _ => x + 1,
    }
}
