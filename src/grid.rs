// Copyright 2023 Kamil Gloc

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//  http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::game::{self, Player, Result, Turn};
pub(crate) type Grid = i32;

macro_rules! NUMBER_CELLS {
  () => {
    9
  };
}

pub(crate) use NUMBER_CELLS;

#[derive(Clone, Copy)]
pub(crate) enum Cell {
  TopLeft,
  TopMiddle,
  TopRight,
  MiddleLeft,
  MiddleMiddle,
  MiddleRight,
  BottomLeft,
  BottomMiddle,
  BottomRight,
}

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

pub(crate) const EMPTY: Grid = 0b000_000_000_000_000_000;

// x
// | x | - | - |
// | - | - | - |
// | - | - | - |
// o
// | o | - | - |
// | - | - | - |
// | - | - | - |

const CELL_XO_TOP_LEFT: Grid = 0b000_000_001_000_000_001;

// x
// | - | x | - |
// | - | - | - |
// | - | - | - |
// o
// | - | o | - |
// | - | - | - |
// | - | - | - |

const CELL_XO_TOP_MIDDLE: Grid = 0b000_000_010_000_000_010;

// x
// | - | - | x |
// | - | - | - |
// | - | - | - |
// o
// | - | - | o |
// | - | - | - |
// | - | - | - |

const CELL_XO_TOP_RIGHT: Grid = 0b000_000_100_000_000_100;

// x
// | - | - | - |
// | x | - | - |
// | - | - | - |
// o
// | - | - | - |
// | o | - | - |
// | - | - | - |

const CELL_XO_MIDDLE_LEFT: Grid = 0b000_001_000_000_001_000;

// x
// | - | - | - |
// | - | x | - |
// | - | - | - |
// o
// | - | - | - |
// | - | o | - |
// | - | - | - |

const CELL_XO_MIDDLE_MIDDLE: Grid = 0b000_010_000_000_010_000;

// x
// | - | - | - |
// | - | - | x |
// | - | - | - |
// o
// | - | - | - |
// | - | - | o |
// | - | - | - |

const CELL_XO_MIDDLE_RIGHT: Grid = 0b000_100_000_000_100_000;

// x
// | - | - | - |
// | - | - | - |
// | x | - | - |
// o
// | - | - | - |
// | - | - | - |
// | o | - | - |

const CELL_XO_BOTTOM_LEFT: Grid = 0b001_000_000_001_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | x | - |
// o
// | - | - | - |
// | - | - | - |
// | - | o | - |

const CELL_XO_BOTTOM_MIDDLE: Grid = 0b010_000_000_010_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | x |
// o
// | - | - | - |
// | - | - | - |
// | - | - | o |

const CELL_XO_BOTTOM_RIGHT: Grid = 0b100_000_000_100_000_000;

// x
// | x | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_TOP_LEFT: Grid = 0b000_000_000_000_000_001;

// x
// | - | x | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_TOP_MIDDLE: Grid = 0b000_000_000_000_000_010;

// x
// | - | - | x |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_TOP_RIGHT: Grid = 0b000_000_000_000_000_100;

// x
// | - | - | - |
// | x | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_MIDDLE_LEFT: Grid = 0b000_000_000_000_001_000;

// x
// | - | - | - |
// | - | x | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_MIDDLE_MIDDLE: Grid = 0b000_000_000_000_010_000;

// x
// | - | - | - |
// | - | - | x |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_MIDDLE_RIGHT: Grid = 0b000_000_000_000_100_000;

// x
// | - | - | - |
// | - | - | - |
// | x | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_BOTTOM_LEFT: Grid = 0b000_000_000_001_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | x | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_BOTTOM_MIDDLE: Grid = 0b000_000_000_010_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | x |
// o
// | - | - | - |
// | - | - | - |
// | - | - | - |

const CELL_X_BOTTOM_RIGHT: Grid = 0b000_000_000_100_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | o | - | - |
// | - | - | - |
// | - | - | - |

const CELL_O_TOP_LEFT: Grid = 0b000_000_001_000_000_000;
// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | o | - |
// | - | - | - |
// | - | - | - |

const CELL_O_TOP_MIDDLE: Grid = 0b000_000_010_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | o |
// | - | - | - |
// | - | - | - |

const CELL_O_TOP_RIGHT: Grid = 0b000_000_100_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | o | - | - |
// | - | - | - |

const CELL_O_MIDDLE_LEFT: Grid = 0b000_001_000_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | o | - |
// | - | - | - |

const CELL_O_MIDDLE_MIDDLE: Grid = 0b000_010_000_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | o |
// | - | - | - |

const CELL_O_MIDDLE_RIGHT: Grid = 0b000_100_000_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | o | - | - |

const CELL_O_BOTTOM_LEFT: Grid = 0b001_000_000_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | o | - |

const CELL_O_BOTTOM_MIDDLE: Grid = 0b010_000_000_000_000_000;

// x
// | - | - | - |
// | - | - | - |
// | - | - | - |
// o
// | - | - | - |
// | - | - | - |
// | - | - | o |

const CELL_O_BOTTOM_RIGHT: Grid = 0b100_000_000_000_000_000;

pub(crate) fn get_result(grid: Grid, turn: Turn) -> Result {
  // x
  // | x | - | - |
  // | - | x | - |
  // | - | - | x |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_TOP_LEFT_TO_BOTTOM_RIGHT: Grid = 0b000_000_000_100_010_001;

  // x
  // | - | - | x |
  // | - | x | - |
  // | x | - | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_TOP_RIGHT_TO_BOTTOM_LEFT: Grid = 0b000_000_000_001_010_100;

  // x
  // | x | x | x |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_TOP_ROW: Grid = 0b000_000_000_000_000_111;

  // x
  // | - | - | - |
  // | x | x | x |
  // | - | - | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_MIDDLE_ROW: Grid = 0b000_000_000_000_111_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | x | x | x |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_BOTTOM_ROW: Grid = 0b000_000_000_111_000_000;

  // x
  // | x | - | - |
  // | x | - | - |
  // | x | - | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_LEFT_COLUMN: Grid = 0b000_000_000_001_001_001;

  // x
  // | - | x | - |
  // | - | x | - |
  // | - | x | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_MIDDLE_COLUMN: Grid = 0b000_000_000_010_010_010;

  // x
  // | - | - | x |
  // | - | - | x |
  // | - | - | x |
  // o
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |

  const X_WIN_RIGHT_COLUMN: Grid = 0b000_000_000_100_100_100;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | o | - | - |
  // | - | o | - |
  // | - | - | o |

  const O_WIN_TOP_LEFT_TO_BOTTOM_RIGHT: Grid = 0b100_010_001_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | - | o |
  // | - | o | - |
  // | o | - | - |

  const O_WIN_TOP_RIGHT_TO_BOTTOM_LEFT: Grid = 0b001_010_100_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | o | o | o |
  // | - | - | - |
  // | - | - | - |

  const O_WIN_TOP_ROW: Grid = 0b000_000_111_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | - | - |
  // | o | o | o |
  // | - | - | - |

  const O_WIN_MIDDLE_ROW: Grid = 0b000_111_000_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | - | - |
  // | - | - | - |
  // | o | o | o |

  const O_WIN_BOTTOM_ROW: Grid = 0b111_000_000_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | o | - | - |
  // | o | - | - |
  // | o | - | - |

  const O_WIN_LEFT_COLUMN: Grid = 0b001_001_001_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | o | - |
  // | - | o | - |
  // | - | o | - |

  const O_WIN_MIDDLE_COLUMN: Grid = 0b010_010_010_000_000_000;

  // x
  // | - | - | - |
  // | - | - | - |
  // | - | - | - |
  // o
  // | - | - | o |
  // | - | - | o |
  // | - | - | o |

  const O_WIN_RIGHT_COLUMN: Grid = 0b100_100_100_000_000_000;

  if grid & X_WIN_TOP_LEFT_TO_BOTTOM_RIGHT == X_WIN_TOP_LEFT_TO_BOTTOM_RIGHT
    || grid & X_WIN_TOP_RIGHT_TO_BOTTOM_LEFT == X_WIN_TOP_RIGHT_TO_BOTTOM_LEFT
    || grid & X_WIN_TOP_ROW == X_WIN_TOP_ROW
    || grid & X_WIN_MIDDLE_ROW == X_WIN_MIDDLE_ROW
    || grid & X_WIN_BOTTOM_ROW == X_WIN_BOTTOM_ROW
    || grid & X_WIN_LEFT_COLUMN == X_WIN_LEFT_COLUMN
    || grid & X_WIN_MIDDLE_COLUMN == X_WIN_MIDDLE_COLUMN
    || grid & X_WIN_RIGHT_COLUMN == X_WIN_RIGHT_COLUMN
  {
    Result::XWin
  } else if grid & O_WIN_TOP_LEFT_TO_BOTTOM_RIGHT == O_WIN_TOP_LEFT_TO_BOTTOM_RIGHT
    || grid & O_WIN_TOP_RIGHT_TO_BOTTOM_LEFT == O_WIN_TOP_RIGHT_TO_BOTTOM_LEFT
    || grid & O_WIN_TOP_ROW == O_WIN_TOP_ROW
    || grid & O_WIN_MIDDLE_ROW == O_WIN_MIDDLE_ROW
    || grid & O_WIN_BOTTOM_ROW == O_WIN_BOTTOM_ROW
    || grid & O_WIN_LEFT_COLUMN == O_WIN_LEFT_COLUMN
    || grid & O_WIN_MIDDLE_COLUMN == O_WIN_MIDDLE_COLUMN
    || grid & O_WIN_RIGHT_COLUMN == O_WIN_RIGHT_COLUMN
  {
    Result::OWin
  } else if turn == game::TURN_MAX {
    Result::Draw
  } else {
    Result::Continue
  }
}

pub(crate) fn mark(grid: Grid, cell: Cell, player: Player) -> Grid {
  match player {
    Player::X => match cell {
      Cell::TopLeft => grid | CELL_X_TOP_LEFT,
      Cell::TopMiddle => grid | CELL_X_TOP_MIDDLE,
      Cell::TopRight => grid | CELL_X_TOP_RIGHT,
      Cell::MiddleLeft => grid | CELL_X_MIDDLE_LEFT,
      Cell::MiddleMiddle => grid | CELL_X_MIDDLE_MIDDLE,
      Cell::MiddleRight => grid | CELL_X_MIDDLE_RIGHT,
      Cell::BottomLeft => grid | CELL_X_BOTTOM_LEFT,
      Cell::BottomMiddle => grid | CELL_X_BOTTOM_MIDDLE,
      Cell::BottomRight => grid | CELL_X_BOTTOM_RIGHT,
    },
    Player::O => match cell {
      Cell::TopLeft => grid | CELL_O_TOP_LEFT,
      Cell::TopMiddle => grid | CELL_O_TOP_MIDDLE,
      Cell::TopRight => grid | CELL_O_TOP_RIGHT,
      Cell::MiddleLeft => grid | CELL_O_MIDDLE_LEFT,
      Cell::MiddleMiddle => grid | CELL_O_MIDDLE_MIDDLE,
      Cell::MiddleRight => grid | CELL_O_MIDDLE_RIGHT,
      Cell::BottomLeft => grid | CELL_O_BOTTOM_LEFT,
      Cell::BottomMiddle => grid | CELL_O_BOTTOM_MIDDLE,
      Cell::BottomRight => grid | CELL_O_BOTTOM_RIGHT,
    },
  }
}

pub(crate) fn is_cell_empty(grid: Grid, cell: Cell) -> bool {
  const EMPTY_CELL: Grid = 0b000_000_000_000_000_000;

  match cell {
    Cell::TopLeft => grid & CELL_XO_TOP_LEFT == EMPTY_CELL,
    Cell::TopMiddle => grid & CELL_XO_TOP_MIDDLE == EMPTY_CELL,
    Cell::TopRight => grid & CELL_XO_TOP_RIGHT == EMPTY_CELL,
    Cell::MiddleLeft => grid & CELL_XO_MIDDLE_LEFT == EMPTY_CELL,
    Cell::MiddleMiddle => grid & CELL_XO_MIDDLE_MIDDLE == EMPTY_CELL,
    Cell::MiddleRight => grid & CELL_XO_MIDDLE_RIGHT == EMPTY_CELL,
    Cell::BottomLeft => grid & CELL_XO_BOTTOM_LEFT == EMPTY_CELL,
    Cell::BottomMiddle => grid & CELL_XO_BOTTOM_MIDDLE == EMPTY_CELL,
    Cell::BottomRight => grid & CELL_XO_BOTTOM_RIGHT == EMPTY_CELL,
  }
}

pub(crate) fn get_empty_cells(grid: Grid) -> Vec<Cell> {
  let mut cells = Vec::with_capacity(NUMBER_CELLS!());

  for cell in [
    Cell::MiddleMiddle,
    Cell::TopLeft,
    Cell::TopRight,
    Cell::BottomLeft,
    Cell::BottomRight,
    Cell::TopMiddle,
    Cell::MiddleLeft,
    Cell::MiddleRight,
    Cell::BottomMiddle,
  ] {
    if is_cell_empty(grid, cell) {
      cells.push(cell);
    }
  }

  cells
}

pub(crate) fn get_random_empty_cell(grid: Grid) -> Cell {
  use rand::prelude::SliceRandom;

  *get_empty_cells(grid)
    .choose(&mut rand::thread_rng())
    .unwrap()
}

pub(crate) fn print(grid: Grid) {
  println!(
    "\n|{} {} {}|\n|{} {} {}|\n|{} {} {}|",
    match grid & CELL_XO_TOP_LEFT {
      CELL_X_TOP_LEFT => 'x',
      CELL_O_TOP_LEFT => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_TOP_MIDDLE {
      CELL_X_TOP_MIDDLE => 'x',
      CELL_O_TOP_MIDDLE => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_TOP_RIGHT {
      CELL_X_TOP_RIGHT => 'x',
      CELL_O_TOP_RIGHT => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_MIDDLE_LEFT {
      CELL_X_MIDDLE_LEFT => 'x',
      CELL_O_MIDDLE_LEFT => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_MIDDLE_MIDDLE {
      CELL_X_MIDDLE_MIDDLE => 'x',
      CELL_O_MIDDLE_MIDDLE => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_MIDDLE_RIGHT {
      CELL_X_MIDDLE_RIGHT => 'x',
      CELL_O_MIDDLE_RIGHT => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_BOTTOM_LEFT {
      CELL_X_BOTTOM_LEFT => 'x',
      CELL_O_BOTTOM_LEFT => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_BOTTOM_MIDDLE {
      CELL_X_BOTTOM_MIDDLE => 'x',
      CELL_O_BOTTOM_MIDDLE => 'o',
      _ => ' ',
    },
    match grid & CELL_XO_BOTTOM_RIGHT {
      CELL_X_BOTTOM_RIGHT => 'x',
      CELL_O_BOTTOM_RIGHT => 'o',
      _ => ' ',
    }
  );
}
