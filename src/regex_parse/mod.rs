/*
 * parse/mod.rs
 *
 * wikidot-html - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith for Project Foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

mod regex;
mod rules;
mod state;
mod token;

pub use state::ParseState;
pub use token::Token;

use rules::RULES;
use super::prelude::*;

#[derive(Debug, Clone)]
pub struct BlockQuoteLine {
    pub contents: String,
    pub depth: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

// TODO
pub fn parse(text: String) -> Result<ParseState> {
    let mut state = ParseState::new(text);
    for rule in &RULES[..] {
        rule.apply(&mut state)?;
    }

    Ok(state)
}