use crate::mino::Mino;


const WIDTH: usize = 10;
const HEIGHT: usize = 15;


#[derive(Debug)]
pub struct Game {
    //游戏盘
    canvas: [[u8;WIDTH];HEIGHT],
    //正在控制的方块
    active: Mino,
    //开始位置
    pos: (usize, usize), 
    //游戏状态
    status: State
}
impl Game {
    pub fn init() -> Self {
        let act_mino = Mino::new_i();
        Self {
            canvas: [[0;WIDTH];HEIGHT],
            pos: (( WIDTH - act_mino.width() ) / 2, 0),
            active: act_mino,
            status: State::Start
        }
    }
    pub fn render(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if j >= self.pos.0 && j <= self.pos.0 + self.active.width() {
                    print!("{}", self.active[i][j]);
                } else {
                    print!("{}", self.canvas[i][j]);
                }
                
                if j == WIDTH - 1 {
                    println!();
                }
                
            }
        }
    }
    pub fn left() {

    }
    pub fn right() {

    }
    pub fn down() {

    }
}

#[derive(Debug)]
enum State {
    Start,
    Pause,
    End
}
