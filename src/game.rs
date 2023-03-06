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

use crate::grid::{self, Cell, Grid};
pub(crate) type Turn = i8;
pub(crate) const TURN_MAX: Turn = 9;
const TURN_MIN: Turn = 0;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum Player {
  X,
  O,
}

#[derive(PartialEq)]
pub(crate) enum Result {
  Continue,
  Draw,
  XWin,
  OWin,
}

pub(crate) struct Game {
  pub(crate) grid: Grid,
  pub(crate) turn: Turn,
}

pub(crate) fn get_player_from_turn(turn: Turn) -> Player {
  if turn % 2 == 0 {
    Player::X
  } else {
    Player::O
  }
}

pub(crate) fn play(player_x: fn(&Game) -> Cell, player_o: fn(&Game) -> Cell) {
  let mut game = Game {
    grid: grid::EMPTY,
    turn: TURN_MIN,
  };
  let mut result;
  grid::print(game.grid);

  loop {
    game.grid = grid::mark(game.grid, player_x(&game), Player::X);
    game.turn += 1;
    grid::print(game.grid);
    result = grid::get_result(game.grid, game.turn);

    if result != Result::Continue {
      break;
    }

    game.grid = grid::mark(game.grid, player_o(&game), Player::O);
    game.turn += 1;
    grid::print(game.grid);
    result = grid::get_result(game.grid, game.turn);

    if result != Result::Continue {
      break;
    }
  }

  println!(
    "result: {}",
    match result {
      Result::XWin => "x win",
      Result::OWin => "o win",
      _ => "draw",
    }
  );
}

pub(crate) fn ai_vs_ai(player_x: fn(&Game) -> Cell, player_o: fn(&Game) -> Cell) {
  let mut x_win = 0;
  let mut o_win = 0;
  let mut draw = 0;

  for _ in 0..101 {
    let mut game = Game {
      grid: grid::EMPTY,
      turn: TURN_MIN,
    };
    let mut result;

    loop {
      game.grid = grid::mark(game.grid, player_x(&game), Player::X);
      game.turn += 1;
      result = grid::get_result(game.grid, game.turn);

      if result != Result::Continue {
        break;
      }

      game.grid = grid::mark(game.grid, player_o(&game), Player::O);
      game.turn += 1;
      result = grid::get_result(game.grid, game.turn);

      if result != Result::Continue {
        break;
      }
    }

    match result {
      Result::XWin => x_win += 1,
      Result::OWin => o_win += 1,
      _ => draw += 1,
    }
  }

  println!("x win: {}\no win: {}\ndraw: {}", x_win, o_win, draw);
}
