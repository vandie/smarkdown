use super::{blocks::BlockType, line::Line, tokens_to_lines};
use crate::parse::Token;

pub fn should_recognise_blank_lines(block_type: BlockType) -> bool {
  match block_type {
    BlockType::List { .. } => false,
    BlockType::IndentedCodeBlock => false,
    _ => true,
  }
}

/// Some multi line blocks need to remove internal blank lines at the start and end
/// but need to allow them within the main body. This (rather inifficiant) method allows us to do so
pub fn trim_empty_lines(tokens: Vec<Token>) -> Vec<Token> {
  let mut lines: Vec<Line> = tokens_to_lines(&tokens);

  // get the position from the start
  let from_start = lines
    .iter()
    .position(|line| line.is_empty() == false)
    .unwrap_or(0);

  // Get the position from the end
  let from_end = lines.len()
    - lines
      .iter()
      .rev()
      .position(|line| line.is_empty() == false)
      .unwrap_or(0);

  // clip the lines
  lines = lines[from_start..from_end].to_vec();

  // Convert back into a straight token vec
  lines.iter_mut().fold(vec![], |mut col, line| {
    if col.len() > 0 {
      col.push(Token::NewLine);
    }
    col.append(&mut line.0);
    col
  })
}
