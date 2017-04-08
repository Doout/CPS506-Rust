struct Player {
    name: String,
    index: i32,
    powerup: i8,
}

struct Tile {
    displacement: i32,
    powerup: i8,
}


fn main() {
    print_board(8, 8);
}


fn print_board(w: i32, h: i32) {
    let heigth = h - 1;
    let width = w - 1;
    let mut index;
    for y in 0..h {
        println!("{}", print_board_border(w));
        for t in 0..2 {
            print!("|");
            for x in 0..w {
                if y % 2 == 0 {
                    index = (heigth - y) * w + (width - x);
                } else {
                    index = (heigth - y) * w + x;
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
    println!("{}", print_board_border(w));
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



