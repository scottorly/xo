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

use crate::{
  game::Game,
  grid::{self, Cell},
};

pub(crate) fn get_move(game: &Game) -> Cell {
  let mut input = String::with_capacity(2);

  loop {
    std::io::stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<i8>().unwrap_or(0);

    if number > 0 && number < 10 {
      let cell = match number {
        1 => Cell::BottomLeft,
        2 => Cell::BottomMiddle,
        3 => Cell::BottomRight,
        4 => Cell::MiddleLeft,
        5 => Cell::MiddleMiddle,
        6 => Cell::MiddleRight,
        7 => Cell::TopLeft,
        8 => Cell::TopMiddle,
        _ => Cell::TopRight,
      };

      if grid::is_cell_empty(game.grid, cell) {
        return cell;
      }

      eprintln!("error: this cell is not empty");
    } else {
      eprintln!("error: invalid input");
    }

    input.clear();
  }
}
