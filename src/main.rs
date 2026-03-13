mod movements;
mod types;

use movements::*;
use types::{Character, Enemy, Point, Stats, Symbol};

use std::io;
use std::io::Write;

use colored::Colorize;

const BUF: Point = Point {x: 10, y: 5};

const PREFIX: &str = "> ";
static mut HP: i8 = 10;

static SYM_LIST: [Symbol;5]= [
    Symbol {
        sym: '8',
        id: "character",
    },
    Symbol {
        sym: 'x',
        id: "enemy",
    },
    Symbol {
        sym: '-',
        id: "enemy_range",
    },
    Symbol {
        sym: '_',
        id: "background",
    },
    Symbol {
        sym: '~',
        id: "error",
    },
    ];

fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let limits = BUF;

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
            move_amount: 1,
        },
        id: 2,
        location: Point { x: 1, y: 2 },
    });

    let render = |c: &Character, e: &Vec<Enemy>| {
        print!("\x1B[2J\x1B[H"); /* Clears current window */

        for h in 0..limits.y {
            for w in 0..limits.x {
                let rendered = Point { x: w, y: h };

                let mut opt: &str = if c.location == rendered {
                    "character"
                } else if e.iter().any(|e| e.location == rendered) {
                    "enemy"
                } else {
                    "none"
                };

                if opt != "none" {queryp(opt); continue}

                for enemy in 0..e.len() {
                    if path_to(&rendered, &e[enemy].location, 2) == true {
                        opt = "enemy_range";
                    }
                }
                if opt == "none" { opt = "background"}

                queryp(opt);
            }
            print!("\n");
        }
    };

    loop {
        unsafe { character.status.hp = HP};
        if character.status.hp == 0 {
            println!("GAME OVER!");
            return;
        };
        render(&character, &enemy_list);
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

        action_checkout(&character.location, &enemy_list)
    }
}

pub fn action_checkout(char_loc: &Point, enemy_list: &Vec<Enemy>) {
    /* let new_enemy_list: Vec<Enemy> = Vec::new(); */
    
    let distance: i8 = 2;

    for enemy in 0..enemy_list.len() {
        let result = path_to(char_loc, &enemy_list[enemy].location, distance);

        if result == true {
            unsafe { HP -= 1};
        };
    }


    // new_enemy_list
}

pub fn path_to(player: &Point, enemy: &Point, range: i8) -> bool {
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
    if distance > range {
        return false
    }
    true
}

pub fn query(opt: &str) -> char {
    let mut symbol: Option<char> = None;

    for index in 0..SYM_LIST.len() {
        if SYM_LIST[index].id == opt {
            symbol = Some(SYM_LIST[index].sym);
            break;
        }
    }

    symbol.expect("sym_err:query")
}

pub fn queryp(opt: &str) {
    let result: &str = &query(opt).to_string();
    match opt {
        "character" => print!("{}", result.blue()),
        "enemy" => print!("{}", result.blue()),
        "enemy_range" => print!("{}", result.yellow()),
        _ => print!("{}", result),
    }
}

