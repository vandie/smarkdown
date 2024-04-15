mod blocks;
mod document;
mod helpers;
mod inlines;
mod line;
mod list;

use crate::{
  parse::list::list_item_content_start,
  tokeniser::{tokenise, Token},
};

use self::{
  blocks::{Block, BlockType},
  document::{DocContext, Document},
  helpers::should_recognise_blank_lines,
  inlines::Inline,
  line::Line,
  list::ListType,
};

pub fn parse(md: &str) -> Document {
  let tokens = tokenise(md);
  parse_tokens(tokens)
}

pub fn parse_tokens(tokens: Vec<Token>) -> Document {
  let mut document = Document::new();
  for block in parse_tokens_with_context(&tokens, &mut document.context) {
    document.add_block(block);
  }
  document
}

pub(crate) fn parse_tokens_with_context(
  tokens: &Vec<Token>,
  context: &mut DocContext,
) -> Vec<Block> {
  let mut current_block_type: Option<BlockType> = None;
  let lines: &mut Vec<Line> = &mut tokens_to_lines(tokens);
  let mut blocks: Vec<Block> = vec![];
  let mut current_block: Vec<Token> = vec![];
  let mut continued_content_start = 0;
  let mut last_line_empty = false;

  // Needed for lists
  let mut count = 0;

  for (_i, line) in lines.iter_mut().enumerate() {
    // Reset Block Type
    if current_block_type == Some(BlockType::Paragraph) && current_block.len() == 0 {
      current_block_type = None;
      continued_content_start = 0;
    }

    let currently_in_list = matches!(current_block_type, Some(BlockType::List(..)));
    let blank_space = line.unindented_leading_spaces();
    line.trim_line_start(blank_space); // Remove up to 3 leading spaces before we grab the line_type
    let mut new_block_type = line.line_type(current_block_type);

    // if this line is still a list without the previous line, then that proves a new line item
    // and we should grab the point where the content starts due to how lists are handled
    if matches!(line.line_type(None), BlockType::List(..)) {
      continued_content_start = list_item_content_start(line) + blank_space;
    }

    let mut handle_as_empty = false;

    if current_block_type.is_some() {
      let current_block_type = current_block_type.unwrap();

      // Some special list based handling
      if current_block_type == new_block_type && currently_in_list {
        // if the line doesn't have enough indentation and this is a list, don't include it as part of the line item if it is preceded by a new line
        if line.is_empty() == false
          && last_line_empty
          && list_item_content_start(line) + blank_space < continued_content_start
        {
          new_block_type = line.line_type(None);
        }
        // if this is a numbered list, the count needs to be checked as otherwise it should become 2 lists
        else if is_num_list_continuation(&current_block_type, &new_block_type, count) {
          count += 1;
          new_block_type = current_block_type;
        }
      }

      // Has the previous block concluded
      handle_as_empty = should_recognise_blank_lines(current_block_type) && line.is_empty();
      let doesent_match = new_block_type.allow_takeover(current_block_type) == false
        && new_block_type != current_block_type;
      let has_enough_to_push = current_block_type.allow_no_content() || current_block.len() > 0;
      let should_terminate = handle_as_empty || doesent_match;

      // track if this line was empty for stacking purposes
      last_line_empty = line.is_empty();

      // terminate existing block
      if has_enough_to_push && should_terminate {
        blocks.push(Block::new(current_block_type, current_block, context));
        new_block_type = line.line_type(None); // This isn't a continuation, rethink the line type
        current_block = vec![];
        count = 0;
      }
    }

    // Special handling for Setext Headers if the line before is empty
    if current_block.len() == 0 {
      new_block_type = match new_block_type {
        BlockType::SetextHeader(1) => BlockType::Paragraph,
        BlockType::SetextHeader(2) => BlockType::ThematicBreak,
        _ => new_block_type,
      };
    }

    // Now that we're sure of the block type, we can remove the line type indicators
    line.remove_type_chars(&new_block_type);

    // Begin building new block
    if Some(new_block_type) != current_block_type {
      current_block_type = Some(new_block_type)
    }

    if handle_as_empty == false {
      if current_block.len() > 0 {
        current_block.push(Token::NewLine);
      }
      current_block.append(&mut line.0);
    }
  }

  // If this block type can be completely empty and you need to, push the empty block
  if current_block_type.is_some_and(|bt| bt.allow_no_content()) || current_block.len() > 0 {
    blocks.push(Block::new(
      current_block_type.unwrap(),
      current_block,
      context,
    ));
  }

  blocks
}

pub fn tokens_to_lines(tokens: &Vec<Token>) -> Vec<Line> {
  let mut lines: Vec<Line> = vec![];
  let mut latest_line = vec![];
  for token in tokens.iter() {
    match token {
      Token::NewLine => {
        lines.push(Line(latest_line.clone()));
        latest_line = vec![];
        continue;
      }
      _ => {
        latest_line.push(token.clone());
      }
    }
  }
  // If the current line length is greater than 0 then the line was never ended and needs to be pushed to the line list
  if latest_line.len() > 0 || lines.last().is_some_and(|line| line.is_empty() == false) {
    lines.push(Line(latest_line.clone()));
  }
  lines
}

pub fn parse_inlines(tokens: &Vec<Token>, context: &mut DocContext) -> Vec<Inline> {
  return tokens
    .iter()
    .map(|token| Inline::Text(token.clone().into()))
    .collect();
}

fn is_num_list_continuation(old_type: &BlockType, new_type: &BlockType, count: usize) -> bool {
  // if this is a list then co what's needed. its kinda gross but is what it is
  match old_type {
    BlockType::List(ListType::Number(start)) => match new_type {
      BlockType::List(ListType::Number(expected_count)) => {
        return *expected_count == start + count + 1;
      }
      _ => false,
    },
    BlockType::List(ListType::BracketedNumber(start)) => match new_type {
      BlockType::List(ListType::BracketedNumber(expected_count)) => {
        return *expected_count == start + count + 1;
      }
      _ => false,
    },
    _ => false,
  }
}

#[cfg(test)]
mod tests;
