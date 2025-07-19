// 6*8 board
pub struct Board {
    pub tiles: [u8; 6 * 8],
    pub white_square: i8,
    pub black_square: i8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [0; 6 * 8],
            white_square: 0,
            black_square: 0,
        }
    }

    pub fn break_tile(&mut self, tile: u8) {
        self.tiles[tile as usize] = 0;
    }

    pub fn move_black(&mut self, offset: i8) {
        self.black_square += offset;

        if self.black_square < 0 {
            panic!("black square cannot be negative");
        }

        if self.black_square > 6 * 8 {
            panic!("black square cannot be greater than 6*8");
        }
    }

    pub fn move_white(&mut self, offset: i8) {
        self.white_square += offset;
        if self.white_square < 0 {
            panic!("white square cannot be negative");
        }
        if self.white_square > 6 * 8 {
            panic!("white square cannot be greater than 6*8");
        }
    }
}
