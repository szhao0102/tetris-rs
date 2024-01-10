#[derive(Debug)]
pub struct Mino {
    pub shape: Vec<Vec<u8>>
}

impl Mino {
    pub fn new_i() -> Self {
        Self {
            shape: vec![
                vec![0,0,0,0],
                vec![1,1,1,1],
                vec![0,0,0,0],
                vec![0,0,0,0],
            ]
        }
    }
    pub fn new_o() -> Self {
        Self {
            shape: vec![
                vec![2,2],
                vec![2,2]
            ]
        }
    }
    pub fn new_t() -> Self {
        Self { 
            shape: vec![
                vec![0,3,0],
                vec![3,3,3],
                vec![0,0,0],
            ]
        }
    }
    pub fn new_l() -> Self {
        Self { 
            shape: vec![
                vec![0,0,4],
                vec![4,4,4],
                vec![0,0,0],
            ]
        }
    }
    pub fn new_j() -> Self {
        Self { 
            shape: vec![
                vec![5,0,0],
                vec![5,5,5],
                vec![0,0,0],
            ]
        }
    }
    pub fn new_s() -> Self {
        Self { 
            shape: vec![
                vec![0,6,6],
                vec![6,6,0],
                vec![0,0,0],
            ]
        }
    }
    pub fn new_z() -> Self {
        Self { 
            shape: vec![
                vec![7,7,0],
                vec![0,7,7],
                vec![0,0,0],
            ]
        }
    }
    pub fn width(&self) -> usize {
        self.shape[0].len()
    }

    pub fn height(&self) -> usize {
        self.shape.len()
    }
    pub fn rotate(&mut self) {
        let len = self.shape.len();
        let half_len = len / 2;
        for i in 0..half_len {
            for j in i..len - 1 - i {
                let tmp = self.shape[i][j];
                self.shape[i][j] = self.shape[len - 1 - j][i];
                self.shape[len - 1 - j][i] = self.shape[len - 1 - i][len - 1 -j];
                self.shape[len - 1 - i][len - 1 -j] = self.shape[j][len - 1 -i];
                self.shape[j][len - 1 - i] = tmp;

            }
        }
    }
}