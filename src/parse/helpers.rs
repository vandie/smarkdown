use crate::lex::{Bracket, Token};

use super::blocks::{BlockType, ListType};

pub fn should_recognise_blank_lines(block_type: BlockType) -> bool {
  match block_type {
    BlockType::List { .. } => false,
    _ => true,
  }
}
