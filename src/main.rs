mod pieces;

use std::io::Read;
use std::io::Stdin;
fn main() {
    // thread::spawn(|| {
        loop {
            let stdin = &mut std::io::stdin();
            match get_input(stdin) {
                Some(k) => println!("{:?}", k),
                None => ()
            }
        }
    // });
}




#[derive(Debug)]
enum Direction {
    Left
}
struct Game;

impl Game {
    fn new() -> Game {
        let mut game = Game;
        game
    }
    //旋转
    fn rotate_piece(&mut self, direction: Direction) -> bool {
        false
    }
    //移动
    fn move_piece(&mut self, x: i8, y: i8) -> bool {
        false
    }

    fn advance_game(&mut self) -> bool {
        false
    }

    fn drop_piece(&mut self) -> bool {
        false
    }

    fn keypress(&mut self, key: Key) {
        match key {
            Key::Up => self.rotate_piece(Direction::Left),
            Key::Down => self.advance_game(),
            Key::Left => self.move_piece(-1, 0),
            Key::Right => self.move_piece(1, 0),
            Key::Space => self.drop_piece(),
            _ => false
        };
    }
}

#[derive(Debug)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    CtrlC
}

fn get_input(stdin: &mut Stdin) -> Option<Key> {
    let c = &mut [0u8]; //字节数组
    match stdin.read(c) {
        Ok(_) => {
            match std::str::from_utf8(c) {
                Ok("w") => Some(Key::Up),
                Ok("s") => Some(Key::Down),
                Ok("a") => Some(Key::Left),
                Ok("d") => Some(Key::Right),
                Ok(" ") => Some(Key::Space),
                Ok("\x03") => Some(Key::CtrlC),
                _ => None
            }
        },
        Err(_) => None
    }
}