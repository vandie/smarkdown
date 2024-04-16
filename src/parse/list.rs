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
  let mut last_line_start = 0;

  for line in lines.iter_mut() {
    let new_line_item = is_new_list_item(line, list_type, count) || last_line_start == 0;
    if new_line_item && !is_indented_past_start(&line, last_line_start) {
      last_line_start = list_item_content_start(&line);
      count += 1;
      if current_block.len() > 0 {
        blocks.push(Block::new(BlockType::LineItem, current_block, context));
      }
      line.trim_line_start(last_line_start);
      current_block = line.0.clone();
    } else {
      if current_block.len() > 0 {
        current_block.push(Token::NewLine);
      }

      if line.is_empty() == false {
        // If this line has been indented to the item start,
        // remove that indentation only to allow for indented code blocks
        if is_indented_past_start(&line, last_line_start) {
          line.trim_line_start(last_line_start);
        }

        // Add the trimmed content to the block
        current_block.append(&mut line.0.clone());
      }
    }
  }
  if current_block.len() > 0 {
    blocks.push(Block::new(BlockType::LineItem, current_block, context));
  }

  blocks
}

/// Returns true if first none space char is after `last_line_start`
///
/// If the `last_line_start` value is 0, will always return false
fn is_indented_past_start(line: &Line, last_line_start: usize) -> bool {
  if last_line_start == 0 {
    return false;
  }
  line.0.len() > last_line_start && Line(line.0[0..last_line_start].to_vec()).is_empty()
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

fn first_char_after_list_indicator(line: &Line) -> usize {
  let leading_spaces = line.leading_spaces();
  for (i, token) in line.0.iter().enumerate() {
    if token == &Token::Space && i >= leading_spaces {
      return i + 1;
    }
  }
  return 0;
}

pub fn list_item_content_start(line: &Line) -> usize {
  let first_char_after_indicator = first_char_after_list_indicator(line);
  line.0[first_char_after_indicator..]
    .iter()
    .position(|token| token != &Token::Space)
    .unwrap_or(line.0.len())
    + first_char_after_indicator
}
