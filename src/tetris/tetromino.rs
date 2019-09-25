use ggez::graphics::Color;

pub struct Tetromino {
  pub block_positions: [[i32; 2]; 4],
  pub color: Color,
}

pub const I_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 1],
    [1, 1],
    [2, 1],
    [3, 1],
  ],
  color: Color::new(1.0, 0.0, 0.0, 1.0),
};

pub const J_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 0],
    [0, 1],
    [1, 1],
    [2, 1],
  ],
  color: Color::new(0.0, 1.0, 0.0, 1.0),
};

pub const L_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 1],
    [1, 1],
    [2, 1],
    [2, 0],
  ],
  color: Color::new(0.0, 0.0, 1.0, 1.0),
};

pub const O_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 0],
    [0, 1],
    [1, 1],
    [1, 0],
  ],
  color: Color::new(1.0, 1.0, 0.0, 1.0),
};

pub const S_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 1],
    [1, 1],
    [1, 0],
    [2, 0],
  ],
  color: Color::new(0.0, 1.0, 1.0, 1.0),
};

pub const T_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 1],
    [1, 1],
    [1, 0],
    [2, 1],
  ],
  color: Color::new(1.0, 0.0, 1.0, 1.0),
};

pub const Z_TETROMINO: Tetromino = Tetromino {
  block_positions: [
    [0, 0],
    [1, 0],
    [1, 1],
    [2, 1],
  ],
  color: Color::new(0.5, 0.0, 1.0, 1.0),
};

pub const ALL_TETROMINOS: [Tetromino; 7] = [
  I_TETROMINO,
  J_TETROMINO,
  L_TETROMINO,
  O_TETROMINO,
  S_TETROMINO,
  T_TETROMINO,
  Z_TETROMINO,
];
