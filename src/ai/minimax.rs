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
  ai::{self, Depth, Score},
  game::{self, Game, Player, Result, Turn},
  grid::{self, Cell, Grid},
};

fn minimax(node: Grid, mut turn: Turn, mut depth: Depth) -> Score {
  match grid::get_result(node, turn) {
    Result::Draw => ai::DRAW,
    Result::XWin => ai::X_WIN - depth,
    Result::OWin => ai::O_WIN + depth,
    Result::Continue => {
      let player = game::get_player_from_turn(turn);
      turn += 1;
      depth += 1;
      let mut value;

      if player == Player::X {
        value = ai::O_WIN;

        for cell in grid::get_empty_cells(node) {
          value = std::cmp::max(
            value,
            minimax(grid::mark(node, cell, Player::X), turn, depth),
          );
        }
      } else {
        value = ai::X_WIN;

        for cell in grid::get_empty_cells(node) {
          value = std::cmp::min(
            value,
            minimax(grid::mark(node, cell, Player::O), turn, depth),
          );
        }
      }

      value
    }
  }
}

pub(crate) fn find_best_move(game: &Game) -> Cell {
  let empty_cells = grid::get_empty_cells(game.grid);
  let turn = game.turn + 1;
  let mut best_move = empty_cells[0];

  if game::get_player_from_turn(game.turn) == Player::X {
    let mut best_score = ai::O_WIN;

    for cell in empty_cells {
      let score = minimax(grid::mark(game.grid, cell, Player::X), turn, 0);

      if score > best_score {
        best_score = score;
        best_move = cell;
      }
    }
  } else {
    let mut best_score = ai::X_WIN;

    for cell in empty_cells {
      let score = minimax(grid::mark(game.grid, cell, Player::O), turn, 0);

      if score < best_score {
        best_score = score;
        best_move = cell;
      }
    }
  }

  best_move
}
