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
    /*let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nturns 5\n".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2\nturns 5".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nladder 5 11\nsnake 8 4\nturns 5".to_string();
    let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nladder 5 11\nsnake 8 4\npowerup escalator 6 9\npowerup antivenom 7\npowerup double 4\nturns 10".to_string();
   */
    let b = read_commands();
    let game = readFrom(b);
    println!("{}", game.as_string());
    // print_board(&game);
}


fn make_emtpy_map() -> Game {
    return Game {
        board: Board { width: 0, height: 0, size: 0 },
        players: (Vec::new()),
        tiles: (Vec::new()),
        dice: Dice { dices: vec![1], index: 0 }
    };
}

fn print(game: &Game) -> String {
    return game.as_string();
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
        if map.get_player_on_tile(map.board.size) != -1 { return map; }
        let mut roll = map.dice.dices[map.dice.index];
        map.dice.index = (map.dice.index + 1) % map.dice.dices.len();
        if map.players[index].have_powerup(3) { roll = roll * 2; }
        map.players[index].remove_powerup(3);
        let new_loc = map.players[index].location + roll as i32;
        // println!("{} : {} : {:b}", map.players[index].name, roll, map.players[index].powerup);
        map = move_player(map, index as i8, new_loc); // check make it roll 1
    }
    return map;
}

#[allow(unused_assignments)]
fn move_player(mut map: Game, player_index: i8, mut new_player_locatin: i32) -> Game {
    let player_index = player_index as usize;
    if new_player_locatin > map.board.size { return map; }
    let specal_tiles = map.get_specal_tile_index(new_player_locatin);
    if specal_tiles != -1 {
        let ref tile = map.tiles[specal_tiles as usize];
        if tile.displacement != -1 {
            let mut d = tile.displacement - new_player_locatin;
            if tile.displacement > new_player_locatin {
                if map.players[player_index].have_powerup(1) {
                    d = d * 2;
                    map.players[player_index].remove_powerup(1)
                }
                new_player_locatin = new_player_locatin + d;
            } else {
                if map.players[player_index].have_powerup(2) {
                    d = 0;
                    map.players[player_index].remove_powerup(2)
                }
                new_player_locatin = new_player_locatin + d;
            }
        }
        map.players[player_index].add_powerup(tile.powerup);
    }
    if new_player_locatin > map.board.size { new_player_locatin = map.board.size; }
    let player_on_tiles = map.get_player_on_tile(new_player_locatin);
    map.players[player_index].location = new_player_locatin;
    if player_on_tiles != -1 {
        if player_on_tiles != player_index as i8 {
            return move_player(map, player_on_tiles, new_player_locatin + 1)
        } else {
            let specal_tiles = map.get_specal_tile_index(new_player_locatin);
            if specal_tiles != -1 {
                map.players[player_index].add_powerup(map.tiles[specal_tiles as usize].powerup);
            }
        }
    }
    return map;
}

fn read_commands() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);
    return buffer;
}

impl Game {
    fn get_player_on_tile(&self, loc: i32) -> i8 {
        for index in 0..self.players.len() {
            if self.players[index].location == loc {
                return index as i8;
            }
        }
        return -1;
    }

    fn get_specal_tile_index(&self, loc: i32) -> i32 {
        for index in 0..self.tiles.len() {
            if self.tiles[index].location == loc {
                return index as i32;
            }
        }
        return -1;
    }

    fn as_string(&self) -> String {
        let height = self.board.height;
        let width = self.board.width;
        let mut out = String::new();
        if height == 0 || width == 0 { return out; }
        let mut index;
        for y in 0..height {
            out.push_str(Game::print_board_border(width).as_str());
            out.push('\n');
            for t in 0..2 {
                for x in 0..width {
                    if (y % 2 == 0 && height % 2 == 0) || (y % 2 == 1 && height % 2 == 1) {
                        index = (height - 1 - y) * width + (width - 1 - x);
                    } else {
                        index = (height - 1 - y) * width + x;
                    }
                    out.push('|');
                    if t == 0 {
                        out.push_str(Game::format_number(index + 1).as_str());
                    } else {
                        out.push_str(Game::format_tile_data(self, index + 1).as_str());
                    }
                }
                out.push_str("|\n");
            }
        }
        out.push_str(Game::print_board_border(width).as_str());
        out.push('\n');
        let player_won_index = self.get_player_on_tile(self.board.size);
        if player_won_index != -1 {
            out.push_str((format!("Player {} won\n", self.players[player_won_index as usize].name)).as_str());
        }
        return out;
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
        let x = map.get_player_on_tile(i);
        if x != -1 { name = map.players[x as usize].name; }
        let x = map.get_specal_tile_index(i);
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
}

#[cfg(test)]
mod test {
    use readFrom;
    use print;

    #[test]
    fn board_test() {
        let b = "board 3 4\nplayers 2\ndice 1 2 2 2 2\nladder 5 11\nsnake 8 4\npowerup escalator 6 9\npowerup antivenom 7\npowerup double 4\nturns 10".to_string();
        let game = readFrom(b);
        let out = "+---+---+---+\n| 12| 11| 10|\n|B  |   |   |\n+---+---+---+\n|  7|  8|  9|\n| a |  S| e |\n+---+---+---+\n|  6|  5|  4|\n| e |  L|Ad |\n+---+---+---+\n|  1|  2|  3|\n|   |   |   |\n+---+---+---+\nPlayer B won\n".to_string();
        assert!(print(&game)== out);
    }

    #[test]
    fn board_test2() {
        let game = readFrom("board 2 3".to_string());
        let out = "+---+---+\n|  5|  6|\n|   |   |\n+---+---+\n|  4|  3|\n|   |   |\n+---+---+\n|  1|  2|\n|   |   |\n+---+---+\n";
        assert!(print(&game) == out.to_string());
    }
}