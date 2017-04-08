#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io::{self, Read};


struct Player {
    name: char,
    location: i32,
    powerup: i8,
}

struct Board {
    width: i32,
    height: i32,
    size: i32,
}

struct Tile {
    loaction: i32,
    displacement: i32,
    powerup: i8,
}

struct Game {
    board: Board,
    players: Vec<Player>,
    tiles: Vec<Tile>,
}

fn main() {
    let b = "board 3 4\nplayers 2\nturns 5".to_string();
    let game = readFrom(b);
    print_board(&game);
}

#[allow(non_snake_case)]
fn readFrom(read: String) -> Game {
    let mut game: Game = Game {
        board: Board { width: 0, height: 0, size: 0 },
        players: (Vec::new()),
        tiles: (Vec::new())
    };
    let lines: Vec<&str> = read.as_str().split('\n').collect();
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "board" {
            game.board.height = words[2].parse().unwrap();
            game.board.width = words[1].parse().unwrap();
            game.board.size = game.board.width * game.board.height;
        } else if words[0] == "players" {
            for index in 0..words[1].parse().unwrap() {
                game.players.push(Player {
                    name: ((65 + index) as u8 as char),
                    location: 0,
                    powerup: 0,
                });
                game = move_player(game, index, 1);
            }
        }
     //   println!("{}", line);
    }
    return game;
}

fn move_player(mut map: Game, player_index: i8, new_player_locatin: i32) -> Game {
    let player_on_tiles = get_player_on_tile(&map, new_player_locatin);
    map.players[player_index as usize].location = new_player_locatin;
    if player_on_tiles != -1 {
        return move_player(map, player_on_tiles, new_player_locatin + 1)
    }
    return map;
}

fn get_player_on_tile(map: &Game, loc: i32) -> i8 {
    for index in 0..map.players.len() {
        if map.players[index].location == loc {
            return index as i8;
        }
    }
    return -1;
}

fn read_commands() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    return buffer;
}

fn print_board(map: &Game) {
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
                    print!("{}|", format_tile_data(map, index + 1));
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

fn format_tile_data(map: &Game, i: i32) -> String {
    let mut name = ' ';
    let x = get_player_on_tile(map, i);
    if x != -1 {
       name = map.players[x as usize].name;
    }
    return format!("{}{}", name, "  ");
}

fn format_number(i: i32) -> String {
    let mut d = i.to_string();
    let mut k = "".to_string();
    while k.len() < (3 - d.len()) {
        k.push(' ');
    }
    return format!("{}{}", k, d);
}



