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
  game::{self, Game, Player, Result},
  grid::{self, Cell},
};

pub(crate) fn find_best_move(game: &Game) -> Cell {
  let ai = game::get_player_from_turn(game.turn);
  let opponent = if ai == Player::X {
    Player::O
  } else {
    Player::X
  };
  let turn = game.turn + 1;
  let empty_cells = grid::get_empty_cells(game.grid);
  let mut best_move = empty_cells[0];

  for cell in empty_cells {
    let mut result = grid::get_result(grid::mark(game.grid, cell, ai), turn);

    if result != Result::Continue && result != Result::Draw {
      return cell;
    }

    result = grid::get_result(grid::mark(game.grid, cell, opponent), turn);

    if result != Result::Continue && result != Result::Draw {
      best_move = cell;
    }
  }

  best_move
}
