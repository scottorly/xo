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
  game::{self, Game, Player, Result, Turn},
  grid::{self, Cell, Grid},
};
const ROOT_NODE: usize = 0;

struct Node {
  childrens: Vec<usize>,
  parent: usize,
  wins: f32,
  playouts: i32,
  grid: Grid,
  turn: Turn,
}

fn uct(wins: f32, playouts: f32, parent_playouts: f32) -> f32 {
  wins / playouts + std::f32::consts::SQRT_2 * (parent_playouts.ln() / playouts).sqrt()
}

fn select(tree: &Vec<Node>) -> usize {
  let mut leaf = ROOT_NODE;

  loop {
    if tree[leaf].childrens.len() == 0 {
      return leaf;
    }

    let mut best_score = f32::MIN;

    for child in &tree[leaf].childrens {
      if tree[*child].playouts == 0 {
        return *child;
      }

      let score = uct(
        tree[*child].wins,
        tree[*child].playouts as f32,
        tree[leaf].playouts as f32,
      );

      if score > best_score {
        best_score = score;
        leaf = *child;
      }
    }
  }
}

fn expand(tree: &mut Vec<Node>, leaf: usize) -> usize {
  if grid::get_result(tree[leaf].grid, tree[leaf].turn) != Result::Continue {
    return leaf;
  }

  let player = game::get_player_from_turn(tree[leaf].turn);
  let turn = tree[leaf].turn + 1;

  for cell in grid::get_empty_cells(tree[leaf].grid) {
    let children = tree.len();
    tree.push(Node {
      childrens: Vec::with_capacity(grid::NUMBER_CELLS!()),
      parent: leaf,
      wins: 0.0,
      playouts: 0,
      grid: grid::mark(tree[leaf].grid, cell, player),
      turn: turn,
    });
    tree[leaf].childrens.push(children);
  }

  use rand::prelude::SliceRandom;

  *tree[leaf]
    .childrens
    .choose(&mut rand::thread_rng())
    .unwrap()
}

fn simulate(mut grid: Grid, mut turn: Turn) -> Result {
  loop {
    let result = grid::get_result(grid, turn);

    if result != Result::Continue {
      return result;
    }

    grid = grid::mark(
      grid,
      grid::get_random_empty_cell(grid),
      game::get_player_from_turn(turn),
    );
    turn += 1;
  }
}

fn backpropagate(tree: &mut Vec<Node>, mut child: usize, result: Result) {
  loop {
    tree[child].playouts += 1;

    if result == Result::Draw {
      tree[child].wins += 0.5;
    } else {
      let player = game::get_player_from_turn(tree[child].turn);

      if result == Result::XWin && player == Player::O
        || result == Result::OWin && player == Player::X
      {
        tree[child].wins += 1.0;
      }
    }

    if tree[child].parent == usize::MAX {
      return;
    }

    child = tree[child].parent;
  }
}

pub(crate) fn mcts(game: &Game) -> Cell {
  macro_rules! INITIAL_TREE_CAPACITY {
    () => {
      262144
    };
  }

  macro_rules! PLAYOUTS {
    () => {
      8191
    };
  }

  let mut tree = Vec::with_capacity(INITIAL_TREE_CAPACITY!());
  tree.push(Node {
    childrens: Vec::with_capacity(grid::NUMBER_CELLS!()),
    parent: usize::MAX,
    wins: 0.0,
    playouts: 0,
    grid: game.grid,
    turn: game.turn,
  });

  for _ in 0..PLAYOUTS!() {
    let leaf = select(&tree);
    let child = expand(&mut tree, leaf);
    let result = simulate(tree[child].grid, tree[child].turn);
    backpropagate(&mut tree, child, result);
  }

  let mut best_grid = tree[tree[ROOT_NODE].childrens[0]].grid;
  let mut most_playouts = 0;

  for child in &tree[ROOT_NODE].childrens {
    if most_playouts < tree[*child].playouts {
      most_playouts = tree[*child].playouts;
      best_grid = tree[*child].grid;
    }
  }

  let player = game::get_player_from_turn(game.turn);
  let empty_cells = grid::get_empty_cells(game.grid);
  let mut best_cell = empty_cells[0];

  for cell in empty_cells {
    if grid::mark(game.grid, cell, player) == best_grid {
      best_cell = cell;
      break;
    }
  }

  best_cell
}
