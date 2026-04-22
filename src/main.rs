mod movements;
mod types;
// mod parser;

use movements::*;
use types::{Character, Enemy, Point, Stats, Symbol};

// use serde::{Serialize, Deserialize};

// use parser::{Action,parse};

use std::io;
use std::io::Write;

use colored::Colorize;

const BUF: Point = Point {x: 15, y: 7};

const PREFIX: &str = "> ";
static mut HP: i8 = 10;

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let limits = BUF;

    let mut round: u8 = 0;

    let mut character = Character {
        status: Stats {
            hp: unsafe { HP },
            attack: 1,
            move_amount: 1,
        },
        location: Point {
            x: limits.x / 2,
            y: limits.y / 2,
        },
    };

    let mut enemy_list = vec![Enemy {
        status: Stats {
            hp: 8,
            attack: 1,
            move_amount: 1,
        },
        id: 1,
        location: Point { x: 0, y: 0 },
    }];

    enemy_list.push(Enemy {
        status: Stats {
            hp: 9,
            attack: 1,
            move_amount: 2,
        },
        id: 2,
        location: Point { x: 1, y: 2 },
    });

    loop {
        unsafe { character.status.hp = HP};
        if character.status.hp <= 0 {
            println!("GAME OVER!");
            return;
        };
        render(&character, &limits, &enemy_list);
        println!(
            "x:{}, y:{}, hp: {}",
            character.location.x, character.location.y, character.status.hp
        );

        print!("{}", PREFIX);

        let mut input = String::new();

        stdout.flush().expect("stdout:main");
        stdin.read_line(&mut input).expect("stdin_input:main");

        let input: &str = input.trim();

        match input {
            "h" => {
                character.location.x = move_left(character.location.x);
            }
            "j" => {
                character.location.y = move_down(character.location.y, limits.y);
            }
            "k" => {
                character.location.y = move_up(character.location.y);
            }
            "l" => {
                character.location.x = move_right(character.location.x, limits.x);
            }
            _ => {
                println!("Invalid Input");
                continue;
            }
        }

        action_checkout(&character.location, &enemy_list);
        round += 1;

        if round % 3 == 0 {
            for enemy in 0..enemy_list.len() {
                enemy_list[enemy].location = enemy_action(&enemy_list[enemy].location, &character.location);
            }
        }
    }
}

pub fn action_checkout(char_loc: &Point, enemy_list: &Vec<Enemy>) {
    /* let new_enemy_list: Vec<Enemy> = Vec::new(); */
    
    for enemy in 0..enemy_list.len() {
        let result = path_to(char_loc, &enemy_list[enemy].location);

        if result <= enemy_list[enemy].status.move_amount {
            unsafe { HP -= 1};
        };
    }


    // new_enemy_list
}

pub fn enemy_action(enemy: &Point, character: &Point) -> Point {
    let x_diff = path_to(&Point {x: character.x, y: 0}, &Point {x: enemy.x, y: 0});
    let y_diff = path_to(&Point {x: 0, y: character.y}, &Point {x: 0, y: enemy.y});

    if y_diff > x_diff {
        if enemy.y > character.y {
            return Point {x: enemy.x, y:enemy.y-1};
        }
        return Point {x: enemy.x, y:enemy.y+1};
    } else if y_diff < x_diff {
        if enemy.x > character.x {
            return Point {x: enemy.x-1, y:enemy.y};
        }
        return Point {x: enemy.x+1, y:enemy.y};
    } else {
        if enemy.x > character.x {
            return Point {x: enemy.x-1, y:enemy.y};
        }
        return Point {x: enemy.x+1, y:enemy.y};
    }
}

pub fn path_to(player: &Point, enemy: &Point) -> i8 {
    let mut distance: i8 = 0;
    distance += if player.x >= enemy.x {
        player.x-enemy.x
    } else {
        enemy.x-player.x
    };

    distance += if player.y >= enemy.y {
        player.y-enemy.y
    } else {
        enemy.y-player.y
    };
    return distance;
}

pub fn render(character: &Character, limits: &Point, enemy_list: &Vec<Enemy>) {
    print!("\x1B[2J\x1B[H"); /* Clears current window */

    for h in 0..limits.y {
        for w in 0..limits.x {
            let rendered = Point { x: w, y: h };

            let opt: Symbol = if character.location == rendered {
                Symbol::Char
            } else if enemy_list.iter().any(|e| e.location == rendered) {
                Symbol::Enemy
            } else {
                let mut sym = Symbol::Background;

                for enemy in 0..enemy_list.len() {
                    if path_to(&rendered, &enemy_list[enemy].location) <= enemy_list[enemy].status.move_amount {
                        sym = Symbol::EnemyRange;
                    };
                };

                sym
            };

            print!("{}", match opt {
                Symbol::Background => "_".normal(),
                Symbol::EnemyRange => "x".red(),
                Symbol::Enemy => "#".yellow(),
                Symbol::Char => "8".blue(),
                Symbol::Unknown => "?".black(),
            });
        }
        print!("\n");
    }
}
