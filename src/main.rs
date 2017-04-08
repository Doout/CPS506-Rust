#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io::{self, Read};


struct Player {
    name: String,
    index: i32,
    powerup: i8,
}

struct Board {
    width: i32,
    height: i32,
    size: i32,
}

struct Tile {
    displacement: i32,
    powerup: i8,
}

struct GameMap {
    board: Board,
    players: Vec<Player>,
    tiles: Vec<Tile>,
}

fn main() {
    let mut game: GameMap = GameMap {
        board: Board { width: 0, height: 0, size: 0 },
        players: (Vec::new()),
        tiles: (Vec::new())
    };
    let b = "board 3 4\nplayers 2\nturns 5".to_string();
    let lines: Vec<&str> = b.as_str().split('\n').collect();
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "board" {
            game.board.height = words[2].parse().unwrap();
            game.board.width = words[1].parse().unwrap();
            game.board.size = game.board.width * game.board.height;
        }
        println!("{}", line);
    }

    print_board(game);
}

fn read_commands() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    return buffer;
}


fn print_board(map: GameMap) {
    let height = map.board.height;
    let width = map.board.width;
    let mut index;
    for y in 0..height {
        println!("{}", print_board_border(width));
        for t in 0..2 {
            print!("|");
            for x in 0..width {
                if (y % 2 == 0 && height % 2 == 0) || (y % 2 == 1 && height % 2 == 1) {
                    index = (height - 1 - y) * width + (width - 1 - x);
                } else {
                    index = (height - 1 - y) * width + x;
                }
                if t == 0 {
                    print!("{}|", format_number(index + 1));
                } else {
                    print!("{}|", format_tile_data(index + 1));
                }
            }
            println!("");
        }
    }
    println!("{}", print_board_border(width));
}

fn print_board_border(w: i32) -> String {
    let mut s = "+".to_string();
    for v in 0..w {
        s.push_str("---+");
    }
    return s;
}

fn format_tile_data(i: i32) -> String {
    return "   ".to_string();
}

fn format_number(i: i32) -> String {
    let mut d = i.to_string();
    let mut k = "".to_string();
    while k.len() < (3 - d.len()) {
        k.push(' ');
    }
    return format!("{}{}", k, d);
}



