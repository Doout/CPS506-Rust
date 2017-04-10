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

struct Dice { dices: Vec<i8>, index: usize }

struct Tile {
    location: i32,
    displacement: i32,
    powerup: i8,
}

impl Player {
    fn remove_powerup(&mut self, powerup: i8) {
        if powerup > 0 {
            self.powerup = self.powerup & (7 ^ (1 << (powerup - 1)));
        }
    }

    fn add_powerup(&mut self, powerup: i8) {
        if powerup > 0 {
            self.powerup = self.powerup | (1 << (powerup - 1));
        }
    }

    fn have_powerup(&self, powerup: i8) -> bool {
        if powerup > 0 {
            return (self.powerup >> (powerup - 1)) & 1 == 1
        }
        return false;
    }
}

impl Tile {
    pub fn new_from(loc: &str, dis: &str, power: &str) -> Tile {
        Tile {
            location: loc.parse::<i32>().unwrap(),
            displacement: dis.parse::<i32>().unwrap(),
            powerup: power.parse::<i8>().unwrap(),
        }
    }
}

struct Game {
    board: Board,
    dice: Dice,
    players: Vec<Player>,
    tiles: Vec<Tile>,
}

fn main() {
    let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nturns 5\n".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2\nturns 5".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nladder 5 11\nsnake 8 4\nturns 5".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nladder 5 11\nsnake 8 4\npowerup escalator 6 9\npowerup antivenom 7\npowerup double 4\nturns 10".to_string();
    //let b = read_commands();
    let game = readFrom(b);
    print_board(&game);
}


fn make_emtpy_map() -> Game {
    return Game {
        board: Board { width: 0, height: 0, size: 0 },
        players: (Vec::new()),
        tiles: (Vec::new()),
        dice: Dice { dices: vec![1], index: 0 }
    };
}

#[allow(non_snake_case)]
fn readFrom(read: String) -> Game {
    let mut game = make_emtpy_map();
    let lines: Vec<&str> = read.as_str().split('\n').collect();
    for line in lines {
        let words: Vec<&str> = line.split(' ').collect();
        match words[0] {
            "board" => {
                game.board.height = words[2].parse().unwrap();
                game.board.width = words[1].parse().unwrap();
                game.board.size = game.board.width * game.board.height;
            }
            "players" => {
                for index in 0..words[1].parse().unwrap() {
                    game.players.push(Player {
                        name: ((65 + index) as u8 as char),
                        location: 0,
                        powerup: 0,
                    });
                    game = move_player(game, index, 1);
                }
            }
            "dice" => {
                game.dice.dices.remove(0);
                for i in 1..(words.len()) {
                    game.dice.dices.push(words[i].parse::<i8>().unwrap());
                }
            }
            "turns" => {
                let turn = words[1].parse::<i32>().unwrap();
                for i in 0..turn {
                    game = player_one_around(game);
                }
            }
            "ladder" | "snake" => game.tiles.push(Tile::new_from(words[1], words[2], "0")),
            "powerup" => {
                for index in 2..words.len() {
                    match words[1] {
                        "escalator" => game.tiles.push(Tile::new_from(words[index], "-1", "1")),
                        "antivenom" => game.tiles.push(Tile::new_from(words[index], "-1", "2")),
                        "double" => game.tiles.push(Tile::new_from(words[index], "-1", "3")),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    return game;
}

fn player_one_around(mut map: Game) -> Game {
    for index in 0..map.players.len() {
        if get_player_on_tile(&map, map.board.size) != -1 { return map; }
        let roll = map.dice.dices[map.dice.index];
        map.dice.index = (map.dice.index + 1) % map.dice.dices.len();
        let new_loc = map.players[index].location + roll as i32;
        map = move_player(map, index as i8, new_loc); // check make it roll 1
    }
    return map;
}

fn move_player(mut map: Game, player_index: i8, mut new_player_locatin: i32) -> Game {
    if new_player_locatin > map.board.size { return map; }
    let specal_tiles = get_specal_tile_index(&map, new_player_locatin);
    if specal_tiles != -1 {
        let ref tile = map.tiles[specal_tiles as usize];
        if tile.displacement != -1 { new_player_locatin = tile.displacement }
        map.players[player_index as usize].add_powerup(tile.powerup);
    }
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

fn get_specal_tile_index(map: &Game, loc: i32) -> i32 {
    for index in 0..map.tiles.len() {
        if map.tiles[index].location == loc {
            return index as i32;
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
    if height == 0 || width == 0 { return; }
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
    let player_won_index = get_player_on_tile(map, map.board.size);
    if player_won_index != -1 { println!("Player {} won", map.players[player_won_index as usize].name) }
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
    let mut powerup = ' ';
    let mut dis = ' ';
    let x = get_player_on_tile(map, i);
    if x != -1 { name = map.players[x as usize].name; }
    let x = get_specal_tile_index(map, i);
    if x != -1 {
        let ref tile = map.tiles[x as usize];
        if tile.displacement != -1 {
            if tile.displacement > tile.location { dis = 'L' } else { dis = 'S' }
        }
        match tile.powerup {
            1 => powerup = 'e',
            2 => powerup = 'a',
            3 => powerup = 'd',
            _ => powerup = ' ',
        }
    }
    return format!("{}{}{}", name, powerup, dis);
}

fn format_number(i: i32) -> String {
    let mut d = i.to_string();
    let mut k = "".to_string();
    while k.len() < (3 - d.len()) {
        k.push(' ');
    }
    return format!("{}{}", k, d);
}


