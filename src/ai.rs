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

pub(crate) mod mcts;
pub(crate) mod minimax;
pub(crate) mod random;
pub(crate) mod simple;
type Depth = i8;
type Score = i8;
const X_WIN: Score = 32;
const DRAW: Score = X_WIN / 2;
const O_WIN: Score = 0;
