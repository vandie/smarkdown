use crate::{
  parse::list,
  tokeniser::{Token, TokenType},
};

use self::super::{
  blocks::{Block, BlockType},
  document::DocContext,
  line::Line,
};

use super::tokens_to_lines;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum ListType {
  Number(usize),
  BracketedNumber(usize),
  Dash,
  Star,
  Plus,
}

/// Special parsing for line items within a list. Shouldn't be called directly
pub fn parse_line_items(
  tokens: &Vec<Token>,
  context: &mut DocContext,
  list_type: ListType,
) -> Vec<Block> {
  let mut lines = tokens_to_lines(tokens);
  let mut blocks: Vec<Block> = vec![];
  let mut current_block = vec![];
  let mut count = 0;

  for line in lines.iter_mut() {
    let new_line_item = is_new_list_item(line, list_type, count);
    if new_line_item {
      count += 1;
      if current_block.len() > 0 {
        blocks.push(Block::new(BlockType::LineItem, current_block, context));
      }
      line.trim_line_start(
        line
          .find_first(TokenType::Space)
          .and_then(|v| Some(v + 1))
          .unwrap_or(0),
      );
      current_block = line.0.clone();
    } else {
      if current_block.len() > 0 {
        current_block.push(Token::NewLine);
      }
      line.unindent();
      current_block.append(&mut line.0.clone());
    }
  }
  if current_block.len() > 0 {
    blocks.push(Block::new(BlockType::LineItem, current_block, context));
  }

  blocks
}

/// Determins if a line should be rendered as a new list item
fn is_new_list_item(line: &Line, list_type: ListType, count: usize) -> bool {
  match list_type {
    ListType::BracketedNumber(start) => match line.line_type(None) {
      BlockType::List(ListType::BracketedNumber(cur_count)) => cur_count == start + count,
      _ => false,
    },
    ListType::Number(start) => match line.line_type(None) {
      BlockType::List(ListType::Number(cur_count)) => cur_count == start + count,
      _ => false,
    },
    _ => match line.line_type(None) {
      BlockType::List(line_list_type) => line_list_type == list_type,
      _ => false,
    },
  }
}
