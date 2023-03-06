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

mod ai;
mod game;
mod grid;
mod user;
use crate::{
  ai::{mcts, minimax, random, simple},
  game::Game,
  grid::Cell,
};
type PlayerId = i8;
type Player = (fn(&Game) -> Cell, String, PlayerId);
const MCTS: PlayerId = 0;
const RANDOM: PlayerId = 1;
const MINIMAX: PlayerId = 2;
const SIMPLE: PlayerId = 3;
const USER: PlayerId = 4;

fn get_player(id: PlayerId) -> Player {
  match id {
    MCTS => (mcts::mcts, String::from("mcts"), MCTS),
    RANDOM => (random::find_best_move, String::from("random"), RANDOM),
    MINIMAX => (minimax::find_best_move, String::from("minimax"), MINIMAX),
    SIMPLE => (simple::find_best_move, String::from("simple"), SIMPLE),
    USER => (user::get_move, String::from("user"), USER),
    _ => panic!("invalid player id"),
  }
}

fn is_ai_player(player: PlayerId) -> bool {
  match player {
    MCTS | RANDOM | MINIMAX | SIMPLE => true,
    USER => false,
    _ => panic!("invalid player id"),
  }
}

fn get_player_id_from_argument(argument: Option<String>) -> PlayerId {
  argument.unwrap().parse::<PlayerId>().unwrap()
}

fn parse_arguments() -> (Player, Player, bool) {
  const DEFAULT_PLAYER_ID: PlayerId = MCTS;
  let mut is_user_o = false;
  let mut ai_vs_ai = false;
  let mut ai = DEFAULT_PLAYER_ID;
  let mut ai_vs_ai_x = DEFAULT_PLAYER_ID;
  let mut ai_vs_ai_o = DEFAULT_PLAYER_ID;
  let mut arguments = std::env::args().skip(1);

  while let Some(argument) = arguments.next() {
    match argument.as_str() {
      "-o" => is_user_o = true,
      "-t" => ai = get_player_id_from_argument(arguments.next()),
      "-m" => {
        ai_vs_ai = true;
        ai_vs_ai_x = get_player_id_from_argument(arguments.next());
        ai_vs_ai_o = get_player_id_from_argument(arguments.next());
      }
      _ => {}
    }
  }

  if ai_vs_ai {
    if is_ai_player(ai_vs_ai_x) == false || is_ai_player(ai_vs_ai_o) == false {
      panic!("invalid ai");
    }

    (get_player(ai_vs_ai_x), get_player(ai_vs_ai_o), ai_vs_ai)
  } else {
    let user = get_player(USER);
    let ai = get_player(ai);

    if is_user_o {
      (ai, user, ai_vs_ai)
    } else {
      (user, ai, ai_vs_ai)
    }
  }
}

fn main() {
  let (player_x, player_o, ai_vs_ai) = parse_arguments();
  println!(
    "player x: {} (id: {})\nplayer o: {} (id: {})",
    player_x.1, player_x.2, player_o.1, player_o.2
  );

  if ai_vs_ai {
    game::ai_vs_ai(player_x.0, player_o.0);
  } else {
    game::play(player_x.0, player_o.0);
  }
}
