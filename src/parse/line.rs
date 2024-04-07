use super::blocks::{BlockType, ListType};
use crate::lex::{Bracket, Token, TokenType, VecNum};

const TAB_SIZE: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Line(pub Vec<Token>);
impl Line {
  // Returns true if the Line is empty
  pub fn is_empty(&self) -> bool {
    self.0.iter().fold(true, |empty, token| match token {
      Token::Space => empty,
      Token::Tab => empty,
      _ => false,
    })
  }

  /// Removes all indentation from a line while returning the indentation level the line was previously
  pub(crate) fn remove_all_indentation(&mut self) -> usize {
    let mut space_count: usize = 0;
    if self.is_empty() {
      self.0 = vec![];
      return 0;
    }

    for (i, token) in self.0.iter().enumerate() {
      match token {
        Token::Space => space_count += 1,
        Token::Tab => space_count += TAB_SIZE,
        _ => {
          self.0 = self.0[i..].to_vec();
          return space_count / TAB_SIZE;
        }
      }
    }

    // This shouldn't ever happen but the compiler can't seem to realise that
    space_count / TAB_SIZE
  }

  /// Counts the amount of leading spaces (up to 3) at the start of a line
  pub fn leading_spaces(&self) -> usize {
    let mut count = 0;
    while self.0.get(count) == Some(&Token::Space) && count < 4 {
      count += 1;
    }
    // Acording to spec, up to 3 leading spaces can be ignore
    if count == 4 {
      count = 0;
    }
    count
  }

  /// Gets the block type for a line so that it can be parsed
  pub fn line_type(&self, fallback: BlockType) -> BlockType {
    let overwritten_fallback = match fallback {
      BlockType::ThematicBreak => BlockType::Paragraph,
      _ => fallback,
    };

    let mut current: Vec<TokenType> = vec![];
    for token in self.0.iter() {
      current.push(token.clone().into());
      match current.as_slice() {
        [TokenType::Dash] => {
          if self.is_all(&TokenType::Dash) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::Underscore] => {
          if self.is_all(&TokenType::Underscore) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::Star] => {
          if self.is_all(&TokenType::Star) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::CloseBracket(Bracket::Angle), TokenType::Space] => {
          return BlockType::BlockQuote
        }
        [TokenType::Dash, TokenType::Space] => return BlockType::List(ListType::Dash),
        [TokenType::Plus, TokenType::Space] => return BlockType::List(ListType::Plus),
        [TokenType::Star, TokenType::Space] => return BlockType::List(ListType::Star),
        [TokenType::Number, TokenType::CloseBracket(Bracket::Parenthesis), TokenType::Space] => {
          return BlockType::List(ListType::BracketedNumber(grab_number(&self, 0).to_usize()))
        }
        [TokenType::Number, TokenType::Dot, TokenType::Space] => {
          return BlockType::List(ListType::Number(grab_number(&self, 0).to_usize()))
        }
        _ => {}
      }
    }

    overwritten_fallback
  }

  fn to_token_types(&self) -> Vec<TokenType> {
    self
      .0
      .iter()
      .map(|t| Into::<TokenType>::into(t.clone()))
      .collect()
  }

  /// Checks if a line is entirely a single token (with the exception of spaces and tabs)
  /// Only really needed for ThematicBreaks as spaces and tabs are allowed between the characters.
  fn is_all(&self, needle: &TokenType) -> bool {
    let tokens = self.to_token_types();
    let mut count = 0;
    tokens.iter().all(|token| {
      if token == needle {
        count += 1
      }
      return token == needle || token == &TokenType::Space || token == &TokenType::Tab;
    }) && count >= 3
  }

  /// Remove the starting chars that label a block as a given type
  pub fn remove_starting_chars(&mut self, line_type: &BlockType) {
    match line_type {
      BlockType::BlockQuote => {
        if self.0.get(self.leading_spaces()) == Some(&Token::CloseBracket(Bracket::Angle)) {
          self.trim_line_start(1);
        }
      }
      _ => {}
    }
  }

  /// Removed 1 level of indentation from a line (1 Tab or 4 Spaces)
  pub fn unindent(&mut self) {
    let indent = self.remove_all_indentation();
    if indent == 0 {
      return;
    }

    let mut new = vec![Token::Tab; indent - 1];
    new.append(&mut self.0);
    self.0 = new;
  }

  /// Removes a number of characters from the start of a line
  pub fn trim_line_start(&mut self, chars: usize) {
    self.0 = (self.0[chars..]).to_owned();
  }

  pub fn find_first(&self, needle: TokenType) -> Option<usize> {
    for (i, token) in self.0.iter().enumerate() {
      if Into::<TokenType>::into(token.clone()) == needle {
        return Some(i);
      }
    }
    None
  }
}

fn grab_number(line: &Line, index: usize) -> VecNum {
  match line.0.get(index) {
    Some(Token::Number(num)) => num.clone(),
    _ => VecNum(vec![0]),
  }
}
